//! Module that provides [`NegativeCall`].

use super::super::Node;
use super::Spanned;

/// Type describing the syntactic construct *negative call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input, N: Node<'input>> {
    pub expr: Spanned<N::Expr>,
}

impl<'input, N: Node<'input>> NegativeCall<'input, N> {
    /// Creates a new `NegativeCall`.
    pub fn new(expr: Spanned<N::Expr>) -> Self {
        Self { expr }
    }
}
