//! Module that provides [`Assign`].

use super::Spanned;
use crate::stages::syntax::Expr;
use std::marker::PhantomData;

/// Type describing the syntactic construct *assign*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input, X: Expr<'input>> {
    pub left: Spanned<X>,
    pub right: Spanned<X>,
    phanthom: PhantomData<&'input str>,
}

impl<'input, X: Expr<'input>> Assign<'input, X> {
    /// Creates a new `Assign`.
    pub fn new(left: Spanned<X>, right: Spanned<X>) -> Self {
        Self {
            left,
            right,
            phanthom: PhantomData,
        }
    }
}
