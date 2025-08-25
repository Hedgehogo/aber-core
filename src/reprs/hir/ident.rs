//! Module that provides [`Ident`].

use super::super::{
    mir::State,
    span::{IntoSpanned, Spanned},
    wast::call::{Call, Ident as WastIdent},
};
use crate::stages::syntax::{self, ident::IdentSealed, Expr, Node};
use string_interner::{DefaultStringInterner, DefaultSymbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident(DefaultSymbol);

impl Ident {
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
    pub(crate) fn from_repr_unchecked(interner: &mut DefaultStringInterner, repr: &str) -> Self {
        Self(interner.get_or_intern(repr))
    }
}

impl Spanned<Ident> {
    pub fn into_spanned_call<X: Expr>(self) -> Spanned<Call<X>>
    where
        X::Node: Node<Ident = Ident>,
    {
        let span = self.span();
        Call::new(self, None).into_spanned(span)
    }
}

impl<'input> IdentSealed<'input, DefaultStringInterner> for Ident {
    fn from_repr_unchecked(state: &mut DefaultStringInterner, repr: &'input str) -> Self {
        Self::from_repr_unchecked(state, repr)
    }
}

impl<'input> IdentSealed<'input, State> for Ident {
    fn from_repr_unchecked(state: &mut State, repr: &'input str) -> Self {
        state.add_ident(WastIdent::from_repr_unchecked(repr))
    }
}

impl syntax::Ident<'_, DefaultStringInterner> for Ident {}
