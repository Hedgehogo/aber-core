//! Module providing types for describing different levels of compilation.

pub mod expr;
pub mod hir;
pub mod span;
pub mod state;
pub mod wast;

use wast::parser_output::ParserOutput;

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

impl<'input> ParserOutput<'input> for Node<'input> {
    type Expr = Expr<'input>;

    fn new_node(wast: Wast<'input>) -> Self {
        Self::Wast(wast)
    }

    fn new_expr(seq: Vec<Spanned<Self>>) -> Self::Expr {
        Expr::from_vec(seq)
    }
}
