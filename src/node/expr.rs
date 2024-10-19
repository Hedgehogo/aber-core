use super::Node;
use super::span::Spanned;

pub type Expr<'input> = Vec<Node<'input>>;
pub type ExprVec<'input> = Vec<Spanned<'input, Expr<'input>>>;
