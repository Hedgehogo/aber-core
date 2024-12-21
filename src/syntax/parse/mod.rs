pub mod character;
pub mod expression;
pub mod meaningful_unit;
pub mod number;
pub mod raw_string;
pub mod string;
pub mod list;
pub mod whitespace;

use crate::node::Node;
use crate::node::Wast;
use chumsky::combinator::MapWith;
use chumsky::error::Error;
use chumsky::input::MapExtra;
use chumsky::prelude::*;
use chumsky::text::{
    unicode::{Grapheme, Graphemes},
    Char,
};
use extra::ParserExtra;

pub trait GraphemeParser<'input, O, E>:
    Parser<'input, &'input Graphemes, O, extra::Err<E>>
where
    E: Error<'input, &'input Graphemes> + 'input,
{
}

impl<'input, O, T, E> GraphemeParser<'input, O, E> for T
where
    T: Parser<'input, &'input Graphemes, O, extra::Err<E>>,
    E: Error<'input, &'input Graphemes> + 'input,
{
}

#[allow(clippy::type_complexity)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {}
}
