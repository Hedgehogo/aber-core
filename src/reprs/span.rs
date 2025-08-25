//! Module that provides types for working with the arrangement of units within a document.

use super::wast::whitespaced::Whitespaced;
use crate::stages::syntax::Expr;
use chumsky::span::SimpleSpan;
use std::{fmt, ops::Range};

/// Type that describes the location of a unit within a document, storing the start and end position of that unit.
#[derive(Default, Clone, PartialEq, Eq)]
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

    /// Gets start.
    pub fn start(&self) -> usize {
        self.range.start
    }

    /// Gets start.
    pub fn end(&self) -> usize {
        self.range.end
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

impl From<Span> for SimpleSpan {
    fn from(value: Span) -> Self {
        SimpleSpan::from(value.range)
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
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
    /// Gets inner value.
    pub fn inner(&self) -> &T {
        &self.0
    }

    /// Gets span.
    pub fn span(&self) -> Span {
        self.1.clone()
    }

    /// Creates a new Spanned, transforming the unit, keeping the span unchanged.
    ///
    /// # Arguments
    /// * `self` Previous unit value.
    /// * `f` Closure transforming unit.
    pub fn map<O, F: FnOnce(T) -> O>(self, f: F) -> Spanned<O> {
        let Self(inner, span) = self;
        Spanned(f(inner), span)
    }

    /// Converts from `&Spanned<T>` to `Spanned<&T>`.
    pub fn as_ref(&self) -> Spanned<&T> {
        let Self(inner, span) = self;
        Spanned(inner, span.clone())
    }

    /// Converts `Spanned` to inner value.
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Creates a vector containing a single element, `self`.
    pub fn into_vec(self) -> Vec<Self> {
        vec![self]
    }

    /// Creates a vector containing one element, `self`, and wraps it in `Spanned`, the vector's span is the same as `self`.
    pub fn into_spanned_vec(self) -> Spanned<Vec<Self>> {
        let span = self.span();
        Spanned(vec![self], span)
    }

    /// Creates [`Whitespaced`], passes `self` as an argument to `right`.
    pub fn into_whitespaced<X: Expr>(self, whitespace: X::Whitespace) -> Whitespaced<X, T> {
        Whitespaced::new(whitespace, self)
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner().fmt(f)?;
        f.write_str(" @ ")?;
        self.span().fmt(f)
    }
}

impl<T, S: Into<Span>> From<(T, S)> for Spanned<T> {
    fn from(value: (T, S)) -> Self {
        let (inner, span) = value;
        Self(inner, span.into())
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
