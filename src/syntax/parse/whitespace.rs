use super::super::error::Error;
use super::GraphemeParser;
use chumsky::prelude::*;
use smallvec::smallvec;
use text::newline;

pub fn whitespace<'input>() -> impl GraphemeParser<'input, (), Error<'input>> + Copy {
    let comment = just("//")
        .map_err(|e: Error| Error::new(smallvec![], e.found(), e.span()))
        .then(newline().not().then(any()).repeated())
        .ignored();

    text::whitespace()
        .at_least(1)
        .or(comment)
        .repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    use text::Graphemes;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            whitespace()
                .parse(Graphemes::new(" //asdsad\n \t \n"))
                .into_result(),
            Ok(())
        );
    }
}
