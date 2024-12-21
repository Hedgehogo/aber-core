use super::super::error::{Error, Expected};
use super::{
    character::character, number::number, raw_string::raw_string, string::string, GraphemeParser,
};
use crate::node::wast::Wast;
use chumsky::prelude::*;

pub fn meaningful_unit<'input>() -> impl GraphemeParser<'input, Wast<'input>, Error<'input>> {
    recursive(|_meaningful_unit| {
        choice((
            number().map(Wast::Number),
            character().map(Wast::Character),
            string().map(Wast::String),
            raw_string().map(Wast::String),
        ))
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
            )))
        );
        assert_eq!(
            meaningful_unit().parse(Graphemes::new("'m'")).into_result(),
            Ok(Wast::Character(Character::new(grapheme("m"))))
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new("\"Hello\""))
                .into_result(),
            Ok(Wast::String("Hello".into()))
        );
        assert_eq!(
            meaningful_unit()
                .parse(Graphemes::new("\"\"\"\nHello\n\"\"\""))
                .into_result(),
            Ok(Wast::String("Hello".into()))
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
