use super::super::error::{Error, Expected};
use super::{parser, GraphemeParser};
use crate::node::{wast::block::Block, Expr, Spanned, Node};
use chumsky::prelude::*;

pub fn block<'input, X>(
    expr: X,
) -> impl GraphemeParser<'input, Block<'input, Node<'input>>, Error<'input>> + Clone
where
    X: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let open = just("{")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::Block));

    let close = just("}")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::BlockClose))
        .recover_with(via_parser(empty()));

    open.ignore_then(parser(expr)).then_ignore(close)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact};
    use crate::node::span::IntoSpanned;
    use crate::node::{
        span::Span,
        wast::{block::Stmt, Wast},
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_block() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new("{}"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Expr::from_vec(vec![]).into_spanned(1..1)
            )),
        );
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new("{"))
                .into_output_errors(),
            (
                Some(Block::new(
                    vec![],
                    Expr::from_vec(vec![]).into_spanned(1..1)
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Tuple,
                        Expected::Block,
                        Expected::BlockClose,
                        Expected::Semicolon,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                        Expected::AssignSpecial,
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new("{'a'}"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(Expr::from_vec)
            )),
        );
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new("{'a'"))
                .into_output_errors(),
            (
                Some(Block::new(
                    vec![],
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                        .map(Expr::from_vec)
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::PairSpecial,
                        Expected::Tuple,
                        Expected::Block,
                        Expected::BlockClose,
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
            block(expr(fact()))
                .parse(Graphemes::new("{'a'; }"))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(Expr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                ))
                .into_spanned(1..4)
                .into_vec(),
                Expr::from_vec(vec![]).into_spanned(6..6),
            )),
        );
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new("{'a'; 'b'}"))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(Expr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                ))
                .into_spanned(1..4)
                .into_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(Expr::from_vec),
            )),
        );
        assert_eq!(
            block(expr(fact()))
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Block],
                    None,
                    Span::new(0..0)
                )]
            )
        );
    }
}
