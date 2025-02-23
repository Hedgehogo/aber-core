//! Module that provides [`NegativeCall`].

use super::{parser_output::ParserOutput, Spanned};

/// Type describing the syntactic construct *negative call*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input, N: ParserOutput<'input>> {
    pub expr: Spanned<N::Expr>,
}

impl<'input, N: ParserOutput<'input>> NegativeCall<'input, N> {
    /// Creates a new `NegativeCall`.
    pub fn new(expr: Spanned<N::Expr>) -> Self {
        Self { expr }
    }
}
