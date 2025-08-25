//! Module that provides types to describe the syntactic construct *call*.
use super::super::span::{IntoSpanned, Spanned};
use super::{whitespaced::Whitespaced, List, Wast};
use crate::stages::syntax::{self, ident::IdentSealed, Expr, Node};
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
    pub fn into_spanned_call<X: Expr>(self) -> Spanned<Call<X>>
    where
        X::Node: Node<Ident = Ident<'input>>,
    {
        let span = self.span();
        Call::new(self, None).into_spanned(span)
    }
}

impl fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

impl<'input, S> IdentSealed<'input, S> for Ident<'input> {
    fn from_repr_unchecked(_state: &mut S, repr: &'input str) -> Self {
        Self::from_repr_unchecked(repr)
    }
}

impl<'input, S> syntax::Ident<'input, S> for Ident<'input> {}

/// Type describing the syntactic construct *generic arguments*
pub type Generics<X> = Whitespaced<X, List<X, X>>;

/// Type describing the syntactic construct *call*
pub struct Call<X: Expr> {
    pub ident: Spanned<<<X as Expr>::Node as Node>::Ident>,
    pub generics: Option<Generics<X>>,
}

impl<X: Expr> Call<X> {
    /// Creates a new `Call`.
    pub fn new(
        ident: Spanned<<<X as Expr>::Node as Node>::Ident>,
        generics: Option<Generics<X>>,
    ) -> Self {
        Self { ident, generics }
    }
}

impl<X: Expr> Spanned<Call<X>> {
    pub fn into_spanned_wast(self) -> Spanned<Wast<X::Node>> {
        let Spanned(call, span) = self;
        Wast::Call(call).into_spanned(span)
    }
}

impl<X> fmt::Debug for Call<X>
where
    X: Expr + fmt::Debug,
    X::Whitespace: fmt::Debug,
    <<X as Expr>::Node as Node>::Ident: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("ident", &self.ident)
            .field("generics", &self.generics)
            .finish()
    }
}

impl<X> Clone for Call<X>
where
    X: Expr + Clone,
    X::Whitespace: Clone,
    <<X as Expr>::Node as Node>::Ident: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.ident.clone(), self.generics.clone())
    }
}

impl<X> PartialEq for Call<X>
where
    X: Expr + PartialEq,
    X::Whitespace: PartialEq,
    <<X as Expr>::Node as Node>::Ident: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.generics == other.generics
    }
}

impl<X> Eq for Call<X>
where
    X: Expr + Eq,
    X::Whitespace: Eq,
    <<X as Expr>::Node as Node>::Ident: Eq,
{
}
