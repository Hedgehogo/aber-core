//! Module that provides types for describing expressions

use super::{CompNode, Spanned};

/// Type that describes an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompExpr<'input> {
    Wast(Vec<Spanned<CompNode<'input>>>),
    Hir(Box<Spanned<CompNode<'input>>>),
}

impl<'input> CompExpr<'input> {
    pub fn new() -> Self {
        Self::Wast(Vec::new())
    }

    pub fn from_vec(value: Vec<Spanned<CompNode<'input>>>) -> Self {
        Self::Wast(value)
    }
}

impl<'input> From<Vec<Spanned<CompNode<'input>>>> for CompExpr<'input> {
    fn from(value: Vec<Spanned<CompNode<'input>>>) -> Self {
        Self::Wast(value)
    }
}

impl Default for CompExpr<'_> {
    fn default() -> Self {
        CompExpr::new()
    }
}
