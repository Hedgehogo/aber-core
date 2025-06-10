use super::{
    super::{
        ctx::Ctx,
        error::{Error, Expected},
    },
    whitespace::{line_separator, not_line_separator},
};
use super::{GraphemeParser, GraphemeParserExtra};
use crate::node::string::{EscapedString, StringData};
use chumsky::prelude::*;
use text::{Char, Grapheme, Graphemes};

fn quote<'input, E>(expected: Expected) -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    just("\"")
        .map_err(move |e: Error| e.replace_expected(expected))
        .ignored()
}

fn escape_sequence<'input, E>() -> impl GraphemeParser<'input, &'input str, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    line_separator()
        .map(|_| "")
        .or(any().try_map(|i: &Grapheme, span: SimpleSpan| {
            i.to_ascii()
                .and_then(|i| match i {
                    b'\"' => Some("\""),
                    b'\\' => Some("\\"),
                    b'n' => Some("\n"),
                    b't' => Some("\t"),
                    _ => None,
                })
                .ok_or_else(|| Error::new_expected(Expected::StringEscaped, Some(i), span.into()))
        }))
        .map_err(|e: Error| e.replace_expected(Expected::StringEscaped))
        .recover_with(via_parser(any().or_not().map(|_| "\u{FFFD}")))
}

fn content<'input, O, P, E>(section: P) -> impl GraphemeParser<'input, O, E> + Copy
where
    O: EscapedString<'input>,
    P: GraphemeParser<'input, &'input str, E> + Copy,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    empty()
        .map(|_| O::Data::with_capacity(0))
        .foldl(section.repeated(), |data, section| {
            data.with_next_section(section)
        })
        .map_with(|data, e| {
            let inner_repr = e.slice().as_str();
            O::from_data_unchecked(data, inner_repr)
        })
}

pub fn separator<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    quote(Expected::String)
        .not()
        .map_err(|e: Error| e.replace_expected(Expected::NonZeroWhitespace))
        .recover_with(via_parser(empty()))
}

pub fn escaped_string<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Copy
where
    O: EscapedString<'input>,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
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

    let recover_section = not_line_separator().ignore_then(section);

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

    use super::super::tests::Extra;
    use crate::node::{
        span::Span,
        string::{self, StringData},
        wast::{self, escaped_string::EscapedStringData},
    };
    use indoc::indoc;
    use smallvec::smallvec;

    fn new_string<'input>(
        capacity: usize,
        sections: Vec<&'input str>,
        inner_repr: &'input str,
    ) -> wast::String<'input> {
        wast::String::Escaped(string::EscapedStringSealed::from_data_unchecked(
            {
                let mut data = EscapedStringData::with_capacity(capacity);
                for section in sections {
                    data = data.with_next_section(section);
                }
                data
            },
            inner_repr,
        ))
    }

    #[test]
    fn test_escaped_string() {
        {
            let input = r#""Hello Aber!""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(11, vec!["Hello Aber!"], "Hello Aber!"))
            );
        }
        {
            let input = r#""Hello Aber!\"""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(
                    12,
                    vec!["Hello Aber!", "\""],
                    r#"Hello Aber!\""#
                ))
            );
        }
        {
            let input = r#""Hello Aber!\\""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(
                    12,
                    vec!["Hello Aber!", "\\"],
                    r#"Hello Aber!\\"#
                ))
            );
        }
        {
            let input = r#""Hello Aber!\n""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(
                    12,
                    vec!["Hello Aber!", "\n"],
                    r#"Hello Aber!\n"#
                ))
            );
        }
        {
            let input = r#""Hello Aber!\t""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(
                    12,
                    vec!["Hello Aber!", "\t"],
                    r#"Hello Aber!\t"#
                ))
            );
        }
        {
            let input = indoc! {r#"
            "Hello Aber!\
            ""#};
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(12, vec!["Hello Aber!", ""], "Hello Aber!\\\n"))
            );
        }
    }

    #[test]
    fn test_escaped_string_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        {
            let input = r#""Hello Aber!"#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(11, vec!["Hello Aber!"], "Hello Aber!")),
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
                escaped_string::<wast::String, Extra>()
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
            let input = r#""Hello Aber!\m""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(
                        12,
                        vec!["Hello Aber!", "\u{FFFD}"],
                        r#"Hello Aber!\m"#
                    )),
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
                escaped_string::<wast::String, Extra>()
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
