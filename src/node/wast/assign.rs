use super::{Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign<'input> {
    pub left: Spanned<'input, Expr<'input>>,
    pub right: Spanned<'input, Expr<'input>>,
}

impl<'input> Assign<'input> {
    pub fn new(left: Spanned<'input, Expr<'input>>, right: Spanned<'input, Expr<'input>>) -> Self {
        Self { left, right }
    }
}
