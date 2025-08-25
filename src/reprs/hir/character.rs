//! Module that provides [`Character`].

use crate::stages::syntax::{self, character::CharacterSealed};
use chumsky::text::{Grapheme, Graphemes};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Character(Option<Box<Grapheme>>);

impl Character {
    /// Creates a new `Character`.
    pub fn new(inner_repr: &str) -> Self {
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

        Self(inner.map(Into::into))
    }

    /// Asks inner data.
    ///
    /// Returns `None` if the content is incorrect.
    pub fn inner(&self) -> Option<&Grapheme> {
        self.0.as_deref()
    }
}

impl Clone for Character {
    fn clone(&self) -> Self {
        Self(self.0.as_deref().map(Into::into))
    }
}

impl<'input> From<&'input Grapheme> for Character {
    fn from(value: &'input Grapheme) -> Self {
        Self(Some(value.into()))
    }
}

impl<'input> CharacterSealed<'input> for Character {
    fn from_repr_unchecked(inner_repr: &'input str, _close: bool) -> Self {
        Self::new(inner_repr)
    }
}

impl<'input> syntax::Character<'input> for Character {}
