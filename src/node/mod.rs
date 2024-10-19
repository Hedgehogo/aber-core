pub mod wast;
pub mod expr;
pub mod span;

pub use wast::Wast;
pub use expr::{Expr, ExprVec};
pub use span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'input> {
    Wast(Wast<'input>)
}
