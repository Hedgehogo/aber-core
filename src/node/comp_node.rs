use super::{CompExpr, Hir, Spanned, Node, Wast};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode<'input> {
    Wast(Wast<'input, Self>),
    Hir(Hir<'input>),
}

impl<'input> Node<'input> for CompNode<'input> {
    type Expr = CompExpr<'input>;

    fn new_node(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }

    fn new_expr(seq: Vec<Spanned<Self>>) -> Self::Expr {
        CompExpr::from_vec(seq)
    }
}
