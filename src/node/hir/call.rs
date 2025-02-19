use super::super::state::{State, unit_ref::{UnitRef, FunctionRef}};
use super::super::super::node::{Node, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    id: usize,
    args: Vec<Spanned<Node<'input>>>,
}

impl<'input> Call<'input> {
    pub fn new(id: usize, args: Vec<Spanned<Node<'input>>>) -> Self {
        Self { id, args }
    }

    pub fn function<'state>(&self, state: &'state State<'input>) -> FunctionRef<'state, 'input> {
        let unit_ref = state.get(self.id).expect("Unit must exist");
        
        match unit_ref {
            UnitRef::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }
}
