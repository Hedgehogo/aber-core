//! Module that provides [`String`].

use super::super::string;
use super::{
    escaped_string::{EscapedString, EscapedStringData},
    raw_string::{RawString, RawStringData},
};

/// Type describing a string literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum String<'input> {
    Escaped(EscapedString<'input>),
    Raw(RawString<'input>),
}

impl String<'_> {
    /// Gets the length of the contents in bytes, if the string has
    /// errors, the maximum possible.
    pub fn capacity(&self) -> usize {
        match self {
            String::Escaped(i) => i.capacity(),
            String::Raw(i) => i.capacity(),
        }
    }
}

impl<'input> From<String<'input>> for std::string::String {
    fn from(value: String<'input>) -> Self {
        match value {
            String::Escaped(i) => i.into(),
            String::Raw(i) => i.into(),
        }
    }
}

impl<'input> string::EscapedString<'input> for String<'input> {
    type Data = EscapedStringData;

    unsafe fn from_data_unchecked(data: Self::Data, inner_repr: &'input str) -> Self {
        Self::Escaped(EscapedString::from_data_unchecked(data, inner_repr))
    }
}

impl<'input> string::RawString<'input> for String<'input> {
    type Data = RawStringData;

    unsafe fn from_data_unchecked(
        data: Self::Data,
        indent: &'input str,
        inner_repr: &'input str,
    ) -> Self {
        Self::Raw(RawString::from_data_unchecked(data, indent, inner_repr))
    }
}
