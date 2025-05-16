use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{whitespace::whitespace, GraphemeParser, GraphemeParserExtra};
use crate::node::{wast::assign::Assign, whitespace::Side, Expr, Spanned};
use chumsky::prelude::*;

pub fn assign<'input, X, P, E>(expr: P) -> impl GraphemeParser<'input, Assign<'input, X>, E> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let special = just("=")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));

    let left = expr
        .clone()
        .then(whitespace(0))
        .map(|(expr, whitespace)| X::whitespaced(expr, whitespace, Side::Right));

    let right = whitespace(0)
        .then(expr)
        .map(|(whitespace, expr)| X::whitespaced(expr, whitespace, Side::Left));

    left.then_ignore(special)
        .then(right)
        .map(|(left, right)| Assign::new(left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::{call::Ident, Wast},
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_assign() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            assign(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("'a' = 'b'"))
                .into_result(),
            Ok(Assign::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
            )),
        );
        assert_eq!(
            assign(expr(fact::<CompNode, Extra>()))
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
            assign(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("foo = bar"))
                .into_result(),
            Ok(Assign::new(
                Wast::Call(
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(0..3)
                        .into_call()
                )
                .into_spanned_node(0..3)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Wast::Call(
                    Ident::from_repr_unchecked("bar")
                        .into_spanned(6..9)
                        .into_call()
                )
                .into_spanned_node(6..9)
                .into_spanned_vec()
                .map(CompExpr::from_vec)
            )),
        );
        assert_eq!(
            assign(expr(fact::<CompNode, Extra>()))
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
