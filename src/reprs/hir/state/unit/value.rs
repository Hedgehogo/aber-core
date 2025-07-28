use super::super::{unit::Unit, Event, State};
use super::make_adapters;
use std::fmt;

pub type ValueData = i32;

#[derive(Default)]
pub(in super::super::super) struct Value {
    pub inner: Option<ValueData>,
}

make_adapters!(Value, ValueRef, ValueMut);

impl<'state, 'input> ValueRef<'state, 'input> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }
}

impl<'state, 'input> fmt::Debug for ValueRef<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef")
            .field("id", &self.id)
            .field("inner", &self.inner())
            .finish()
    }
}

impl<'state, 'input> ValueMut<'state, 'input> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }

    pub fn set(&mut self, value: ValueData) {
        self.state.log.push(Event::AddArgCount(self.id));
        self.unit_mut().inner = Some(value);
    }

    pub(in super::super::super) fn rewind_set(&mut self) {
        self.unit_mut().inner = None;
    }
}

impl<'state, 'input> fmt::Debug for ValueMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id).finish()
    }
}
