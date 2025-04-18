pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod escaped_string;
pub mod expr;
pub mod fact;
pub mod list;
pub mod number;
pub mod raw_string;
pub mod whitespace;

use super::error::{Error, Expected};
use crate::node::{
    wast::block::{Block, Stmt},
    Expr, Spanned,
};
use assign::assign;
use chumsky::{
    combinator::MapWith, error, extra::ParserExtra, input::MapExtra, prelude::*,
    text::unicode::Graphemes,
};
use whitespace::whitespace;

pub trait GraphemeParser<'input, O, E>:
    Parser<'input, &'input Graphemes, O, extra::Err<E>>
where
    E: error::Error<'input, &'input Graphemes> + 'input,
{
}

impl<'input, O, T, E> GraphemeParser<'input, O, E> for T
where
    T: Parser<'input, &'input Graphemes, O, extra::Err<E>>,
    E: error::Error<'input, &'input Graphemes> + 'input,
{
}

#[expect(clippy::type_complexity)]
pub fn spanned<'src, P, I, O, E>(
    parser: P,
) -> MapWith<P, O, impl Fn(O, &mut MapExtra<'src, '_, I, E>) -> (O, I::Span) + Copy>
where
    P: Parser<'src, I, O, E>,
    I: Input<'src>,
    E: ParserExtra<'src, I>,
{
    parser.map_with(|i, e| (i, e.span()))
}

pub fn parser<'input, X, P>(
    expr: P,
) -> impl GraphemeParser<'input, Block<'input, X>, Error<'input>> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, Error<'input>> + Clone,
{
    let semicolon = just(";")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Semicolon));

    let expr = expr.or(spanned(empty().map(|_| X::from_seq(vec![]))).map(Spanned::from));

    let stmt = choice((
        spanned(assign(expr.clone()))
            .map(Spanned::from)
            .map(|i| i.map(Stmt::Assign)),
        expr.clone().map(|i| i.map(Stmt::Expr)),
    ))
    .then_ignore(whitespace(0))
    .then_ignore(semicolon);

    let content = stmt
        .then_ignore(whitespace(0))
        .repeated()
        .collect()
        .then(expr)
        .map(|(stmts, expr)| Block::new(stmts, expr));

    whitespace(0)
        .ignore_then(content)
        .then_ignore(whitespace(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{
        span::{IntoSpanned, Span},
        wast::{assign::Assign, Wast},
        CompExpr, CompNode,
    };
    use expr::expr;
    use fact::fact;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_parser() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            parser(expr(fact::<CompNode>()))
                .parse(Graphemes::new(""))
                .into_result(),
            Ok(Block::new(
                vec![],
                CompExpr::from_vec(vec![]).into_spanned(0..0)
            )),
        );
        assert_eq!(
            parser(expr(fact::<CompNode>()))
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
            parser(expr(fact::<CompNode>()))
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
            parser(expr(fact::<CompNode>()))
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
            parser(expr(fact::<CompNode>()))
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
            parser(expr(fact::<CompNode>()))
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
