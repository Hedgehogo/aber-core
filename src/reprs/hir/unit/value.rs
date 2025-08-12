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

pub type ValueRef<'input, 'state> = UnitRef<'input, 'state, Value>;

impl<'input, 'state> ValueRef<'input, 'state> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }
}

impl<'input, 'state> fmt::Debug for ValueRef<'input, 'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef")
            .field("id", &self.id())
            .field("inner", &self.inner())
            .finish()
    }
}

pub type ValueMut<'input, 'state> = UnitMut<'input, 'state, Value>;

impl<'input, 'state> ValueMut<'input, 'state> {
    pub fn inner(&self) -> Option<ValueData> {
        self.unit().inner
    }

    pub fn into_inner(self) -> WithState<'input, 'state, Option<ValueData>> {
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

impl<'input, 'state> fmt::Debug for ValueMut<'input, 'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut")
            .field("id", &self.id())
            .finish()
    }
}
