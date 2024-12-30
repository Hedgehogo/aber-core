use super::super::error::{Error, Expected};
use super::{spanned, whitespace::whitespace, GraphemeParser};
use crate::node::wast::block::Block;
use crate::node::{
    wast::{assign::Assign, block::Statement},
    Expr, Spanned,
};
use chumsky::prelude::*;

pub fn assign<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, Assign<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let special = just("=")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));

    expression
        .clone()
        .then_ignore(whitespace().then(special).then(whitespace()))
        .then(expression)
        .map(|(left, right)| Assign::new(left, right))
}

pub fn block<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, Block<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let open = just("{")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::Block));

    let semicolon = just(";")
        .ignored()
        .map_err(|e: Error| e.replace_expected(Expected::Semicolon));

    let close = just("}")
        .ignored()
        .map_err(move |e: Error| e.replace_expected(Expected::BlockClose))
        .recover_with(via_parser(empty()));

    let statement = choice((
        expression.clone().map(|i| i.map(Statement::Expr)),
        spanned(assign(expression.clone()))
            .map(Spanned::from)
            .map(|i| i.map(Statement::Assign)),
    ))
    .then_ignore(whitespace())
    .then_ignore(semicolon)
    .then_ignore(whitespace());

    let expression = expression.or(spanned(empty().map(|_| vec![])).map(Spanned::from));

    open.ignore_then(whitespace())
        .ignore_then(statement.repeated().collect())
        .then(expression)
        .map(|(statements, expr)| Block::new(statements, expr))
        .then_ignore(whitespace())
        .then_ignore(close)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{expression::expression, meaningful_unit::meaningful_unit};
    use crate::node::span::IntoSpanned;
    use crate::node::{span::Span, wast::Wast};
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_block() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{}"))
                .into_result(),
            Ok(Block::new(vec![], vec![].into_spanned(1..1))),
        );
        assert_eq!(
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{"))
                .into_output_errors(),
            (
                Some(Block::new(vec![], vec![].into_spanned(1..1))),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Tuple,
                        Expected::BlockClose,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(1..1)
                )]
            )
        );
        assert_eq!(
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{'a'}"))
                .into_result(),
            Ok(Block::new(
                vec![],
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(1..4)
                    .into_spanned_vec()
            )),
        );
        assert_eq!(
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{'a'"))
                .into_output_errors(),
            (
                Some(Block::new(
                    vec![],
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_spanned_vec()
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::PairSpecial,
                        Expected::Tuple,
                        Expected::BlockClose,
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
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{'a'; }"))
                .into_result(),
            Ok(Block::new(
                Statement::Expr(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                )
                .into_spanned(1..4)
                .into_vec(),
                vec![].into_spanned(6..6),
            )),
        );
        assert_eq!(
            block(expression(meaningful_unit()))
                .parse(Graphemes::new("{'a'; 'b'}"))
                .into_result(),
            Ok(Block::new(
                Statement::Expr(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(1..4)
                        .into_vec()
                )
                .into_spanned(1..4)
                .into_vec(),
                Wast::Character(grapheme("b").into())
                    .into_spanned_node(6..9)
                    .into_spanned_vec(),
            )),
        );
        assert_eq!(
            block(expression(meaningful_unit()))
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
