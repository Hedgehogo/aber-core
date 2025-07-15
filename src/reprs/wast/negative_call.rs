//! Module that provides [`NegativeCall`].

use super::Spanned;
use crate::stages::syntax::Expr;
use std::marker::PhantomData;

/// Type describing the syntactic construct *negative call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input, X: Expr<'input>> {
    pub expr: Spanned<X>,
    phanthom: PhantomData<&'input str>,
}

impl<'input, X: Expr<'input>> NegativeCall<'input, X> {
    /// Creates a new `NegativeCall`.
    pub fn new(expr: Spanned<X>) -> Self {
        Self {
            expr,
            phanthom: PhantomData,
        }
    }
}
