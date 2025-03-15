//! Module that provides [`String`].

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
