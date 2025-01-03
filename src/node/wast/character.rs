//! Module that provides types for character literal description.

use std::fmt;
use chumsky::text::Grapheme;

/// Type describing a set of ASCII characters.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ascii(u8);

impl Ascii {
    /// Creates a new `Ascii` from `u8` if it contains a value less than `128`, otherwise returns `None`.
    pub fn new(inner: u8) -> Option<Self> {
        if inner.is_ascii() {
            Some(Self(inner))
        } else {
            None
        }
    }
}

impl From<Ascii> for u8 {
    fn from(value: Ascii) -> Self {
        value.0
    }
}

impl fmt::Debug for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0 as char)
    }
}

/// Type describing a character literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Character<'input> {
    inner: &'input Grapheme,
}

impl<'input> Character<'input> {
    /// Creates a new `Character`.
    pub fn new(inner: &'input Grapheme) -> Self {
        Self { inner }
    }
}

impl<'input> From<&'input Grapheme> for Character<'input> {
    fn from(value: &'input Grapheme) -> Self {
        Character::new(value)
    }
}
