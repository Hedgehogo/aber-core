use std::{fmt, ops::Range};

use chumsky::span::SimpleSpan;

#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    pub range: Range<usize>,
}

impl Span {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

impl From<SimpleSpan> for Span {
    fn from(value: SimpleSpan) -> Self {
        Self::new(value.into_range())
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.range.start, self.range.end)
    }
}

impl chumsky::span::Span for Span {
    type Context = ();

    type Offset = usize;

    fn new(_context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self::new(range)
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        self.range.start
    }

    fn end(&self) -> Self::Offset {
        self.range.end
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Spanned<T>(Span, T);

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} @ {:?}", self.1, self.0)
    }
}
