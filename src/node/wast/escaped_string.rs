//! Module that provides [`EscapedString`].

use crate::syntax::string::{self, EscapedStringCtx};
use std::fmt;

/// Type describing a escaped string literal.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct EscapedString<'input> {
    inner_repr: &'input str,
    section_count: usize,
    capacity: usize,
    ctx: EscapedStringCtx,
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

    /// Gets the context that the string had during parsing.
    pub fn ctx(&self) -> &EscapedStringCtx {
        &self.ctx
    }

    /// Gets an iterator over sections.
    pub fn sections<'str>(&'str self) -> SectionIter<'str, 'input> {
        SectionIter {
            rest: self.inner_repr,
            rest_length: self.section_count,
            string: self,
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

impl<'input> Section<'input> {
    pub fn as_str(&self) -> &'input str {
        match self {
            Section::Escape(i) => i,
            Section::Characters(i) => i,
        }
    }
}

impl<'input> AsRef<str> for Section<'input> {
    fn as_ref(&self) -> &'input str {
        self.as_str()
    }
}

/// Type describing an iterator over sections of an escaped string
/// literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionIter<'str, 'input> {
    rest: &'input str,
    rest_length: usize,
    string: &'str EscapedString<'input>,
}

impl<'str, 'input> Iterator for SectionIter<'str, 'input> {
    type Item = Section<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::syntax::parse::escaped_string::section;
        use chumsky::{extra::Err, prelude::*, text::Graphemes};

        match self.rest_length {
            0 => None,

            _ => {
                let (result, rest) = section()
                    .with_ctx(self.string.ctx().clone())
                    .then(any::<_, Err<EmptyErr>>().repeated().to_slice())
                    .parse(Graphemes::new(self.rest))
                    .into_output()?;

                self.rest = rest.as_str();
                self.rest_length -= 1;

                Some(result)
            }
        }
    }
}

impl ExactSizeIterator for SectionIter<'_, '_> {
    fn len(&self) -> usize {
        self.rest_length
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

    fn from_data_unchecked(
        data: Self::Data,
        inner_repr: &'input str,
        ctx: &EscapedStringCtx,
    ) -> Self {
        Self {
            inner_repr,
            section_count: data.section_count,
            capacity: data.capacity,
            ctx: ctx.clone(),
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
                &Default::default(),
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
                &Default::default(),
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
}
