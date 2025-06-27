//! Module that provides [`String`].

use super::{
    escaped_string::{EscapedString, EscapedStringData},
    raw_string::RawString,
};
use crate::syntax::string::{self, EscapedStringCtx, RawStringCtx};

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

impl<'input> string::EscapedStringSealed<'input> for String<'input> {
    type Data = EscapedStringData;

    fn from_data_unchecked(
        data: Self::Data,
        inner_repr: &'input str,
        ctx: &EscapedStringCtx,
    ) -> Self {
        Self::Escaped(EscapedString::from_data_unchecked(data, inner_repr, ctx))
    }
}

impl<'input> string::EscapedString<'input> for String<'input> {}

impl<'input> string::RawStringSealed<'input> for String<'input> {
    type Data = ();

    fn from_data_unchecked(
        data: Self::Data,
        inner_repr: &'input str,
        ctx: &RawStringCtx<'input>,
    ) -> Self {
        Self::Raw(RawString::from_data_unchecked(data, inner_repr, ctx))
    }
}

impl<'input> string::RawString<'input> for String<'input> {}
