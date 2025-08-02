use super::{Unit, UnitRef, UnitMut, UnitConv, UnitEvent, impl_unit_conv};
use std::fmt;

#[derive(Default)]
pub struct Function {
    pub arguments: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FunctionEvent {
    AddArgCount,
}

impl_unit_conv!(Function, FunctionEvent);

pub type FunctionRef<'state, 'input> = UnitRef<'state, 'input, Function>;

impl<'state, 'input> FunctionRef<'state, 'input> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }
}

impl<'state, 'input> fmt::Debug for FunctionRef<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef").field("id", &self.id()).finish()
    }
}

pub type FunctionMut<'state, 'input> = UnitMut<'state, 'input, Function>;

impl<'state, 'input> FunctionMut<'state, 'input> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }

    pub fn add_arg_count(&mut self, count: usize) {
        self.unit_mut().arguments = Some(count);
        self.log(FunctionEvent::AddArgCount);
    }

    pub(super) fn rewind(&mut self, event: FunctionEvent) {
        match event {
            FunctionEvent::AddArgCount => self.unit_mut().arguments = None,
        }
    }
}

impl<'state, 'input> fmt::Debug for FunctionMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id()).finish()
    }
}
