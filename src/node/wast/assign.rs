//! Module that provides [`Assign`].

use super::{Expr, Spanned};

/// Type describing the syntactic construct *assign*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input> {
    pub left: Spanned<Expr<'input>>,
    pub right: Spanned<Expr<'input>>,
}

impl<'input> Assign<'input> {
    /// Creates a new `Assign`.
    pub fn new(left: Spanned<Expr<'input>>, right: Spanned<Expr<'input>>) -> Self {
        Self { left, right }
    }
}
