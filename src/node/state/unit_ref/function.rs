use super::super::{
    unit::{Function, Unit},
    State,
};
use std::fmt;

#[derive(Clone, Copy)]
pub struct FunctionRef<'state, 'input> {
    state: &'state State<'input>,
    id: usize,
}

impl<'state, 'input> FunctionRef<'state, 'input> {
    pub(in super::super) fn new(state: &'state State<'input>, id: usize) -> Self {
        Self { state, id }
    }

    fn unit(&self) -> &'state Function {
        let unit = self.state.get_unit(self.id).expect("Unit must exist");

        #[expect(unreachable_patterns)]
        match unit {
            Unit::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }

    pub fn argument_count(&self) -> Option<usize> {
        self.unit().arguments
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<'state, 'input> fmt::Debug for FunctionRef<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef").field("id", &self.id).finish()
    }
}

impl<'state, 'input> PartialEq for FunctionRef<'state, 'input> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'state, 'input> Eq for FunctionRef<'state, 'input> {}
