use super::{hir::String, CompExpr, Expr, Hir, Node, Spanned, Wast};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode<'input> {
    Wast(Wast<'input, Self>),
    Hir(Hir<'input>),
}

impl<'input> Node<'input> for CompNode<'input> {
    type Expr = CompExpr<'input>;
    type String = String;
    type Whitespace = ();

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }

    fn whitespaced(self, _whitespace: Self::Whitespace, _side: super::Side) -> Self {
        self
    }
}

impl<'input> Expr<'input> for CompExpr<'input> {
    type Node = CompNode<'input>;

    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self {
        CompExpr::from_vec(seq)
    }
}
