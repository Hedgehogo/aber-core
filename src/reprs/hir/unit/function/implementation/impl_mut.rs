use super::super::super::super::{State, WithState};
use super::super::super::{Id, Value};
use super::super::FunctionMut;
use super::{ComptimeImpl, Impl};

pub struct ImplMut<'input, 'state> {
    function: FunctionMut<'input, 'state>,
    implementation: Impl,
}

impl<'input, 'state> ImplMut<'input, 'state> {
    pub(in super::super) fn new(function: FunctionMut<'input, 'state>) -> Self {
        let implementation = function.unit().implementation.unwrap();
        Self {
            function,
            implementation,
        }
    }

    pub fn state(self) -> &'state mut State<'input> {
        self.function.state()
    }

    pub fn comptime(self) -> Result<ComptimeImplMut<'input, 'state>, Self> {
        match self.implementation.comptime() {
            Some(implementation) => Ok(ComptimeImplMut {
                function: self.function,
                implementation,
            }),

            None => Err(self),
        }
    }
}

pub struct ComptimeImplMut<'input, 'state> {
    function: FunctionMut<'input, 'state>,
    implementation: ComptimeImpl,
}

impl<'input, 'state> ComptimeImplMut<'input, 'state> {
    pub fn state(self) -> &'state mut State<'input> {
        self.function.state()
    }

    pub fn execute<I>(self, args: I) -> WithState<'input, 'state, Result<Id<Value>, ()>>
    where
        I: Iterator<Item = Id<Value>>,
    {
        let state = self.function.state();
        self.implementation.execute(state, args)
    }
}
