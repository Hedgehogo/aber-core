//! Module that provides [`EscapedString`].

use super::super::string;
use chumsky::text::{Char, Graphemes};
use std::fmt;

/// Type describing a escaped string literal.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct EscapedString<'input> {
    inner_repr: &'input str,
    section_count: usize,
    capacity: usize,
}

impl<'input> EscapedString<'input> {
    /// Gets representation of a string between opening and closing
    /// sequences.
    pub fn inner_repr(&self) -> &'input str {
        self.inner_repr
    }

    /// Gets the length of the contents in bytes, if the string has
    /// errors, the maximum possible.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Gets an iterator over sections.
    pub fn sections(&self) -> SectionIter<'input> {
        SectionIter {
            rest: self.inner_repr,
            length: self.section_count,
        }
    }
}

impl<'input> From<EscapedString<'input>> for String {
    fn from(value: EscapedString<'input>) -> Self {
        value.sections().fold(
            String::with_capacity(value.capacity),
            |mut result, section| {
                match section {
                    Section::Escape(repr) => match Escape::from_repr(repr) {
                        Some(escape) => {
                            escape.content().inspect(|i| result.push(*i));
                        }

                        None => result.push('\u{FFFD}'),
                    },

                    Section::Characters(repr) => result.push_str(repr),
                }

                result
            },
        )
    }
}

impl fmt::Debug for EscapedString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EscapedString")
            .field("inner_repr", &self.inner_repr)
            .field("section_count", &self.section_count)
            .field("capacity", &self.capacity)
            .field("sections", &self.sections().collect::<Vec<_>>())
            .finish()
    }
}

/// Type describing an escape sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Escape {
    Quote,
    Slash,
    Newline,
    Tab,
    Nothing,
}

impl Escape {
    /// Creates an `Escape` from an escape sequence written as a
    /// string.
    pub fn from_repr(repr: &str) -> Option<Self> {
        match repr {
            "\\\"" => Some(Escape::Quote),
            "\\\\" => Some(Escape::Slash),
            "\\n" => Some(Escape::Newline),
            "\\t" => Some(Escape::Tab),
            "\\\n" => Some(Escape::Nothing),
            _ => None,
        }
    }

    /// Gets the contents of the escape sequence. `None` means that
    /// the content is a sequence of zero-length code points.
    pub fn content(&self) -> Option<char> {
        match self {
            Escape::Quote => Some('\"'),
            Escape::Slash => Some('\\'),
            Escape::Newline => Some('\n'),
            Escape::Tab => Some('\t'),
            Escape::Nothing => None,
        }
    }

    /// Gets a representation of the escape sequence.
    pub fn repr(&self) -> &'static str {
        match self {
            Escape::Quote => "\\\"",
            Escape::Slash => "\\\\",
            Escape::Newline => "\\n",
            Escape::Tab => "\\t",
            Escape::Nothing => "\\\n",
        }
    }
}

/// Type describing a single segment of the escaped string, i.e.,
/// either an escape sequence or a sequence of characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section<'input> {
    Escape(&'input str),
    Characters(&'input str),
}

/// Type describing an iterator over sections of an escaped string
/// literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionIter<'input> {
    rest: &'input str,
    length: usize,
}

impl<'input> Iterator for SectionIter<'input> {
    type Item = Section<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = Graphemes::new(self.rest).iter();
        let mut rest = self.rest;

        let is_escape = if iter.next()?.to_ascii() == Some(b'\\') {
            iter.next();
            rest = iter.clone().as_str();
            true
        } else {
            while iter
                .next()
                .filter(|i| i.to_ascii().filter(|i| *i == b'\\').is_none())
                .is_some()
            {
                rest = iter.clone().as_str();
            }
            false
        };

        let len = self.rest.len() - rest.len();
        let (section, _) = self.rest.split_at(len);

        self.rest = rest;
        self.length -= 1;

        let section = if is_escape {
            Section::Escape(section)
        } else {
            Section::Characters(section)
        };

        Some(section)
    }
}

impl ExactSizeIterator for SectionIter<'_> {
    fn len(&self) -> usize {
        self.length
    }
}

/// Type that collects string data for `EscapedString`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscapedStringData {
    section_count: usize,
    capacity: usize,
}

impl<'input> string::StringData<'input> for EscapedStringData {
    fn with_capacity(_capacity: usize) -> Self {
        EscapedStringData {
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

impl<'input> string::EscapedStringSealed<'input> for EscapedString<'input> {
    type Data = EscapedStringData;

    fn from_data_unchecked(data: Self::Data, inner_repr: &'input str) -> Self {
        Self {
            inner_repr,
            section_count: data.section_count,
            capacity: data.capacity,
        }
    }
}

impl<'input> string::EscapedString<'input> for EscapedString<'input> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escaped_string() {
        {
            let escaped_string: EscapedString = string::EscapedStringSealed::from_data_unchecked(
                EscapedStringData {
                    section_count: 5,
                    capacity: 13,
                },
                r#"Hello\n\mAber!\"#,
            );

            assert_eq!(escaped_string.capacity(), 13);
            assert_eq!(escaped_string.sections().len(), 5);

            let mut sections = escaped_string.sections();
            assert_eq!(sections.next(), Some(Section::Characters(r#"Hello"#)));
            assert_eq!(sections.next(), Some(Section::Escape(r#"\n"#)));
            assert_eq!(sections.next(), Some(Section::Escape(r#"\m"#)));
            assert_eq!(sections.next(), Some(Section::Characters(r#"Aber!"#)));
            assert_eq!(sections.next(), Some(Section::Escape(r#"\"#)));
            assert_eq!(sections.next(), None);
            assert_eq!(String::from(escaped_string), "Hello\n\u{FFFD}Aber!\u{FFFD}");
        }
        {
            let escaped_string: EscapedString = string::EscapedStringSealed::from_data_unchecked(
                EscapedStringData {
                    section_count: 1,
                    capacity: 12,
                },
                r#"Hello World!"#,
            );

            assert_eq!(escaped_string.capacity(), 12);
            assert_eq!(escaped_string.sections().len(), 1);

            let mut sections = escaped_string.sections();
            assert_eq!(
                sections.next(),
                Some(Section::Characters(r#"Hello World!"#))
            );
            assert_eq!(sections.next(), None);
            assert_eq!(String::from(escaped_string), "Hello World!");
        }
    }

    #[test]
    fn test_debug() {}
}
