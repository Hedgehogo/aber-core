use super::super::{
    span::{Span, Spanned},
    CompNode,
};
use chumsky::input::{Input, MappedInput};

pub type Nodes<'comp> = MappedInput<
    CompNode,
    Span,
    &'comp [Spanned<CompNode>],
    fn(&'comp Spanned<CompNode>) -> (&'comp CompNode, &'comp Span),
>;

pub fn nodes<'comp>(expr: Spanned<&'comp [Spanned<CompNode>]>) -> Nodes<'comp> {
    let Spanned(expr, span) = expr;
    let eoi = Span::new(span.end()..span.end());
    expr.map(eoi, move |tok| {
        let Spanned(tok, span) = tok;
        (tok, span)
    })
}
