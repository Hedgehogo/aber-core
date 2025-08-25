use super::super::{ctx::Ctx, error::Expected, whitespace::Side, Expr, ExprOp, Node, Whitespace};
use super::{whitespace::whitespace, GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
use crate::reprs::{wast::List, Spanned, SpannedVec};
use chumsky::prelude::*;

fn list<'input, N, P, E>(
    expr: P,
    open: (&'static str, Expected),
    close: (&'static str, Expected),
) -> impl GraphemeParser<'input, List<N::Expr, N::Expr>, E> + Clone
where
    N: Node,
    <N::Expr as Expr>::Whitespace: Whitespace<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let expected = open.1;
    let open = just(open.0).ignored();
    let comma = just(",").ignored().labelled(Expected::Comma);
    let close = just(close.0).ignored().labelled(close.1);
    let close = close.to(true).recover_with(via_parser(empty().to(false)));

    let item = whitespace()
        .then(expr)
        .map(|(whitespace, expr)| expr.whitespaced(whitespace, Side::Left))
        .then(whitespace())
        .map(|(expr, whitespace)| expr.whitespaced(whitespace, Side::Right))
        .map(ExprOp::into_spanned_expr);

    group((
        open.ignore_then(item.separated_by(comma).collect()),
        comma.ignore_then(whitespace()).or_not(),
        close,
    ))
    .map(|(items, whitespace, close)| List::new(items, whitespace, close))
    .labelled(expected)
}

pub fn tuple<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<N::Expr, N::Expr>, E> + Clone
where
    N: Node,
    <N::Expr as Expr>::Whitespace: Whitespace<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    list(expr, ("(", Expected::Tuple), (")", Expected::TupleClose))
}

pub fn generics<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<N::Expr, N::Expr>, E> + Clone
where
    N: Node,
    <N::Expr as Expr>::Whitespace: Whitespace<'input>,
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
    use crate::reprs::{
        span::{IntoSpanned, Span},
        wast::{wast_node::WastNode, Wast, Whitespace},
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(List::new(vec![], None, true)),
        );
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("('a')"))
                .into_result(),
            Ok(List::new(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()],
                None,
                true
            )),
        );
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("('a', )"))
                .into_result(),
            Ok(List::new(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()],
                Some(Whitespace::from_repr_unchecked(" ")),
                true
            )),
        );
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("('a' 'b')"))
                .into_result(),
            Ok(List::new(
                vec![vec![
                    Wast::Character(grapheme("a").into()).into_spanned_node(1..4),
                    Wast::Character(grapheme("b").into()).into_spanned_node(5..8)
                ]
                .into_spanned(1..8)],
                None,
                true
            )),
        );
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("('a', 'b')"))
                .into_result(),
            Ok(List::new(
                vec![
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec(),
                    Wast::Character(grapheme("b").into())
                        .into_spanned_node(6..9)
                        .into_spanned_vec(),
                ],
                None,
                true
            )),
        );
    }

    #[test]
    fn test_tuple_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
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
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("("))
                .into_output_errors(),
            (
                Some(List::new(vec![], None, false)),
                vec![Error::new(
                    smallvec![Expected::TupleClose, Expected::Comma, Expected::Expr],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            tuple(expr(fact::<WastNode, Extra>()))
                .parse(Graphemes::new("('a'"))
                .into_output_errors(),
            (
                Some(List::new(
                    vec![Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()],
                    None,
                    false
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
