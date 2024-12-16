#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct String {
    inner: std::string::String
}

impl String {
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}

impl<T: Into<std::string::String>> From<T> for String {
    fn from(value: T) -> Self {
        String::new(value.into())
    }
}
