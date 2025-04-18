//! Module that provides types to describe the syntactic construct *call*.
//!
use super::super::{Expr, Spanned};
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
    /// Creates a `Ident` from its representation.
    /// 
    /// # Arguments
    /// - `repr` Representation of the identifier in the source document.
    ///
    /// # Safeguards
    /// The representation must be a valid identifier.
    /// 
    /// That is, meet the following requirements:
    /// - It must not start with a decimal digit.
    /// - It must not start with the character `-` followed by a decimal digit.
    /// - It must not contain sequences `.`, `,`, `;`, `:`, `'`, `"`, `@`, `//`, `(`, `)`, `{`, `}`, `[`, `]`.
    /// - It must not contain whitespace.
    /// - It doesn't have to be `=`.
    pub(crate) fn from_repr_unchecked(repr: &'input str) -> Self {
        Self { content: repr }
    }

    /// Gets a slice of the string.
    pub fn as_str(&self) -> &str {
        self.content
    }
}

impl<'input> Spanned<Ident<'input>> {
    pub fn into_call<X: Expr<'input>>(self) -> Call<'input, X> {
        Call::new(self, None)
    }
}

impl fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

/// Type describing the syntactic construct *call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input, X: Expr<'input>> {
    pub ident: Spanned<Ident<'input>>,
    pub generics: Option<Spanned<ExprVec<'input, X>>>,
}

impl<'input, X: Expr<'input>> Call<'input, X> {
    /// Creates a new `Call`.
    pub fn new(
        ident: Spanned<Ident<'input>>,
        generics: Option<Spanned<ExprVec<'input, X>>>,
    ) -> Self {
        Self { ident, generics }
    }
}
