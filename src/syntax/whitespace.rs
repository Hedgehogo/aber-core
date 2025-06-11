//! Module providing abstractions related to whitespace.

/// Trait describing a type that stores information about whitespace
/// content (amount of indentation, comments, documentation
/// comments).
pub(crate) trait WhitespaceSealed<'input>: Sized {
    /// Creates a whitespace from a sequence of nodes.
    ///
    /// # Arguments
    /// - `repr` Representation of a whitespace.
    ///
    /// # Safeguards
    /// The representation must be valid whitespace.
    ///
    /// That is, contain only comments and whitespace as defined by
    /// the Unicode standard.
    fn from_repr_unchecked(repr: &'input str) -> Self;
}

impl<'input> WhitespaceSealed<'input> for () {
    fn from_repr_unchecked(_repr: &'input str) -> Self {}
}

/// Trait describing a type that stores information about whitespace
/// content (amount of indentation, comments, documentation
/// comments).
#[expect(private_bounds)]
pub trait Whitespace<'input>: WhitespaceSealed<'input> + Sized {}

impl Whitespace<'_> for () {}

pub enum Side {
    Right,
    Left,
}
