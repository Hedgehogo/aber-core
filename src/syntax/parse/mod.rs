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

use super::{ctx::Ctx, error::Error};
use crate::node::{expr::Expr, wast::block::Block};
use chumsky::{
    combinator::MapWith, extra::ParserExtra, input::MapExtra, prelude::*, text::unicode::Graphemes,
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

pub fn parser<'input, X>(
) -> impl GraphemeParser<'input, Block<'input, X>, extra::Err<Error<'input>>> + Clone
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
            Ok(Block::new(
                vec![],
                CompExpr::from_vec(vec![]).into_spanned(0..0)
            )),
        );
    }
}
