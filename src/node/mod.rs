//! Module providing types for describing different levels of compilation.

pub mod comp_expr;
pub mod comp_node;
pub mod expr;
pub mod hir;
pub mod span;
pub mod state;
pub mod wast;
pub mod string;

pub use comp_expr::CompExpr;
pub use comp_node::CompNode;
pub use expr::{Expr, ExprVec};
pub use hir::Hir;
pub use span::Spanned;
use string::{EscapedString, RawString};
pub use wast::Wast;

/// Trait realized by the types that the [`fact`](`crate::syntax::parse::fact`) function can
/// return. It is intended to avoid unnecessary conversion of the returned type into a type with a
/// larger set of values.
pub trait Node<'input>: Sized {
    /// Type describing the expression.
    type Expr: Expr<'input, Node = Self>;

    /// Type describing the string.
    type String: EscapedString<'input> + RawString<'input>;

    /// Creates a node from WAST fact.
    ///
    /// # Arguments
    /// - `wast` WAST fact.
    fn from_wast(wast: Wast<'input, Self>) -> Self;
}
