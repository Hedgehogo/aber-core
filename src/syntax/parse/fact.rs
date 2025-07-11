use super::super::{ctx::Ctx, error::Expected, Node};
use super::{
    block::block, call::call, character::character, escaped_string::escaped_string, expr::expr,
    list::tuple, number::number, raw_string::raw_string, spanned, whitespace::whitespace,
    GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use crate::node::{
    wast::{Pair, Wast},
    Spanned,
};
use chumsky::prelude::*;

pub fn fact<'input, N, E>() -> impl GraphemeParser<'input, Spanned<N>, E> + Clone
where
    N: Node<'input> + 'input,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
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
        ))
        .boxed();

        let pair_special = just(":")
            .then(just(":").not())
            .labelled(Expected::PairSpecial);

        spanned(choice)
            .map(Spanned::from)
            .then(whitespace().then_ignore(pair_special).or_not())
            .map_with(|(node, pair), extra| match pair {
                Some(whitespace) => Wast::Pair(Pair::new(Box::new(node), whitespace))
                    .into_spanned_node(extra.span()),

                None => node,
            })
            .labelled(Expected::Fact)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
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
    }

    #[test]
    fn test_fact_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            fact::<CompNode, Extra>()
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(Expected::Fact, None, Span::new(0..0))]
            )
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
                    smallvec![Expected::Fact],
                    Some(grapheme(":")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
