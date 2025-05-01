//! Module that provides [`Whitespace`].

use super::super::whitespace;

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
}

impl<'input> whitespace::Whitespace<'input> for Whitespace<'input> {
    fn from_repr_unchecked(repr: &'input str) -> Self {
        Self::from_repr_unchecked(repr)
    }
}
