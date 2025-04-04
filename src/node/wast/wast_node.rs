use super::{Wast, String};
use super::super::{Node, Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WastNode<'input>(pub Wast<'input, Self>);

impl<'input> Node<'input> for WastNode<'input> {
    type Expr = Vec<Spanned<Self>>;
    type String = String<'input>;

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self(wast)
    }
}

impl<'input> Expr<'input> for Vec<Spanned<WastNode<'input>>> {
    type Node = WastNode<'input>;

    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self {
        seq
    }
}
