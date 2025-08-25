//! Module that provides types for character literal description.

use crate::stages::parsing::{self, character::CharacterSealed};
use chumsky::text::{Grapheme, Graphemes};
use std::fmt;

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
        (self.0 as char).fmt(f)
    }
}

/// Type describing a character literal.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Character<'input> {
    inner: Option<&'input Grapheme>,
    inner_repr: &'input str,
    close: bool,
}

impl<'input> Character<'input> {
    /// Creates a new `Character`.
    pub fn new(inner_repr: &'input str, close: bool) -> Self {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        let mut iter = Graphemes::new(inner_repr).iter().map(Grapheme::as_str);
        let inner = match (iter.next(), iter.next(), iter.next()) {
            (Some(_), Some(_), Some(_)) => None,

            (Some("\\"), Some(escape), _) => match escape {
                "\\" => Some(grapheme("\\")),
                "n" => Some(grapheme("\n")),
                "t" => Some(grapheme("\t")),
                _ => None,
            },

            (Some(character), _, _) => match character {
                "\\" => None,
                _ => Some(grapheme(character)),
            },

            (_, _, _) => None,
        };

        Self {
            inner,
            inner_repr,
            close,
        }
    }

    /// Creates a new `Character` from inner representation.
    pub fn from_repr(repr: &'input str) -> Self {
        Self::new(repr, true)
    }

    /// Asks inner representation.
    pub fn inner_repr(&self) -> &'input str {
        self.inner_repr
    }

    /// Asks inner data.
    ///
    /// Returns `None` if the content is incorrect.
    pub fn inner(&self) -> Option<&'input Grapheme> {
        self.inner
    }

    /// Asks if the closing `'` was present.
    pub fn is_closed(&self) -> bool {
        self.close
    }
}

impl<'input> Default for Character<'input> {
    fn default() -> Self {
        Self {
            inner: None,
            inner_repr: "",
            close: true,
        }
    }
}

impl<'input> fmt::Debug for Character<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Character");

        match self.inner {
            Some(inner) if inner.as_str() != self.inner_repr => {
                debug.field("inner", &inner).field("repr", &self.inner_repr)
            }

            Some(inner) => debug.field("inner", &inner),

            None => debug.field("repr", &self.inner_repr),
        }
        .field("close", &self.close)
        .finish()
    }
}

impl<'input> From<&'input Grapheme> for Character<'input> {
    fn from(value: &'input Grapheme) -> Self {
        let inner_repr = match value.as_str() {
            "\n" => "\\n",
            "\t" => "\\t",
            "\\" => "\\\\",
            _ => value.as_str(),
        };

        Self {
            inner: Some(value),
            inner_repr,
            close: true,
        }
    }
}

impl<'input> CharacterSealed<'input> for Character<'input> {
    fn from_repr_unchecked(inner_repr: &'input str, close: bool) -> Self {
        Self::new(inner_repr, close)
    }
}

impl<'input> parsing::Character<'input> for Character<'input> {}
