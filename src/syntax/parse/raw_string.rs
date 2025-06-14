use super::super::{
    ctx::Ctx,
    error::{Error, Expected},string::{RawString, StringData}
};
use super::{
    escaped_string::separator,
    whitespace::{line_separator, not_line_separator},
    GraphemeParser, GraphemeParserExtra,
};
use chumsky::{
    combinator::Repeated,
    prelude::*,
    text::{inline_whitespace, Graphemes},
};

/// Context required for parsing the raw string after the opening
/// sequence.
#[derive(Clone, Copy)]
struct RawCtx<C> {
    quotes_count: usize,
    additional: C,
}

/// Context required when parsing the inner meaningful part of the
/// string.
#[derive(Clone, Copy)]
struct RawContentCtx<'input> {
    capacity: usize,
    line_break_count: usize,
    indent: &'input str,
}

/// Content information that can be retrieved during the first pass.
///
/// `precapacity` is equal to: real content length + indentation
/// length * number of lines (Values are given in bytes).
#[derive(Clone, Copy)]
struct RawContentInfo {
    precapacity: usize,
    line_break_count: usize,
}

/// Creates a parser that parses some number of opening quotes.
fn quotes<'input, E>(
) -> Repeated<impl GraphemeParser<'input, (), E> + Copy, (), &'input Graphemes, E>
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    just("\"").ignored().repeated()
}

/// Creates a parser that parses the opening sequence of a raw
/// string.
///
/// The opening sequence includes a line break.
///
/// The parser returns the number of quotes in the opening sequence
/// (minimum 3).
fn open<'input, E, C>() -> impl GraphemeParser<'input, usize, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<C>>,
{
    quotes()
        .at_least(3)
        .count()
        .then_ignore(line_separator())
        .map_err(|e: Error| e.replace_expected(Expected::RawString))
}

/// Creates a parser that parses the indentation and then the closing
/// sequence of the raw string.
///
/// The parser returns indentation (sequence W in the specification).
fn close<'input, E, C>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<RawCtx<C>>>,
{
    inline_whitespace().to_slice().then_ignore(
        quotes()
            .configure(|cfg, ctx: &Ctx<RawCtx<C>>| cfg.exactly(ctx.additional.quotes_count))
            .map_err(|e: Error| e.replace_expected(Expected::RawStringClose)),
    )
}

/// Creates a parser that parses one significant line of a raw
/// string, not including line breaks.
///
/// The parser returns this line (sequence C in the specification).
fn line<'input, E, C>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<RawCtx<C>>>,
{
    close().not().ignore_then(
        not_line_separator()
            .ignore_then(any())
            .repeated()
            .to_slice(),
    )
}

/// Creates a parser that parses the inner meaningful part of a
/// string without indentation data.
///
/// The inner meaningful part of the string is what is after the
/// opening sequence and the last line break of the raw string.
///
/// The parser returns content information.
fn precontent<'input, E>() -> impl GraphemeParser<'input, RawContentInfo, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<RawCtx<()>>>,
{
    let repeated = line_separator().then(line()).repeated();

    line()
        .map(|i| RawContentInfo {
            precapacity: i.as_bytes().len(),
            line_break_count: 0,
        })
        .foldl(repeated, |mut info, (newline, line)| {
            info.precapacity += newline.as_bytes().len() + line.as_bytes().len();
            info.line_break_count += 1;
            info
        })
}

/// Creates a parser that parses the indentation (sequence W in the
/// specification).
fn indent<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<
        'input,
        Error = Error<'input>,
        Context = Ctx<RawCtx<RawContentCtx<'input>>>,
    >,
{
    just("")
        .configure(|cfg, ctx: &Ctx<RawCtx<RawContentCtx<'input>>>| {
            cfg.seq(ctx.additional.additional.indent)
        })
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::RawStringIndent))
}

/// Creates a parser that parses the inner meaningful part of a string.
///
/// The inner meaningful part of the string is what is after the opening sequence
/// and the last line break of the raw string.
///
/// The parser returns the contents of the string.
fn content<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Copy
where
    O: StringData<'input>,
    E: GraphemeParserExtra<
        'input,
        Error = Error<'input>,
        Context = Ctx<RawCtx<RawContentCtx<'input>>>,
    >,
{
    let line = indent().ignore_then(line());

    let repeated = line_separator().then(line).repeated();

    line.map_with(|first: &Graphemes, e| {
        let ctx: &Ctx<RawCtx<RawContentCtx<'input>>> = e.ctx();
        O::with_capacity(ctx.additional.additional.capacity).with_next_section(first.as_str())
    })
    .foldl(
        repeated.configure(|cfg, ctx: &Ctx<RawCtx<RawContentCtx<'input>>>| {
            cfg.exactly(ctx.additional.additional.line_break_count)
        }),
        |data, (newline, line)| {
            data.with_next_section(newline.as_str())
                .with_next_section(line.as_str())
        },
    )
}

