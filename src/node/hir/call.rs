use crate::node::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    id: usize,
    args: Vec<Node<'input>>,
}

impl<'input> Call<'input> {
    pub fn new(id: usize, args: Vec<Node<'input>>) -> Self {
        Self { id, args }
    }
}
