//! Module that provides [`String`].

use super::super::string::{EscapedStringSealed, EscapedString, RawStringSealed, RawString};

/// Type describing the contents of a string literal.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct String(std::string::String);

impl String {
    /// Creates a new `String`.
    pub fn new(inner: std::string::String) -> Self {
        Self(inner)
    }

    /// Gets a slice of the string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<T: Into<std::string::String>> From<T> for String {
    fn from(value: T) -> Self {
        String::new(value.into())
    }
}

impl<'input> EscapedStringSealed<'input> for String {
    type Data = std::string::String;

    fn from_data_unchecked(data: Self::Data, _inner_repr: &'input str) -> Self {
        Self::new(data)
    }
}

impl EscapedString<'_> for String {}

impl<'input> RawStringSealed<'input> for String {
    type Data = std::string::String;

    fn from_data_unchecked(
        data: Self::Data,
        _indent: &'input str,
        _inner_repr: &'input str,
    ) -> Self {
        Self::new(data)
    }
}

impl RawString<'_> for String {}
