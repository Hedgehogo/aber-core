use super::super::{ctx::Ctx, error::Expected, whitespace::Side, ExprOp, Node};
use super::{whitespace::whitespace, GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
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
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let expected = open.1;
    let open = just(open.0).ignored();
    let comma = just(",").ignored().labelled(Expected::Comma);
    let close = just(close.0).ignored().labelled(close.1);
    let close = close.recover_with(via_parser(empty()));

    let item = whitespace()
        .then(expr)
        .map(|(whitespace, expr)| expr.whitespaced(whitespace, Side::Left))
        .then(whitespace())
        .map(|(expr, whitespace)| expr.whitespaced(whitespace, Side::Right))
        .map(ExprOp::into_spanned_expr);

    open.ignore_then(item.separated_by(comma).collect())
        .then(comma.ignore_then(whitespace()).or_not())
        .map(|(items, whitespace)| List::new(items, whitespace))
        .then_ignore(close)
        .labelled(expected)
}

pub fn tuple<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<'input, N::Expr, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    list(expr, ("(", Expected::Tuple), (")", Expected::TupleClose))
}

pub fn generics<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<'input, N::Expr, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
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

    use super::super::super::error::Error;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::Wast,
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(List::new(vec![], None)),
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
                None
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
                Some(())
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
                None
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
                None
            )),
        );
    }

    #[test]
    fn test_tuple_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
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
        assert_eq!(
            tuple(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("("))
                .into_output_errors(),
            (
                Some(List::new(vec![], None)),
                vec![Error::new(
                    smallvec![Expected::TupleClose, Expected::Comma, Expected::Expr],
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
                    None
                )),
                vec![Error::new(
                    smallvec![
                        Expected::TupleClose,
                        Expected::Initialization,
                        Expected::Comma,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                        Expected::Fact,
                    ],
                    None,
                    Span::new(4..4)
                )]
            )
        );
    }
}
