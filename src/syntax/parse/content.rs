use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{spanned, whitespace::whitespace, GraphemeParser, GraphemeParserExtra};
use crate::node::{
    span::IntoSpanned,
    wast::{
        assign::Assign,
        block::{Block, Stmt},
    },
    whitespace::Side,
    Expr, Spanned,
};
use chumsky::prelude::*;

pub fn content<'input, X, P, E>(expr: P) -> impl GraphemeParser<'input, Block<'input, X>, E> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let assign = just("=")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));

    let semicolon = just(";")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Semicolon));

    let expr = whitespace(0)
        .then(expr.or(spanned(empty().map(|_| X::from_seq(vec![]))).map(Spanned::from)))
        .map(|(whitespace, expr)| X::whitespaced(expr, whitespace, Side::Left))
        .then(whitespace(0))
        .map(|(expr, whitespace)| X::whitespaced(expr, whitespace, Side::Right));

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
        assert_eq!(
            content(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("[]"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Block,
                        Expected::Tuple,
                        Expected::Semicolon,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                        Expected::AssignSpecial,
                        Expected::Eof,
                    ],
                    Some(grapheme("[")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
