use super::super::super::{CompNode, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    left: Box<Spanned<CompNode>>,
    right: Box<Spanned<CompNode>>,
}

impl Pair {
    pub fn new(left: Box<Spanned<CompNode>>, right: Box<Spanned<CompNode>>) -> Self {
        Self { left, right }
    }
}
