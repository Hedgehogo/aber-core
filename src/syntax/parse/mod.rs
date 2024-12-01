pub mod number;

use crate::node::Node;
use crate::node::Wast;
use chumsky::prelude::*;
use chumsky::text::{
    unicode::{Grapheme, Graphemes},
    Char,
};
use extra::ParserExtra;

type GraphemesExtra<'input> = extra::Err<Rich<'input, &'input Grapheme>>;

pub trait GraphemeParser<'input, O>:
    Parser<'input, &'input Graphemes, O, GraphemesExtra<'input>>
{
}

impl<'input, O, T> GraphemeParser<'input, O> for T where
    T: Parser<'input, &'input Graphemes, O, GraphemesExtra<'input>>
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
    use chumsky::util::Maybe;

    #[test]
    fn test_parser() {}
}
