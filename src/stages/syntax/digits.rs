/// Trait describing a type storing information about an
/// consecutive digits.
pub(crate) trait DigitsSealed<'input>: Default + Sized {
    /// Creates an instance of the type based on the representation.
    ///
    /// # Arguments
    /// - `inner_repr` Representation of digit sequence.
    ///
    /// # Safeguards
    /// The representation must be a valid digit sequence.
    ///
    /// The string must contain only the characters `0` - `9`, `A` - `Z` and `_`.
    fn from_repr_unchecked(repr: &'input str) -> Self;
}

/// Trait describing a type storing information about an
/// character literal.
#[expect(private_bounds)]
pub trait Digits<'input>: DigitsSealed<'input> + Default + Sized {}
