//! Module providing types for describing different levels of compilation.

pub mod expr;
pub mod span;
pub mod wast;

pub use expr::{Expr, ExprVec};
pub use span::Spanned;
pub use wast::Wast;

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'input> {
    Wast(Wast<'input>),
}
