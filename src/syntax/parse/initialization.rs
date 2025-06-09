use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{call::ident, spanned, whitespace::whitespace, GraphemeParser, GraphemeParserExtra};
use crate::node::{
    span::IntoSpanned,
    wast::{initialization::Argument, List},
    whitespace::Side,
    Expr, Spanned,
};
use chumsky::prelude::*;

pub fn initialization<'input, X, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, List<'input, Argument<'input, X>, X>, E> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let open = just("::(").map_err(|e: Error| e.replace_expected(Expected::Initialization));
    let close = just(")").map_err(|e: Error| e.replace_expected(Expected::InitializationClose));
    let assign = just("=").map_err(|e: Error| e.replace_expected(Expected::AssignSpecial));
    let comma = just(",").map_err(|e: Error| e.replace_expected(Expected::Comma));

    let close = close.ignored().recover_with(via_parser(empty()));

    let argument = spanned(
        whitespace()
            .then(
                ident()
                    .then(whitespace())
                    .then_ignore(assign)
                    .then(whitespace())
                    .or_not(),
            )
            .then(expr),
    );

    let separator = whitespace().then_ignore(comma);

    let repeat = argument
        .then(separator.or_not())
        .map(|((argument, span), after_ws)| {
            let ((before_ws, name), expr) = argument;

            let (name, expr_ws) = match name {
                Some(((ident, after_name_ws), after_assign_ws)) => {
                    (Some((before_ws, ident, after_name_ws)), after_assign_ws)
                }

                None => (None, before_ws),
            };

            let whitespaced_left = X::whitespaced(expr, expr_ws, Side::Left);

            let whitespaced = match after_ws {
                Some(i) => X::whitespaced(whitespaced_left, i, Side::Right),
                None => whitespaced_left,
            };

            Argument::new(name, whitespaced).into_spanned(span)
        });

    open.ignore_then(repeat.repeated().collect())
        .then(whitespace())
        .map(|(items, whitespace)| List::new(items, whitespace))
        .then_ignore(close)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{expr, fact, tests::Extra};
    use crate::node::{
        span::Span,
        wast::call::{Call, Ident},
        CompExpr, CompNode, Wast,
    };
    use chumsky::text::Graphemes;
    use smallvec::smallvec;

    #[test]
    fn test_initialization() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::()"))
                .into_result(),
            Ok(List::new(vec![], ()))
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::("))
                .into_output_errors(),
            (
                Some(List::new(vec![], ())),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::InitializationClose,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(3..3)
                )]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a')"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    None,
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(3..6)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                )
                .into_spanned(3..6)],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a'"))
                .into_output_errors(),
            (
                Some(List::new(
                    vec![Argument::new(
                        None,
                        Wast::Character(grapheme("a").into())
                            .into_spanned_node(3..6)
                            .into_spanned_vec()
                            .map(CompExpr::from_vec)
                    )
                    .into_spanned(3..6)],
                    ()
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::PairSpecial,
                        Expected::InitializationClose,
                        Expected::Block,
                        Expected::Comma,
                        Expected::Ident,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(6..6)
                )]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a', )"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    None,
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(3..6)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                )
                .into_spanned(3..6)],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a' 'b')"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    None,
                    vec![
                        Wast::Character(grapheme("a").into()).into_spanned_node(3..6),
                        Wast::Character(grapheme("b").into()).into_spanned_node(7..10)
                    ]
                    .into_spanned(3..10)
                    .map(CompExpr::from_vec)
                )
                .into_spanned(3..10)],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a', 'b')"))
                .into_result(),
            Ok(List::new(
                vec![
                    Argument::new(
                        None,
                        Wast::Character(grapheme("a").into())
                            .into_spanned_node(3..6)
                            .into_spanned_vec()
                            .map(CompExpr::from_vec)
                    )
                    .into_spanned(3..6),
                    Argument::new(
                        None,
                        Wast::Character(grapheme("b").into())
                            .into_spanned_node(8..11)
                            .into_spanned_vec()
                            .map(CompExpr::from_vec)
                    )
                    .into_spanned(8..11),
                ],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Initialization],
                    None,
                    Span::new(0..0)
                )]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo)"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    None,
                    Wast::Call(Call::new(
                        Ident::from_repr_unchecked("foo").into_spanned(3..6),
                        None
                    ))
                    .into_spanned_node(3..6)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec)
                )
                .into_spanned(3..6)],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo = 'a')"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    Some(((), Ident::from_repr_unchecked("foo"), ())),
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(9..12)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                )
                .into_spanned(9..12)],
                ()
            )),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo = "))
                .into_output_errors(),
            (
                Some(List::new(vec![], ())),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                    ],
                    None,
                    Span::new(3..3)
                )]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo"))
                .into_output_errors(),
            (
                Some(List::new(
                    vec![Argument::new(
                        None,
                        Wast::Call(Call::new(
                            Ident::from_repr_unchecked("foo").into_spanned(3..6),
                            None
                        ))
                        .into_spanned_node(3..6)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                    )
                    .into_spanned(3..6)],
                    ()
                )),
                vec![Error::new(
                    smallvec![
                        Expected::Number,
                        Expected::Char,
                        Expected::String,
                        Expected::RawString,
                        Expected::InitializationClose,
                        Expected::Block,
                        Expected::Ident,
                        Expected::NegativeSpecial,
                        Expected::AssignSpecial,
                    ],
                    None,
                    Span::new(3..3)
                )]
            ),
        );
    }
}
