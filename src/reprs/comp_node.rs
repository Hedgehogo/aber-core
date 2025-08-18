//! Module that provides [`CompNode`].

use super::{mir::node::String, CompExpr, Mir, Spanned, SpannedVec, Wast};
use crate::stages::syntax::{whitespace::Side, Expr, Node};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode<'input> {
    Wast(Wast<'input, Self>),
    Mir(Mir<'input>),
}

impl<'input> CompNode<'input> {
    pub fn mir(&self) -> Option<&Mir<'input>> {
        match self {
            Self::Mir(mir) => Some(mir),
            _ => None,
        }
    }
}

impl<'input> Node<'input> for CompNode<'input> {
    type Expr = CompExpr<'input>;
    type String = String;

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }
}

impl<'input> Expr<'input> for CompExpr<'input> {
    type Node = CompNode<'input>;
    type Whitespace = ();

    fn whitespaced_seq(
        expr: Spanned<SpannedVec<Self::Node>>,
        _whitespace: Self::Whitespace,
        _side: Side,
    ) -> Spanned<SpannedVec<Self::Node>> {
        expr
    }
}
