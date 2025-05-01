//! Module providing types for describing different levels of compilation.

pub mod comp_expr;
pub mod comp_node;
pub mod expr;
pub mod hir;
pub mod span;
pub mod state;
pub mod string;
pub mod wast;
pub mod whitespace;

use string::{EscapedString, RawString};
use whitespace::Side;

pub use comp_expr::CompExpr;
pub use comp_node::CompNode;
pub use expr::{Expr, ExprVec};
pub use hir::Hir;
pub use span::Spanned;
pub use wast::Wast;
pub use whitespace::Whitespace;

/// Trait realized by the types that the
/// [`fact`](`crate::syntax::parse::fact`) function can return. It is
/// intended to avoid unnecessary conversion of the returned type
/// into a type with a larger set of values.
pub trait Node<'input>: Sized {
    /// Type describing the expression.
    type Expr: Expr<'input, Node = Self>;

    /// Type describing the string.
    type String: EscapedString<'input> + RawString<'input>;

    /// Type describing the whitespace.
    type Whitespace: Whitespace<'input>;

    /// Creates a node from WAST fact.
    ///
    /// # Arguments
    /// - `wast` WAST fact.
    fn from_wast(wast: Wast<'input, Self>) -> Self;

    /// Creates a node with added whitespace information.
    ///
    /// # Arguments
    /// - `whitespace` Added information about whitespace.
    /// - `side` Side on which the whitespace is located from the
    ///   node.
    fn whitespaced(self, whitespace: Self::Whitespace, side: Side) -> Self;
}
