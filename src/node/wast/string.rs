//! Module that provides [`String`].

/// Type describing a string literal.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct String {
    inner: std::string::String
}

impl String {
    /// Creates a new `String`.
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}

impl<T: Into<std::string::String>> From<T> for String {
    fn from(value: T) -> Self {
        String::new(value.into())
    }
}
