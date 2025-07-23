pub mod call;

use crate::reprs::{
    hir::{Nodes, NodesMapper, State},
    span::{Span, Spanned},
    CompExpr, CompNode, Hir,
};
use call::call;
use chumsky::{error::Cheap, extra::ParserExtra, prelude::*};

pub trait CompParser<'input, 'comp, O, E, F>: Parser<'comp, Nodes<'input, 'comp, F>, O, E>
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp, F>,
    E::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
}

impl<'input, 'comp, T, O, E, F> CompParser<'input, 'comp, O, E, F> for T
where
    'input: 'comp,
    T: Parser<'comp, Nodes<'input, 'comp, F>, O, E>,
    E: CompParserExtra<'input, 'comp, F>,
    E::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
}

pub trait CompParserExtra<'input, 'comp, F>:
    ParserExtra<'comp, Nodes<'input, 'comp, F>, State = State<'input>, Error = Cheap<Span>>
where
    'input: 'comp,
    Self::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
}

impl<'input, 'comp, F, T> CompParserExtra<'input, 'comp, F> for T
where
    'input: 'comp,
    T: ParserExtra<'comp, Nodes<'input, 'comp, F>, State = State<'input>, Error = Cheap<Span>>,
    T::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
}

pub fn fact<'input, 'comp, E, F>() -> impl CompParser<'input, 'comp, CompNode<'input>, E, F> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp, F>,
    E::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
    recursive(|fact| call(fact).map(|call| CompNode::Hir(Hir::Call(call))))
}

pub fn expr<'input, 'comp, E, F>() -> impl CompParser<'input, 'comp, CompExpr<'input>, E, F> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp, F>,
    E::Context: Clone,
    F: NodesMapper<'input, 'comp>,
{
    fact()
        .map_with(|fact, extra| Spanned(fact, extra.span()))
        .repeated()
        .collect()
        .map(CompExpr::Wast)
}
