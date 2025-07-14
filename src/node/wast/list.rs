//! Module that provides [`List`].

use super::SpannedVec;
use crate::syntax::Expr;
use std::fmt;

/// Type describing syntactic constructions containing comma-separated enumerated items.
///
/// # Fields
/// - `args` Items listed comma-separately.
/// - `whitespace` Whitespace after the trailing comma.
#[derive(Clone, PartialEq, Eq)]
pub struct List<'input, I, X: Expr<'input>> {
    pub items: SpannedVec<I>,
    pub whitespace: Option<X::Whitespace>,
    close: bool,
}

impl<'input, I, X: Expr<'input>> List<'input, I, X> {
    /// Creates a new `List`.
    pub fn new(items: SpannedVec<I>, whitespace: Option<X::Whitespace>, close: bool) -> Self {
        Self {
            items,
            whitespace,
            close,
        }
    }

    /// Asks if the closing parenthesis was present.
    pub fn is_closed(&self) -> bool {
        self.close
    }
}

impl<'input, I, X> fmt::Debug for List<'input, I, X>
where
    I: fmt::Debug,
    X: Expr<'input> + fmt::Debug,
    X::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("List")
            .field("items", &self.items)
            .field("whitespace", &self.whitespace)
            .field("close", &self.close)
            .finish()
    }
}
