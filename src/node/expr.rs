use super::Node;
use super::span::Spanned;

pub type Expr<'input> = Vec<Spanned<Node<'input>>>;
pub type ExprVec<'input> = Vec<Spanned<Expr<'input>>>;
