use super::super::{
    ctx::Ctx,
    error::Expected,
    string::{EscapedString, StringData},
};
use super::{
    // Don't use it when the error shown in the test
    // `syntax::parse::escaped_string::tests::test` is fixed.
    whitespace::{line_break, line_start},
    whitespace::{line_separator, not_line_separator},
    GraphemeLabelError,
    GraphemeParser,
    GraphemeParserExtra,
    // Use it when the error shown in the test
    // `syntax::parse::escaped_string::tests::test` is fixed.
    //
    // end_cursor, end_cursor_slice,
    // whitespace::line_separator_cursor,
};
use crate::reprs::wast::escaped_string::{Escape, Section};
use chumsky::{label::LabelError, prelude::*};
use text::{Grapheme, Graphemes};

fn quote<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input>,
{
    just("\"").ignored()
}

pub fn section<'input, E>() -> impl GraphemeParser<'input, Section<'input>, E> + Clone
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let line_break_section = line_separator()
        .labelled(Expected::StringUnescaped)
        .map(Grapheme::as_str)
        .map(Section::Characters);

    let characters_section = not_line_separator()
        .then(none_of("\\\""))
        .labelled(Expected::StringUnescaped)
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Graphemes::as_str)
        .map(Section::Characters);

    // Use it when the error shown in the test
    // `syntax::parse::escaped_string::tests::test` is fixed.
    //
    // let escape_section = end_cursor_slice(just("\\").labelled(Expected::StringEscape).ignore_then(
    //     line_separator_cursor().or(not_line_separator().ignore_then(end_cursor(any().or_not()))),
    // ))
    // .map(Graphemes::as_str)
    // .map(Section::Escape);

    let escape = just("\\").labelled(Expected::StringEscape);
    let other_escape = escape.then(any().or_not()).to_slice();
    let line_break_escape = escape
        .then(line_break())
        .to_slice()
        .then_ignore(line_start());
    let escape_section = line_break_escape
        .or(other_escape)
        .map(Graphemes::as_str)
        .map(Section::Escape);

    choice((line_break_section, characters_section, escape_section))
}

fn content<'input, O, P, E>(section: P) -> impl GraphemeParser<'input, O, E> + Clone
where
    O: EscapedString<'input>,
    P: GraphemeParser<'input, &'input str, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    empty()
        .map(|_| O::Data::with_capacity(0))
        .foldl(section.repeated(), |data, section| {
            data.with_next_section(section)
        })
        .map_with(|data, e| {
            let inner_repr = e.slice().as_str();
            O::from_data_unchecked(data, inner_repr, e.ctx())
        })
}

pub fn separator<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    quote()
        .not()
        .labelled(Expected::NonZeroWhitespace)
        .recover_with(via_parser(empty()))
}

pub fn escaped_string<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Clone
where
    O: EscapedString<'input>,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let section = section().validate(|section, extra, emitter| match section {
        Section::Escape(repr) => match Escape::from_repr(repr) {
            Some(escape) => match escape {
                Escape::Quote => "\"",
                Escape::Slash => "\\",
                Escape::Newline => "\n",
                Escape::Tab => "\t",
                Escape::Nothing => "",
            },

            None => {
                let found = Graphemes::new(repr).iter().nth(1).map(Into::into);
                let span = ((extra.span().start() + 1)..extra.span().end()).into();
                let mut error = E::Error::expected_found([Expected::StringEscaped], found, span);
                error.in_context(Expected::StringEscape, extra.span());
                emitter.emit(error);
                repr
            }
        },

        Section::Characters(repr) => repr,
    });

    let recover_section = not_line_separator().ignore_then(section.clone());

    quote()
        .ignore_then(just("\"\"").not())
        .ignore_then(
            content(section)
                .then_ignore(quote().labelled(Expected::StringClose))
                .then_ignore(separator())
                .recover_with(via_parser(content(recover_section))),
        )
        .labelled(Expected::String)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::{
        error::Error,
        string::{self, StringData},
    };
    use super::super::tests::Extra;
    use crate::reprs::{
        span::Span,
        wast::{self, escaped_string},
    };
    use indoc::indoc;
    use smallvec::smallvec;

    #[test]
    fn test() {
        use chumsky::{extra::Err, prelude::*, text::newline};

        fn parser<'input>() -> impl Parser<'input, &'input str, &'input str, Err<EmptyErr>> {
            custom(|i| {
                let end_cursor = custom(|i| {
                    let result = i.parse(newline());
                    let cursor = i.cursor();
                    let cursor: &usize = cursor.inner();
                    let cursor: usize = cursor.clone();
                    result.map(|_| cursor)
                });

                let start_cursor = i.cursor();
                let start: &usize = start_cursor.inner();
                let start: usize = start.clone();
                i.parse(end_cursor).map(|end| {
                    if end > start {
                        let length = end - start;
                        let slice: &str = i.slice_from(&start_cursor..);
                        let (slice, _) = slice.split_at(length);
                        slice
                    } else {
                        ""
                    }
                })
            })
        }

        assert_eq!(parser().parse("n").into_output(), None);
    }

    fn new_string<'input>(
        sections: Vec<&'input str>,
        inner_repr: &'input str,
    ) -> wast::String<'input> {
        wast::String::Escaped(string::EscapedStringSealed::from_data_unchecked(
            {
                let mut data: escaped_string::EscapedStringData = StringData::with_capacity(0);
                for section in sections {
                    data = data.with_next_section(section);
                }
                data
            },
            inner_repr,
            &Ctx::default(),
        ))
    }

    #[test]
    fn test_escaped_string() {
        {
            let input = r#""Hello Aber!""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (Some(new_string(vec!["Hello Aber!"], "Hello Aber!")), vec![])
            );
        }
        {
            let input = r#""Hello Aber!\"""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(vec!["Hello Aber!", "\""], r#"Hello Aber!\""#)),
                    vec![]
                )
            );
        }
        {
            let input = r#""Hello Aber!\\""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(vec!["Hello Aber!", "\\"], r#"Hello Aber!\\"#)),
                    vec![]
                )
            );
        }
        {
            let input = r#""Hello Aber!\n""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(vec!["Hello Aber!", "\n"], r#"Hello Aber!\n"#)),
                    vec![]
                )
            );
        }
        {
            let input = r#""Hello Aber!\t""#;
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(vec!["Hello Aber!", "\t"], r#"Hello Aber!\t"#)),
                    vec![]
                )
            );
        }
        {
            let input = indoc! {r#"
            "Hello Aber!\
            ""#};
            assert_eq!(
                escaped_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    Some(new_string(vec!["Hello Aber!", ""], "Hello Aber!\\\n")),
                    vec![]
                )
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
                    Some(new_string(vec!["Hello Aber!"], "Hello Aber!")),
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
                    Some(new_string(vec!["Hello Aber!", "\\m"], r#"Hello Aber!\m"#)),
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
