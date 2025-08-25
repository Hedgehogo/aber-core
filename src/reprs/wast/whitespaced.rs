//! Module that provides [`Whitespaced`].

use super::Spanned;
use crate::stages::parsing::Expr;
use std::fmt;

/// Type that describes a construct containing whitespace before it.
///
/// # Fields
/// - `whitespace` Whitespace in front of the element.
/// - `right` Consturction located after whitespace.
pub struct Whitespaced<X: Expr, R> {
    pub whitespace: X::Whitespace,
    pub right: Spanned<R>,
}

impl<X: Expr, R> Whitespaced<X, R> {
    /// Creates a new `Whitespaced`.
    pub fn new(whitespace: X::Whitespace, right: Spanned<R>) -> Self {
        Self { whitespace, right }
    }
}

impl<X, R> fmt::Debug for Whitespaced<X, R>
where
    X: Expr,
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

impl<X, R> Clone for Whitespaced<X, R>
where
    X: Expr,
    X::Whitespace: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.whitespace.clone(), self.right.clone())
    }
}

impl<X, R> PartialEq for Whitespaced<X, R>
where
    X: Expr,
    X::Whitespace: PartialEq,
    R: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.whitespace == other.whitespace && self.right == other.right
    }
}

impl<X, R> Eq for Whitespaced<X, R>
where
    X: Expr,
    X::Whitespace: PartialEq,
    R: PartialEq,
{
}
