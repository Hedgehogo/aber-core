//! Module that provides abstractions over expressions.

use super::{whitespace::Side, Node};
use crate::reprs::span::{IntoSpanned, Spanned, SpannedVec};

pub trait Expr: Sized + From<SpannedVec<Self::Node>> {
    /// Type describing the node.
    type Node: Node<Expr = Self>;

    /// Type describing the whitespace.
    type Whitespace;

    /// Creates an expression from a sequence of nodes.
    ///
    /// # Arguments
    /// - `seq` node sequence with spans.
    fn from_seq(seq: SpannedVec<Self::Node>) -> Self {
        seq.into()
    }

    /// Creates an sequence of nodes with added whitespace information.
    ///
    /// # Arguments
    /// - `expr` Sequence of nodes without whitespace information.
    /// - `whitespace` Added information about whitespace.
    /// - `side` Side on which the whitespace is located from the
    ///   expresion.
    fn whitespaced_seq(
        expr: Spanned<SpannedVec<Self::Node>>,
        whitespace: Self::Whitespace,
        side: Side,
    ) -> Spanned<SpannedVec<Self::Node>>;
}

/// An extension trait describing operations on `Spanned<SpannedVec<X::Node>>`.
pub trait ExprOp<X: Expr>: Sized {
    /// Creates an expresion.
    fn into_spanned_expr(self) -> Spanned<X>;

    /// Creates an sequence of nodes from two neighboring.
    ///
    /// # Arguments
    /// - `left` Sequence of nodes on the left.
    /// - `right` Sequence of nodes on the right.
    fn concat(self, right: Self) -> Self;

    /// Add whitespace information to an sequence of nodes.
    ///
    /// # Arguments
    /// - `whitespace` Information about whitespace.
    /// - `side` Side on which the whitespace is located from the
    ///   expresion.
    fn whitespace(&mut self, whitespace: X::Whitespace, side: Side);

    /// Creates an sequence of nodes with added whitespace information.
    ///
    /// See [`ExprOp::whitespace`].
    fn whitespaced(self, whitespace: X::Whitespace, side: Side) -> Self;
}

impl<N: Node> ExprOp<N::Expr> for Spanned<SpannedVec<N>> {
    fn into_spanned_expr(self) -> Spanned<N::Expr> {
        self.map(N::Expr::from_seq)
    }

    fn concat(self, right: Self) -> Self {
        let Spanned(mut left, left_span) = self;
        let Spanned(mut right, right_span) = right;
        left.append(&mut right);
        left.into_spanned(left_span.range.start..right_span.range.end)
    }

    fn whitespaced(self, whitespace: <<N as Node>::Expr as Expr>::Whitespace, side: Side) -> Self {
        N::Expr::whitespaced_seq(self, whitespace, side)
    }

    fn whitespace(&mut self, whitespace: <<N as Node>::Expr as Expr>::Whitespace, side: Side) {
        let mut seq = std::mem::take(self);
        seq = seq.whitespaced(whitespace, side);
        *self = seq;
    }
}
