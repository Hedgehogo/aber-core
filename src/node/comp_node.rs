use super::{
    hir::String, span::IntoSpanned, whitespace::Side, CompExpr, Expr, Hir, Node, Spanned, Wast,
};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode<'input> {
    Wast(Wast<'input, Self>),
    Hir(Hir<'input>),
}

impl<'input> Node<'input> for CompNode<'input> {
    type Expr = CompExpr<'input>;
    type String = String;

    fn from_wast(wast: Wast<'input, Self>) -> Self {
        Self::Wast(wast)
    }
}

impl<'input> Expr<'input> for CompExpr<'input> {
    type Node = CompNode<'input>;
    type Whitespace = ();

    fn from_seq(seq: Vec<Spanned<Self::Node>>) -> Self {
        Self::from_vec(seq)
    }

    fn concat(left: Spanned<Self>, right: Spanned<Self>) -> Option<Spanned<Self>> {
        if let (
            Spanned(Self::Wast(mut left), left_span),
            Spanned(Self::Wast(mut right), right_span),
        ) = (left, right)
        {
            left.append(&mut right);
            let expr = Self::from_vec(left);
            return Some(expr.into_spanned(left_span.range.start..right_span.range.end));
        }
        None
    }

    fn whitespaced(
        expr: Spanned<Self>,
        _whitespace: Self::Whitespace,
        _side: Side,
    ) -> Spanned<Self> {
        expr
    }
}
