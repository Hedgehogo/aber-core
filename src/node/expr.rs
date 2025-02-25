use super::{Spanned, Node};

pub trait Expr<'input>: Sized {
    /// Type describing the node.
    type Node: Node<'input, Expr = Self>;

    /// Creates an expression from a sequence of nodes.
    ///
    /// # Arguments
    /// - `seq` node sequence with spans.
    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self;
}

/// Type that describes a sequence of expressions.
pub type ExprVec<'input, X> = Vec<Spanned<X>>;
