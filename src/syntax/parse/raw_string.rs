use crate::syntax::ctx::DocCtx;

use super::super::{
    ctx::Ctx,
    error::Expected,
    string::{RawString, RawStringCtx, StringData},
};
use super::{
    end_cursor_slice,
    escaped_string::separator,
    whitespace::{inline_whitespace, line_separator, line_separator_cursor, not_line_separator},
    GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use chumsky::{combinator::Repeated, prelude::*, text::Graphemes};

/// Context required for parsing the raw string after the opening
/// sequence.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RawCtx<C> {
    pub quotes_count: usize,
    pub additional: C,
}

impl<C: Default> Default for RawCtx<C> {
    fn default() -> Self {
        Self {
            quotes_count: 3,
            additional: C::default(),
        }
    }
}

/// Context required when parsing the inner meaningful part of the
/// string.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct RawContentCtx<'input> {
    pub capacity: usize,
    pub line_break_count: usize,
    pub indent: &'input str,
}

impl<'input> RawStringCtx<'input> {
    pub fn new_raw(
        doc_ctx: DocCtx,
        quotes_count: usize,
        capacity: usize,
        line_break_count: usize,
        indent: &'input str,
    ) -> Self {
        Self {
            doc_ctx,
            additional: RawCtx {
                quotes_count,
                additional: RawContentCtx {
                    capacity,
                    line_break_count,
                    indent,
                },
            },
        }
    }
}

/// Content information that can be retrieved during the first pass.
///
/// `precapacity` is equal to: real content length + indentation
/// length * number of lines (Values are given in bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawContentInfo {
    precapacity: usize,
    line_break_count: usize,
}

/// Creates a parser that parses some number of opening quotes.
fn quotes<'input, E>(
) -> Repeated<impl GraphemeParser<'input, (), E> + Copy, (), &'input Graphemes, E>
where
    E: GraphemeParserExtra<'input>,
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
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    quotes().at_least(3).count().then_ignore(line_separator())
}

/// Creates a parser that parses the indentation and then the closing
/// sequence of the raw string.
///
/// The parser returns indentation (sequence W in the specification).
fn close<'input, E, C>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<RawCtx<C>>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    inline_whitespace().to_slice().then_ignore(
        quotes()
            .configure(|cfg, ctx: &Ctx<RawCtx<C>>| cfg.exactly(ctx.additional.quotes_count))
            .labelled(Expected::RawStringClose)
            .as_context(),
    )
}

/// Creates a parser that parses one significant line of a raw
/// string, not including line breaks.
///
/// The parser returns this line (sequence C in the specification).
fn preline<'input, E, C>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<RawCtx<C>>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    close().not().ignore_then(
        not_line_separator()
            .ignore_then(any().labelled(Expected::RawStringUnit))
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
    E: GraphemeParserExtra<'input, Context = Ctx<RawCtx<()>>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let repeated = line_separator().then(preline()).repeated();

    preline()
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
    E: GraphemeParserExtra<'input, Context = RawStringCtx<'input>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    just("")
        .configure(|cfg, ctx: &RawStringCtx<'input>| cfg.seq(ctx.additional.additional.indent))
        .ignored()
        .labelled(Expected::RawStringIndent)
        .as_context()
}

/// Creates a parser that parses a line with a line break ending it
/// and an indentation.
pub fn line<'input, E>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Clone
where
    E: GraphemeParserExtra<'input, Context = RawStringCtx<'input>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let repeated = not_line_separator().ignore_then(any().labelled(Expected::RawStringUnit));
    let line_content = repeated.repeated().ignore_then(line_separator_cursor());
    indent().ignore_then(end_cursor_slice(line_content))
}

/// Creates a parser that parses a line without a line break ending
/// it and with an indentation.
pub fn last_line<'input, E>() -> impl GraphemeParser<'input, &'input Graphemes, E> + Clone
where
    E: GraphemeParserExtra<'input, Context = RawStringCtx<'input>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let repeated = not_line_separator().ignore_then(any().labelled(Expected::RawStringUnit));
    let line_content = repeated.repeated();
    indent().ignore_then(line_content.to_slice())
}

/// Creates a parser that parses the inner meaningful part of a string.
///
/// The inner meaningful part of the string is what is after the opening sequence
/// and the last line break of the raw string.
///
/// The parser returns the contents of the string.
fn content<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Clone
where
    O: StringData<'input>,
    E: GraphemeParserExtra<'input, Context = RawStringCtx<'input>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    empty()
        .map_with(|_, extra| {
            let ctx: &RawStringCtx<'input> = extra.ctx();
            O::with_capacity(ctx.additional.additional.capacity)
        })
        .foldl(
            line()
                .repeated()
                .configure(|cfg, ctx: &RawStringCtx<'input>| {
                    cfg.exactly(ctx.additional.additional.line_break_count)
                }),
            |data, line| data.with_next_section(line.as_str()),
        )
        .then(last_line())
        .map(|(data, line)| data.with_next_section(line.as_str()))
}

/// Creates a parser that parses the raw string.
pub fn raw_string<'input, O, E>() -> impl GraphemeParser<'input, O, E> + Clone
where
    O: RawString<'input>,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let prerest = precontent()
        .then_ignore(line_separator())
        .then(close());

    let rest = content()
        .map_with(|data, e| (data, e.slice()))
        .then_ignore(line_separator())
        .then_ignore(close())
        .then_ignore(separator());

    open()
        .map_with(|quotes_count, extra| {
            let ctx: &Ctx<()> = extra.ctx();
            Ctx {
                doc_ctx: ctx.doc_ctx.clone(),
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
        .map(|(ctx, (data, inner_repr))| O::from_data_unchecked(data, inner_repr.as_str(), &ctx))
        .labelled(Expected::RawString)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::{error::Error, string};
    use super::super::tests::Extra;
    use crate::node::{
        span::Span,
        wast::{self},
    };
    use indoc::indoc;
    use smallvec::smallvec;

    fn new_string<'input>(
        inner_repr: &'input str,
        quotes_count: usize,
        capacity: usize,
        line_break_count: usize,
        indent: &'input str,
    ) -> wast::String<'input> {
        wast::String::Raw(string::RawStringSealed::from_data_unchecked(
            (),
            inner_repr,
            &Ctx::new_raw(
                Default::default(),
                quotes_count,
                capacity,
                line_break_count,
                indent,
            ),
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
                Ok(new_string("Hello Aber!", 3, 11, 0, ""))
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
                Ok(new_string("  Hello Aber!", 3, 11, 0, "  "))
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
                Ok(new_string("  Hello Aber!", 3, 12, 0, " ",))
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
                Ok(new_string("  Hello Aber!", 4, 12, 0, " ",))
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
                    vec![Error::new(
                        smallvec![Expected::RawStringUnit],
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
