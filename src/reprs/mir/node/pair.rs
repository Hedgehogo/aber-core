use super::super::super::{CompNode, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'input> {
    left: Box<Spanned<CompNode<'input>>>,
    right: Box<Spanned<CompNode<'input>>>,
}

impl<'input> Pair<'input> {
    pub fn new(
        left: Box<Spanned<CompNode<'input>>>,
        right: Box<Spanned<CompNode<'input>>>,
    ) -> Self {
        Self { left, right }
    }
}
