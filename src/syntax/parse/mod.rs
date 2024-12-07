pub mod number;

use crate::node::Node;
use crate::node::Wast;
use chumsky::error::Error;
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

pub fn spanned<'src, P, I, O, E>(parser: P) -> impl Parser<'src, I, (O, I::Span), E>
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
