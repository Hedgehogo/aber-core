//! Module that provides [`ExprCall`].

use super::{call::Call, parser_output::ParserOutput, Spanned};

/// Type describing syntactic constructs *method call* and *child call*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall<'input, N: ParserOutput<'input>> {
    pub expr: Spanned<N::Expr>,
    pub call: Spanned<Call<'input, N>>,
}

impl<'input, N: ParserOutput<'input>> ExprCall<'input, N> {
    /// Creates a new `ExprCall`.
    pub fn new(expr: Spanned<N::Expr>, call: Spanned<Call<'input, N>>) -> Self {
        Self { expr, call }
    }
}
