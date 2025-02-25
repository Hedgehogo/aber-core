//! Module that provides types to describe the syntactic construct *call*.
//!
use super::super::{Node, Spanned};
use super::ExprVec;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentError {}

/// Type describing the syntactic construct *identifier*
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl<'input> Spanned<Ident<'input>> {
    pub fn into_call<N: Node<'input>>(self) -> Call<'input, N> {
        Call::new(self, None)
    }
}

impl fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

/// Type describing the syntactic construct *call*
pub struct Call<'input, N: Node<'input>> {
    pub ident: Spanned<Ident<'input>>,
    pub generics: Option<Spanned<ExprVec<'input, N>>>,
}

impl<'input, N: Node<'input>> Call<'input, N> {
    /// Creates a new `Call`.
    pub fn new(
        ident: Spanned<Ident<'input>>,
        generics: Option<Spanned<ExprVec<'input, N>>>,
    ) -> Self {
        Self { ident, generics }
    }
}

impl<'input, N> fmt::Debug for Call<'input, N>
where
    N: Node<'input>,
    N::Expr: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("ident", &self.ident)
            .field("generics", &self.generics)
            .finish()
    }
}

impl<'input, N> Clone for Call<'input, N>
where
    N: Node<'input>,
    N::Expr: Clone,
{
    fn clone(&self) -> Self {
        Self {
            ident: self.ident.clone(),
            generics: self.generics.clone(),
        }
    }
}

impl<'input, N> PartialEq for Call<'input, N>
where
    N: Node<'input>,
    N::Expr: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.generics == other.generics
    }
}

impl<'input, N: Node<'input>> Eq for Call<'input, N> where N::Expr: Eq {}
