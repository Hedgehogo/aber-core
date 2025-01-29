mod function;

pub use function::FunctionRef;

pub enum UnitRef<'state, 'input> {
    Function(FunctionRef<'state, 'input>),
}
