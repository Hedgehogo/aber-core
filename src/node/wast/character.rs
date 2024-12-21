use std::fmt;

use chumsky::text::Grapheme;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ascii(u8);

impl Ascii {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Character<'input> {
    inner: &'input Grapheme,
}

impl<'input> Character<'input> {
    pub fn new(inner: &'input Grapheme) -> Self {
        Self { inner }
    }
}