/// Creates a parser that parses the raw string.
pub fn raw_string<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Copy
where
    O: RawString<'input>,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let prerest = precontent()
        .then_ignore(line_separator())
        .then(close())
        .then_ignore(separator());

    let rest = content()
        .map_with(|data, e| (data, e.slice()))
        .then_ignore(line_separator())
        .then_ignore(close());

    open()
        .map_with(|quotes_count, extra| {
            let ctx: &Ctx<()> = extra.ctx();
            Ctx {
                doc_ctx: ctx.doc_ctx,
                additional: RawCtx {
                    quotes_count,
                    additional: (),
                },
            }
        })
        .then_with_ctx(prerest.rewind())
        .map(|(ctx, (info, indent))| Ctx {
            doc_ctx: ctx.doc_ctx,
            additional: RawCtx {
                quotes_count: ctx.additional.quotes_count,
                additional: RawContentCtx {
                    capacity: {
                        let line_count = info.line_break_count + 1;
                        let indent_capacity = line_count * indent.as_bytes().len();
                        info.precapacity.saturating_sub(indent_capacity)
                    },
                    line_break_count: info.line_break_count,
                    indent: indent.as_str(),
                },
            },
        })
        .then_with_ctx(rest)
        .map(|(ctx, (data, inner_repr))| {
            O::from_data_unchecked(data, ctx.additional.additional.indent, inner_repr.as_str())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::string::{self, StringData};
    use super::super::tests::Extra;
    use crate::node::{
        span::Span,
        wast::{self, raw_string::RawStringData},
    };
    use indoc::indoc;

    fn new_string<'input>(
        capacity: usize,
        sections: Vec<&'input str>,
        indent: &'input str,
        inner_repr: &'input str,
    ) -> wast::String<'input> {
        wast::String::Raw(string::RawStringSealed::from_data_unchecked(
            {
                let mut data = RawStringData::with_capacity(capacity);
                for section in sections {
                    data = data.with_next_section(section);
                }
                data
            },
            indent,
            inner_repr,
        ))
    }

    #[test]
    fn test_raw_string() {
        {
            let input = indoc! {r#"
                """
                Hello Aber!
                """"#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(11, vec!["Hello Aber!"], "", "Hello Aber!"))
            );
        }
        {
            let input = indoc! {r#"
                """
                  Hello Aber!
                  """"#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(11, vec!["Hello Aber!"], "  ", "  Hello Aber!"))
            );
        }
        {
            let input = indoc! {r#"
                """
                  Hello Aber!
                 """"#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(12, vec![" Hello Aber!"], " ", "  Hello Aber!"))
            );
        }
        {
            let input = indoc! {r#"
                """"
                  Hello Aber!
                 """""#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_result(),
                Ok(new_string(12, vec![" Hello Aber!"], " ", "  Hello Aber!"))
            );
        }
    }

    #[test]
    fn test_raw_string_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        {
            let input = indoc! {r#"
                """
                 Hello Aber!
                  """"#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    None,
                    vec![Error::new_expected(
                        Expected::RawStringIndent,
                        Some(grapheme("H")),
                        Span::new(5..6)
                    )]
                )
            );
        }
        {
            let input = indoc! {r#"
                """"
                  Hello Aber!
                 """"#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    None,
                    vec![Error::new_expected(
                        Expected::RawStringClose,
                        None,
                        Span::new(23..23)
                    )]
                )
            );
        }
        {
            let input = indoc! {r#"
                """
                  Hello Aber!
                 """"""#};
            assert_eq!(
                raw_string::<wast::String, Extra>()
                    .parse(Graphemes::new(input))
                    .into_output_errors(),
                (
                    None,
                    vec![
                        Error::new_expected(
                            Expected::NonZeroWhitespace,
                            Some(grapheme("\"")),
                            Span::new(22..23)
                        ),
                        Error::new_expected(Expected::Eof, Some(grapheme("\"")), Span::new(22..23))
                    ]
                )
            );
        }
    }
}
