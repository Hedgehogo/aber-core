//! Module that provides [`ExprCall`].

use super::{call::Call, whitespaced::Whitespaced, Spanned};
use crate::stages::syntax::{Expr, Node};
use std::fmt;

/// Type describing syntactic constructs *method call* and *child call*.
///
/// # Fields
/// - `expr` Expression before the operator.
/// - `whitespace` Whitespace after the operator.
/// - `call` Call after the operator.
pub struct ExprCall<X: Expr> {
    pub expr: Spanned<X>,
    pub call: Whitespaced<X, Call<X>>,
}

impl<X: Expr> ExprCall<X> {
    /// Creates a new `ExprCall`.
    pub fn new(expr: Spanned<X>, call: Whitespaced<X, Call<X>>) -> Self {
        Self { expr, call }
    }
}

impl<X> fmt::Debug for ExprCall<X>
where
    X: Expr + fmt::Debug,
    X::Whitespace: fmt::Debug,
    <<X as Expr>::Node as Node>::Ident: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExprCall")
            .field("expr", &self.expr)
            .field("call", &self.call)
            .finish()
    }
}

impl<X> Clone for ExprCall<X>
where
    X: Expr + Clone,
    X::Whitespace: Clone,
    <<X as Expr>::Node as Node>::Ident: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.expr.clone(), self.call.clone())
    }
}

impl<X> PartialEq for ExprCall<X>
where
    X: Expr + PartialEq,
    X::Whitespace: PartialEq,
    <<X as Expr>::Node as Node>::Ident: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.call == other.call
    }
}

impl<X> Eq for ExprCall<X>
where
    X: Expr + Eq,
    X::Whitespace: Eq,
    <<X as Expr>::Node as Node>::Ident: Eq,
{
}
