use super::super::error::{Error, Expected};
use super::GraphemeParser;
use crate::node::string::{EscapedString, StringData};
use chumsky::prelude::*;
use extra::ParserExtra;
use text::{newline, Char, Grapheme, Graphemes};

fn quote<'input, E>(expected: Expected) -> impl Parser<'input, &'input Graphemes, (), E> + Copy
where
    E: ParserExtra<'input, &'input Graphemes, Error = Error<'input>>,
{
    just("\"")
        .map_err(move |e: Error| e.replace_expected(expected))
        .ignored()
}

fn escape_sequence<'input>() -> impl GraphemeParser<'input, &'input str, Error<'input>> + Copy {
    any()
        .try_map(|i: &Grapheme, span: SimpleSpan| {
            i.to_ascii()
                .and_then(|i| match i {
                    b'\"' => Some("\""),
                    b'\\' => Some("\\"),
                    b'n' => Some("\n"),
                    b't' => Some("\t"),
                    b'\n' => Some(""),
                    _ => None,
                })
                .ok_or_else(|| Error::new_expected(Expected::StringEscaped, Some(i), span.into()))
        })
        .map_err(|e: Error| e.replace_expected(Expected::StringEscaped))
        .recover_with(via_parser(any().or_not().map(|_| "\u{FFFD}")))
}

fn content<'input, O, P>(section: P) -> impl GraphemeParser<'input, O, Error<'input>> + Copy
where
    O: EscapedString<'input>,
    P: GraphemeParser<'input, &'input str, Error<'input>> + Copy,
{
    empty()
        .map(|_| O::Data::with_capacity(0))
        .foldl(section.repeated(), |data, section| {
            data.with_next_section(section)
        })
        .map_with(|data, e| unsafe {
            let inner_repr = e.slice().as_str();
            O::from_data_unchecked(data, inner_repr)
        })
}

pub fn separator<'input, E>() -> impl Parser<'input, &'input Graphemes, (), E> + Copy
where
    E: ParserExtra<'input, &'input Graphemes, Error = Error<'input>>,
{
    quote(Expected::String)
        .not()
        .map_err(|e: Error| e.replace_expected(Expected::NonZeroWhitespace))
        .recover_with(via_parser(empty()))
}

pub fn string<'input, O>() -> impl GraphemeParser<'input, O, Error<'input>> + Copy
where
    O: EscapedString<'input>,
{
    let escape_section = just("\\")
        .map_err(|e: Error| e.replace_expected(Expected::StringEscape))
        .ignore_then(escape_sequence());

    let unescaped_section = none_of("\\\"")
        .map_err(|e: Error| e.replace_expected(Expected::StringUnescaped))
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Graphemes::as_str);

    let section = unescaped_section.or(escape_section);

    let recover_section = newline().not().ignore_then(section);

    quote(Expected::String)
        .ignore_then(
            just("\"\"")
                .not()
                .map_err(|e: Error| e.replace_expected(Expected::String)),
        )
        .ignore_then(
            content(section)
                .then_ignore(quote(Expected::StringClose))
                .then_ignore(separator())
                .recover_with(via_parser(content(recover_section))),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::Span;
    use crate::node::wast;
    use indoc::indoc;
    use smallvec::smallvec;

    #[test]
    fn test_string() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        {
            let input = r#""Hello Aber!""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = r#""Hello Aber!"#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some("Hello Aber!".into()),
                    vec![Error::new(
                        smallvec![
                            Expected::StringUnescaped,
                            Expected::StringEscape,
                            Expected::StringClose
                        ],
                        None,
                        Span::new(12..12)
                    )]
                )
            );
        }
        {
            let input = r#"Hello Aber!""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    None,
                    vec![Error::new_expected(
                        Expected::String,
                        Some(grapheme("H")),
                        Span::new(0..1)
                    )]
                )
            );
        }
        {
            let input = r#""Hello Aber!\"""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!\"".into())
            );
        }
        {
            let input = r#""Hello Aber!\\""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!\\".into())
            );
        }
        {
            let input = r#""Hello Aber!\n""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!\n".into())
            );
        }
        {
            let input = r#""Hello Aber!\t""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!\t".into())
            );
        }
        {
            let input = indoc! {r#"
            "Hello Aber!\
            ""#};
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = r#""Hello Aber!\m""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some("Hello Aber!\u{FFFD}".into()),
                    vec![Error::new_expected(
                        Expected::StringEscaped,
                        Some(grapheme("m")),
                        Span::new(13..14)
                    )]
                )
            );
        }
        {
            let input = r#""Hello Aber!""""#;
            assert_eq!(
                string::<wast::String>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    None,
                    vec![
                        Error::new_expected(
                            Expected::NonZeroWhitespace,
                            Some(grapheme("\"")),
                            Span::new(13..14)
                        ),
                        Error::new_expected(Expected::Eof, Some(grapheme("\"")), Span::new(13..14))
                    ]
                )
            );
        }
    }
}
