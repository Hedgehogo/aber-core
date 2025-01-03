//! Module that provides types to describe the syntactic construct *block*.

use super::{Assign, Expr, Spanned};

/// Type describing the syntactic construct *statement*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'input> {
    Expr(Expr<'input>),
    Assign(Assign<'input>),
}

/// Type describing the syntactic construct *block*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'input> {
    pub stmts: Vec<Spanned<Stmt<'input>>>,
    pub expr: Spanned<Expr<'input>>,
}

impl<'input> Block<'input> {
    /// Creates a new `Block`.
    pub fn new(stmts: Vec<Spanned<Stmt<'input>>>, expr: Spanned<Expr<'input>>) -> Self {
        Self { stmts, expr }
    }
}
