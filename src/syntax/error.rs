use std::fmt::Debug;

use crate::node::{span::Span, wast::number::Radix};
use chumsky::{
    span::SimpleSpan,
    text::{Char, Grapheme, Graphemes},
    util::MaybeRef,
};
use smallvec::{smallvec, SmallVec};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Expected {
    #[default]
    Eof,
    Ascii(u8),
    Digit(Radix),
    Minus,
    Radix,
}

impl Debug for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "Eof"),
            Self::Ascii(i) => write!(f, "Ascii({:?})", *i as char),
            Self::Digit(i) => write!(f, "Digit({:?})", i),
            Self::Minus => write!(f, "Minus"),
            Self::Radix => write!(f, "Radix"),
        }
    }
}

type ExpectedVec = SmallVec<[Expected; 2]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'input> {
    expected: ExpectedVec,
    found: Option<&'input Grapheme>,
    span: Span,
}

impl<'input> Error<'input> {
    pub fn new(expected: ExpectedVec, found: Option<&'input Grapheme>, span: Span) -> Self {
        Self {
            expected,
            found,
            span,
        }
    }

    pub fn new_expected(expected: Expected, found: Option<&'input Grapheme>, span: Span) -> Self {
        let expected = smallvec![expected];
        Self {
            expected,
            found,
            span,
        }
    }

    pub fn replace_expected(self, expected: Expected) -> Self {
        Self::new_expected(expected, self.found, self.span)
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'input> chumsky::error::Error<'input, &'input Graphemes> for Error<'input> {
    fn expected_found<E>(
        expected: E,
        found: Option<MaybeRef<'input, &'input Grapheme>>,
        span: SimpleSpan,
    ) -> Self
    where
        E: IntoIterator<Item = Option<MaybeRef<'input, &'input Grapheme>>>,
    {
        let found = found.map(MaybeRef::into_inner);
        let expected = expected
            .into_iter()
            .filter_map(|i| match i.map(MaybeRef::into_inner) {
                Some(i) => i.to_ascii().map(Expected::Ascii),
                None => Some(Expected::Eof),
            })
            .collect();
        Self::new(expected, found, span.into())
    }

    fn merge(mut self, mut other: Self) -> Self {
        self.expected.append(&mut other.expected);
        self
    }
}
