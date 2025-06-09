//! Module that provides `List`.

use super::super::Expr;
use super::SpannedVec;

/// Type describing syntactic constructions containing comma-separated enumerated items.
///
/// # Fields
/// - `args` Items listed comma-separately.
/// - `whitespace` Whitespace after the trailing comma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'input, X: Expr<'input>> {
    pub items: SpannedVec<X>,
    pub whitespace: X::Whitespace,
}

impl<'input, X: Expr<'input>> List<'input, X> {
    /// Creates a new `List`.
    pub fn new(items: SpannedVec<X>, whitespace: X::Whitespace) -> Self {
        Self { items, whitespace }
    }
}
