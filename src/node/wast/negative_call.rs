use super::{Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input> {
    pub expr: Spanned<Expr<'input>>,
}

impl<'input> NegativeCall<'input> {
    pub fn new(expr: Spanned<Expr<'input>>) -> Self {
        Self { expr }
    }
}
