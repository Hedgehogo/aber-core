pub mod call;

use crate::reprs::{
    mir::{Nodes, State},
    span::{Span, Spanned},
    CompExpr, CompNode, Mir,
};
use call::call;
use chumsky::{error::Cheap, extra::ParserExtra, prelude::*};

pub trait CompParser<'comp, O, E>: Parser<'comp, Nodes<'comp>, O, E>
where
    E: CompParserExtra<'comp>,
    E::Context: Clone,
{
}

impl<'comp, T, O, E> CompParser<'comp, O, E> for T
where
    T: Parser<'comp, Nodes<'comp>, O, E>,
    E: CompParserExtra<'comp>,
    E::Context: Clone,
{
}

pub trait CompParserExtra<'comp>:
    ParserExtra<'comp, Nodes<'comp>, State = State, Error = Cheap<Span>>
where
    Self::Context: Clone,
{
}

impl<'comp, T> CompParserExtra<'comp> for T
where
    T: ParserExtra<'comp, Nodes<'comp>, State = State, Error = Cheap<Span>>,
    T::Context: Clone,
{
}

pub fn fact<'comp, E>() -> impl CompParser<'comp, CompNode, E> + Clone
where
    E: CompParserExtra<'comp>,
    E::Context: Clone,
{
    recursive(|fact| call(fact).map(|call| CompNode::Mir(Mir::Call(call))))
}

pub fn expr<'comp, E>() -> impl CompParser<'comp, CompExpr, E> + Clone
where
    E: CompParserExtra<'comp>,
    E::Context: Clone,
{
    fact()
        .map_with(|fact, extra| Spanned(fact, extra.span()))
        .repeated()
        .collect()
        .map(CompExpr::Wast)
}
