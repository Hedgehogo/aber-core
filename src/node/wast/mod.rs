pub mod assign;
pub mod block;
pub mod call;
pub mod character;
pub mod expr_call;
pub mod negative_call;
pub mod number;
pub mod string;

use super::{span::Span, Expr, ExprVec, Node, Spanned};
use assign::Assign;
use block::Block;
use call::Call;
use character::Character;
use expr_call::ExprCall;
use negative_call::NegativeCall;
use number::Number;
use string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wast<'input> {
    Number(Number<'input>),
    Character(Character<'input>),
    String(String),
    Pair(Box<Spanned<Node<'input>>>),
    Tuple(ExprVec<'input>),
    Block(Block<'input>),
    Call(Call<'input>),
    MethodCall(ExprCall<'input>),
    ChildCall(ExprCall<'input>),
    NegativeCall(NegativeCall<'input>),
}

impl<'input> Wast<'input> {
    pub fn into_node(self) -> Node<'input> {
        self.into()
    }
}

impl<'input> Wast<'input> {
    pub fn into_spanned_node<S: Into<Span>>(self, span: S) -> Spanned<Node<'input>> {
        Spanned(self.into(), span.into())
    }
}

impl<'input> From<Wast<'input>> for Node<'input> {
    fn from(value: Wast<'input>) -> Self {
        Node::Wast(value)
    }
}
