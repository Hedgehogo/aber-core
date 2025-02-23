//! Module that provides types for describing expressions

use super::{span::Spanned, wast::parser_output::ParserOutput, Node};

/// Type that describes an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'input> {
    Wast(Vec<Spanned<Node<'input>>>),
    Hir(Box<Spanned<Node<'input>>>),
}

impl<'input> Expr<'input> {
    pub fn new() -> Self {
        Self::Wast(Vec::new())
    }

    pub fn from_vec(value: Vec<Spanned<Node<'input>>>) -> Self {
        Self::Wast(value)
    }
}

impl<'input> From<Vec<Spanned<Node<'input>>>> for Expr<'input> {
    fn from(value: Vec<Spanned<Node<'input>>>) -> Self {
        Self::Wast(value)
    }
}

/// Type that describes a sequence of expressions.
pub type ExprVec<'input, N> = Vec<Spanned<<N as ParserOutput<'input>>::Expr>>;
