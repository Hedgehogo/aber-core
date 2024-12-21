pub mod expr;
pub mod span;
pub mod wast;

pub use expr::{Expr, ExprVec};
use span::Span;
pub use span::Spanned;
pub use wast::Wast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'input> {
    Wast(Wast<'input>),
}

impl Node<'_> {
    pub fn into_spanned<S: Into<Span>>(self, span: S) -> Spanned<Self> {
        Spanned(self, span.into())
    }
}
