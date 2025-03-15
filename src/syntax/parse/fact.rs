use super::super::error::{Error, Expected};
use super::{
    block::block, call::call, character::character, expr::expr, list::tuple, number::number,
    raw_string::raw_string, spanned, string::string, whitespace::whitespace, GraphemeParser,
};
use crate::node::{wast::Wast, Node, Spanned};
use chumsky::prelude::*;

pub fn fact<'input, N>() -> impl GraphemeParser<'input, Spanned<N>, Error<'input>> + Clone
where
    N: Node<'input> + 'input,
{
    recursive(|fact| {
        let choice = choice((
            number().map(Wast::Number),
            character().map(Wast::Character),
            string().map(Wast::String),
            raw_string().map(Wast::String),
            call(expr(fact.clone())).map(Wast::Call),
            tuple(expr(fact.clone())).map(Wast::Tuple),
            block(expr(fact)).map(Wast::Block),
        ));

        let pair_special = just(":")
            .then(just(":").not())
            .map_err(|e: Error| e.replace_expected(Expected::PairSpecial));

        spanned(choice.map(N::from_wast))
            .map(Spanned::from)
            .then(whitespace().ignore_then(pair_special).or_not())
            .map_with(|(i, pair), extra| match pair {
                Some(_) => Wast::Pair(Box::new(i)).into_spanned_node(extra.span()),
                None => i,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use crate::node::{
        span::Span,
        wast::number::{Digits, Number, Radix},
        CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_fact() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        let digits = |s| unsafe { Digits::from_str_unchecked(s) };
        assert_eq!(
            fact::<CompNode>().parse(Graphemes::new("10")).into_result(),
            Ok(
                Wast::Number(Number::new(true, Radix::DECIMAL, digits("10"), None))
                    .into_spanned_node(0..2)
            )
        );
        assert_eq!(
            fact::<CompNode>()
                .parse(Graphemes::new("'m'"))
                .into_result(),
            Ok(Wast::Character(grapheme("m").into()).into_spanned_node(0..3))
        );
        assert_eq!(
            fact::<CompNode>()
                .parse(Graphemes::new("\"Hello\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_spanned_node(0..7))
        );
        assert_eq!(
            fact::<CompNode>()
                .parse(Graphemes::new("\"\"\"\nHello\n\"\"\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_spanned_node(0..13))
        );
        assert_eq!(
            fact::<CompNode>()
                .parse(Graphemes::new("'g:"))
                .into_output_errors(),
            (
                Some(
                    Wast::Pair(Box::new(
                        Wast::Character(grapheme("g").into()).into_spanned_node(0..2)
                    ))
                    .into_spanned_node(0..3)
                ),
                vec![Error::new_expected(
                    Expected::CharClose,
                    Some(grapheme(":")),
                    Span::new(2..3)
                )]
            )
        );
        assert_eq!(
            fact::<CompNode>()
                .parse(Graphemes::new(":"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Block,
                        Expected::Tuple,
                        Expected::Ident,
                    ],
                    Some(grapheme(":")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
