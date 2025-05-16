use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{call::call, spanned, whitespace::whitespace, GraphemeParser, GraphemeParserExtra};
use crate::node::{
    span::{IntoSpanned, Span},
    wast::{expr_call::ExprCall, negative_call::NegativeCall, Wast},
    whitespace::Side,
    Expr, Node, Spanned,
};
use chumsky::pratt::*;
use chumsky::prelude::*;

pub fn expr<'input, N, P, E>(fact: P) -> impl GraphemeParser<'input, Spanned<N::Expr>, E> + Clone
where
    N: Node<'input> + 'input,
    P: GraphemeParser<'input, Spanned<N>, E> + Clone + 'input,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    recursive(|expr| {
        let atom = fact
            .map(|i| {
                let span = i.1.clone();
                Spanned(N::Expr::from_seq(vec![i]), span)
            })
            .boxed();

        let negative_special = just("@")
            .ignore_then(whitespace(0))
            .map_err(|e: Error| e.replace_expected(Expected::NegativeSpecial))
            .boxed();

        let expr_call = |s: &'static str, expected| {
            whitespace(0)
                .then_ignore(just(s))
                .then(whitespace(0))
                .then(spanned(call::<N::Expr, _, _>(expr.clone())).map(Spanned::from))
                .map_err(move |e: Error| e.replace_expected(expected))
                .boxed()
        };

        let method_special = expr_call(".", Expected::MethodSpecial);
        let child_special = expr_call("::", Expected::ChildSpecial);
        let whitespace = whitespace(0).then_ignore(choice((just("."), just("::"))).not());

        let into_atom = move |wast: Wast<'input, N>, span: SimpleSpan| {
            let seq = vec![Spanned(wast.into_node(), Span::from(span))];
            N::Expr::from_seq(seq).into_spanned(span)
        };

        atom.pratt((
            postfix(1, method_special, move |expr, call, extra| {
                let ((left_ws, right_ws), call) = call;
                let whitespaced = N::Expr::whitespaced(expr, left_ws, Side::Right);
                let node = Wast::MethodCall(ExprCall::new(whitespaced, right_ws, call));

                into_atom(node, extra.span())
            }),
            postfix(1, child_special, move |expr, call, extra| {
                let ((left_ws, right_ws), call) = call;
                let whitespaced = N::Expr::whitespaced(expr, left_ws, Side::Right);
                let node = Wast::ChildCall(ExprCall::new(whitespaced, right_ws, call));

                into_atom(node, extra.span())
            }),
            prefix(2, negative_special, move |ws, expr, extra| {
                let whitespaced = N::Expr::whitespaced(expr, ws, Side::Left);
                let node = Wast::NegativeCall(NegativeCall::new(whitespaced));

                into_atom(node, extra.span())
            }),
            infix(left(3), whitespace, move |left, ws, right, _| {
                let whitespaced = N::Expr::whitespaced(right, ws, Side::Left);

                N::Expr::concat(left, whitespaced).unwrap()
            }),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::{fact::fact, tests::Extra};
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
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'"))
                .into_result(),
            Ok(Wast::Character(grapheme("a").into())
                .into_spanned_node(0..3)
                .into_spanned_vec()
                .map(CompExpr::from_vec))
        );
        assert_eq!(
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'.foo"))
                .into_result(),
            Ok(Wast::MethodCall(ExprCall::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                (),
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
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'::foo"))
                .into_result(),
            Ok(Wast::ChildCall(ExprCall::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                (),
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
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'.foo::bar"))
                .into_result(),
            Ok(Wast::ChildCall(ExprCall::new(
                Wast::MethodCall(ExprCall::new(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    (),
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(4..7)
                        .into_call()
                        .into_spanned(4..7)
                ))
                .into_spanned_node(0..7)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                (),
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
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'::foo.bar"))
                .into_result(),
            Ok(Wast::MethodCall(ExprCall::new(
                Wast::ChildCall(ExprCall::new(
                    Wast::Character(grapheme("a").into())
                        .into_spanned_node(0..3)
                        .into_spanned_vec()
                        .map(CompExpr::from_vec),
                    (),
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(5..8)
                        .into_call()
                        .into_spanned(5..8)
                ))
                .into_spanned_node(0..8)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                (),
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
            expr(fact::<CompNode, Extra>())
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
                (),
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
            expr(fact::<CompNode, Extra>())
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
