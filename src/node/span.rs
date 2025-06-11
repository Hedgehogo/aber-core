//! Module that provides types for working with the arrangement of units within a document.

use chumsky::span::SimpleSpan;
use std::{fmt, ops::Range};

/// Type that describes the location of a unit within a document, storing the start and end position of that unit.
#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    pub range: Range<usize>,
}

impl Span {
    /// Creates a new `Span`, taking the end and start unit values.
    ///
    /// # Arguments
    /// * `range` End and start unit values as a range containing the offsets in the string in bytes.
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

/// Type that stores the unit and its location within the document.
#[derive(Clone, PartialEq, Eq)]
pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
    /// Creates a new Spanned, transforming the unit, keeping the span unchanged.
    ///
    /// # Arguments
    /// * `self` Previous unit value.
    /// * `f` Closure transforming unit.
    pub fn map<O, F: FnOnce(T) -> O>(self, f: F) -> Spanned<O> {
        Spanned(f(self.0), self.1)
    }

    /// Creates a vector containing a single element, `self`.
    pub fn into_vec(self) -> Vec<Self> {
        vec![self]
    }

    /// Creates a vector containing one element, `self`, and wraps it in `Spanned`, the vector's span is the same as `self`.
    pub fn into_spanned_vec(self) -> Spanned<Vec<Self>> {
        let span = self.1.clone();
        Spanned(vec![self], span)
    }
}

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

/// Trait providing the ability to wrap any type in [`Spanned`].
pub trait IntoSpanned<S: Into<Span>>: Sized {
    /// Creates a new `Spanned`.
    ///
    /// # Arguments
    /// * `self` Unit.
    /// * `span` Object of the type whose type is implements `Into<Span>`.
    fn into_spanned(self, span: S) -> Spanned<Self>;
}

impl<T, S: Into<Span>> IntoSpanned<S> for T {
    fn into_spanned(self, span: S) -> Spanned<Self> {
        Spanned(self, span.into())
    }
}

/// Vector consisting of [`Spanned`].
pub type SpannedVec<T> = Vec<Spanned<T>>;
