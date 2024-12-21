use super::super::error::{Error, Expected};
use super::spanned;
use super::{expression::expression, whitespace::whitespace, GraphemeParser};
use crate::node::{ExprVec, Node, Spanned};
use chumsky::prelude::*;

pub fn tuple<'input, M>(
    meaningful_unit: M,
) -> impl GraphemeParser<'input, ExprVec<'input>, Error<'input>> + Clone
where
    M: GraphemeParser<'input, Spanned<Node<'input>>, Error<'input>> + Clone,
{
    let left_bracket = just("(")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::TupleLeftBracket));

    let comma = just(",")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Comma));

    let right_bracket = just(")")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::TupleRightBracket))
        .recover_with(via_parser(empty()));

    let expression = spanned(expression(meaningful_unit, 1)).map(Spanned::from);
    let item = expression.then_ignore(whitespace());
    let separator = comma.then_ignore(whitespace());

    left_bracket
        .ignore_then(whitespace())
        .ignore_then(item.separated_by(separator).allow_trailing().collect())
        .then_ignore(right_bracket)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::meaningful_unit::meaningful_unit;
    use crate::node::{
        span::Span,
        wast::{
            number::{Digit, Radix},
            Wast,
        },
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_tuple() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("()"))
                .into_result(),
            Ok(vec![]),
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("("))
                .into_output_errors(),
            (
                Some(vec![]),
                vec![Error::new(
                    smallvec![
                        Expected::Minus,
                        Expected::Digit(Radix::DECIMAL),
                        Expected::CharSpecial,
                        Expected::StringSpecial,
                        Expected::RawStringStart,
                        Expected::TupleLeftBracket,
                        Expected::TupleRightBracket,
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("('a')"))
                .into_result(),
            Ok(vec![Spanned(
                vec![Wast::Character(grapheme("a").into())
                    .into_node()
                    .into_spanned(1..4)],
                Span::new(1..4)
            )]),
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("('a'"))
                .into_output_errors(),
            (
                Some(vec![Spanned(
                    vec![Wast::Character(grapheme("a").into())
                        .into_node()
                        .into_spanned(1..4)],
                    Span::new(1..4)
                )]),
                vec![Error::new(
                    smallvec![
                        Expected::Minus,
                        Expected::Digit(Radix::DECIMAL),
                        Expected::CharSpecial,
                        Expected::StringSpecial,
                        Expected::RawStringStart,
                        Expected::PairSpecial,
                        Expected::TupleLeftBracket,
                        Expected::TupleRightBracket,
                        Expected::Comma
                    ],
                    None,
                    Span::new(4..4)
                )]
            )
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("('a', )"))
                .into_result(),
            Ok(vec![Spanned(
                vec![Wast::Character(grapheme("a").into())
                    .into_node()
                    .into_spanned(1..4)],
                Span::new(1..4)
            )]),
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("('a' 'b')"))
                .into_result(),
            Ok(vec![Spanned(
                vec![
                    Wast::Character(grapheme("a").into())
                        .into_node()
                        .into_spanned(1..4),
                    Wast::Character(grapheme("b").into())
                        .into_node()
                        .into_spanned(5..8)
                ],
                Span::new(1..8)
            )]),
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new("('a', 'b')"))
                .into_result(),
            Ok(vec![
                Spanned(
                    vec![Wast::Character(grapheme("a").into())
                        .into_node()
                        .into_spanned(1..4)],
                    Span::new(1..4)
                ),
                Spanned(
                    vec![Wast::Character(grapheme("b").into())
                        .into_node()
                        .into_spanned(6..9)],
                    Span::new(6..9)
                ),
            ]),
        );
        assert_eq!(
            tuple(meaningful_unit())
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::TupleLeftBracket],
                    None,
                    Span::new(0..0)
                )]
            )
        );
    }
}
