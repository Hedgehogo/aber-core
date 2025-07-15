mod function;

pub use function::FunctionRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitRef<'state, 'input> {
    Function(FunctionRef<'state, 'input>),
}
