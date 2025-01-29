use super::super::{
    unit::{Function, Unit},
    State,
};

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
        match unit {
            Unit::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }

    pub fn argument_count(&self) -> Option<usize> {
        self.unit().arguments
    }
}
