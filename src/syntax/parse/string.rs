use super::super::error::{Error, Expected};
use super::GraphemeParser;
use crate::node::wast::string::String;
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

fn content<'input, P>(unit: P) -> impl GraphemeParser<'input, String, Error<'input>> + Copy
where
    P: GraphemeParser<'input, &'input str, Error<'input>> + Copy,
{
    empty()
        .map(|_| std::string::String::new())
        .foldl(unit.repeated(), |mut result, unit| {
            result.push_str(unit);
            result
        })
        .map(String::new)
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

pub fn string<'input>() -> impl GraphemeParser<'input, String, Error<'input>> + Copy {
    let escape = just("\\").map_err(|e: Error| e.replace_expected(Expected::StringEscape));

    let escaped = escape.ignore_then(escape_sequence());

    let unescaped = none_of("\\\"")
        .to_slice()
        .map(Graphemes::as_str)
        .map_err(|e: Error| e.replace_expected(Expected::StringUnescaped));

    let unit = unescaped.or(escaped);

    let recover_unit = newline().not().ignore_then(unit);

    quote(Expected::String)
        .ignore_then(
            just("\"\"")
                .not()
                .map_err(|e: Error| e.replace_expected(Expected::String)),
        )
        .ignore_then(
            content(unit)
                .then_ignore(quote(Expected::StringClose))
                .then_ignore(separator())
                .recover_with(via_parser(content(recover_unit))),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::Span;
    use indoc::indoc;
    use smallvec::smallvec;

    #[test]
    fn test_string() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        {
            let input = r#""Hello Aber!""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = r#""Hello Aber!"#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Err(vec![Error::new(
                    smallvec![
                        Expected::StringUnescaped,
                        Expected::StringEscape,
                        Expected::StringClose
                    ],
                    None,
                    Span::new(12..12)
                )])
            );
        }
        {
            let input = r#"Hello Aber!""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Err(vec![Error::new_expected(
                    Expected::String,
                    Some(grapheme("H")),
                    Span::new(0..1)
                )])
            );
        }
        {
            let input = r#""Hello Aber!\"""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!\"".into())
            );
        }
        {
            let input = r#""Hello Aber!\\""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!\\".into())
            );
        }
        {
            let input = r#""Hello Aber!\n""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!\n".into())
            );
        }
        {
            let input = r#""Hello Aber!\t""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!\t".into())
            );
        }
        {
            let input = indoc! {r#"
            "Hello Aber!\
            ""#};
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = r#""Hello Aber!\m""#;
            let result = string().parse(Graphemes::new(input));
            assert_eq!(result.output().cloned(), Some("Hello Aber!\u{FFFD}".into()));
            assert_eq!(
                result.into_errors(),
                vec![Error::new_expected(
                    Expected::StringEscaped,
                    Some(grapheme("m")),
                    Span::new(13..14)
                )]
            );
        }
        {
            let input = r#""Hello Aber!""""#;
            assert_eq!(
                string().parse(Graphemes::new(input)).into_result(),
                Err(vec![
                    Error::new_expected(
                        Expected::NonZeroWhitespace,
                        Some(grapheme("\"")),
                        Span::new(13..14)
                    ),
                    Error::new_expected(Expected::Eof, Some(grapheme("\"")), Span::new(13..14))
                ])
            );
        }
    }
}
