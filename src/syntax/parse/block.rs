use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
    Node,
};
use super::{content::content, GraphemeParser, GraphemeParserExtra};
use crate::node::{wast::block::Block, Spanned, SpannedVec};
use chumsky::prelude::*;

pub fn block<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, Block<'input, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let open = just("{")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::Block));

    let close = just("}")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::BlockClose))
        .recover_with(via_parser(empty()));

    open.ignore_then(content(expr)).then_ignore(close)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::span::IntoSpanned;
    use crate::node::{
        span::Span,
        wast::{block::Stmt, Wast},
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_block() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{}"))
                .into_result(),
            Ok(Block::new(
                vec![],
                CompExpr::from_vec(vec![]).into_spanned(1..1)
            )),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'}"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
            )),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'; }"))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(CompExpr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                ))
                .into_spanned(1..4)
                .into_vec(),
                CompExpr::from_vec(vec![]).into_spanned(6..6),
            )),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'; 'b'}"))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(CompExpr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                ))
                .into_spanned(1..4)
                .into_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
            )),
        );
    }

    #[test]
    fn test_block_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{"))
                .into_output_errors(),
            (
                Some(Block::new(
                    vec![],
                    CompExpr::from_vec(vec![]).into_spanned(1..1)
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
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'"))
                .into_output_errors(),
            (
                Some(Block::new(
                    vec![],
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
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
            block(expr(fact::<CompNode, Extra>()))
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
