use super::super::{span::IntoSpanned, whitespace::Side, Expr, Node, Spanned};
use super::{String, Wast};

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
    type Whitespace = ();

    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self {
        seq
    }

    fn concat(left: Spanned<Self>, right: Spanned<Self>) -> Option<Spanned<Self>> {
        let Spanned(mut left, left_span) = left;
        let Spanned(mut right, right_span) = right;
        left.append(&mut right);
        Some(left.into_spanned(left_span.range.start..right_span.range.end))
    }

    fn whitespaced(
        expr: Spanned<Self>,
        _whitespace: Self::Whitespace,
        _side: Side,
    ) -> Spanned<Self> {
        expr
    }
}
