//! Module providing abstractions related to string literals.

use super::{
    ctx::Ctx,
    parse::raw_string::{RawContentCtx, RawCtx},
};

pub type RawStringCtx<'input> = Ctx<RawCtx<RawContentCtx<'input>>>;
pub type EscapedStringCtx = Ctx<()>;

/// Trait describing some data from which a string can be created.
pub trait StringData<'input>: Sized {
    /// Creates data based on string capacity information.
    ///
    /// # Arguments
    /// - `capacity` String capacity in bytes.
    fn with_capacity(capacity: usize) -> Self;

    /// Creates data based on the previous data and adds information
    /// about the next section of the string to it.
    ///
    /// # Arguments
    /// - `section` The next section of the string.
    fn with_next_section(self, section: &'input str) -> Self;
}

impl<'input> StringData<'input> for std::string::String {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn with_next_section(mut self, section: &'input str) -> Self {
        std::string::String::push_str(&mut self, section);
        self
    }
}

impl<'input> StringData<'input> for () {
    fn with_capacity(_capacity: usize) -> Self {}

    fn with_next_section(self, _section: &'input str) -> Self {
        self
    }
}

/// Trait describing a type storing information about a string
/// literal containing escape sequences.
pub(crate) trait EscapedStringSealed<'input>: Sized {
    type Data: StringData<'input>;

    /// Creates an instance of the type based on the collected data
    /// about the string and its representation.
    ///
    /// # Arguments
    /// - `data` Collected string data.
    /// - `inner_repr` Representation of a string between opening and
    ///   closing sequences.
    /// - `ctx` .
    ///
    /// # Safeguards
    /// `inner_repr` must be a valid string representation, that is,
    /// it must not contain `\` or `"` characters outside of escape
    /// sequences and must contain only existing escape sequences.
    fn from_data_unchecked(
        data: Self::Data,
        inner_repr: &'input str,
        ctx: &EscapedStringCtx,
    ) -> Self;
}

/// Trait describing a type storing information about a string
/// literal containing escape sequences.
#[expect(private_bounds)]
pub trait EscapedString<'input>: EscapedStringSealed<'input> + Sized {}

/// Trait describing a type storing information about a raw string
/// literal.
pub(crate) trait RawStringSealed<'input>: Sized {
    type Data: StringData<'input>;

    /// Creates an instance of the type based on the collected data
    /// about the string, its representation and indentation.
    ///
    /// # Arguments
    /// - `data` Collected string data.
    /// - `inner_repr` Representation of the string between the
    ///   opening sequence and the last line break as part of the raw
    ///   string.
    /// - `ctx` .
    ///
    /// # Safeguards
    /// `indent` must be a valid indent, that is, it must contain
    /// only inline whitespace.
    ///
    /// `inner_repr` must be a valid representation of the raw
    /// string, that is, each line break must be followed by
    /// `indent`.
    fn from_data_unchecked(
        data: Self::Data,
        inner_repr: &'input str,
        ctx: &RawStringCtx<'input>,
    ) -> Self;
}

/// Trait describing a type storing information about a raw string
/// literal.
#[expect(private_bounds)]
pub trait RawString<'input>: RawStringSealed<'input> + Sized {}
