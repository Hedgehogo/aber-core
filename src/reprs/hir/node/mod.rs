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
pub enum Hir<'input> {
    Call(Call<'input>),
    Nil,
}

impl<'input> Hir<'input> {
    pub fn call(&self) -> Option<&Call<'input>> {
        match self {
            Self::Call(call) => Some(call),
            _ => None,
        }
    }
}

impl<'input> Spanned<Hir<'input>> {
    pub fn into_spanned_node(self) -> Spanned<CompNode<'input>> {
        let Spanned(hir, span) = self;
        CompNode::Hir(hir).into_spanned(span)
    }
}
