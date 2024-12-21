use super::super::error::Error;
use super::{
    character::character, number::number, raw_string::raw_string, spanned, string::string,
    whitespace::whitespace, GraphemeParser,
};
use crate::node::wast::Wast;
use crate::node::{Node, Spanned};
use chumsky::prelude::*;

pub fn meaningful_unit<'input>() -> impl GraphemeParser<'input, Spanned<Node<'input>>, Error<'input>> + Clone
{
    recursive(|_meaningful_unit| {
        let choice = choice((
            number().map(Wast::Number),
            character().map(Wast::Character),
            string().map(Wast::String),
            raw_string().map(Wast::String),
        ));

        spanned(choice.map(Node::Wast))
            .map(Spanned::from)
            .then(whitespace().ignore_then(just(":")).or_not())
            .map_with(|(i, pair), extra| match pair {
                Some(_) => Wast::Pair(Box::new(i))
                    .into_node()
                    .into_spanned(extra.span()),
                None => i,
            })
    })
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
    fn test_meaningful_unit() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        let digits = |s| unsafe { Digits::from_str_unchecked(s) };
        assert_eq!(
            meaningful_unit().parse(Graphemes::new("10")).into_result(),
            Ok(Wast::Number(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                None
            )).into_node()
            .into_spanned(0..2))
        );
        assert_eq!(
            meaningful_unit().parse(Graphemes::new("'m'")).into_result(),
            Ok(Wast::Character(grapheme("m").into())
                .into_node()
                .into_spanned(0..3))
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new("\"Hello\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_node().into_spanned(0..7))
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new("\"\"\"\nHello\n\"\"\""))
                .into_result(),
            Ok(Wast::String("Hello".into()).into_node().into_spanned(0..13))
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new("'g:"))
                .into_output_errors(),
            (
                Some(
                    Wast::Pair(Box::new(
                        Wast::Character(grapheme("g").into())
                            .into_node()
                            .into_spanned(0..2)
                    ))
                    .into_node()
                    .into_spanned(0..3)
                ),
                vec![Error::new_expected(
                    Expected::CharSpecial,
                    None,
                    Span::new(2..2)
                )]
            )
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new(":"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Digit(Radix::DECIMAL),
                        Expected::CharSpecial,
                        Expected::StringSpecial,
                        Expected::RawStringStart
                    ],
                    Some(grapheme(":")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
