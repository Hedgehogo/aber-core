mod function;

pub(super) use function::Function;

pub(super) enum Unit {
    #[expect(dead_code)]
    Function(Function),
}
