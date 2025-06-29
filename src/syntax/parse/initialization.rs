use super::super::{ctx::Ctx, error::Expected, whitespace::Side, ExprOp, Node};
use super::{
    call::ident, spanned, whitespace::whitespace, GraphemeLabelError, GraphemeParser,
    GraphemeParserExtra,
};
use crate::node::{
    span::{IntoSpanned, Span},
    wast::{
        initialization::{Argument, Arguments},
        whitespaced::Whitespaced,
        List,
    },
    Spanned, SpannedVec,
};
use chumsky::prelude::*;

pub fn initialization<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, Arguments<'input, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let open = just("::")
        .ignore_then(spanned(whitespace().then_ignore(just("("))))
        .map(|(whitespace, span)| (whitespace, Span::from(span)));

    let close = just(")").labelled(Expected::InitializationClose);
    let assign = just("=").labelled(Expected::AssignSpecial);
    let comma = just(",").labelled(Expected::Comma);

    let close = close
        .ignored()
        .recover_with(via_parser(empty()))
        .to_span()
        .map(Span::from);

    let argument = whitespace().then(spanned(
        spanned(ident())
            .map(Spanned::from)
            .then(whitespace())
            .then_ignore(assign)
            .then(whitespace())
            .or_not()
            .then(expr),
    ));

    let separator = whitespace().then_ignore(comma);

    let repeat = argument
        .then(separator.or_not())
        .map(|(argument, after_ws)| {
            let (before_ws, ((name, expr), span)) = argument;

            let (name, expr_ws) = match name {
                Some(((ident, after_name_ws), after_assign_ws)) => {
                    let name = Whitespaced::new(before_ws, ident);
                    (Some((name, after_name_ws)), after_assign_ws)
                }

                None => (None, before_ws),
            };

            let whitespaced_left = expr.whitespaced(expr_ws, Side::Left);
            let whitespaced = match after_ws {
                Some(i) => whitespaced_left.whitespaced(i, Side::Right),
                None => whitespaced_left,
            };

            Argument::new(name, whitespaced.into_spanned_expr()).into_spanned(span)
        });

    open.then(
        repeat
            .repeated()
            .collect()
            .then(whitespace())
            .map(|(items, whitespace)| List::new(items, whitespace))
            .then(close),
    )
    .map(|((whitespace, open_span), (right, close_span))| {
        let span = open_span.range.start..close_span.range.end;
        Whitespaced::new(whitespace, right.into_spanned(span))
    })
    .labelled(Expected::Initialization)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
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
            Ok(List::new(vec![], ())
                .into_spanned(2..4)
                .into_whitespaced(()))
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
            )
            .into_spanned(2..7)
            .into_whitespaced(())),
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
            )
            .into_spanned(2..9)
            .into_whitespaced(())),
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
            )
            .into_spanned(2..11)
            .into_whitespaced(())),
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
            )
            .into_spanned(2..12)
            .into_whitespaced(())),
        );
    }

    #[test]
    fn test_initialization_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::("))
                .into_output_errors(),
            (
                Some(
                    List::new(vec![], ())
                        .into_spanned(2..3)
                        .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![Expected::InitializationClose, Expected::Expr],
                    None,
                    Span::new(3..3)
                )]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::('a'"))
                .into_output_errors(),
            (
                Some(
                    List::new(
                        vec![Argument::new(
                            None,
                            Wast::Character(grapheme("a").into())
                                .into_spanned_node(3..6)
                                .into_spanned_vec()
                                .map(CompExpr::from_vec)
                        )
                        .into_spanned(3..6)],
                        ()
                    )
                    .into_spanned(2..6)
                    .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![
                        Expected::InitializationClose,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                        Expected::AssignSpecial,
                        Expected::Fact,
                    ],
                    None,
                    Span::new(6..6)
                )]
            )
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
            )
            .into_spanned(2..7)
            .into_whitespaced(())),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo = 'a')"))
                .into_result(),
            Ok(List::new(
                vec![Argument::new(
                    Some((
                        Ident::from_repr_unchecked("foo")
                            .into_spanned(3..6)
                            .into_whitespaced(()),
                        ()
                    )),
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(9..12)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec)
                )
                .into_spanned(9..12)],
                ()
            )
            .into_spanned(2..13)
            .into_whitespaced(())),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo = "))
                .into_output_errors(),
            (
                Some(
                    List::new(vec![], ())
                        .into_spanned(2..9)
                        .into_whitespaced(())
                ),
                vec![Error::new(smallvec![Expected::Expr], None, Span::new(3..3))]
            )
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo"))
                .into_output_errors(),
            (
                Some(
                    List::new(
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
                    )
                    .into_spanned(2..6)
                    .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![
                        Expected::InitializationClose,
                        Expected::Generics,
                        Expected::Comma,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                        Expected::Fact,
                    ],
                    None,
                    Span::new(3..3)
                )]
            ),
        );
    }
}
