use super::{Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCall<'input> {
    pub expr: Spanned<'input, Expr<'input>>,
}

impl<'input> NegativeCall<'input> {
    pub fn new(expr: Spanned<'input, Expr<'input>>) -> Self {
        Self { expr }
    }
}
