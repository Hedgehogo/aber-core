use super::super::error::{Error, Expected};
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::{Expr, ExprVec, Spanned};
use chumsky::prelude::*;

fn list<'input, E>(
    expression: E,
    open: (&'static str, Expected),
    close: (&'static str, Expected),
) -> impl GraphemeParser<'input, ExprVec<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
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
    let item = expression.then_ignore(whitespace());
    let separator = comma.then_ignore(whitespace());

    open.ignore_then(whitespace())
        .ignore_then(item.separated_by(separator).allow_trailing().collect())
        .then_ignore(close)
}

pub fn tuple<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, ExprVec<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    list(
        expression,
        ("(", Expected::Tuple),
        (")", Expected::TupleClose),
    )
}

pub fn generics<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, ExprVec<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    list(
        expression,
        ("[", Expected::Generics),
        ("]", Expected::GenericsClose),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expression::expression, meaningful_unit::meaningful_unit};
    use crate::node::{span::Span, wast::Wast};
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(vec![]),
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
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
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("('a')"))
                .into_result(),
            Ok(vec![Spanned(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)],
                Span::new(1..4)
            )]),
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("('a'"))
                .into_output_errors(),
            (
                Some(vec![Spanned(
                    vec![Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)],
                    Span::new(1..4)
                )]),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::PairSpecial,
                        Expected::Tuple,
                        Expected::TupleClose,
                        Expected::Comma,
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
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("('a', )"))
                .into_result(),
            Ok(vec![Spanned(
                vec![Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)],
                Span::new(1..4)
            )]),
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("('a' 'b')"))
                .into_result(),
            Ok(vec![Spanned(
                vec![
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4),
                    Wast::Character(grapheme("b").into())
                        .into_spanned_node(5..8)
                ],
                Span::new(1..8)
            )]),
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
                .parse(Graphemes::new("('a', 'b')"))
                .into_result(),
            Ok(vec![
                Spanned(
                    vec![Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)],
                    Span::new(1..4)
                ),
                Spanned(
                    vec![Wast::Character(grapheme("b").into())
                        .into_spanned_node(6..9)],
                    Span::new(6..9)
                ),
            ]),
        );
        assert_eq!(
            tuple(expression(meaningful_unit()))
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
