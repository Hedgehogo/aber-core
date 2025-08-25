//! Module that provides types to describe the compilation level that results in a weak abstract syntax tree (WAST).

pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod escaped_string;
pub mod expr_call;
pub mod initialization;
pub mod list;
pub mod negative_call;
pub mod number;
pub mod pair;
pub mod raw_string;
pub mod string;
pub mod wast_node;
pub mod whitespace;
pub mod whitespaced;

use super::span::{IntoSpanned, Span, Spanned, SpannedVec};
use crate::stages::syntax::{Expr, Node};
use std::fmt;

pub use assign::Assign;
pub use block::Block;
pub use call::Call;
pub use character::Character;
pub use escaped_string::EscapedString;
pub use expr_call::ExprCall;
pub use initialization::Initialization;
pub use list::List;
pub use negative_call::NegativeCall;
pub use number::Number;
pub use pair::Pair;
pub use raw_string::RawString;
pub use string::String;
pub use whitespace::Whitespace;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
pub enum Wast<N: Node> {
    Number(Number<N::Digits>),
    Character(N::Character),
    String(N::String),
    Pair(Pair<N>),
    Tuple(List<N::Expr, N::Expr>),
    Block(Block<N::Expr>),
    Call(Call<N::Expr>),
    MethodCall(ExprCall<N::Expr>),
    ChildCall(ExprCall<N::Expr>),
    NegativeCall(NegativeCall<N::Expr>),
    Initialization(Initialization<N::Expr>),
}

impl<N: Node> Wast<N> {
    /// Wraps in [`Node::from_wast`].
    pub fn into_node(self) -> N {
        N::from_wast(self)
    }

    /// Wraps in [`Node::from_wast`] and then in [`Spanned`].
    ///
    /// # Arguments
    /// * `span` Object of the type whose type is implements `Into<Span>`.
    pub fn into_spanned_node<S: Into<Span>>(self, span: S) -> Spanned<N> {
        Spanned(self.into_node(), span.into())
    }
}

impl<N: Node> Spanned<Wast<N>> {
    pub fn into_spanned_node(self) -> Spanned<N> {
        let Spanned(wast, span) = self;
        N::from_wast(wast).into_spanned(span)
    }
}

impl<N> fmt::Debug for Wast<N>
where
    N: Node + fmt::Debug,
    N::Expr: fmt::Debug,
    N::Ident: fmt::Debug,
    N::Digits: fmt::Debug,
    N::Character: fmt::Debug,
    N::String: fmt::Debug,
    <N::Expr as Expr>::Whitespace: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Character(arg0) => f.debug_tuple("Character").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Pair(arg0) => f.debug_tuple("Pair").field(arg0).finish(),
            Self::Tuple(arg0) => f.debug_tuple("Tuple").field(arg0).finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Call(arg0) => f.debug_tuple("Call").field(arg0).finish(),
            Self::MethodCall(arg0) => f.debug_tuple("MethodCall").field(arg0).finish(),
            Self::ChildCall(arg0) => f.debug_tuple("ChildCall").field(arg0).finish(),
            Self::NegativeCall(arg0) => f.debug_tuple("NegativeCall").field(arg0).finish(),
            Self::Initialization(arg0) => f.debug_tuple("Initialization").field(arg0).finish(),
        }
    }
}

impl<N> Clone for Wast<N>
where
    N: Node + Clone,
    N::Expr: Clone,
    N::Ident: Clone,
    N::Digits: Clone,
    N::Character: Clone,
    N::String: Clone,
    <N::Expr as Expr>::Whitespace: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::Character(arg0) => Self::Character(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Pair(arg0) => Self::Pair(arg0.clone()),
            Self::Tuple(arg0) => Self::Tuple(arg0.clone()),
            Self::Block(arg0) => Self::Block(arg0.clone()),
            Self::Call(arg0) => Self::Call(arg0.clone()),
            Self::MethodCall(arg0) => Self::MethodCall(arg0.clone()),
            Self::ChildCall(arg0) => Self::ChildCall(arg0.clone()),
            Self::NegativeCall(arg0) => Self::NegativeCall(arg0.clone()),
            Self::Initialization(arg0) => Self::Initialization(arg0.clone()),
        }
    }
}

impl<N> PartialEq for Wast<N>
where
    N: Node + PartialEq,
    N::Expr: PartialEq,
    N::Ident: PartialEq,
    N::Digits: PartialEq,
    N::Character: PartialEq,
    N::String: PartialEq,
    <N::Expr as Expr>::Whitespace: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Character(l0), Self::Character(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Pair(l0), Self::Pair(r0)) => l0 == r0,
            (Self::Tuple(l0), Self::Tuple(r0)) => l0 == r0,
            (Self::Block(l0), Self::Block(r0)) => l0 == r0,
            (Self::Call(l0), Self::Call(r0)) => l0 == r0,
            (Self::MethodCall(l0), Self::MethodCall(r0)) => l0 == r0,
            (Self::ChildCall(l0), Self::ChildCall(r0)) => l0 == r0,
            (Self::NegativeCall(l0), Self::NegativeCall(r0)) => l0 == r0,
            (Self::Initialization(l0), Self::Initialization(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<N> Eq for Wast<N>
where
    N: Node + Eq,
    N::Expr: Eq,
    N::Ident: Eq,
    N::Digits: Eq,
    N::Character: Eq,
    N::String: Eq,
    <N::Expr as Expr>::Whitespace: Eq,
{
}
