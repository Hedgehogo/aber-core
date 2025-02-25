//! Module that provides types for describing expressions

use super::{span::Spanned, wast::parser_output::ParserOutput, CompNode};

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

/// Type that describes a sequence of expressions.
pub type ExprVec<'input, N> = Vec<Spanned<<N as ParserOutput<'input>>::Expr>>;
