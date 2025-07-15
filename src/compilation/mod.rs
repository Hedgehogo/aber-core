pub mod call;

use crate::node::{
    state::{Nodes, State},
    CompExpr, CompNode, Hir, Spanned,
};
use call::call;
use chumsky::{error::Cheap, extra::ParserExtra, prelude::*};

pub trait CompParser<'input, O, E>: Parser<'input, Nodes<'input>, O, E>
where
    E: CompParserExtra<'input>,
    E::Context: Clone,
{
}

impl<'input, T, O, E> CompParser<'input, O, E> for T
where
    T: Parser<'input, Nodes<'input>, O, E>,
    E: CompParserExtra<'input>,
    E::Context: Clone,
{
}

pub trait CompParserExtra<'input>:
    ParserExtra<'input, Nodes<'input>, State = State<'input>, Error = Cheap>
where
    Self::Context: Clone,
{
}

impl<'input, T> CompParserExtra<'input> for T
where
    T: ParserExtra<'input, Nodes<'input>, State = State<'input>, Error = Cheap>,
    T::Context: Clone,
{
}

pub fn fact<'input, E>() -> impl CompParser<'input, Spanned<CompNode<'input>>, E> + Clone
where
    E: CompParserExtra<'input>,
    E::Context: Clone,
{
    recursive(|fact| call(fact).map(|call| call.map(|call| CompNode::Hir(Hir::Call(call)))))
}

pub fn expr<'input, E>() -> impl CompParser<'input, CompExpr<'input>, E> + Clone
where
    E: CompParserExtra<'input>,
    E::Context: Clone,
{
    fact().repeated().collect().map(CompExpr::Wast)
}
