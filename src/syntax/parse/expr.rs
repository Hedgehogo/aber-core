use super::super::{ctx::Ctx, error::Expected, whitespace::Side, ExprOp, Node};
use super::entirely;
use super::{
    call::call,
    initialization::initialization,
    whitespace::{whitespace, whitespaced},
    GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use crate::node::{
    span::IntoSpanned,
    wast::{
        expr_call::ExprCall, initialization::Initialization, negative_call::NegativeCall, Wast,
    },
    Spanned, SpannedVec,
};
use chumsky::pratt::*;
use chumsky::prelude::*;

pub fn expr<'input, N, P, E>(
    fact: P,
) -> impl GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone
where
    N: Node<'input> + 'input,
    P: GraphemeParser<'input, Spanned<N>, E> + Clone + 'input,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    recursive(|expr| {
        let atom = fact
            .map(|i| {
                let span = i.1.clone();
                vec![i].into_spanned(span)
            })
            .boxed();

        let negative_special = just("@")
            .ignore_then(whitespace())
            .labelled(Expected::NegativeSpecial)
            .boxed();

        let expr_op_special = |s: &'static str, expected| {
            whitespace()
                .then_ignore(entirely(just(s), expected))
                .then(whitespaced::<_, N::Expr, _, E, _>(call(expr.clone())))
                .labelled(expected)
                .boxed()
        };

        let method_special = expr_op_special(".", Expected::MethodSpecial).boxed();
        let child_special = expr_op_special("::", Expected::ChildSpecial).boxed();
        let initialization = initialization(expr.clone()).boxed();
        let whitespace = whitespace()
            .then_ignore(choice((just("."), just("::"))).not())
            .boxed();

        let into_atom = move |wast: Wast<'input, N>, span: SimpleSpan| {
            wast.into_spanned_node(span).into_spanned_vec()
        };

        atom.pratt((
            postfix(1, method_special, move |seq, call, extra| {
                let seq: Spanned<SpannedVec<N>> = seq;
                let (left_ws, call) = call;
                let whitespaced = seq.whitespaced(left_ws, Side::Right).into_spanned_expr();
                let node = Wast::MethodCall(ExprCall::new(whitespaced, call));

                into_atom(node, extra.span())
            }),
            postfix(1, child_special, move |seq, call, extra| {
                let seq: Spanned<SpannedVec<N>> = seq;
                let (left_ws, call) = call;
                let whitespaced = seq.whitespaced(left_ws, Side::Right).into_spanned_expr();
                let node = Wast::ChildCall(ExprCall::new(whitespaced, call));

                into_atom(node, extra.span())
            }),
            postfix(1, initialization, move |seq, args, extra| {
                let seq: Spanned<SpannedVec<N>> = seq;
                let whitespaced = seq.into_spanned_expr();
                let node = Wast::Initialization(Initialization::new(whitespaced, args));

                into_atom(node, extra.span())
            }),
            prefix(2, negative_special, move |ws, seq, extra| {
                let seq: Spanned<SpannedVec<N>> = seq;
                let whitespaced = seq.whitespaced(ws, Side::Left).into_spanned_expr();
                let node = Wast::NegativeCall(NegativeCall::new(whitespaced));

                into_atom(node, extra.span())
            }),
            infix(left(3), whitespace, move |left, ws, right, _| {
                let left: Spanned<SpannedVec<N>> = left;
                let whitespaced = right.whitespaced(ws, Side::Left);

                left.concat(whitespaced)
            }),
        ))
    })
    .labelled(Expected::Expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
    use super::super::{fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::{call::Ident, initialization::Argument, list::List, Character, Wast},
        CompExpr, CompNode,
    };
    use smallvec::smallvec;
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
                .into_spanned_vec())
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
                Ident::from_repr_unchecked("foo")
                    .into_spanned(4..7)
                    .into_call()
                    .into_spanned(4..7)
                    .into_whitespaced(())
            ))
            .into_spanned_node(0..7)
            .into_spanned_vec()),
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
                Ident::from_repr_unchecked("foo")
                    .into_spanned(5..8)
                    .into_call()
                    .into_spanned(5..8)
                    .into_whitespaced(())
            ))
            .into_spanned_node(0..8)
            .into_spanned_vec()),
        );
        assert_eq!(
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'::('b')"))
                .into_result(),
            Ok(Wast::Initialization(Initialization::new(
                Wast::Character(grapheme("a").into())
                    .into_spanned_node(0..3)
                    .into_spanned_vec()
                    .map(CompExpr::from_vec),
                List::new(
                    vec![Argument::new(
                        None,
                        Wast::Character(grapheme("b").into())
                            .into_spanned_node(6..9)
                            .into_spanned_vec()
                            .map(CompExpr::from_vec)
                    )
                    .into_spanned(6..9)],
                    None,
                    true,
                )
                .into_spanned(5..10)
                .into_whitespaced(())
            ))
            .into_spanned_node(0..10)
            .into_spanned_vec()),
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
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(4..7)
                        .into_call()
                        .into_spanned(4..7)
                        .into_whitespaced(())
                ))
                .into_spanned_node(0..7)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("bar")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
                    .into_whitespaced(())
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()),
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
                    Ident::from_repr_unchecked("foo")
                        .into_spanned(5..8)
                        .into_call()
                        .into_spanned(5..8)
                        .into_whitespaced(())
                ))
                .into_spanned_node(0..8)
                .into_spanned_vec()
                .map(CompExpr::from_vec),
                Ident::from_repr_unchecked("bar")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
                    .into_whitespaced(())
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()),
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
                Ident::from_repr_unchecked("foo")
                    .into_spanned(9..12)
                    .into_call()
                    .into_spanned(9..12)
                    .into_whitespaced(())
            ))
            .into_spanned_node(0..12)
            .into_spanned_vec()),
        );
    }

    #[test]
    fn test_expr_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(Expected::Expr, None, Span::new(0..0))]
            )
        );
        assert_eq!(
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("'a'["))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Initialization,
                        Expected::PairSpecial,
                        Expected::MethodSpecial,
                        Expected::ChildSpecial,
                        Expected::NegativeSpecial,
                        Expected::Fact,
                        Expected::Eof,
                    ],
                    Some(grapheme("[")),
                    Span::new(3..4)
                )]
            )
        );
        assert_eq!(
            expr(fact::<CompNode, Extra>())
                .parse(Graphemes::new("\"hello\" //hello\n 'h"))
                .into_output_errors(),
            (
                Some(
                    vec![
                        Wast::String("hello".into()).into_spanned_node(0..7),
                        Wast::Character(Character::new("h", false)).into_spanned_node(17..19),
                    ]
                    .into_spanned(0..19)
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
