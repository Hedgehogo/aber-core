/// Trait describing a type storing information about an
/// character literal.
pub(crate) trait CharacterSealed<'input>: Sized {
    /// Creates an instance of the type based on the representation.
    ///
    /// # Arguments
    /// - `inner_repr` Representation of a character between opening and
    ///   closing sequences.
    /// - `close` Is the closing sequence presented.
    ///
    /// # Safeguards
    /// The representation must be a valid character inner.
    ///
    /// That is, meet one of the following requirements:
    /// - It must be exactly one grapheme claster.
    /// - It must start with `\` and end with valid escape sequence.
    fn from_repr_unchecked(inner_repr: &'input str, close: bool) -> Self;
}

/// Trait describing a type storing information about an
/// character literal.
#[expect(private_bounds)]
pub trait Character<'input>: CharacterSealed<'input> + Sized {}
