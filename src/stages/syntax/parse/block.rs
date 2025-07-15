use super::super::{ctx::Ctx, error::Expected, Node};
use super::{content::content, GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
use crate::reprs::{wast::Block, Spanned, SpannedVec};
use chumsky::prelude::*;

pub fn block<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, Block<'input, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let open = just("{").ignored();

    let close = just("}")
        .to(true)
        .labelled(Expected::BlockClose)
        .recover_with(via_parser(empty().to(false)));

    open.ignore_then(content(expr))
        .then(close)
        .map(|(content, close)| Block::new(content, close))
        .labelled(Expected::Block)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::reprs::{
        span::{IntoSpanned, Span},
        wast::{
            block::{Content, Stmt},
            Wast,
        },
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
            Ok(Content::new(vec![], CompExpr::from_vec(vec![]).into_spanned(1..1)).into()),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'}"))
                .into_result(),
            Ok(Content::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
            )
            .into()),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'; }"))
                .into_result(),
            Ok(Content::new(
                Stmt::Expr(CompExpr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                ))
                .into_spanned(1..4)
                .into_vec(),
                CompExpr::from_vec(vec![]).into_spanned(6..6),
            )
            .into()),
        );
        assert_eq!(
            block(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("{'a'; 'b'}"))
                .into_result(),
            Ok(Content::new(
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
            )
            .into()),
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
                Some(Block::from_stmts(
                    vec![],
                    CompExpr::from_vec(vec![]).into_spanned(1..1),
                    false,
                )),
                vec![Error::new(
                    smallvec![Expected::BlockClose, Expected::Expr, Expected::Stmt],
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
                Some(Block::from_stmts(
                    vec![],
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    false,
                )),
                vec![Error::new(
                    smallvec![
                        Expected::BlockClose,
                        Expected::Initialization,
                        Expected::Semicolon,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                        Expected::AssignSpecial,
                        Expected::Fact,
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
