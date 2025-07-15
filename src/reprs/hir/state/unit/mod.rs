mod function;

pub(super) use function::Function;

pub(super) enum Unit {
    Function(Function),
}
