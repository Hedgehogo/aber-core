use super::super::{ctx::Ctx, error::Expected, Expr, Whitespace};
use super::{
    end_cursor, entirely, spanned, Cursor, GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use crate::reprs::wast::whitespaced::Whitespaced;
use chumsky::{
    combinator::Repeated,
    prelude::*,
    text::{Char, Grapheme, Graphemes},
};

pub(crate) fn line_break<'input, E>() -> impl GraphemeParser<'input, &'input Grapheme, E> + Copy
where
    E: GraphemeParserExtra<'input>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    any()
        .filter(|c: &&Grapheme| c.is_newline())
        .labelled(Expected::Whitespace)
}

pub(crate) fn line_start<'input, E, C>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let outer_doc = entirely(just("///"), Expected::DocOuter);

    let doc = outer_doc;

    inline_whitespace()
        .ignore_then(doc)
        .repeated()
        .configure(|cfg, ctx: &Ctx<C>| cfg.exactly(ctx.doc_ctx.depth()))
}

pub fn line_separator<'input, E, C>() -> impl GraphemeParser<'input, &'input Grapheme, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    line_break().then_ignore(line_start())
}

pub fn line_separator_cursor<'input, E, C>() -> impl GraphemeParser<'input, Cursor, E> + Clone
where
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    end_cursor(line_break()).then_ignore(line_start())
}

pub fn not_line_separator<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    line_break().not()
}

pub fn inline_whitespace<'input, E>(
) -> Repeated<impl GraphemeParser<'input, (), E> + Copy, (), &'input Graphemes, E>
where
    E: GraphemeParserExtra<'input>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    any()
        .filter(|c: &&Grapheme| c.is_inline_whitespace())
        .labelled(Expected::Whitespace)
        .ignored()
        .repeated()
}

pub fn whitespace<'input, W, E, C>() -> impl GraphemeParser<'input, W, E> + Copy
where
    W: Whitespace<'input>,
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let comment = just("//")
        .then(not_line_separator().then(any()).repeated())
        .labelled(Expected::Whitespace)
        .as_context()
        .ignored();

    let line = inline_whitespace().then(comment.or_not());

    line.then(line_separator().ignore_then(line).repeated())
        .to_slice()
        .map(Graphemes::as_str)
        .map(W::from_repr_unchecked)
}

pub fn whitespaced<'input, R, X, P, E, C>(
    right: P,
) -> impl GraphemeParser<'input, Whitespaced<X, R>, E> + Clone
where
    X: Expr,
    X::Whitespace: Whitespace<'input>,
    E: GraphemeParserExtra<'input, Context = Ctx<C>>,
    E::Error: GraphemeLabelError<'input, Expected>,
    P: GraphemeParser<'input, R, E> + Clone,
{
    whitespace()
        .then(spanned(right))
        .map(|(whitespace, right)| Whitespaced::new(whitespace, right.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::Extra;
    use crate::reprs::wast::Whitespace;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            whitespace::<_, Extra, ()>()
                .parse(Graphemes::new(" //asdsad\n \t \n"))
                .into_result(),
            Ok(Whitespace::from_repr_unchecked(" //asdsad\n \t \n"))
        );
    }
}
