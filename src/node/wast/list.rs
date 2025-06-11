//! Module that provides [`List`].

use super::SpannedVec;
use crate::syntax::Expr;

/// Type describing syntactic constructions containing comma-separated enumerated items.
///
/// # Fields
/// - `args` Items listed comma-separately.
/// - `whitespace` Whitespace after the trailing comma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'input, I, X: Expr<'input>> {
    pub items: SpannedVec<I>,
    pub whitespace: X::Whitespace,
}

impl<'input, I, X: Expr<'input>> List<'input, I, X> {
    /// Creates a new `List`.
    pub fn new(items: SpannedVec<I>, whitespace: X::Whitespace) -> Self {
        Self { items, whitespace }
    }
}
