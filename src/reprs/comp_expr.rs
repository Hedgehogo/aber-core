//! Module that provides types for describing expressions

use super::{CompNode, Spanned};

/// Type that describes an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompExpr {
    Wast(Vec<Spanned<CompNode>>),
    Mir(Box<Spanned<CompNode>>),
}

impl CompExpr {
    pub fn new() -> Self {
        Self::Wast(Vec::new())
    }

    pub fn from_vec(value: Vec<Spanned<CompNode>>) -> Self {
        Self::Wast(value)
    }
}

impl From<Vec<Spanned<CompNode>>> for CompExpr {
    fn from(value: Vec<Spanned<CompNode>>) -> Self {
        Self::Wast(value)
    }
}

impl Default for CompExpr {
    fn default() -> Self {
        CompExpr::new()
    }
}
