//! Module that provides [`WastNode`].

use super::super::{span::IntoSpanned, Spanned, SpannedVec};
use super::{call::Ident, number::Digits, Character, String, Wast, Whitespace};
use crate::stages::syntax::{whitespace::Side, Expr, Node};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WastNode<'input> {
    Wast(Wast<Self>),
    Whitespace(Whitespace<'input>),
}

impl<'input> Node for WastNode<'input> {
    type Expr = Vec<Spanned<Self>>;
    type Ident = Ident<'input>;
    type Digits = Digits<'input>;
    type Character = Character<'input>;
    type String = String<'input>;

    fn from_wast(wast: Wast<Self>) -> Self {
        Self::Wast(wast)
    }
}

pub type WastExpr<'input> = Vec<Spanned<WastNode<'input>>>;

impl<'input> Expr for Vec<Spanned<WastNode<'input>>> {
    type Node = WastNode<'input>;
    type Whitespace = Whitespace<'input>;

    fn whitespaced_seq(
        expr: Spanned<SpannedVec<Self::Node>>,
        whitespace: Self::Whitespace,
        side: Side,
    ) -> Spanned<SpannedVec<Self::Node>> {
        let Spanned(mut expr, mut span) = expr;
        match side {
            Side::Left => {
                if !whitespace.is_empty() {
                    let start = span.range.start - whitespace.repr().len();
                    let node =
                        WastNode::Whitespace(whitespace).into_spanned(start..span.range.start);
                    span.range.start = start;
                    expr.insert(0, node);
                }
                Spanned(expr, span)
            }
            Side::Right => {
                if !whitespace.is_empty() {
                    let end = span.range.end + whitespace.repr().len();
                    let node = WastNode::Whitespace(whitespace).into_spanned(span.range.end..end);
                    span.range.end = end;
                    expr.push(node);
                }
                Spanned(expr, span)
            }
        }
    }
}
