pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod content;
pub mod escaped_string;
pub mod expr;
pub mod fact;
pub mod list;
pub mod number;
pub mod raw_string;
pub mod whitespace;

use super::error::Error;
use crate::node::{expr::Expr, wast::block::Block};
use chumsky::{
    combinator::MapWith, error, extra::ParserExtra, input::MapExtra, prelude::*,
    text::unicode::Graphemes,
};
use content::content;
use expr::expr;
use fact::fact;

pub trait GraphemeParser<'input, O, E>:
    Parser<'input, &'input Graphemes, O, extra::Err<E>>
where
    E: error::Error<'input, &'input Graphemes> + 'input,
{
}

impl<'input, O, T, E> GraphemeParser<'input, O, E> for T
where
    T: Parser<'input, &'input Graphemes, O, extra::Err<E>>,
    E: error::Error<'input, &'input Graphemes> + 'input,
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

pub fn parser<'input, X>() -> impl GraphemeParser<'input, Block<'input, X>, Error<'input>> + Clone
where
    X: Expr<'input> + 'input,
{
    content(expr(fact::<X::Node>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{span::IntoSpanned, CompExpr};
    use text::Graphemes;

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
