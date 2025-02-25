pub mod next_stage;

use chumsky::span::Span;

use crate::node::{
    hir::{call::Call, pair::Pair},
    span::IntoSpanned,
    state::{unit_ref::UnitRef, State},
    CompExpr, Hir, CompNode, Spanned, Wast,
};

pub fn to_hir_expr_recursive<'input, 'expr>(
    state: &mut State,
    expr: &'expr [Spanned<CompNode<'input>>],
) -> Result<(Spanned<CompNode<'input>>, &'expr [Spanned<CompNode<'input>>]), ()> {
    let Spanned(node, node_span) = expr.get(0).ok_or(())?;
    let (_, mut rest) = expr.split_at(0);

    match node {
        CompNode::Wast(Wast::Call(call)) => {
            let Spanned(ident, mut span) = call.ident.clone();

            let function = match state.find(ident).ok_or(())? {
                UnitRef::Function(function) => function,
            };

            let argument_count = function.argument_count().ok_or(())?;
            let id = function.id();

            let mut result = Vec::with_capacity(argument_count);

            for _ in 0..argument_count {
                let (argument, expr) = to_hir_expr_recursive(state, rest)?;
                span.range = span.start()..argument.1.end();
                result.push(argument);
                rest = expr;
            }

            let node = CompNode::Hir(Hir::Call(Call::new(id, result)));
            Ok((node.into_spanned(span), rest))
        }

        CompNode::Wast(Wast::Pair(pair)) => {
            let (right, rest) = to_hir_expr_recursive(state, rest)?;
            let left_node = to_hir(state, (**pair).clone().0)?;
            let left = left_node.into_spanned((*pair).1.clone());
            let span = left.1.start()..right.1.end();
            let node = CompNode::Hir(Hir::Pair(Pair::new(Box::new(left), Box::new(right))));

            Ok((node.into_spanned(span), rest))
        }

        i => {
            let node = to_hir(state, i.clone())?;
            Ok((node.into_spanned(node_span.clone()), rest))
        }
    }
}

pub fn to_hir_expr<'input, 'expr>(
    state: &mut State,
    expr: &'expr [Spanned<CompNode<'input>>],
) -> Result<CompNode<'input>, ()> {
    match expr.get(0) {
        Some(_) => to_hir_expr_recursive(state, expr).map(|(i, _)| i.0),
        None => Ok(CompNode::Hir(Hir::Nil)),
    }
}

pub fn to_hir<'input>(state: &mut State, node: CompNode<'input>) -> Result<CompNode<'input>, ()> {
    match node {
        CompNode::Wast(wast) => match wast {
            Wast::Tuple(mut tuple) => {
                for Spanned(expr, span) in &mut tuple {
                    if let CompExpr::Wast(i) = expr {
                        *expr = to_hir_expr(state, i.as_slice())
                            .map(|i| CompExpr::Hir(Box::new(i.into_spanned(span.clone()))))?;
                    }
                }
                Ok(CompNode::Wast(Wast::Tuple(tuple)))
            }

            Wast::Block(block) => todo!(),

            Wast::MethodCall(method_call) => todo!(),

            Wast::ChildCall(child_call) => todo!(),

            Wast::NegativeCall(negative_call) => todo!(),

            Wast::Pair(_) => panic!("Pair can't exist in this context"),

            Wast::Call(_) => panic!("Call can't exist in this context"),

            i => Ok(CompNode::Wast(i)),
        },

        CompNode::Hir(hir) => Ok(CompNode::Hir(hir)),
    }
}
