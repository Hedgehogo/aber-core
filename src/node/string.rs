/// Trait describing some data from which a string can be created.
pub trait StringData<'input>: Sized {
    /// Creates data based on string capacity information.
    ///
    /// # Arguments
    /// - `capacity` String capacity in bytes.
    fn with_capacity(capacity: usize) -> Self;

    /// Creates data based on the previous data and adds information
    /// about the next section of the string to it.
    ///
    /// # Arguments
    /// - `section` The next section of the string.
    fn with_next_section(self, section: &'input str) -> Self;
}

impl<'input> StringData<'input> for std::string::String {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn with_next_section(mut self, section: &'input str) -> Self {
        std::string::String::push_str(&mut self, section);
        self
    }
}

impl<'input> StringData<'input> for () {
    fn with_capacity(_capacity: usize) -> Self {}

    fn with_next_section(#[allow(unused)] self, _section: &'input str) -> Self {}
}

/// Trait describing a type storing information about a string
/// literal containing escape sequences.
pub trait EscapedString<'input>: Sized {
    type Data: StringData<'input>;

    /// Creates an instance of the type based on the collected data
    /// about the string and its representation.
    ///
    /// # Arguments
    /// - `data` Collected string data.
    /// - `inner_repr` Representation of a string between opening and
    ///   closing sequences.
    ///
    /// # Safety
    /// `inner_repr` must be a valid string representation, that is,
    /// it must not contain `\` or `"` characters outside of escape
    /// sequences and must contain only existing escape sequences.
    unsafe fn from_data_unchecked(data: Self::Data, inner_repr: &'input str) -> Self;
}

/// Trait describing a type storing information about a raw string
/// literal.
pub trait RawString<'input>: Sized {
    type Data: StringData<'input>;

    /// Creates an instance of the type based on the collected data
    /// about the string, its representation and indentation.
    ///
    /// # Arguments
    /// - `data` Collected string data.
    /// - `indent` Non-informative indentation at the beginning of
    ///   each line (In specification sequence W).
    /// - `inner_repr` Representation of the string between the
    ///   opening sequence and the last line break as part of the raw
    ///   string.
    ///
    /// # Safety
    /// `indent` must be a valid indent, that is, it must contain
    /// only inline whitespace.
    ///
    /// `inner_repr` must be a valid representation of the raw
    /// string, that is, each line break must be followed by
    /// `indent`.
    unsafe fn from_data_unchecked(
        data: Self::Data,
        indent: &'input str,
        inner_repr: &'input str,
    ) -> Self;
}
