//! Module that provides types to describe the compilation level that results in a weak abstract syntax tree (WAST).

pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod expr_call;
pub mod negative_call;
pub mod number;
pub mod parser_output;
pub mod string;

use super::{span::Span, Expr, ExprVec, Spanned};
use assign::Assign;
use block::Block;
use call::Call;
use character::Character;
use expr_call::ExprCall;
use negative_call::NegativeCall;
use number::Number;
use parser_output::ParserOutput;
use std::fmt;
use string::String;

/// Type that describes a weak abstract syntax tree. In this case "weak" means that not all nestings can be explicitly resolved at this stage.
pub enum Wast<'input, N: ParserOutput<'input>> {
    Number(Number<'input>),
    Character(Character<'input>),
    String(String),
    Pair(Box<Spanned<N>>),
    Tuple(ExprVec<'input, N>),
    Block(Block<'input, N>),
    Call(Call<'input, N>),
    MethodCall(ExprCall<'input, N>),
    ChildCall(ExprCall<'input, N>),
    NegativeCall(NegativeCall<'input, N>),
}

impl<'input, N: ParserOutput<'input>> Wast<'input, N> {
    /// Wraps in [`Node::Wast`].
    pub fn into_node(self) -> N {
        N::new_node(self)
    }

    /// Wraps in [`Node::Wast`] and then in [`Spanned`].
    ///
    /// # Arguments
    /// * `span` Object of the type whose type is implements `Into<Span>`.
    pub fn into_spanned_node<S: Into<Span>>(self, span: S) -> Spanned<N> {
        Spanned(self.into_node(), span.into())
    }
}

impl<'input, N> fmt::Debug for Wast<'input, N>
where
    N: ParserOutput<'input> + fmt::Debug,
    N::Expr: fmt::Debug,
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
        }
    }
}

impl<'input, N> Clone for Wast<'input, N>
where
    N: ParserOutput<'input> + Clone,
    N::Expr: Clone,
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
        }
    }
}

impl<'input, N: ParserOutput<'input>> PartialEq for Wast<'input, N>
where
    N: ParserOutput<'input> + PartialEq,
    N::Expr: PartialEq,
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
            _ => false,
        }
    }
}

impl<'input, N: ParserOutput<'input> + Eq> Eq for Wast<'input, N> where N::Expr: Eq {}
