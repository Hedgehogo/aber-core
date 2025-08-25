//! Module that provides [`Pair`].

use super::Spanned;
use crate::stages::parsing::{Expr, Node};
use std::fmt;

/// Type describing the syntactic construct *pair*
///
/// # Fields
/// - `node` Nested node.
/// - `whitespace` Whitespace before the colon.
pub struct Pair<N: Node> {
    pub node: Box<Spanned<N>>,
    pub whitespace: <N::Expr as Expr>::Whitespace,
}

impl<N: Node> Pair<N> {
    /// Creates a new `Pair`.
    pub fn new(node: Box<Spanned<N>>, whitespace: <N::Expr as Expr>::Whitespace) -> Self {
        Self { node, whitespace }
    }
}

impl<N> fmt::Debug for Pair<N>
where
    N: Node + fmt::Debug,
    <N::Expr as Expr>::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pair")
            .field("node", &self.node)
            .field("whitespace", &self.whitespace)
            .finish()
    }
}

impl<N> Clone for Pair<N>
where
    N: Node + Clone,
    <N::Expr as Expr>::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.node.clone(), self.whitespace.clone())
    }
}

impl<N> PartialEq for Pair<N>
where
    N: Node + PartialEq,
    <N::Expr as Expr>::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.whitespace == other.whitespace
    }
}

impl<N> Eq for Pair<N>
where
    N: Node + Eq,
    <N::Expr as Expr>::Whitespace: Eq,
{
}
