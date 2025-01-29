//! Module providing types for describing different levels of compilation.

pub mod expr;
pub mod hir;
pub mod span;
pub mod wast;
pub mod state;

pub use expr::{Expr, ExprVec};
pub use hir::Hir;
pub use span::Spanned;
pub use wast::Wast;

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'input> {
    Wast(Wast<'input>),
    Hir(Hir<'input>),
}
