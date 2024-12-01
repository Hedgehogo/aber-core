use super::{call::Call, Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall<'input> {
    pub expr: Spanned<Expr<'input>>,
    pub call: Spanned<Call<'input>>,
}

impl<'input> ExprCall<'input> {
    pub fn new(expr: Spanned<Expr<'input>>, call: Spanned<Call<'input>>) -> Self {
        Self { expr, call }
    }
}
