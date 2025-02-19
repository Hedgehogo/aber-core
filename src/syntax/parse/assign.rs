use super::super::error::{Error, Expected};
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::{wast::assign::Assign, Expr, Spanned};
use chumsky::prelude::*;

pub fn assign<'input, X>(
    expr: X,
) -> impl GraphemeParser<'input, Assign<'input>, Error<'input>> + Clone
where
    X: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let special = just("=")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));

    expr.clone()
        .then_ignore(whitespace().then(special).then(whitespace()))
        .then(expr)
        .map(|(left, right)| Assign::new(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact};
    use crate::node::span::IntoSpanned;
    use crate::node::wast::call::Ident;
    use crate::node::{span::Span, wast::Wast};
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_assign() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            assign(expr(fact()))
                .parse(Graphemes::new("'a' = 'b'"))
                .into_result(),
            Ok(Assign::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(Expr::from_vec),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(Expr::from_vec)
            )),
        );
        assert_eq!(
            assign(expr(fact()))
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
            assign(expr(fact()))
                .parse(Graphemes::new("foo = bar"))
                .into_result(),
            Ok(Assign::new(
                Wast::Call(Ident::new("foo").into_spanned(0..3).into_call())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(Expr::from_vec),
                Wast::Call(Ident::new("bar").into_spanned(6..9).into_call())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(Expr::from_vec)
            )),
        );
        assert_eq!(
            assign(expr(fact()))
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
