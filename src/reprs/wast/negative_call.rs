//! Module that provides [`NegativeCall`].

use super::Spanned;
use crate::stages::parsing::Expr;

/// Type describing the syntactic construct *negative call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<X: Expr> {
    pub expr: Spanned<X>,
}

impl<X: Expr> NegativeCall<X> {
    /// Creates a new `NegativeCall`.
    pub fn new(expr: Spanned<X>) -> Self {
        Self { expr }
    }
}
