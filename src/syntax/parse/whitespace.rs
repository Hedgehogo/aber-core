use super::super::{ctx::Ctx, error::Error};
use super::{GraphemeParser, GraphemeParserExtra};
use crate::node::whitespace::Whitespace;
use chumsky::prelude::*;
use smallvec::smallvec;
use text::{inline_whitespace, newline, Grapheme, Graphemes};

fn line_break<'input, E>() -> impl GraphemeParser<'input, &'input Grapheme, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    newline()
        .to_slice()
        .map(|i: &Graphemes| i.iter().next().unwrap())
}

fn line_start<'input, E, C>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<C>>,
{
    let outer_doc = just("///");
    let doc = outer_doc;

    inline_whitespace()
        .ignore_then(doc)
        .repeated()
        .configure(|cfg, ctx: &Ctx<C>| cfg.exactly(ctx.doc_ctx.depth()))
}

pub fn line_separator<'input, E, C>() -> impl GraphemeParser<'input, &'input Grapheme, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<C>>,
{
    line_break().then_ignore(line_start())
}

pub fn not_line_separator<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    newline().not()
}

pub fn whitespace<'input, W, E, C>(at_least: usize) -> impl GraphemeParser<'input, W, E> + Copy
where
    W: Whitespace<'input>,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<C>>,
{
    let comment = just("//")
        .map_err(|e: Error| Error::new(smallvec![], e.found(), e.span()))
        .then(not_line_separator().then(any()).repeated())
        .ignored();

    let line = inline_whitespace().then(comment.or_not());

    line.then(line_separator().ignore_then(line).repeated())
        .to_slice()
        .map(Graphemes::as_str)
        .try_map(move |i, span| {
            if i.len() >= at_least {
                Ok(W::from_repr_unchecked(i))
            } else {
                Err(Error::new(smallvec![], None, span.into()))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::Extra;
    use crate::node::wast::Whitespace;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            whitespace::<_, Extra, ()>(0)
                .parse(Graphemes::new(" //asdsad\n \t \n"))
                .into_result(),
            Ok(Whitespace::from_repr_unchecked(" //asdsad\n \t \n"))
        );
    }
}
