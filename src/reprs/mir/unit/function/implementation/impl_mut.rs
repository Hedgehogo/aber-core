use super::super::super::super::{State, WithState};
use super::super::super::{Id, Value};
use super::super::FunctionMut;
use super::{ComptimeImpl, Impl};

pub struct ImplMut<'state> {
    function: FunctionMut<'state>,
    implementation: Impl,
}

impl<'state> ImplMut<'state> {
    pub(in super::super) fn new(function: FunctionMut<'state>) -> Self {
        let implementation = function.unit().implementation.unwrap();
        Self {
            function,
            implementation,
        }
    }

    pub fn state(self) -> &'state mut State {
        self.function.state()
    }

    pub fn comptime(self) -> Result<ComptimeImplMut<'state>, Self> {
        match self.implementation.comptime() {
            Some(implementation) => Ok(ComptimeImplMut {
                function: self.function,
                implementation,
            }),

            None => Err(self),
        }
    }
}

pub struct ComptimeImplMut<'state> {
    function: FunctionMut<'state>,
    implementation: ComptimeImpl,
}

impl<'state> ComptimeImplMut<'state> {
    pub fn state(self) -> &'state mut State {
        self.function.state()
    }

    pub fn execute<I>(self, args: I) -> WithState<'state, Result<Id<Value>, ()>>
    where
        I: Iterator<Item = Id<Value>>,
    {
        let state = self.function.state();
        self.implementation.execute(state, args)
    }
}
