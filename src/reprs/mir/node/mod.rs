pub mod call;
pub mod pair;
pub mod string;

use super::super::{
    span::{IntoSpanned, Spanned},
    CompNode,
};

pub use call::Call;
pub use pair::Pair;
pub use string::String;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mir<'input> {
    Call(Call<'input>),
    Nil,
}

impl<'input> Mir<'input> {
    pub fn call(&self) -> Option<&Call<'input>> {
        match self {
            Self::Call(call) => Some(call),
            _ => None,
        }
    }
}

impl<'input> Spanned<Mir<'input>> {
    pub fn into_spanned_node(self) -> Spanned<CompNode<'input>> {
        let Spanned(mir, span) = self;
        CompNode::Mir(mir).into_spanned(span)
    }
}
