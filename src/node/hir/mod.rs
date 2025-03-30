pub mod call;
pub mod pair;
pub mod string;

use super::{CompNode, Spanned};

pub use call::Call;
pub use pair::Pair;
pub use string::String;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hir<'input> {
    Constant(Box<CompNode<'input>>),
    Call(Call<'input>),
    Pair(Pair<'input>),
    Tuple(Vec<Spanned<CompNode<'input>>>),
    Nil,
}
