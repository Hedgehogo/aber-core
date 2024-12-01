use super::{Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input> {
    pub left: Spanned<Expr<'input>>,
    pub right: Spanned<Expr<'input>>,
}

impl<'input> Assign<'input> {
    pub fn new(left: Spanned<Expr<'input>>, right: Spanned<Expr<'input>>) -> Self {
        Self { left, right }
    }
}
