pub mod block;
pub mod call;
pub mod character;
pub mod content;
pub mod escaped_string;
pub mod expr;
pub mod fact;
pub mod initialization;
pub mod list;
pub mod number;
pub mod raw_string;
pub mod whitespace;

use super::{ctx::Ctx, error::Error, Expr};
use crate::node::wast::block::Content;
use chumsky::{
    combinator::MapWith, extra::ParserExtra, input::MapExtra, label::LabelError, prelude::*,
    text::unicode::Graphemes,
};
use content::content;
use expr::expr;
use fact::fact;

pub trait GraphemeParser<'input, O, E>: Parser<'input, &'input Graphemes, O, E>
where
    E: ParserExtra<'input, &'input Graphemes>,
{
}

impl<'input, O, T, E> GraphemeParser<'input, O, E> for T
where
    T: Parser<'input, &'input Graphemes, O, E>,
    E: ParserExtra<'input, &'input Graphemes> + 'input,
{
}

pub trait GraphemeParserExtra<'input>: ParserExtra<'input, &'input Graphemes> {}

impl<'input, T> GraphemeParserExtra<'input> for T where T: ParserExtra<'input, &'input Graphemes> {}

pub trait GraphemeLabelError<'input, L>: LabelError<'input, &'input Graphemes, L> {}

impl<'input, T, L> GraphemeLabelError<'input, L> for T where
    T: LabelError<'input, &'input Graphemes, L>
{
}

#[expect(clippy::type_complexity)]
pub fn spanned<'src, P, I, O, E>(
    parser: P,
) -> MapWith<P, O, impl Fn(O, &mut MapExtra<'src, '_, I, E>) -> (O, I::Span) + Copy>
where
    P: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: ParserExtra<'src, I>,
{
    parser.map_with(|i, e| (i, e.span()))
}

pub fn entirely<'src, P, O, E, L>(parser: P, label: L) -> impl GraphemeParser<'src, O, E> + Copy
where
    P: GraphemeParser<'src, O, E> + Copy,
    E: GraphemeParserExtra<'src>,
    E::Error: GraphemeLabelError<'src, L>,
    L: Copy,
{
    custom(move |inp| {
        let found = inp.peek_maybe();
        let span = inp.span_since(&inp.cursor());
        let res = inp.parse(parser);

        match res {
            Ok(out) => Ok(out),
            Err(_) => Err(E::Error::expected_found([label], found, span)),
        }
    })
}

pub struct Cursor(usize);

pub fn end_cursor<'src, P, O, E>(parser: P) -> impl GraphemeParser<'src, Cursor, E> + Clone
where
    P: GraphemeParser<'src, O, E> + Clone,
    E: GraphemeParserExtra<'src>,
{
    custom(move |i| {
        let result = i.parse(&parser);
        let cursor = i.cursor();
        let cursor: &usize = cursor.inner();
        result.map(|_| Cursor(*cursor))
    })
}

pub fn end_cursor_slice<'src, P, E>(
    parser: P,
) -> impl GraphemeParser<'src, &'src Graphemes, E> + Clone
where
    P: GraphemeParser<'src, Cursor, E> + Clone,
    E: GraphemeParserExtra<'src>,
{
    custom(move |i| {
        let start_cursor = i.cursor();
        let start: &usize = start_cursor.inner();
        let start: usize = *start;
        i.parse(&parser).map(|end| {
            let Cursor(end) = end;

            let result = if end > start {
                let length = end - start;
                let slice = i.slice_from(&start_cursor..).as_str();
                let (slice, _) = slice.split_at(length);
                slice
            } else {
                ""
            };

            Graphemes::new(result)
        })
    })
}

pub fn parser<'input, X>(
) -> impl GraphemeParser<'input, Content<'input, X>, extra::Err<Error<'input>>> + Clone
where
    X: Expr<'input> + 'input,
{
    content(expr(fact::<X::Node, _>()))
        .with_ctx(Ctx::default())
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{span::IntoSpanned, CompExpr};
    use extra::Full;
    use text::Graphemes;

    pub type Extra = Full<Error<'static>, (), Ctx<()>>;

    #[test]
    fn test_parser() {
        assert_eq!(
            parser::<CompExpr>().parse(Graphemes::new("")).into_result(),
            Ok(Content::new(
                vec![],
                CompExpr::from_vec(vec![]).into_spanned(0..0)
            )),
        );
    }
}
