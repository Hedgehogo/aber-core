use super::super::error::Error;
use super::GraphemeParser;
use crate::node::wast::Wast;
use chumsky::prelude::*;
use text::newline;

pub fn whitespace<'input>() -> impl GraphemeParser<'input, (), Error<'input>> {
    let comment = just("//")
        .then(newline().not().then(any()).repeated())
        .ignored();
    text::whitespace().at_least(1).or(comment).repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use crate::node::{
        span::Span,
        wast::{
            character::Character,
            number::{Digits, Number, Radix},
        },
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            whitespace().parse(Graphemes::new(" //asdsad\n \t \n")).into_result(),
            Ok(())
        );
    }
}
