use super::super::error::Error;
use super::GraphemeParser;
use crate::node::whitespace::Whitespace;
use chumsky::prelude::*;
use smallvec::smallvec;
use text::{newline, Graphemes};

pub fn whitespace<'input, W>(
    at_least: usize,
) -> impl GraphemeParser<'input, W, Error<'input>> + Copy
where
    W: Whitespace<'input>,
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

    use text::Graphemes;
    use crate::node::wast::Whitespace;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            whitespace(0)
                .parse(Graphemes::new(" //asdsad\n \t \n"))
                .into_result(),
            Ok(Whitespace::from_repr_unchecked(" //asdsad\n \t \n"))
        );
    }
}
