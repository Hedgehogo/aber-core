pub mod call;

use crate::reprs::{
    mir::{Nodes, State},
    span::{Span, Spanned},
    CompExpr, CompNode, Mir,
};
use call::call;
use chumsky::{error::Cheap, extra::ParserExtra, prelude::*};

pub trait CompParser<'input, 'comp, O, E>: Parser<'comp, Nodes<'input, 'comp>, O, E>
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
{
}

impl<'input, 'comp, T, O, E> CompParser<'input, 'comp, O, E> for T
where
    'input: 'comp,
    T: Parser<'comp, Nodes<'input, 'comp>, O, E>,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
{
}

pub trait CompParserExtra<'input, 'comp>:
    ParserExtra<'comp, Nodes<'input, 'comp>, State = State<'input>, Error = Cheap<Span>>
where
    'input: 'comp,
    Self::Context: Clone,
{
}

impl<'input, 'comp, T> CompParserExtra<'input, 'comp> for T
where
    'input: 'comp,
    T: ParserExtra<'comp, Nodes<'input, 'comp>, State = State<'input>, Error = Cheap<Span>>,
    T::Context: Clone,
{
}

pub fn fact<'input, 'comp, E>() -> impl CompParser<'input, 'comp, CompNode<'input>, E> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
{
    recursive(|fact| call(fact).map(|call| CompNode::Mir(Mir::Call(call))))
}

pub fn expr<'input, 'comp, E>() -> impl CompParser<'input, 'comp, CompExpr<'input>, E> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
{
    fact()
        .map_with(|fact, extra| Spanned(fact, extra.span()))
        .repeated()
        .collect()
        .map(CompExpr::Wast)
}
