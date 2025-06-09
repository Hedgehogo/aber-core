//! Module that provides `Initialization`.

use super::super::Expr;
use super::{call::Ident, Spanned, SpannedVec};
use std::fmt;

/// Type describing sequence I from the initialization syntax specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument<'input, X: Expr<'input>> {
    pub name: Option<(X::Whitespace, Ident<'input>, X::Whitespace)>,
    pub expr: Spanned<X>,
}

impl<'input, X: Expr<'input>> Argument<'input, X> {
    /// Creates a new `Argument`.
    pub fn new(
        name: Option<(X::Whitespace, Ident<'input>, X::Whitespace)>,
        expr: Spanned<X>,
    ) -> Self {
        Self { name, expr }
    }
}

/// Type describing syntactic constructions containing comma-separated enumerated items.
///
/// # Fields
/// - `args` Items listed comma-separately.
/// - `whitespace` Whitespace after the trailing comma.
pub struct Initialization<'input, X: Expr<'input>> {
    pub args: SpannedVec<Argument<'input, X>>,
}

impl<'input, X: Expr<'input>> Initialization<'input, X> {
    /// Creates a new `Initialization`.
    pub fn new(args: SpannedVec<Argument<'input, X>>) -> Self {
        Self { args }
    }
}

impl<'input, X> fmt::Debug for Initialization<'input, X>
where
    X: Expr<'input> + fmt::Debug,
    X::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Initialization")
            .field("items", &self.args)
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
        self.args == other.args
    }
}

impl<'input, X> Eq for Initialization<'input, X>
where
    X: Expr<'input> + Eq,
    X::Whitespace: Eq,
{
}
