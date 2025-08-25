/// Trait describing a type storing information about an
/// identifier.
pub(crate) trait IdentSealed<'input, S>: Sized {
    /// Creates an instance of the type based on the representation.
    ///
    /// # Arguments
    /// - `state` An external state that can save some data. For
    ///   example, string interner.
    /// - `repr` Representation of an identifier.
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
    fn from_repr_unchecked(state: &mut S, repr: &'input str) -> Self;
}

/// Trait describing a type storing information about an
/// identifier.
#[expect(private_bounds)]
pub trait Ident<'input, S>: IdentSealed<'input, S> + Sized {}
