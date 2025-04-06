//! Module that provides [`Whitespace`].

/// Type describing a whitespace.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Whitespace<'input> {
    repr: &'input str,
}

impl<'input> Whitespace<'input> {
    /// Creates a `Whitespace` from its representation.
    ///
    /// The representation must be valid whitespace.
    pub fn from_repr_unchecked(repr: &'input str) -> Self {
        Self { repr }
    }

    /// Gets representation of a string between opening and closing
    /// sequences.
    pub fn repr(&self) -> &'input str {
        self.repr
    }
}
