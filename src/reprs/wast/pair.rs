//! Module that provides [`Pair`].

use super::Spanned;
use crate::stages::syntax::{Expr, Node};
use std::{fmt, marker::PhantomData};

/// Type describing the syntactic construct *pair*
///
/// # Fields
/// - `node` Nested node.
/// - `whitespace` Whitespace before the colon.
pub struct Pair<'input, N: Node<'input>> {
    pub node: Box<Spanned<N>>,
    pub whitespace: <N::Expr as Expr<'input>>::Whitespace,
    phantom: PhantomData<&'input str>,
}

impl<'input, N: Node<'input>> Pair<'input, N> {
    /// Creates a new `Pair`.
    pub fn new(node: Box<Spanned<N>>, whitespace: <N::Expr as Expr<'input>>::Whitespace) -> Self {
        Self {
            node,
            whitespace,
            phantom: PhantomData,
        }
    }
}

impl<'input, N> fmt::Debug for Pair<'input, N>
where
    N: Node<'input> + fmt::Debug,
    <N::Expr as Expr<'input>>::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pair")
            .field("node", &self.node)
            .field("whitespace", &self.whitespace)
            .finish()
    }
}

impl<'input, N> Clone for Pair<'input, N>
where
    N: Node<'input> + Clone,
    <N::Expr as Expr<'input>>::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.node.clone(), self.whitespace.clone())
    }
}

impl<'input, N> PartialEq for Pair<'input, N>
where
    N: Node<'input> + PartialEq,
    <N::Expr as Expr<'input>>::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.whitespace == other.whitespace
            && self.phantom == other.phantom
    }
}

impl<'input, N> Eq for Pair<'input, N>
where
    N: Node<'input> + Eq,
    <N::Expr as Expr<'input>>::Whitespace: Eq,
{
}
