use super::{CompExpr, Hir, Spanned, Node, Wast, Expr};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode<'input> {
    Wast(Wast<'input, Self>),
    Hir(Hir<'input>),
}

impl<'input> Node<'input> for CompNode<'input> {
    type Expr = CompExpr<'input>;

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }
}

impl<'input> Expr<'input> for CompExpr<'input> {
    type Node = CompNode<'input>;

    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self {
        CompExpr::from_vec(seq)
    }
}
