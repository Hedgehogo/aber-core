//! Module providing types for describing different levels of compilation.

pub mod comp_expr;
pub mod comp_node;
pub mod hir;
pub mod span;
pub mod state;
pub mod wast;

pub use comp_expr::{CompExpr, ExprVec};
pub use comp_node::CompNode;
pub use hir::Hir;
pub use span::Spanned;
pub use wast::Wast;

/// Trait realized by the types that the [`fact`](`crate::syntax::parse::fact`) function can 
/// return. It is intended to avoid unnecessary conversion of the returned type into a type with a 
/// larger set of values.
pub trait Node<'input>: Sized {
    /// Type describing the expression.
    type Expr: Sized;

    /// Creates a node from WAST fact.
    /// 
    /// # Arguments
    /// - `wast` WAST fact.
    fn new_node(wast: Wast<'input, Self>) -> Self;

    /// Creates an expression from a sequence of WAST facts.
    /// 
    /// # Arguments
    /// - `seq` WAST fact sequence with spans.
    fn new_expr(seq: Vec<Spanned<Self>>) -> Self::Expr;
}
