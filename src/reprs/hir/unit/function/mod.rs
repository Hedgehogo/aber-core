pub mod implementation;

use super::{impl_unit_conv, Unit, UnitConv, UnitEvent, UnitMut, UnitRef};
use implementation::{Impl, ImplMut};
use std::fmt;

pub use implementation::impls;

#[derive(Default)]
pub enum Time {
    #[default]
    Any,
    Comptime,
    Runtime,
}

#[derive(Default)]
pub struct Function {
    time: Time,
    arguments: Option<usize>,
    implementation: Option<Impl>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FunctionEvent {
    AddArgCount,
    AddImpl,
    SpecifyTime,
}

impl_unit_conv!(Function, FunctionEvent);

pub type FunctionRef<'input, 'state> = UnitRef<'input, 'state, Function>;

impl<'input, 'state> FunctionRef<'input, 'state> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }
}

impl<'input, 'state> fmt::Debug for FunctionRef<'input, 'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionRef")
            .field("id", &self.id())
            .finish()
    }
}

pub type FunctionMut<'input, 'state> = UnitMut<'input, 'state, Function>;

impl<'input, 'state> FunctionMut<'input, 'state> {
    pub fn arg_count(&self) -> Option<usize> {
        self.unit().arguments
    }

    pub fn add_arg_count(&mut self, count: usize) {
        if self.unit().arguments.is_none() {
            self.unit_mut().arguments = Some(count);
            self.log(FunctionEvent::AddArgCount);
        }
    }

    pub fn implementation(self) -> Result<ImplMut<'input, 'state>, Self> {
        match self.unit().implementation {
            Some(_) => Ok(ImplMut::new(self)),
            None => Err(self),
        }
    }

    pub fn add_impl(&mut self, implementation: Impl) {
        if self.unit().implementation.is_none() {
            let arg_count = implementation.arg_count();
            let unit = self.unit_mut();
            unit.implementation = Some(implementation);
            unit.arguments = Some(arg_count);
            self.log(FunctionEvent::AddImpl);
        }
    }

    pub fn specify_time(&mut self, time: Time) {
        if let Time::Any = self.unit().time {
            self.unit_mut().time = time;
            self.log(FunctionEvent::SpecifyTime);
        }
    }

    pub(super) fn rewind(&mut self, event: FunctionEvent) {
        match event {
            FunctionEvent::AddArgCount => self.unit_mut().arguments = None,
            FunctionEvent::AddImpl => self.unit_mut().implementation = None,
            FunctionEvent::SpecifyTime => self.unit_mut().time = Time::Any,
        }
    }
}

impl<'input, 'state> fmt::Debug for FunctionMut<'input, 'state> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMut")
            .field("id", &self.id())
            .finish()
    }
}
