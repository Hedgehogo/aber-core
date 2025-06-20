use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
    whitespace::Side,
    ExprOp, Node,
};
use super::{whitespace::whitespace, GraphemeParser, GraphemeParserExtra};
use crate::node::{wast::List, Spanned, SpannedVec};
use chumsky::prelude::*;

fn list<'input, N, P, E>(
    expr: P,
    open: (&'static str, Expected),
    close: (&'static str, Expected),
) -> impl GraphemeParser<'input, List<'input, N::Expr, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let open = just(open.0)
        .ignored()
        .map_err(move |e: Error| e.replace_expected(open.1));

    let comma = just(",")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Comma));

    let close = just(close.0)
        .ignored()
        .map_err(move |e: Error| e.replace_expected(close.1));

    let close = close.recover_with(via_parser(empty()));

    let item = whitespace()
        .then(expr)
        .map(|(whitespace, expr)| expr.whitespaced(whitespace, Side::Left));

    let separator = whitespace().then_ignore(comma);

    let repeat = item.then(separator.or_not()).map(|(item, whitespace)| {
        match whitespace {
            Some(whitespace) => item.whitespaced(whitespace, Side::Right),
            None => item,
        }
        .into_spanned_expr()
    });

    open.ignore_then(repeat.repeated().collect())
        .then(whitespace())
        .map(|(items, whitespace)| List::new(items, whitespace))
        .then_ignore(close)
}

pub fn tuple<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<'input, N::Expr, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    list(expr, ("(", Expected::Tuple), (")", Expected::TupleClose))
}

pub fn generics<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<'input, N::Expr, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    list(
        expr,
        ("[", Expected::Generics),
        ("]", Expected::GenericsClose),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::Wast,
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test() {
        use chumsky::{
            error::{Error, LabelError, Rich},
            extra,
            text::{whitespace, TextExpected},
            DefaultExpected,
        };

        fn tuple<'input>(
        ) -> impl Parser<'input, &'input str, (), extra::Err<Rich<'input, char, SimpleSpan>>>
        {
            just("a")
                .repeated()
                .then_ignore(whitespace())
                .separated_by(just(","))
                .then_ignore(just(")"))
        }

        assert_eq!(
            tuple().parse("a").into_output_errors(),
            (
                None,
                vec![Error::<&str>::merge(
                    LabelError::<&str, _>::expected_found(
                        vec![TextExpected::<&str>::Whitespace],
                        None,
                        SimpleSpan::new((), 1..1)
                    ),
                    LabelError::<&str, _>::expected_found(
                        vec![
                            DefaultExpected::Token('a'.into()),
                            DefaultExpected::Token(','.into()),
                            DefaultExpected::Token(')'.into()),
                        ],
                        None,
                        SimpleSpan::new((), 1..1)
                    )
                )]
            )
        );
    }

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(List::new(vec![], ())),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("('a')"))
                .into_result(),
            Ok(List::new(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)],
                ()
            )),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("('a', )"))
                .into_result(),
            Ok(List::new(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)],
                ()
            )),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("('a' 'b')"))
                .into_result(),
            Ok(List::new(
                vec![vec![
                    Wast::Character(grapheme("a").into()).into_spanned_node(1..4),
                    Wast::Character(grapheme("b").into()).into_spanned_node(5..8)
                ]
                .into_spanned(1..8)
                .map(CompExpr::from_vec)],
                ()
            )),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("('a', 'b')"))
                .into_result(),
            Ok(List::new(
                vec![
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    Wast::Character(grapheme("b").into())
                        .into_spanned_node(6..9)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                ],
                ()
            )),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Tuple],
                    None,
                    Span::new(0..0)
                )]
            )
        );
    }

    #[test]
    fn test_tuple_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("("))
                .into_output_errors(),
            (
                Some(List::new(vec![], ())),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Tuple,
                        Expected::TupleClose,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("('a'"))
                .into_output_errors(),
            (
                Some(List::new(
                    vec![Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)],
                    ()
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::PairSpecial,
                        Expected::Tuple,
                        Expected::TupleClose,
                        Expected::Block,
                        Expected::Comma,
                        Expected::Ident,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial
                    ],
                    None,
                    Span::new(4..4)
                )]
            )
        );
    }
}
