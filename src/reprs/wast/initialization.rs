//! Module that provides [`Initialization`].

use super::{call::Ident, whitespaced::Whitespaced, List, Spanned};
use crate::stages::syntax::Expr;
use std::fmt;

/// Type describing sequence I from the initialization syntax specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument<'input, X: Expr<'input>> {
    pub name: Option<(Whitespaced<'input, X, Ident<'input>>, X::Whitespace)>,
    pub expr: Spanned<X>,
}

impl<'input, X: Expr<'input>> Argument<'input, X> {
    /// Creates a new `Argument`.
    pub fn new(
        name: Option<(Whitespaced<'input, X, Ident<'input>>, X::Whitespace)>,
        expr: Spanned<X>,
    ) -> Self {
        Self { name, expr }
    }
}

/// Type describing elements of initialization syntax except expression.
pub type Arguments<'input, X> = Whitespaced<'input, X, List<'input, Argument<'input, X>, X>>;

/// Type describing syntactic construct *initialization*.
///
/// # Fields
/// - `expr` Expression before the operator.
/// - `args` Items listed comma-separately.
pub struct Initialization<'input, X: Expr<'input>> {
    pub expr: Spanned<X>,
    pub args: Arguments<'input, X>,
}

impl<'input, X: Expr<'input>> Initialization<'input, X> {
    /// Creates a new `Initialization`.
    pub fn new(expr: Spanned<X>, args: Arguments<'input, X>) -> Self {
        Self { expr, args }
    }
}

impl<'input, X> fmt::Debug for Initialization<'input, X>
where
    X: Expr<'input> + fmt::Debug,
    X::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Initialization")
            .field("expr", &self.expr)
            .field("args", &self.args)
            .finish()
    }
}

impl<'input, X> Clone for Initialization<'input, X>
where
    X: Expr<'input> + Clone,
    X::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        Self {
            expr: self.expr.clone(),
            args: self.args.clone(),
        }
    }
}

impl<'input, X> PartialEq for Initialization<'input, X>
where
    X: Expr<'input> + PartialEq,
    X::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.args == other.args
    }
}

impl<'input, X> Eq for Initialization<'input, X>
where
    X: Expr<'input> + Eq,
    X::Whitespace: Eq,
{
}
