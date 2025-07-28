use super::make_adapters;
use std::fmt;

pub type ValueData = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(in super::super) enum ValueEvent {
    Set,
}

#[derive(Default)]
pub(in super::super) struct Value {
    pub inner: Option<ValueData>,
}

make_adapters!(Value, ValueRef, ValueMut, ValueEvent);

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
        self.unit_mut().inner = Some(value);
        self.log(ValueEvent::Set);
    }

    pub(in super::super) fn rewind(&mut self, event: ValueEvent) {
        match event {
            ValueEvent::Set => self.unit_mut().inner = None,
        }
    }
}

impl<'state, 'input> fmt::Debug for ValueMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id).finish()
    }
}
