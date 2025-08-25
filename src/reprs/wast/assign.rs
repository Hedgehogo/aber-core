//! Module that provides [`Assign`].

use super::Spanned;
use crate::stages::parsing::Expr;

/// Type describing the syntactic construct *assign*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<X: Expr> {
    pub left: Spanned<X>,
    pub right: Spanned<X>,
}

impl<X: Expr> Assign<X> {
    /// Creates a new `Assign`.
    pub fn new(left: Spanned<X>, right: Spanned<X>) -> Self {
        Self { left, right }
    }
}
