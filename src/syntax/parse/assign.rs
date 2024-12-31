use super::super::error::{Error, Expected};
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::{wast::assign::Assign, Expr, Spanned};
use chumsky::prelude::*;

pub fn assign<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, Assign<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let special = just("=")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));

    expression
        .clone()
        .then_ignore(whitespace().then(special).then(whitespace()))
        .then(expression)
        .map(|(left, right)| Assign::new(left, right))
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
    fn test_assign() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            assign(expression(meaningful_unit()))
                .parse(Graphemes::new("'a' = 'b'"))
                .into_result(),
            Ok(Assign::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
            )),
        );
        assert_eq!(
            assign(expression(meaningful_unit()))
                .parse(Graphemes::new("'a' = "))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Tuple,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(6..6)
                )]
            )
        );
        assert_eq!(
            assign(expression(meaningful_unit()))
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Tuple,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(0..0)
                )]
            )
        );
    }
}
