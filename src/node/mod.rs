pub mod expr;
pub mod span;
pub mod wast;

pub use expr::{Expr, ExprVec};
pub use span::Spanned;
pub use wast::Wast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'input> {
    Wast(Wast<'input>),
}
