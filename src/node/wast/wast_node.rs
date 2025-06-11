use super::super::{span::IntoSpanned, whitespace::Side, Expr, Node, Spanned, SpannedVec};
use super::{String, Wast, Whitespace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WastNode<'input> {
    Wast(Wast<'input, Self>),
    Whitespace(Whitespace<'input>),
}

impl<'input> Node<'input> for WastNode<'input> {
    type Expr = Vec<Spanned<Self>>;
    type String = String<'input>;

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }
}

impl<'input> Expr<'input> for Vec<Spanned<WastNode<'input>>> {
    type Node = WastNode<'input>;
    type Whitespace = Whitespace<'input>;

    fn whitespaced_seq(
        expr: Spanned<SpannedVec<Self::Node>>,
        whitespace: Self::Whitespace,
        side: Side,
    ) -> Spanned<SpannedVec<Self::Node>> {
        let Spanned(mut expr, mut span) = expr;
        match side {
            Side::Left => {
                let start = span.range.start - whitespace.repr().len();
                let node = WastNode::Whitespace(whitespace).into_spanned(start..span.range.start);
                span.range.start = start;
                expr.insert(0, node);
                Spanned(expr, span)
            }
            Side::Right => {
                let end = span.range.end + whitespace.repr().len();
                let node = WastNode::Whitespace(whitespace).into_spanned(span.range.end..end);
                span.range.end = end;
                expr.push(node);
                Spanned(expr, span)
            }
        }
    }
}
