pub mod call;
pub mod pair;

use super::super::{
    span::{IntoSpanned, Spanned},
    CompNode,
};

pub use call::Call;
pub use pair::Pair;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mir {
    Call(Call),
    Nil,
}

impl Mir {
    pub fn call(&self) -> Option<&Call> {
        match self {
            Self::Call(call) => Some(call),
            _ => None,
        }
    }
}

impl Spanned<Mir> {
    pub fn into_spanned_node(self) -> Spanned<CompNode> {
        let Spanned(mir, span) = self;
        CompNode::Mir(mir).into_spanned(span)
    }
}
