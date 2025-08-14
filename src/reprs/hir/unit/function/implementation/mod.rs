pub mod impl_mut;

mod standart;

use super::super::super::{Id, State, WithState};
use super::super::Value;
use super::Time;

pub use impl_mut::ImplMut;

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum AnyBuiltInImpl {
    OneI32,
    SameI32,
    AddI32,
    PrintlnI32,
}

impl AnyBuiltInImpl {
    pub(crate) fn execute<'input: 'state, 'state, I>(
        &self,
        state: &'state mut State<'input>,
        mut args: I,
    ) -> WithState<'input, 'state, Result<Id<Value>, ()>>
    where
        I: Iterator<Item = Id<Value>>,
    {
        match self {
            Self::OneI32 => standart::one_i32(state),
            Self::SameI32 => standart::same_i32(state, args.next().unwrap()),
            Self::AddI32 => standart::add_i32(state, args.next().unwrap(), args.next().unwrap()),
            Self::PrintlnI32 => standart::println_i32(state, args.next().unwrap()),
        }
    }

    pub fn arg_count(&self) -> usize {
        match self {
            AnyBuiltInImpl::OneI32 => 0,
            AnyBuiltInImpl::SameI32 => 1,
            AnyBuiltInImpl::AddI32 => 2,
            AnyBuiltInImpl::PrintlnI32 => 1,
        }
    }
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum ComptimeBuiltInImpl {}

impl ComptimeBuiltInImpl {
    #[expect(unused_variables)]
    pub(crate) fn execute<'input, 'state: 'state, I>(
        &self,
        state: &'state mut State<'input>,
        args: I,
    ) -> WithState<'input, 'state, Result<Id<Value>, ()>>
    where
        I: Iterator<Item = Id<Value>>,
    {
        #[expect(clippy::match_single_binding)]
        match self {
            _ => todo!(),
        }
    }

    pub fn arg_count(&self) -> usize {
        #[expect(clippy::match_single_binding)]
        match self {
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum RuntimeBuiltInImpl {
    RunI32,
}

impl RuntimeBuiltInImpl {
    pub fn arg_count(&self) -> usize {
        match self {
            RuntimeBuiltInImpl::RunI32 => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BuiltInImpl {
    Any(AnyBuiltInImpl),
    Comptime(ComptimeBuiltInImpl),
    Runtime(RuntimeBuiltInImpl),
}

impl BuiltInImpl {
    pub fn arg_count(&self) -> usize {
        match self {
            BuiltInImpl::Any(any) => any.arg_count(),
            BuiltInImpl::Comptime(comptime) => comptime.arg_count(),
            BuiltInImpl::Runtime(runtime) => runtime.arg_count(),
        }
    }
}

impl From<AnyBuiltInImpl> for BuiltInImpl {
    fn from(value: AnyBuiltInImpl) -> Self {
        Self::Any(value)
    }
}

impl From<ComptimeBuiltInImpl> for BuiltInImpl {
    fn from(value: ComptimeBuiltInImpl) -> Self {
        Self::Comptime(value)
    }
}

impl From<RuntimeBuiltInImpl> for BuiltInImpl {
    fn from(value: RuntimeBuiltInImpl) -> Self {
        Self::Runtime(value)
    }
}

#[derive(Clone, Copy)]
pub enum ComptimeImpl {
    Any(AnyBuiltInImpl),
    Comptime(ComptimeBuiltInImpl),
}

impl ComptimeImpl {
    pub(crate) fn execute<'input, 'state: 'state, I>(
        &self,
        state: &'state mut State<'input>,
        args: I,
    ) -> WithState<'input, 'state, Result<Id<Value>, ()>>
    where
        I: Iterator<Item = Id<Value>>,
    {
        match self {
            Self::Any(any) => any.execute(state, args),
            Self::Comptime(comptime) => comptime.execute(state, args),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RuntimeImpl {
    Any(AnyBuiltInImpl),
    Runtime(RuntimeBuiltInImpl),
}

impl BuiltInImpl {
    pub fn comptime(&self) -> Option<ComptimeImpl> {
        match *self {
            Self::Any(any) => Some(ComptimeImpl::Any(any)),
            Self::Comptime(comptime) => Some(ComptimeImpl::Comptime(comptime)),
            Self::Runtime(_) => None,
        }
    }

    pub fn runtime(&self) -> Option<RuntimeImpl> {
        match *self {
            Self::Any(any) => Some(RuntimeImpl::Any(any)),
            Self::Runtime(runtime) => Some(RuntimeImpl::Runtime(runtime)),
            Self::Comptime(_) => None,
        }
    }

    pub fn time(&self) -> Time {
        match self {
            Self::Any(_) => Time::Any,
            Self::Comptime(_) => Time::Comptime,
            Self::Runtime(_) => Time::Runtime,
        }
    }

    pub fn is_valid_time(&self, time: Time) -> bool {
        #[expect(clippy::match_like_matches_macro)]
        match (time, self.time()) {
            (Time::Any, Time::Runtime) => false,
            (Time::Comptime, Time::Runtime) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Impl {
    BuiltIn(BuiltInImpl),
}

impl Impl {
    pub fn comptime(&self) -> Option<ComptimeImpl> {
        match self {
            Self::BuiltIn(built_in) => built_in.comptime(),
        }
    }

    pub fn runtime(&self) -> Option<RuntimeImpl> {
        match self {
            Self::BuiltIn(built_in) => built_in.runtime(),
        }
    }

    pub fn arg_count(&self) -> usize {
        match self {
            Self::BuiltIn(built_in) => built_in.arg_count(),
        }
    }

    pub fn is_valid_time(&self, time: Time) -> bool {
        match self {
            Self::BuiltIn(built_in) => built_in.is_valid_time(time),
        }
    }
}

impl From<AnyBuiltInImpl> for Impl {
    fn from(value: AnyBuiltInImpl) -> Self {
        Self::BuiltIn(value.into())
    }
}

impl From<ComptimeBuiltInImpl> for Impl {
    fn from(value: ComptimeBuiltInImpl) -> Self {
        Self::BuiltIn(value.into())
    }
}

impl From<RuntimeBuiltInImpl> for Impl {
    fn from(value: RuntimeBuiltInImpl) -> Self {
        Self::BuiltIn(value.into())
    }
}

impl From<BuiltInImpl> for Impl {
    fn from(value: BuiltInImpl) -> Self {
        Self::BuiltIn(value)
    }
}

pub mod impls {
    pub use super::AnyBuiltInImpl::*;

    #[expect(unused_imports)]
    pub use super::ComptimeBuiltInImpl::*;

    pub use super::RuntimeBuiltInImpl::*;
}
