use super::super::error::{Error, Expected};
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::{Expr, ExprVec, Spanned};
use chumsky::prelude::*;

fn list<'input, X, P>(
    expr: P,
    open: (&'static str, Expected),
    close: (&'static str, Expected),
) -> impl GraphemeParser<'input, ExprVec<'input, X>, Error<'input>> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, Error<'input>> + Clone,
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
    let item = expr.then_ignore(whitespace(0));
    let separator = comma.then_ignore(whitespace(0));

    open.ignore_then(whitespace(0))
        .ignore_then(item.separated_by(separator).allow_trailing().collect())
        .then_ignore(close)
}

pub fn tuple<'input, X, P>(
    expr: P,
) -> impl GraphemeParser<'input, ExprVec<'input, X>, Error<'input>> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, Error<'input>> + Clone,
{
    list::<X, _>(expr, ("(", Expected::Tuple), (")", Expected::TupleClose))
}

pub fn generics<'input, X, P>(
    expr: P,
) -> impl GraphemeParser<'input, ExprVec<'input, X>, Error<'input>> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, Error<'input>> + Clone,
{
    list::<X, _>(
        expr,
        ("[", Expected::Generics),
        ("]", Expected::GenericsClose),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::Wast,
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test() {
        use crate::node::wast::character::Ascii;
        use text::whitespace;

        fn tuple<'input>() -> impl GraphemeParser<'input, (), Error<'input>> + Clone {
            just("a")
                .repeated()
                .then_ignore(whitespace())
                .separated_by(just(","))
                .then_ignore(just(")"))
        }

        assert_eq!(
            tuple().parse(Graphemes::new("a")).into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Ascii(Ascii::new(b')').unwrap()),
                        Expected::Ascii(Ascii::new(b',').unwrap()),
                        Expected::Ascii(Ascii::new(b'a').unwrap()),
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
    }

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(vec![]),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("("))
                .into_output_errors(),
            (
                Some(vec![]),
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
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("('a')"))
                .into_result(),
            Ok(vec![Wast::Character(grapheme("a").into())
                .into_spanned_node(1..4)
                .into_spanned_vec()
                .map(CompExpr::from_vec)]),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("('a'"))
                .into_output_errors(),
            (
                Some(vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)]),
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
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("('a', )"))
                .into_result(),
            Ok(vec![Wast::Character(grapheme("a").into())
                .into_spanned_node(1..4)
                .into_spanned_vec()
                .map(CompExpr::from_vec)]),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("('a' 'b')"))
                .into_result(),
            Ok(vec![vec![
                Wast::Character(grapheme("a").into()).into_spanned_node(1..4),
                Wast::Character(grapheme("b").into()).into_spanned_node(5..8)
            ]
            .into_spanned(1..8)
            .map(CompExpr::from_vec)]),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
                .parse(Graphemes::new("('a', 'b')"))
                .into_result(),
            Ok(vec![
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
            ]),
        );
        assert_eq!(
            tuple(expr(fact::<CompNode>()))
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
}
