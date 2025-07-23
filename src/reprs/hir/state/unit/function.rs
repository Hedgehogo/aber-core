use super::super::{unit::Unit, State};
use std::fmt;

pub(in super::super::super) struct Function {
    pub arguments: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct FunctionRef<'state, 'input> {
    state: &'state State<'input>,
    id: usize,
}

impl<'state, 'input> FunctionRef<'state, 'input> {
    pub(in super::super::super) fn new(state: &'state State<'input>, id: usize) -> Self {
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

pub struct FunctionMut<'state, 'input> {
    state: &'state mut State<'input>,
    id: usize,
}

impl<'state, 'input> FunctionMut<'state, 'input> {
    pub(in super::super::super) fn new(state: &'state mut State<'input>, id: usize) -> Self {
        Self { state, id }
    }

    fn unit(&self) -> &Function {
        let unit = self.state.get_unit(self.id).expect("Unit must exist");

        #[expect(unreachable_patterns)]
        match unit {
            Unit::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }

    fn unit_mut(&mut self) -> &mut Function {
        let unit = self.state.get_unit_mut(self.id).expect("Unit must exist");

        #[expect(unreachable_patterns)]
        match unit {
            Unit::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }

    pub fn add_argument_count(&mut self, count: usize) {
        self.unit_mut().arguments = Some(count);
    }

    pub fn argument_count(&self) -> Option<usize> {
        self.unit().arguments
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<'state, 'input> fmt::Debug for FunctionMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id).finish()
    }
}

impl<'state, 'input> PartialEq for FunctionMut<'state, 'input> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'state, 'input> Eq for FunctionMut<'state, 'input> {}
