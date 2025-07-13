//! Module that provides types to describe the syntactic construct *block*.

use super::{Assign, Spanned};
use crate::syntax::Expr;
use std::fmt;

/// Type describing the syntactic construct *statement*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'input, X: Expr<'input>> {
    Expr(X),
    Assign(Assign<'input, X>),
}

/// Type that describes all the contents of a document, as well as
/// the contents of a block syntax construct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content<'input, X: Expr<'input>> {
    pub stmts: Vec<Spanned<Stmt<'input, X>>>,
    pub expr: Spanned<X>,
}

impl<'input, X: Expr<'input>> Content<'input, X> {
    /// Creates a new `Content`.
    pub fn new(stmts: Vec<Spanned<Stmt<'input, X>>>, expr: Spanned<X>) -> Self {
        Self { stmts, expr }
    }
}

/// Type describing the syntactic construct *block*.
#[derive(Clone, PartialEq, Eq)]
pub struct Block<'input, X: Expr<'input>> {
    content: Content<'input, X>,
    close: bool,
}

impl<'input, X: Expr<'input>> Block<'input, X> {
    /// Creates a new `Block`.
    pub fn new(content: Content<'input, X>, close: bool) -> Self {
        Self { content, close }
    }

    /// Creates a new `Block` from statements, expression and closing bracket.
    pub fn from_stmts(stmts: Vec<Spanned<Stmt<'input, X>>>, expr: Spanned<X>, close: bool) -> Self {
        let content = Content::new(stmts, expr);
        Self { content, close }
    }

    /// Asks content.
    pub fn content(&self) -> &Content<'input, X> {
        &self.content
    }

    /// Asks if the closing `}` was present.
    pub fn is_closed(&self) -> bool {
        self.close
    }

    /// Converts `Block` to [`Content`], losing semantically irrelevant information.
    pub fn into_content(self) -> Content<'input, X> {
        self.content
    }
}

impl<'input, X: Expr<'input> + fmt::Debug> fmt::Debug for Block<'input, X> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ ")?;
        if self.close {
            let content = self.content();
            write!(f, "stmts: {:?}, expr: {:?}", content.stmts, content.expr)?;
        } else {
            write!(f, "content: {:?}, close: false", self.content)?;
        }
        write!(f, " }}")
    }
}

impl<'input, X: Expr<'input>> From<Content<'input, X>> for Block<'input, X> {
    fn from(value: Content<'input, X>) -> Self {
        Block::new(value, true)
    }
}

impl<'input, X: Expr<'input>> From<Block<'input, X>> for Content<'input, X> {
    fn from(value: Block<'input, X>) -> Self {
        value.into_content()
    }
}
