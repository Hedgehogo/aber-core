//! Module that provides [`ExprCall`].

use super::{call::Call, Spanned};
use super::super::Expr;

/// Type describing syntactic constructs *method call* and *child call*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall<'input, X: Expr<'input>> {
    pub expr: Spanned<X>,
    pub call: Spanned<Call<'input, X>>,
}

impl<'input, X: Expr<'input>> ExprCall<'input, X> {
    /// Creates a new `ExprCall`.
    pub fn new(expr: Spanned<X>, call: Spanned<Call<'input, X>>) -> Self {
        Self { expr, call }
    }
}
