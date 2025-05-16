use super::super::{ctx::Ctx, error::Error};
use super::{GraphemeParser, GraphemeParserExtra};
use crate::node::whitespace::Whitespace;
use chumsky::prelude::*;
use smallvec::smallvec;
use text::{newline, Graphemes};

pub fn whitespace<'input, W, E, C>(at_least: usize) -> impl GraphemeParser<'input, W, E> + Copy
where
    W: Whitespace<'input>,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<C>>,
{
    let comment = just("//")
        .map_err(|e: Error| Error::new(smallvec![], e.found(), e.span()))
        .then(newline().not().then(any()).repeated())
        .ignored();

    text::whitespace()
        .at_least(1)
        .or(comment)
        .repeated()
        .at_least(at_least)
        .to_slice()
        .map(Graphemes::as_str)
        .map(W::from_repr_unchecked)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::Extra;
    use crate::node::wast::Whitespace;
    use text::Graphemes;

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
