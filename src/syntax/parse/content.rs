use super::super::{ctx::Ctx, error::Expected, whitespace::Side, ExprOp, Node};
use super::{
    spanned, whitespace::whitespace, GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use crate::node::{
    span::IntoSpanned,
    wast::{
        assign::Assign,
        block::{Block, Stmt},
    },
    Spanned, SpannedVec,
};
use chumsky::prelude::*;

pub fn content<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, Block<'input, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let assign = just("=").ignored().labelled(Expected::AssignSpecial);

    let semicolon = just(";").ignored().labelled(Expected::Semicolon);

    let expr = whitespace()
        .then(expr.or(spanned(empty().map(|_| vec![])).map(Spanned::from)))
        .map(|(whitespace, expr)| expr.whitespaced(whitespace, Side::Left))
        .then(whitespace())
        .map(|(expr, whitespace)| expr.whitespaced(whitespace, Side::Right))
        .map(|i| i.into_spanned_expr());

    let stmt = expr
        .clone()
        .then(assign.ignore_then(expr.clone()).or_not())
        .then_ignore(semicolon)
        .map(|(left, right)| match right {
            Some(right) => {
                let span = left.1.range.start..right.1.range.end;
                let assign = Assign::new(left, right);
                Stmt::Assign(assign).into_spanned(span)
            }

            None => left.map(Stmt::Expr),
        });

    stmt.repeated()
        .collect()
        .then(expr)
        .map(|(stmts, expr)| Block::new(stmts, expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::{assign::Assign, Wast},
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_content() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new(""))
                .into_result(),
            Ok(Block::new(
                vec![],
                CompExpr::from_vec(vec![]).into_spanned(0..0)
            )),
        );
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("'a'"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
            )),
        );
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("'a'; "))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(CompExpr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_vec()
                ))
                .into_spanned(0..3)
                .into_vec(),
                CompExpr::from_vec(vec![]).into_spanned(5..5),
            )),
        );
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("'a'; 'b'"))
                .into_result(),
            Ok(Block::new(
                Stmt::Expr(CompExpr::from_vec(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_vec()
                ))
                .into_spanned(0..3)
                .into_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(5..8)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
            )),
        );
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("'a' = 'b';"))
                .into_result(),
            Ok(Block::new(
                Stmt::Assign(Assign::new(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    Wast::Character(grapheme("b").into())
                        .into_spanned_node(6..9)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                ))
                .into_spanned(0..9)
                .into_vec(),
                CompExpr::new().into_spanned(10..10),
            )),
        );
    }

    #[test]
    fn test_content_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("[]"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Expr, Expected::Stmt, Expected::Eof],
                    Some(grapheme("[")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
