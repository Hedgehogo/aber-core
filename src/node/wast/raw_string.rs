//! Module that provides [`RawString`].

use super::super::string;
use chumsky::text::{Char, Graphemes};
use std::fmt;

/// Type describing a escaped string literal.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct RawString<'input> {
    indent: &'input str,
    inner_repr: &'input str,
    line_break_count: usize,
    capacity: usize,
}

impl<'input> RawString<'input> {
    /// Gets representation of the string between the opening
    /// sequence and the last line break as part of the raw string.
    pub fn inner_repr(&self) -> &'input str {
        self.inner_repr
    }

    /// Gets indentation (sequence W in the specification).
    pub fn indent(&self) -> &'input str {
        self.indent
    }

    /// Gets the number of line breaks between the opening sequence
    /// and the last line break as part of the raw string.
    pub fn line_break_count(&self) -> usize {
        self.line_break_count
    }

    /// Gets the length of the contents in bytes, if the string has
    /// errors, the maximum possible.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gets an iterator over lines.
    pub fn lines(&self) -> LineIter<'input> {
        LineIter {
            rest: self.inner_repr,
            indent_length: self.indent.len(),
            length: self.line_break_count + 1,
        }
    }
}

impl<'input> From<RawString<'input>> for String {
    fn from(value: RawString<'input>) -> Self {
        value
            .lines()
            .fold(String::with_capacity(value.capacity), |mut result, line| {
                result.push_str(line);
                result
            })
    }
}

impl fmt::Debug for RawString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawString")
            .field("indent", &self.indent)
            .field("inner_repr", &self.inner_repr)
            .field("line_break_count", &self.line_break_count)
            .field("capacity", &self.capacity)
            .field("lines", &self.lines().collect::<Vec<_>>())
            .finish()
    }
}

/// Type describing an iterator over sections of an escaped string
/// literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineIter<'input> {
    rest: &'input str,
    indent_length: usize,
    length: usize,
}

impl<'input> Iterator for LineIter<'input> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length != 0 {
            let (_, rest) = self.rest.split_at_checked(self.indent_length)?;
            let iter = Graphemes::new(rest).iter();

            let line = if self.length != 1 {
                let line_length = iter
                    .scan(0usize, |length, i| {
                        *length += i.as_bytes().len();
                        Some((*length, i))
                    })
                    .find(|(_, i)| i.is_newline())
                    .map(|(length, _)| length)?;

                let (line, rest) = rest.split_at(line_length);

                self.rest = rest;
                line
            } else {
                self.rest = "";
                rest
            };

            self.length -= 1;

            Some(line)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for LineIter<'_> {
    fn len(&self) -> usize {
        self.length
    }
}

/// Type that collects string data for [`RawString`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawStringData {
    section_count: usize,
    capacity: usize,
}

impl<'input> string::StringData<'input> for RawStringData {
    fn with_capacity(_capacity: usize) -> Self {
        RawStringData {
            section_count: 0,
            capacity: 0,
        }
    }

    fn with_next_section(mut self, section: &'input str) -> Self {
        self.capacity += section.len();
        self.section_count += 1;
        self
    }
}

impl<'input> string::RawString<'input> for RawString<'input> {
    type Data = RawStringData;

    unsafe fn from_data_unchecked(
        data: Self::Data,
        indent: &'input str,
        inner_repr: &'input str,
    ) -> Self {
        Self {
            indent,
            inner_repr,
            line_break_count: data.section_count / 2,
            capacity: data.capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_string() {
        let raw_string: RawString = unsafe {
            string::RawString::from_data_unchecked(
                RawStringData {
                    section_count: 3,
                    capacity: 13,
                },
                "\t  ",
                "\t  Hello\r\n\t   Aber!",
            )
        };

        assert_eq!(raw_string.capacity(), 13);
        assert_eq!(
            raw_string.lines().collect::<Vec<_>>(),
            vec!["Hello\r\n", " Aber!"]
        );
        assert_eq!(String::from(raw_string), "Hello\r\n Aber!");
    }
}
