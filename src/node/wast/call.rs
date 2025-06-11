//! Module that provides types to describe the syntactic construct *call*.

use super::super::Spanned;
use super::List;
use crate::syntax::Expr;
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

/// Type describing the syntactic construct *generic arguments*
///
/// # Fields
/// - `whitespace` Whitespace before the opening bracket.
/// - `args` List of expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generics<'input, X: Expr<'input>> {
    pub whitespace: X::Whitespace,
    pub args: Spanned<List<'input, X, X>>,
}

impl<'input, X: Expr<'input>> Generics<'input, X> {
    /// Creates a new `Generics`.
    pub fn new(whitespace: X::Whitespace, args: Spanned<List<'input, X, X>>) -> Self {
        Self { whitespace, args }
    }
}

/// Type describing the syntactic construct *call*
pub struct Call<'input, X: Expr<'input>> {
    pub ident: Spanned<Ident<'input>>,
    pub generics: Option<Generics<'input, X>>,
}

impl<'input, X: Expr<'input>> Call<'input, X> {
    /// Creates a new `Call`.
    pub fn new(ident: Spanned<Ident<'input>>, generics: Option<Generics<'input, X>>) -> Self {
        Self { ident, generics }
    }
}

impl<'input, X> fmt::Debug for Call<'input, X>
where
    X: Expr<'input> + fmt::Debug,
    X::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("ident", &self.ident)
            .field("generics", &self.generics)
            .finish()
    }
}

impl<'input, X> Clone for Call<'input, X>
where
    X: Expr<'input> + Clone,
    X::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.ident.clone(), self.generics.clone())
    }
}

impl<'input, X> PartialEq for Call<'input, X>
where
    X: Expr<'input> + PartialEq,
    X::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.generics == other.generics
    }
}

impl<'input, X> Eq for Call<'input, X>
where
    X: Expr<'input> + Eq,
    X::Whitespace: Eq,
{
}
