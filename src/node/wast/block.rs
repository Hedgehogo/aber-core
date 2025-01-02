use super::{Assign, Expr, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt<'input> {
    Expr(Expr<'input>),
    Assign(Assign<'input>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'input> {
    pub stmts: Vec<Spanned<Stmt<'input>>>,
    pub expr: Spanned<Expr<'input>>,
}

impl<'input> Block<'input> {
    pub fn new(stmts: Vec<Spanned<Stmt<'input>>>, expr: Spanned<Expr<'input>>) -> Self {
        Self { stmts, expr }
    }
}
