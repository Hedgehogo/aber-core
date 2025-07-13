use super::super::{ctx::Ctx, error::Expected, whitespace::Side, ExprOp, Node};
use super::{
    call::ident, entirely, spanned, whitespace::whitespace, GraphemeLabelError, GraphemeParser,
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
    let open = entirely(just("::"), Expected::Other)
        .ignore_then(spanned(whitespace().then_ignore(just("("))))
        .map(|(whitespace, span)| (whitespace, Span::from(span)));

    let close = just(")").labelled(Expected::InitializationClose);
    let assign = just("=").labelled(Expected::AssignSpecial);
    let comma = just(",").labelled(Expected::Comma);

    let close = close
        .to(true)
        .recover_with(via_parser(empty().to(false)))
        .map_with(|value, extra| (value, Span::from(extra.span())));

    let argument = group((
        whitespace(),
        spanned(group((
            group((
                spanned(ident()).map(Spanned::from),
                whitespace().then_ignore(assign),
                whitespace(),
            ))
            .or_not(),
            expr,
        ))),
        whitespace(),
    ))
    .map(|(before_ws, argument, after_ws)| {
        let ((name, mut expr), span) = argument;

        let (name, expr_ws) = match name {
            Some((ident, after_name_ws, after_assign_ws)) => {
                let name = Whitespaced::new(before_ws, ident);
                (Some((name, after_name_ws)), after_assign_ws)
            }

            None => (None, before_ws),
        };

        expr.whitespace(expr_ws, Side::Left);
        expr.whitespace(after_ws, Side::Right);

        Argument::new(name, expr.into_spanned_expr()).into_spanned(span)
    });

    group((
        open,
        argument.separated_by(comma).collect(),
        comma.ignore_then(whitespace()).or_not(),
        close,
    ))
    .map(|(open, items, list_whitespace, close)| {
        let (whitespace, open_span) = open;
        let (close, close_span) = close;
        let span = open_span.range.start..close_span.range.end;
        let right = List::new(items, list_whitespace, close);
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
            Ok(List::new(vec![], None, true)
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
                None,
                true
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
                Some(()),
                true
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
                None,
                true
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
                None,
                true
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
                    List::new(vec![], None, false)
                        .into_spanned(2..3)
                        .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![
                        Expected::Ident,
                        Expected::InitializationClose,
                        Expected::Comma,
                        Expected::Expr
                    ],
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
                        None,
                        false
                    )
                    .into_spanned(2..6)
                    .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![
                        Expected::Initialization,
                        Expected::InitializationClose,
                        Expected::Comma,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
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
                None,
                true
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
                .into_spanned(3..12)],
                None,
                true
            )
            .into_spanned(2..13)
            .into_whitespaced(())),
        );
        assert_eq!(
            initialization(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("::(foo = "))
                .into_output_errors(),
            (
                None,
                vec![
                    Error::new(smallvec![Expected::Expr], None, Span::new(9..9)),
                    Error::new(
                        smallvec![Expected::Eof],
                        Some(grapheme("f")),
                        Span::new(3..4)
                    )
                ]
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
                        None,
                        false
                    )
                    .into_spanned(2..6)
                    .into_whitespaced(())
                ),
                vec![Error::new(
                    smallvec![
                        Expected::Ident,
                        Expected::Generics,
                        Expected::Initialization,
                        Expected::InitializationClose,
                        Expected::Comma,
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
            ),
        );
    }
}
