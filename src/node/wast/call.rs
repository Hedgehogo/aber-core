use super::ExprVec;
use super::Spanned;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentError {}

#[derive(Clone, PartialEq, Eq)]
pub struct Ident<'input> {
    content: &'input str,
}

impl<'input> Ident<'input> {
    pub fn new(content: &'input str) -> Result<Self, IdentError> {
        Ok(Self { content })
    }

    pub fn as_str(&self) -> &str {
        self.content
    }
}

impl fmt::Debug for Ident<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.content)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    pub ident: Spanned<Ident<'input>>,
    pub generics: Spanned<ExprVec<'input>>,
}

impl<'input> Call<'input> {
    pub fn new(ident: Spanned<Ident<'input>>, generics: Spanned<ExprVec<'input>>) -> Self {
        Self { ident, generics }
    }
}
