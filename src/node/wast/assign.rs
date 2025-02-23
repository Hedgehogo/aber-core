//! Module that provides [`Assign`].

use super::{parser_output::ParserOutput, Spanned};

/// Type describing the syntactic construct *assign*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input, N: ParserOutput<'input>> {
    pub left: Spanned<N::Expr>,
    pub right: Spanned<N::Expr>,
}

impl<'input, N: ParserOutput<'input>> Assign<'input, N> {
    /// Creates a new `Assign`.
    pub fn new(left: Spanned<N::Expr>, right: Spanned<N::Expr>) -> Self {
        Self { left, right }
    }
}
