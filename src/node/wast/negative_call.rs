//! Module that provides [`NegativeCall`].

use super::{Expr, Spanned};

/// Type describing the syntactic construct *negative call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input> {
    pub expr: Spanned<Expr<'input>>,
}

impl<'input> NegativeCall<'input> {
    /// Creates a new `NegativeCall`.
    pub fn new(expr: Spanned<Expr<'input>>) -> Self {
        Self { expr }
    }
}
