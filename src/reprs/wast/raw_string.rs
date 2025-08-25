//! Module that provides [`RawString`].

use crate::stages::parsing::string::{self, RawStringCtx};
use std::fmt;

/// Type describing a escaped string literal.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct RawString<'input> {
    inner_repr: &'input str,
    ctx: RawStringCtx<'input>,
}

impl<'input> RawString<'input> {
    /// Gets representation of the string between the opening
    /// sequence and the last line break as part of the raw string.
    pub fn inner_repr(&self) -> &'input str {
        self.inner_repr
    }

    /// Gets indentation (sequence W in the specification).
    pub fn indent(&self) -> &'input str {
        self.ctx.additional.additional.indent
    }

    /// Gets the number of line breaks between the opening sequence
    /// and the last line break as part of the raw string.
    pub fn line_break_count(&self) -> usize {
        self.ctx.additional.additional.line_break_count
    }

    /// Gets the length of the contents in bytes, if the string has
    /// errors, the maximum possible.
    pub fn capacity(&self) -> usize {
        self.ctx.additional.additional.capacity
    }

    /// Gets the number of quotation marks used to open and close a string.
    pub fn quotes_count(&self) -> usize {
        self.ctx.additional.quotes_count
    }

    /// Gets the number of doc comments within which parsing is performed.
    pub fn doc_depth(&self) -> usize {
        self.ctx.doc_ctx.depth()
    }

    /// Gets the context that the string had during parsing.
    pub fn ctx(&self) -> &RawStringCtx<'input> {
        &self.ctx
    }

    /// Gets an iterator over lines.
    pub fn lines<'str>(&'str self) -> LineIter<'str, 'input> {
        LineIter {
            rest: self.inner_repr,
            rest_length: self.line_break_count() + 1,
            string: self,
        }
    }
}

impl<'input> From<RawString<'input>> for String {
    fn from(value: RawString<'input>) -> Self {
        value.lines().fold(
            String::with_capacity(value.capacity()),
            |mut result, line| {
                result.push_str(line);
                result
            },
        )
    }
}

impl fmt::Debug for RawString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawString")
            .field("inner_repr", &self.inner_repr)
            .field("indent", &self.indent())
            .field("line_break_count", &self.line_break_count())
            .field("capacity", &self.capacity())
            .field("quotes_count", &self.quotes_count())
            .field("doc_depth", &self.doc_depth())
            .field("lines", &self.lines().collect::<Vec<_>>())
            .finish()
    }
}

/// Type describing an iterator over sections of an escaped string
/// literal.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LineIter<'str, 'input> {
    rest: &'input str,
    rest_length: usize,
    string: &'str RawString<'input>,
}

impl fmt::Debug for LineIter<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LineIter")
            .field("rest", &self.rest)
            .field("rest_length", &self.rest_length)
            .field("string", &self.string)
            .finish()
    }
}

impl<'str, 'input> Iterator for LineIter<'str, 'input> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::stages::parsing::parse::raw_string::{last_line, line};
        use chumsky::{extra::Err, prelude::*, text::Graphemes};

        match self.rest_length {
            0 => None,

            1 => {
                let result = last_line()
                    .with_ctx(self.string.ctx().clone())
                    .then_ignore(empty::<_, Err<EmptyErr>>())
                    .parse(Graphemes::new(self.rest))
                    .into_output()?;

                self.rest = "";
                self.rest_length = 0;

                Some(result.as_str())
            }

            _ => {
                let (result, rest) = line()
                    .with_ctx(self.string.ctx().clone())
                    .then(any::<_, Err<EmptyErr>>().repeated().to_slice())
                    .parse(Graphemes::new(self.rest))
                    .into_output()?;

                self.rest = rest.as_str();
                self.rest_length -= 1;

                Some(result.as_str())
            }
        }
    }
}

impl ExactSizeIterator for LineIter<'_, '_> {
    fn len(&self) -> usize {
        self.rest_length
    }
}

impl<'input> string::RawStringSealed<'input> for RawString<'input> {
    type Data = ();

    fn from_data_unchecked(
        _data: Self::Data,
        inner_repr: &'input str,
        ctx: &RawStringCtx<'input>,
    ) -> Self {
        Self {
            inner_repr,
            ctx: ctx.clone(),
        }
    }
}

impl<'input> string::RawString<'input> for RawString<'input> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_string() {
        let raw_string: RawString = string::RawStringSealed::from_data_unchecked(
            (),
            "\t  Hello\r\n\t   Aber!",
            &RawStringCtx::new_raw(Default::default(), 3, 13, 1, "\t  "),
        );

        assert_eq!(raw_string.capacity(), 13);
        assert_eq!(
            raw_string.lines().collect::<Vec<_>>(),
            vec!["Hello\r\n", " Aber!"]
        );
        assert_eq!(String::from(raw_string), "Hello\r\n Aber!");
    }
}
