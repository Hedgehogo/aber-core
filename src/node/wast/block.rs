//! Module that provides types to describe the syntactic construct *block*.

use super::super::Expr;
use super::{Assign, Spanned};

/// Type describing the syntactic construct *statement*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'input, X: Expr<'input>> {
    Expr(X),
    Assign(Assign<'input, X>),
}

/// Type describing the syntactic construct *block*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'input, X: Expr<'input>> {
    pub stmts: Vec<Spanned<Stmt<'input, X>>>,
    pub expr: Spanned<X>,
}

impl<'input, X: Expr<'input>> Block<'input, X> {
    /// Creates a new `Block`.
    pub fn new(stmts: Vec<Spanned<Stmt<'input, X>>>, expr: Spanned<X>) -> Self {
        Self { stmts, expr }
    }
}
