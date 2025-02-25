//! Module that provides types to describe the syntactic construct *block*.

use super::{Assign, Spanned};
use super::super::Node;

/// Type describing the syntactic construct *statement*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'input, N: Node<'input>> {
    Expr(N::Expr),
    Assign(Assign<'input, N>),
}

/// Type describing the syntactic construct *block*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'input, N: Node<'input>> {
    pub stmts: Vec<Spanned<Stmt<'input, N>>>,
    pub expr: Spanned<N::Expr>,
}

impl<'input, N: Node<'input>> Block<'input, N> {
    /// Creates a new `Block`.
    pub fn new(stmts: Vec<Spanned<Stmt<'input, N>>>, expr: Spanned<N::Expr>) -> Self {
        Self { stmts, expr }
    }
}
