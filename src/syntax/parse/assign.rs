use super::super::error::{Error, Expected};
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::{wast::assign::Assign, Node, Spanned};
use chumsky::prelude::*;

pub fn assign<'input, N, P>(
    expr: P,
) -> impl GraphemeParser<'input, Assign<'input, N>, Error<'input>> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<N::Expr>, Error<'input>> + Clone,
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
            assign::<CompNode, _>(expr(fact::<CompNode>()))
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
            assign::<CompNode, _>(expr(fact::<CompNode>()))
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
            assign::<CompNode, _>(expr(fact::<CompNode>()))
                .parse(Graphemes::new("foo = bar"))
                .into_result(),
            Ok(Assign::new(
                Wast::Call(Ident::new("foo").into_spanned(0..3).into_call())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                Wast::Call(Ident::new("bar").into_spanned(6..9).into_call())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
            )),
        );
        assert_eq!(
            assign::<CompNode, _>(expr(fact::<CompNode>()))
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
