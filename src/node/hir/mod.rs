pub mod call;

use super::Node;
use call::Call;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hir<'input> {
    Constant(Box<Node<'input>>),
    Call(Call<'input>),
    Nil,
}
