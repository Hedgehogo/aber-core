use super::super::{
    span::{Span, Spanned},
    CompNode,
};
use chumsky::input::{Input, MappedInput};

pub trait NodesMapper<'input: 'comp, 'comp>:
    Fn(&'comp Spanned<CompNode<'input>>) -> (&'comp CompNode<'input>, &'comp Span) + 'comp + 'input
{
}

impl<'input, 'comp, T> NodesMapper<'input, 'comp> for T
where
    'input: 'comp,
    T: Fn(&'comp Spanned<CompNode<'input>>) -> (&'comp CompNode<'input>, &'comp Span),
    T: 'comp + 'input,
{
}

pub type Nodes<'input, 'comp, F> =
    MappedInput<CompNode<'input>, Span, &'comp [Spanned<CompNode<'input>>], F>;

pub fn nodes<'input: 'comp, 'comp>(
    expr: Spanned<&'comp [Spanned<CompNode<'input>>]>,
) -> Nodes<'input, 'comp, impl NodesMapper<'input, 'comp>> {
    let Spanned(expr, span) = expr;
    let eoi = Span::new(span.end()..span.end());
    expr.map(eoi, move |tok| {
        let Spanned(tok, span) = tok;
        (tok, span)
    })
}
