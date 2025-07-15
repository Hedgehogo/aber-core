//! Module that provides [`Whitespaced`].

use super::Spanned;
use crate::stages::syntax::Expr;
use std::fmt;

/// Type that describes a construct containing whitespace before it.
///
/// # Fields
/// - `whitespace` Whitespace in front of the element.
/// - `right` Consturction located after whitespace.
pub struct Whitespaced<'input, X: Expr<'input>, R> {
    pub whitespace: X::Whitespace,
    pub right: Spanned<R>,
}

impl<'input, X: Expr<'input>, R> Whitespaced<'input, X, R> {
    /// Creates a new `Whitespaced`.
    pub fn new(whitespace: X::Whitespace, right: Spanned<R>) -> Self {
        Self { whitespace, right }
    }
}

impl<'input, X, R> fmt::Debug for Whitespaced<'input, X, R>
where
    X: Expr<'input>,
    X::Whitespace: fmt::Debug,
    R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Whitespaced")
            .field("whitespace", &self.whitespace)
            .field("right", &self.right)
            .finish()
    }
}

impl<'input, X, R> Clone for Whitespaced<'input, X, R>
where
    X: Expr<'input>,
    X::Whitespace: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.whitespace.clone(), self.right.clone())
    }
}

impl<'input, X, R> PartialEq for Whitespaced<'input, X, R>
where
    X: Expr<'input>,
    X::Whitespace: PartialEq,
    R: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.whitespace == other.whitespace && self.right == other.right
    }
}

impl<'input, X, R> Eq for Whitespaced<'input, X, R>
where
    X: Expr<'input>,
    X::Whitespace: PartialEq,
    R: PartialEq,
{
}
