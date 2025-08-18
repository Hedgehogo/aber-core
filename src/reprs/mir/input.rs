use super::super::{
    span::{Span, Spanned},
    CompNode,
};
use chumsky::input::{Input, MappedInput};

pub type Nodes<'input, 'comp> = MappedInput<
    CompNode<'input>,
    Span,
    &'comp [Spanned<CompNode<'input>>],
    fn(&'comp Spanned<CompNode<'input>>) -> (&'comp CompNode<'input>, &'comp Span),
>;

pub fn nodes<'input: 'comp, 'comp>(
    expr: Spanned<&'comp [Spanned<CompNode<'input>>]>,
) -> Nodes<'input, 'comp> {
    let Spanned(expr, span) = expr;
    let eoi = Span::new(span.end()..span.end());
    expr.map(eoi, move |tok| {
        let Spanned(tok, span) = tok;
        (tok, span)
    })
}
