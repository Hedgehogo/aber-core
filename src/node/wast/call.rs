//! Module that provides types to describe the syntactic construct *call*.

use super::super::span::Spanned;
use super::ExprVec;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentError {}

/// Type describing the syntactic construct *identifier*
#[derive(Clone, PartialEq, Eq)]
pub struct Ident<'input> {
    content: &'input str,
}

impl<'input> Ident<'input> {
    /// Creates a new `Ident`.
    pub fn new(content: &'input str) -> Self {
        Self { content }
    }

    /// Gets a slice of the string.
    pub fn as_str(&self) -> &str {
        self.content
    }
}

impl fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

/// Type describing the syntactic construct *call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    pub ident: Spanned<Ident<'input>>,
    pub generics: Spanned<ExprVec<'input>>,
}

impl<'input> Call<'input> {
    /// Creates a new `Call`.
    pub fn new(ident: Spanned<Ident<'input>>, generics: Spanned<ExprVec<'input>>) -> Self {
        Self { ident, generics }
    }
}
