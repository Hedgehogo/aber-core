//! Module that provides `List`.

use super::super::{Expr, Spanned};
use super::ExprVec;

/// Type describing syntactic constructions containing comma-separated enumerated items.
///
/// # Fields
/// - `args` Items listed comma-separately.
/// - `whitespace` Whitespace after the trailing comma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'input, X: Expr<'input>> {
    pub args: Spanned<ExprVec<'input, X>>,
    pub whitespace: X::Whitespace,
}

impl<'input, X: Expr<'input>> List<'input, X> {
    /// Creates a new `List`.
    pub fn new(args: Spanned<ExprVec<'input, X>>, whitespace: X::Whitespace) -> Self {
        Self { args, whitespace }
    }
}
