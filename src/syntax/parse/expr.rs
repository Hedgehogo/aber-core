use super::super::error::{Error, Expected};
use super::{call::call, spanned, whitespace::whitespace, GraphemeParser};
use crate::node::{
    span::Span,
    wast::{expr_call::ExprCall, negative_call::NegativeCall, Wast},
    Expr, Node, Spanned,
};
use chumsky::pratt::*;
use chumsky::prelude::*;

pub fn expr<'input, N, P>(
    fact: P,
) -> impl GraphemeParser<'input, Spanned<N::Expr>, Error<'input>> + Clone
where
    N: Node<'input> + 'input,
    P: GraphemeParser<'input, Spanned<N>, Error<'input>> + Clone + 'input,
{
    recursive(|expr| {
        let atom = fact.map(|i| {
            let span = i.1.clone();
            Spanned(vec![i], span)
        });

        let negative_special = just("@")
            .then_ignore(whitespace(0))
            .map_err(|e: Error| e.replace_expected(Expected::NegativeSpecial));

        let expr_call = |s: &'static str, expected| {
            just(s)
                .padded_by(whitespace(0))
                .ignore_then(spanned(call::<N::Expr, _>(expr.clone())).map(Spanned::from))
                .map_err(move |e: Error| e.replace_expected(expected))
        };

        let method_special = expr_call(".", Expected::MethodSpecial);
        let child_special = expr_call("::", Expected::ChildSpecial);
        let whitespace = whitespace(0).then_ignore(choice((just("."), just("::"))).not());

        let into_atom = move |wast: Wast<'input, N>, span: SimpleSpan| {
            Spanned(
                vec![Spanned(wast.into_node(), Span::from(span))],
                Span::from(span),
            )
        };

        let concat_spanned = |mut i: Spanned<Vec<_>>, mut j: Spanned<Vec<_>>, span: SimpleSpan| {
            i.0.append(&mut j.0);
            Spanned(i.0, span.into())
        };

        atom.pratt((
            postfix(1, method_special, move |i, call, extra| {
                into_atom(
                    Wast::MethodCall(ExprCall::new(Spanned::map(i, N::Expr::from_seq), call)),
                    extra.span(),
                )
            }),
            postfix(1, child_special, move |i, call, extra| {
                into_atom(
                    Wast::ChildCall(ExprCall::new(Spanned::map(i, N::Expr::from_seq), call)),
                    extra.span(),
                )
            }),
            prefix(2, negative_special, move |_, i, extra| {
                into_atom(
                    Wast::NegativeCall(NegativeCall::new(Spanned::map(i, N::Expr::from_seq))),
                    extra.span(),
                )
            }),
            infix(left(3), whitespace, move |i, _, j, extra| {
                concat_spanned(i, j, extra.span())
            }),
        ))
        .map(|i| i.map(N::Expr::from_seq))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::fact::fact;
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::{call::Ident, Wast},
        CompExpr, CompNode,
    };
    use text::Graphemes;

    #[test]
    fn test() {
        use chumsky::error::Rich;
        let atom = just::<_, _, extra::Err<Rich<char>>>("a").ignored();
        let expr = atom.pratt(infix(right(1), just("+"), |_, _, _, _| ()));
        let parser = expr.then_ignore(just("+"));
        assert_eq!(parser.parse("a+a+").into_result(), Ok(()))
    }

    #[test]
    fn test_expr() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("'a'"))
                .into_result(),
            Ok(Wast::Character(grapheme("a").into())
                .into_spanned_node(0..3)
                .into_spanned_vec()
                .map(CompExpr::from_vec))
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("'a'.foo"))
                .into_result(),
            Ok(Wast::MethodCall(ExprCall::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("foo")
                    .into_spanned(4..7)
                    .into_call()
                    .into_spanned(4..7)
            ))
            .into_spanned_node(0..7)
            .into_spanned_vec()
            .map(CompExpr::from_vec)),
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("'a'::foo"))
                .into_result(),
            Ok(Wast::ChildCall(ExprCall::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("foo")
                    .into_spanned(5..8)
                    .into_call()
                    .into_spanned(5..8)
            ))
            .into_spanned_node(0..8)
            .into_spanned_vec()
            .map(CompExpr::from_vec)),
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("'a'.foo::bar"))
                .into_result(),
            Ok(Wast::ChildCall(ExprCall::new(
                Wast::MethodCall(ExprCall::new(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(4..7)
                        .into_call()
                        .into_spanned(4..7)
                ))
                .into_spanned_node(0..7)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("bar")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()
            .map(CompExpr::from_vec)),
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("'a'::foo.bar"))
                .into_result(),
            Ok(Wast::MethodCall(ExprCall::new(
                Wast::ChildCall(ExprCall::new(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(5..8)
                        .into_call()
                        .into_spanned(5..8)
                ))
                .into_spanned_node(0..8)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("bar")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()
            .map(CompExpr::from_vec)),
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("@'a''b'::foo"))
                .into_result(),
            Ok(Wast::ChildCall(ExprCall::new(
                Wast::NegativeCall(NegativeCall::new(
                    CompExpr::from_vec(vec![
                        Wast::Character(grapheme("a").into()).into_spanned_node(1..4),
                        Wast::Character(grapheme("b").into()).into_spanned_node(4..7),
                    ])
                    .into_spanned(1..7)
                ))
                .into_spanned_node(0..7)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("foo")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()
            .map(CompExpr::from_vec)),
        );
        assert_eq!(
            expr(fact::<CompNode>())
                .parse(Graphemes::new("\"hello\" //hello\n 'h"))
                .into_output_errors(),
            (
                Some(
                    vec![
                        Wast::String("hello".into()).into_spanned_node(0..7),
                        Wast::Character(grapheme("h").into()).into_spanned_node(17..19),
                    ]
                    .into_spanned(0..19)
                    .map(CompExpr::from_vec)
                ),
                vec![Error::new_expected(
                    Expected::CharClose,
                    None,
                    Span::new(19..19)
                )]
            )
        );
    }
}
