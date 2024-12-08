#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    inner: std::string::String
}

impl String {
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}
