//! Module that provides [`ExprCall`].

use super::super::Expr;
use super::{call::Call, Spanned};
use std::fmt;

/// Type describing syntactic constructs *method call* and *child call*.
///
/// # Fields
/// - `expr` Expression before the operator.
/// - `whitespace` Whitespace after the operator.
/// - `call` Call after the operator.
pub struct ExprCall<'input, X: Expr<'input>> {
    pub expr: Spanned<X>,
    pub whitespace: X::Whitespace,
    pub call: Spanned<Call<'input, X>>,
}

impl<'input, X: Expr<'input>> ExprCall<'input, X> {
    /// Creates a new `ExprCall`.
    pub fn new(
        expr: Spanned<X>,
        whitespace: X::Whitespace,
        call: Spanned<Call<'input, X>>,
    ) -> Self {
        Self {
            expr,
            whitespace,
            call,
        }
    }
}

impl<'input, X> fmt::Debug for ExprCall<'input, X>
where
    X: Expr<'input> + fmt::Debug,
    X::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprCall")
            .field("expr", &self.expr)
            .field("whitespace", &self.whitespace)
            .field("call", &self.call)
            .finish()
    }
}

impl<'input, X> Clone for ExprCall<'input, X>
where
    X: Expr<'input> + Clone,
    X::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        Self::new(
            self.expr.clone(),
            self.whitespace.clone(),
            self.call.clone(),
        )
    }
}

impl<'input, X> PartialEq for ExprCall<'input, X>
where
    X: Expr<'input> + PartialEq,
    X::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.call == other.call
    }
}

impl<'input, X> Eq for ExprCall<'input, X>
where
    X: Expr<'input> + Eq,
    X::Whitespace: Eq,
{
}
