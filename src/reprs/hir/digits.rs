//! Module that provides [`Digits`].

use super::super::wast::number::{self, Digit};
use crate::stages::syntax::{self, digits::DigitsSealed};
use std::fmt::Debug;

/// Type describing consecutive digits.
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Digits(Vec<Digit>);

impl Digits {
    /// Creates a sequence of numbers from a string without checking the string for validity.
    ///
    /// # Safeguards
    /// The string must contain only the characters `0` - `9`, `A` - `Z` and `_`.
    pub fn from_repr_unchecked(repr: &str) -> Self {
        let digits = number::Digits::from_repr_unchecked(repr);
        let iter = digits.iter();
        Self(iter.collect())
    }

    /// Gets the iterator by digits.
    pub fn iter(&self) -> std::slice::Iter<Digit> {
        self.0.as_slice().iter()
    }
}

impl<'input> DigitsSealed<'input> for Digits {
    fn from_repr_unchecked(repr: &'input str) -> Self {
        Self::from_repr_unchecked(repr)
    }
}

impl<'input> syntax::Digits<'input> for Digits {}
