//! Module that provides [`Whitespace`].

use super::{wast_node::WastNode, Span, Spanned};
use crate::stages::parsing::whitespace;
use chumsky::text::{Char, Graphemes};

/// Type describing a whitespace.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Whitespace<'input> {
    repr: &'input str,
}

impl<'input> Whitespace<'input> {
    /// Creates a `Whitespace` from its representation.
    ///
    /// # Safeguards
    /// The representation must be valid whitespace.
    ///
    /// That is, contain only comments and whitespace as defined by the Unicode standard.
    pub(crate) fn from_repr_unchecked(repr: &'input str) -> Self {
        Self { repr }
    }

    /// Gets representation of a string between opening and closing
    /// sequences.
    pub fn repr(&self) -> &'input str {
        self.repr
    }

    pub fn is_empty(&self) -> bool {
        Graphemes::new(self.repr)
            .iter()
            .all(|i| i.is_inline_whitespace())
    }
}

impl<'input> Whitespace<'input> {
    /// Wraps in [`WastNode::Whitespace`] and then in [`Spanned`].
    ///
    /// # Arguments
    /// * `span` Object of the type whose type is implements `Into<Span>`.
    pub fn into_spanned_node<S: Into<Span>>(self, span: S) -> Spanned<WastNode<'input>> {
        Spanned(WastNode::Whitespace(self), span.into())
    }
}

impl<'input> whitespace::WhitespaceSealed<'input> for Whitespace<'input> {
    fn from_repr_unchecked(repr: &'input str) -> Self {
        Self::from_repr_unchecked(repr)
    }
}

impl<'input> whitespace::Whitespace<'input> for Whitespace<'input> {}
