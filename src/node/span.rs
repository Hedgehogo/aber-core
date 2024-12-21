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

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::new(value)
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
pub struct Spanned<T>(pub T, pub Span);

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} @ {:?}", self.0, self.1)
    }
}

impl<T, S: Into<Span>> From<(T, S)> for Spanned<T> {
    fn from(value: (T, S)) -> Self {
        Self(value.0, value.1.into())
    }
}
