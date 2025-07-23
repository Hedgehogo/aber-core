pub mod function;

use function::Function;

pub use function::{FunctionMut, FunctionRef};

#[non_exhaustive]
pub(super) enum Unit {
    Function(Function),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitRef<'state, 'input> {
    Function(FunctionRef<'state, 'input>),
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitMut<'state, 'input> {
    Function(FunctionMut<'state, 'input>),
}
