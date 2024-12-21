use std::fmt::Debug;

use crate::node::{
    span::Span,
    wast::{character::Ascii, number::Radix},
};
use chumsky::{
    span::SimpleSpan,
    text::{Char, Grapheme, Graphemes},
    util::MaybeRef,
};
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expected {
    Ascii(Ascii),
    Minus,
    Digit(Radix),
    Radix,
    RadixSpecial,
    NumberDot,
    NumberSpacer,
    CharSpecial,
    CharEscape,
    CharEscaped,
    CharUnescaped,
    StringSpecial,
    StringEscape,
    StringEscaped,
    StringUnescaped,
    RawStringStart,
    RawStringEnd,
    RawStringIndent,
    NonZeroWhitespace,
    #[default]
    Eof,
}

type ExpectedVec = SmallVec<[Expected; 2]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'input> {
    expected: ExpectedVec,
    found: Option<&'input Grapheme>,
    span: Span,
}

impl<'input> Error<'input> {
    pub fn new(mut expected: ExpectedVec, found: Option<&'input Grapheme>, span: Span) -> Self {
        expected.sort();
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

    pub fn expected(&self) -> &[Expected] {
        self.expected.as_slice()
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
                Some(i) => i
                    .to_ascii()
                    .map(|i| Expected::Ascii(Ascii::new(i).unwrap())),

                None => Some(Expected::Eof),
            })
            .collect();
        
        Self::new(expected, found, span.into())
    }

    fn merge(mut self, other: Self) -> Self {
        self.expected = merge_sorted_vec(self.expected, other.expected);
        self
    }
}

fn merge_sorted_vec<T: PartialOrd>(
    mut first: SmallVec<[T; 2]>,
    second: SmallVec<[T; 2]>,
) -> SmallVec<[T; 2]> {
    first.reserve(second.len());
    let mut i = 0usize;
    for item in second.into_iter() {
        while first.get(i).is_some_and(|i| *i < item) {
            i += 1;
        }
        if !first.get(i).is_some_and(|i| *i == item) {
            first.insert(i, item);
            i += 1;
        }
    }
    first
}

mod tests {
    use super::*;

    #[test]
    fn test_merge_expected_vec() {
        let first = smallvec![1, 4, 7];
        let second = smallvec![3, 4, 6];
        let result: SmallVec<[i32; 2]> = smallvec![1, 3, 4, 6, 7];
        assert_eq!(merge_sorted_vec(first, second), result);

        let first = smallvec![2, 3];
        let second = smallvec![3, 4, 6];
        let result: SmallVec<[i32; 2]> = smallvec![2, 3, 4, 6];
        assert_eq!(merge_sorted_vec(first, second), result);

        let first = smallvec![];
        let second = smallvec![3, 4, 6];
        let result: SmallVec<[i32; 2]> = smallvec![3, 4, 6];
        assert_eq!(merge_sorted_vec(first, second), result);
    }
}
