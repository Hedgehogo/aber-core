//! Module that provides [`CompNode`].
use super::{
    hir::{Character, Digits, Ident, String},
    CompExpr, Mir, Spanned, SpannedVec, Wast,
};
use crate::stages::parsing::{whitespace::Side, Expr, Node};

/// Type describing compilation units of any level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompNode {
    Wast(Wast<Self>),
    Mir(Mir),
}

impl CompNode {
    pub fn mir(&self) -> Option<&Mir> {
        match self {
            Self::Mir(mir) => Some(mir),
            _ => None,
        }
    }
}

impl Node for CompNode {
    type Expr = CompExpr;
    type Ident = Ident;
    type Digits = Digits;
    type Character = Character;
    type String = String;

    fn from_wast(wast: Wast<Self>) -> Self {
        Self::Wast(wast)
    }
}

impl Expr for CompExpr {
    type Node = CompNode;
    type Whitespace = ();

    fn whitespaced_seq(
        expr: Spanned<SpannedVec<Self::Node>>,
        _whitespace: Self::Whitespace,
        _side: Side,
    ) -> Spanned<SpannedVec<Self::Node>> {
        expr
    }
}
