use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{
    block::block, call::call, character::character, escaped_string::escaped_string, expr::expr,
    list::tuple, number::number, raw_string::raw_string, spanned, whitespace::whitespace,
    GraphemeParser, GraphemeParserExtra,
};
use crate::node::{
    wast::{Pair, Wast},
    Node, Spanned,
};
use chumsky::prelude::*;

pub fn fact<'input, N, E>() -> impl GraphemeParser<'input, Spanned<N>, E> + Clone
where
    N: Node<'input> + 'input,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    recursive(|fact| {
        let choice = choice((
            number().map(Wast::Number).map(N::from_wast),
            character().map(Wast::Character).map(N::from_wast),
            escaped_string().map(Wast::String).map(N::from_wast),
            raw_string().map(Wast::String).map(N::from_wast),
            call(expr(fact.clone())).map(Wast::Call).map(N::from_wast),
            tuple(expr(fact.clone())).map(Wast::Tuple).map(N::from_wast),
            block(expr(fact)).map(Wast::Block).map(N::from_wast),
        ));

        let pair_special = just(":")
            .then(just(":").not())
            .map_err(|e: Error| e.replace_expected(Expected::PairSpecial));

        spanned(choice)
            .map(Spanned::from)
            .then(whitespace().then_ignore(pair_special).or_not())
            .map_with(|(node, pair), extra| match pair {
                Some(whitespace) => Wast::Pair(Pair::new(Box::new(node), whitespace))
                    .into_spanned_node(extra.span()),

                None => node,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::tests::Extra;
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
        let digits = |s| Digits::from_repr_unchecked(s);
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new("10"))
                .into_result(),
            Ok(
                Wast::Number(Number::new(true, Radix::DECIMAL, digits("10"), None))
                    .into_spanned_node(0..2)
            )
        );
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new("'m'"))
                .into_result(),
            Ok(Wast::Character(grapheme("m").into()).into_spanned_node(0..3))
        );
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new("\"Hello\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_spanned_node(0..7))
        );
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new("\"\"\"\nHello\n\"\"\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_spanned_node(0..13))
        );
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new("'g:"))
                .into_output_errors(),
            (
                Some(
                    Wast::Pair(Pair::new(
                        Box::new(Wast::Character(grapheme("g").into()).into_spanned_node(0..2)),
                        ()
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
            fact::<CompNode, Extra>()
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
