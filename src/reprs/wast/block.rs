//! Module that provides types to describe the syntactic construct *block*.

use super::{Assign, Spanned};
use crate::stages::syntax::Expr;
use std::fmt;

/// Type describing the syntactic construct *statement*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<X: Expr> {
    Expr(X),
    Assign(Assign<X>),
}

/// Type that describes all the contents of a document, as well as
/// the contents of a block syntax construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content<X: Expr> {
    pub stmts: Vec<Spanned<Stmt<X>>>,
    pub expr: Spanned<X>,
}

impl<X: Expr> Content<X> {
    /// Creates a new `Content`.
    pub fn new(stmts: Vec<Spanned<Stmt<X>>>, expr: Spanned<X>) -> Self {
        Self { stmts, expr }
    }
}

/// Type describing the syntactic construct *block*.
#[derive(Clone, PartialEq, Eq)]
pub struct Block<X: Expr> {
    content: Content<X>,
    close: bool,
}

impl<X: Expr> Block<X> {
    /// Creates a new `Block`.
    pub fn new(content: Content<X>, close: bool) -> Self {
        Self { content, close }
    }

    /// Creates a new `Block` from statements, expression and closing bracket.
    pub fn from_stmts(stmts: Vec<Spanned<Stmt<X>>>, expr: Spanned<X>, close: bool) -> Self {
        let content = Content::new(stmts, expr);
        Self { content, close }
    }

    /// Asks content.
    pub fn content(&self) -> &Content<X> {
        &self.content
    }

    /// Asks if the closing `}` was present.
    pub fn is_closed(&self) -> bool {
        self.close
    }

    /// Converts `Block` to [`Content`], losing semantically irrelevant information.
    pub fn into_content(self) -> Content<X> {
        self.content
    }
}

impl<X: Expr + fmt::Debug> fmt::Debug for Block<X> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("content.stmts", &self.content.stmts)
            .field("content.expr", &self.content.expr)
            .field("close", &self.close)
            .finish()
    }
}

impl<X: Expr> From<Content<X>> for Block<X> {
    fn from(value: Content<X>) -> Self {
        Block::new(value, true)
    }
}

impl<X: Expr> From<Block<X>> for Content<X> {
    fn from(value: Block<X>) -> Self {
        value.into_content()
    }
}
