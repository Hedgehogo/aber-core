//! Module that provides [`String`].

use super::super::string::{EscapedString, RawString};

/// Type describing a string literal.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct String {
    inner: std::string::String,
}

impl String {
    /// Creates a new `String`.
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }

    /// Creates a new `String`.
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl<T: Into<std::string::String>> From<T> for String {
    fn from(value: T) -> Self {
        String::new(value.into())
    }
}

impl<'input> EscapedString<'input> for String {
    type Data = std::string::String;

    unsafe fn from_data_unchecked(data: Self::Data, _inner_repr: &'input str) -> Self {
        Self::new(data)
    }
}

impl<'input> RawString<'input> for String {
    type Data = std::string::String;

    unsafe fn from_data_unchecked(
        data: Self::Data,
        _indent: &'input str,
        _inner_repr: &'input str,
    ) -> Self {
        Self::new(data)
    }
}
