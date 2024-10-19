use super::{call::Call, Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprCall<'input> {
    pub expr: Spanned<'input, Expr<'input>>,
    pub call: Spanned<'input, Call<'input>>,
}

impl<'input> ExprCall<'input> {
    pub fn new(expr: Spanned<'input, Expr<'input>>, call: Spanned<'input, Call<'input>>) -> Self {
        Self { expr, call }
    }
}
