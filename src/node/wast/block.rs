use super::{Assign, Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'input> {
    Expr(Expr<'input>),
    Assign(Assign<'input>),
}

pub type StatementVec<'input> = Vec<Spanned<Statement<'input>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'input> {
    pub statements: StatementVec<'input>,
    pub expr: Spanned<Expr<'input>>,
}

impl<'input> Block<'input> {
    pub fn new(statements: StatementVec<'input>, expr: Spanned<Expr<'input>>) -> Self {
        Self { statements, expr }
    }
}
