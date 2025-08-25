//! Module that provides types for number literal description.

use crate::stages::parsing::{self, digits::DigitsSealed};
use std::fmt::Debug;

/// Type guarantee that contains the base of a numeral system in the range of `2` to `36` inclusive.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Radix {
    inner: u8,
}

impl Radix {
    pub const BINARY: Self = Self { inner: 2 };
    pub const OCTAL: Self = Self { inner: 8 };
    pub const DECIMAL: Self = Self { inner: 10 };
    pub const HEXADECIMAL: Self = Self { inner: 16 };
    pub const MAX: Self = Self { inner: 36 };

    /// Creates a new base of the numeral system from a number, returns `None` if unsuccessful.
    pub fn new(inner: u8) -> Option<Self> {
        match inner {
            2..=36 => Some(Self { inner }),
            _ => None,
        }
    }

    /// Returns the maximum digit that can be written in this number system.
    pub fn max(self) -> Digit {
        Digit {
            inner: self.inner - 1,
        }
    }
}

impl Default for Radix {
    fn default() -> Self {
        Self::DECIMAL
    }
}

impl From<Radix> for u8 {
    fn from(value: Radix) -> Self {
        value.inner
    }
}

impl Debug for Radix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}'", self.inner)
    }
}

/// Type guarantee that contains a digit from `0` to `35` (`Z` in base-36 numeral system) inclusive.
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Digit {
    inner: u8,
}

impl Digit {
    pub const ZERO: Self = Self { inner: 0 };

    /// Creates a digit from a number, returns `None` if unsuccessful.
    pub fn new(inner: u8) -> Option<Self> {
        match inner {
            0..36 => Some(Self { inner }),
            _ => None,
        }
    }

    /// Creates a digit from an ascii character specifying a numeral system.
    pub fn from_ascii(ascii: u8, radix: Radix) -> Option<Self> {
        let radix = Into::<u8>::into(radix) as u32;
        // If not a digit, a number greater than radix will be created.
        let mut digit = (ascii as u32).wrapping_sub('0' as u32);
        if radix > 10 {
            if digit < 10 {
                return Some(Self { inner: digit as u8 });
            }
            digit = (ascii as u32).wrapping_sub('A' as u32).saturating_add(10);
        }
        (digit < radix).then_some(Self { inner: digit as u8 })
    }

    /// Returns the minimum numeral system in which the digit can be written.
    pub fn min_radix(self) -> Radix {
        let inner = match self.inner {
            0 => 2,
            i => i + 1,
        };
        Radix { inner }
    }
}

impl From<Digit> for u8 {
    fn from(value: Digit) -> Self {
        value.inner
    }
}

impl Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

/// Type describing consecutive digits.
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Digits<'input> {
    repr: &'input str,
}

impl<'input> Digits<'input> {
    /// Creates a sequence of numbers from a string without checking the string for validity.
    ///
    /// # Safeguards
    /// The string must contain only the characters `0` - `9`, `A` - `Z` and `_`.
    pub fn from_repr_unchecked(repr: &'input str) -> Self {
        Self { repr }
    }

    /// Gets a slice of code points, which is a representation of a sequence of digits.
    pub fn as_str(&self) -> &'input str {
        self.repr
    }
    /// Gets the iterator by digits.
    pub fn iter(&self) -> DigitIter<'input> {
        DigitIter::new(self.repr.as_bytes().iter())
    }
}

impl AsRef<str> for Digits<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for Digits<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Digits({:?})", self.repr)
    }
}

impl<'input> DigitsSealed<'input> for Digits<'input> {
    fn from_repr_unchecked(repr: &'input str) -> Self {
        Self::from_repr_unchecked(repr)
    }
}

impl<'input> parsing::Digits<'input> for Digits<'input> {}

#[derive(Debug, Clone)]
pub struct DigitIter<'input> {
    iter: std::slice::Iter<'input, u8>,
}

impl<'input> DigitIter<'input> {
    fn new(iter: std::slice::Iter<'input, u8>) -> Self {
        Self { iter }
    }

    fn iter(&mut self) -> impl DoubleEndedIterator<Item = Digit> + use<'_, 'input> {
        (&mut self.iter)
            .filter(|&i| (*i != b'_'))
            .map(|i| Digit::from_ascii(*i, Radix::MAX).unwrap())
    }
}

impl Iterator for DigitIter<'_> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter().next()
    }
}

impl DoubleEndedIterator for DigitIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter().next_back()
    }
}

/// Type that stores information about how a number was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Number<D> {
    pub sign: bool,
    pub radix: Radix,
    pub int: D,
    pub frac: Option<D>,
}

impl<D> Number<D> {
    pub fn new(sign: bool, radix: Radix, int: D, frac: Option<D>) -> Self {
        Self {
            sign,
            radix,
            int,
            frac,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix() {
        assert_eq!(Radix::new(0), None);
        assert_eq!(u8::from(Radix::new(2).unwrap()), 2);
        assert_eq!(u8::from(Radix::new(36).unwrap()), 36);
        assert_eq!(Radix::new(37), None);
    }

    #[test]
    fn test_digit() {
        assert_eq!(u8::from(Digit::new(0).unwrap()), 0);
        assert_eq!(u8::from(Digit::new(35).unwrap()), 35);
        assert_eq!(Digit::new(36), None);

        assert_eq!(u8::from(Digit::new(0).unwrap().min_radix()), 2);
        assert_eq!(u8::from(Digit::new(1).unwrap().min_radix()), 2);
        assert_eq!(u8::from(Digit::new(35).unwrap().min_radix()), 36);
    }

    #[test]
    fn test_digits() {
        let digits = Digits::from_repr_unchecked("0_12_");
        assert_eq!(
            digits.iter().map(u8::from).collect::<Vec<_>>(),
            vec![0, 1, 2]
        )
    }
}
