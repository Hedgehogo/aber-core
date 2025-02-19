use super::super::super::node::{Node, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'input> {
    left: Box<Spanned<Node<'input>>>,
    right: Box<Spanned<Node<'input>>>,
}

impl<'input> Pair<'input> {
    pub fn new(left: Box<Spanned<Node<'input>>>, right: Box<Spanned<Node<'input>>>) -> Self {
        Self { left, right }
    }
}
