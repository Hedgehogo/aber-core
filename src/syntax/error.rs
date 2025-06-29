//! Module that provides types describing parsing error.

use std::fmt::Debug;

use crate::node::{span::Span, wast::number::Radix};
use chumsky::{
    span::SimpleSpan,
    text::{Grapheme, Graphemes},
    util::MaybeRef,
    DefaultExpected,
};
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expected {
    Number,
    Digit(Radix),
    Radix,
    RadixSpecial,
    NumberDot,
    NumberSpacer,
    Char,
    CharClose,
    CharEscape,
    CharEscaped,
    CharUnescaped,
    String,
    StringClose,
    StringEscape,
    StringEscaped,
    StringUnescaped,
    RawString,
    RawStringClose,
    RawStringIndent,
    Ident,
    ValidIdent,
    Tuple,
    TupleClose,
    Block,
    BlockClose,
    Generics,
    GenericsClose,
    Initialization,
    InitializationClose,
    Comma,
    Semicolon,
    PairSpecial,
    MethodSpecial,
    ChildSpecial,
    NegativeSpecial,
    AssignSpecial,
    Fact,
    Expr,
    Stmt,
    DocOuter,
    Comment,
    NonZeroWhitespace,
    Eof,
    #[default]
    Other,
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

    pub fn found(&self) -> Option<&'input Grapheme> {
        self.found
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<'input>
    chumsky::error::LabelError<'input, &'input Graphemes, DefaultExpected<'input, &'input Grapheme>>
    for Error<'input>
{
    fn expected_found<E>(
        expected: E,
        found: Option<
            MaybeRef<'input, <&'input Graphemes as chumsky::prelude::Input<'input>>::Token>,
        >,
        span: <&'input Graphemes as chumsky::prelude::Input<'input>>::Span,
    ) -> Self
    where
        E: IntoIterator<Item = DefaultExpected<'input, &'input Grapheme>>,
    {
        let found = found.map(MaybeRef::into_inner);
        let expected = expected
            .into_iter()
            .map(|i| match i.to_owned() {
                DefaultExpected::EndOfInput => Expected::Eof,
                _ => Expected::Other,
            })
            .collect();

        Self::new(expected, found, span.into())
    }
}

impl<'input> chumsky::error::LabelError<'input, &'input Graphemes, Expected> for Error<'input> {
    fn expected_found<E>(
        expected: E,
        found: Option<
            MaybeRef<'input, <&'input Graphemes as chumsky::prelude::Input<'input>>::Token>,
        >,
        span: <&'input Graphemes as chumsky::prelude::Input<'input>>::Span,
    ) -> Self
    where
        E: IntoIterator<Item = Expected>,
    {
        let found = found.map(MaybeRef::into_inner);
        Self::new(expected.into_iter().collect(), found, span.into())
    }

    fn label_with(&mut self, label: Expected) {
        if label == Expected::Comment {
            self.expected = smallvec![];
        } else {
            self.expected = smallvec![label];
        }
    }

    fn in_context(&mut self, label: Expected, span: SimpleSpan) {
        match label {
            Expected::Number
            | Expected::Char
            | Expected::CharEscape
            | Expected::String
            | Expected::StringEscape
            | Expected::RawString
            | Expected::Tuple
            | Expected::Block
            | Expected::Generics
            | Expected::Initialization
            | Expected::DocOuter
            | Expected::Fact
            | Expected::Expr
            | Expected::Stmt => {}

            Expected::Comment => {
                self.expected = smallvec![];
                self.span = span.into();
            }

            _ => {
                self.expected = smallvec![label];
                self.span = span.into();
                return;
            }
        }

        match self.expected.as_slice() {
            [Expected::Other] | [] => {
                self.expected = smallvec![label];
                self.span = span.into();
            }
            _ => {}
        }
    }
}

impl<'input> chumsky::error::Error<'input, &'input Graphemes> for Error<'input> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_vec() {
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
