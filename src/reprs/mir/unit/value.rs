use super::super::WithState;
use super::{impl_unit_conv, Unit, UnitConv, UnitEvent, UnitMut, UnitRef};
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

pub type ValueRef<'state> = UnitRef<'state, Value>;

impl<'state> ValueRef<'state> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }
}

impl<'state> fmt::Debug for ValueRef<'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef")
            .field("id", &self.id())
            .field("inner", &self.inner())
            .finish()
    }
}

pub type ValueMut<'state> = UnitMut<'state, Value>;

impl<'state> ValueMut<'state> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }

    pub fn into_inner(self) -> WithState<'state, Option<ValueData>> {
        let inner = self.inner();
        WithState(self.state(), inner)
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

impl<'state> fmt::Debug for ValueMut<'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut")
            .field("id", &self.id())
            .finish()
    }
}
