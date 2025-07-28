use super::make_adapters;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(in super::super) enum FunctionEvent {
    AddArgCount,
}

#[derive(Default)]
pub(in super::super) struct Function {
    pub arguments: Option<usize>,
}

make_adapters!(Function, FunctionRef, FunctionMut, FunctionEvent);

impl<'state, 'input> FunctionRef<'state, 'input> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }
}

impl<'state, 'input> fmt::Debug for FunctionRef<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef").field("id", &self.id).finish()
    }
}

impl<'state, 'input> FunctionMut<'state, 'input> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }

    pub fn add_arg_count(&mut self, count: usize) {
        self.unit_mut().arguments = Some(count);
        self.log(FunctionEvent::AddArgCount);
    }

    pub(in super::super) fn rewind(&mut self, event: FunctionEvent) {
        match event {
            FunctionEvent::AddArgCount => self.unit_mut().arguments = None,
        }
    }
}

impl<'state, 'input> fmt::Debug for FunctionMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id).finish()
    }
}
