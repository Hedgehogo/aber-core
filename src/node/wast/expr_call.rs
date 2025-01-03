//! Module that provides [`ExprCall`].

use super::{call::Call, Expr, Spanned};

/// Type describing syntactic constructs *method call* and *child call*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall<'input> {
    pub expr: Spanned<Expr<'input>>,
    pub call: Spanned<Call<'input>>,
}

impl<'input> ExprCall<'input> {
    /// Creates a new `ExprCall`.
    pub fn new(expr: Spanned<Expr<'input>>, call: Spanned<Call<'input>>) -> Self {
        Self { expr, call }
    }
}
