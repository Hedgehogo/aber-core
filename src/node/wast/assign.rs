//! Module that provides [`Assign`].

use super::Spanned;
use super::super::Node;

/// Type describing the syntactic construct *assign*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input, N: Node<'input>> {
    pub left: Spanned<N::Expr>,
    pub right: Spanned<N::Expr>,
}

impl<'input, N: Node<'input>> Assign<'input, N> {
    /// Creates a new `Assign`.
    pub fn new(left: Spanned<N::Expr>, right: Spanned<N::Expr>) -> Self {
        Self { left, right }
    }
}
