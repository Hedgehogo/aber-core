use super::{whitespace::Side, Node, Spanned, Whitespace};

pub trait Expr<'input>: Sized {
    /// Type describing the node.
    type Node: Node<'input, Expr = Self>;

    /// Type describing the whitespace.
    type Whitespace: Whitespace<'input>;

    /// Creates an expression from a sequence of nodes.
    ///
    /// # Arguments
    /// - `seq` node sequence with spans.
    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self;

    /// Creates an expresion with added whitespace information.
    ///
    /// # Arguments
    /// - `expr` Expression without whitespace information.
    /// - `whitespace` Added information about whitespace.
    /// - `side` Side on which the whitespace is located from the
    ///   expresion.
    fn whitespaced(expr: Spanned<Self>, whitespace: Self::Whitespace, side: Side) -> Spanned<Self>;
}

/// Type that describes a sequence of expressions.
pub type ExprVec<'input, X> = Vec<Spanned<X>>;
