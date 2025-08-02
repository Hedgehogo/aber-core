use super::{Unit, UnitRef, UnitMut, UnitConv, UnitEvent, impl_unit_conv};
use std::fmt;

pub type ValueData = i32;

#[derive(Default)]
pub struct Value {
    pub inner: Option<ValueData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueEvent {
    Set,
}

impl_unit_conv!(Value, ValueEvent);

pub type ValueRef<'state, 'input> = UnitRef<'state, 'input, Value>;

impl<'state, 'input> ValueRef<'state, 'input> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }
}

impl<'state, 'input> fmt::Debug for ValueRef<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef")
            .field("id", &self.id())
            .field("inner", &self.inner())
            .finish()
    }
}

pub type ValueMut<'state, 'input> = UnitMut<'state, 'input, Value>;

impl<'state, 'input> ValueMut<'state, 'input> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }

    pub fn set(&mut self, value: ValueData) {
        self.unit_mut().inner = Some(value);
        self.log(ValueEvent::Set);
    }

    pub(super) fn rewind(&mut self, event: ValueEvent) {
        match event {
            ValueEvent::Set => self.unit_mut().inner = None,
        }
    }
}

impl<'state, 'input> fmt::Debug for ValueMut<'state, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut").field("id", &self.id()).finish()
    }
}
