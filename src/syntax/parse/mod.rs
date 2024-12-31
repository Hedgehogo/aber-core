pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod expression;
pub mod list;
pub mod meaningful_unit;
pub mod number;
pub mod raw_string;
pub mod string;
pub mod whitespace;

use super::error::{Error, Expected};
use crate::node::{
    wast::block::{Block, Statement},
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

#[allow(clippy::type_complexity)]
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

pub fn parser<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, Block<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let semicolon = just(";")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Semicolon));

    let statement = choice((
        expression.clone().map(|i| i.map(Statement::Expr)),
        spanned(assign(expression.clone()))
            .map(Spanned::from)
            .map(|i| i.map(Statement::Assign)),
    ))
    .then_ignore(whitespace())
    .then_ignore(semicolon);

    let expression = expression.or(spanned(empty().map(|_| vec![])).map(Spanned::from));

    statement
        .then_ignore(whitespace())
        .repeated()
        .collect()
        .then(expression)
        .map(|(statements, expr)| Block::new(statements, expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::IntoSpanned;
    use crate::node::{span::Span, wast::Wast};
    use expression::expression;
    use meaningful_unit::meaningful_unit;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_parser() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            parser(expression(meaningful_unit()))
                .parse(Graphemes::new(""))
                .into_result(),
            Ok(Block::new(vec![], vec![].into_spanned(0..0))),
        );
        assert_eq!(
            parser(expression(meaningful_unit()))
                .parse(Graphemes::new("'a'"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
            )),
        );
        assert_eq!(
            parser(expression(meaningful_unit()))
                .parse(Graphemes::new("'a'; "))
                .into_result(),
            Ok(Block::new(
                Statement::Expr(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_vec()
                )
                .into_spanned(0..3)
                .into_vec(),
                vec![].into_spanned(5..5),
            )),
        );
        assert_eq!(
            parser(expression(meaningful_unit()))
                .parse(Graphemes::new("'a'; 'b'"))
                .into_result(),
            Ok(Block::new(
                Statement::Expr(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_vec()
                )
                .into_spanned(0..3)
                .into_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(5..8)
                    .into_spanned_vec(),
            )),
        );
        assert_eq!(
            parser(expression(meaningful_unit()))
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
                        Expected::Ident
                    ],
                    Some(grapheme("[")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
