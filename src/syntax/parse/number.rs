use super::{spanned, GraphemeParser};
use crate::node::wast::number::{Digit, Digits, Number, Radix};
use chumsky::prelude::*;
use chumsky::text::{unicode::Grapheme, Char};

pub fn digit<'input>(radix: Radix) -> impl GraphemeParser<'input, Digit> + Copy {
    any().try_map(move |i: &Grapheme, span| {
        i.to_ascii()
            .and_then(|i| Digit::from_ascii(i, radix))
            .ok_or_else(|| Rich::custom(span, "A digit was expected"))
    })
}

pub fn digits<'input>(radix: Radix) -> impl GraphemeParser<'input, Digits<'input>> + Copy {
    digit(radix)
        .then(digit(radix).ignored().or(just("_").ignored()).repeated())
        .to_slice()
        .map(|i| unsafe { Digits::from_str_unchecked(i.as_str()) })
}

pub fn number<'input>() -> impl GraphemeParser<'input, Number<'input>> {
    let frac = move |radix| digits(radix).or(empty().map(|_| Digits::default()));

    let unsigned = custom(move |input| {
        let (radix_or_int, span) = input.parse(spanned(digits(Radix::DECIMAL)))?;

        let (radix, int) = match input.parse(just("'").or_not())? {
            Some(_) => {
                let radix = radix_or_int
                    .as_str()
                    .parse::<u8>()
                    .ok()
                    .and_then(Radix::new)
                    .ok_or_else(|| {
                        let msg = "A decimal number in the range from 1 to 36 was expected";
                        Rich::custom(span, msg)
                    })?;

                (radix, input.parse(digits(radix))?)
            }

            None => (Radix::DECIMAL, radix_or_int),
        };

        input
            .parse(just(".").ignore_then(frac(radix)).or_not())
            .map(|frac| (radix, int, frac))
    });

    just("-")
        .or_not()
        .then(unsigned)
        .map(|(sign, (radix, int, frac))| Number::new(sign.is_none(), radix, int, frac))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::util::Maybe;
    use text::Graphemes;

    #[test]
    fn test_number() {
        let digits = |s| unsafe { Digits::from_str_unchecked(s) };
        let digits_some = |s| Some(digits(s));
        assert_eq!(
            number().parse(Graphemes::new("10")).into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("10"), None))
        );
        assert_eq!(
            number().parse(Graphemes::new("36_000")).into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("36_000"), None))
        );
        assert_eq!(
            number().parse(Graphemes::new("10.05")).into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                digits_some("05")
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("10.")).into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                digits_some("")
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("4'10.02")).into_result(),
            Ok(Number::new(
                true,
                Radix::new(4).unwrap(),
                digits("10"),
                digits_some("02")
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("36'ABER.")).into_result(),
            Ok(Number::new(
                true,
                Radix::MAX,
                digits("ABER"),
                digits_some("")
            ))
        );
    }
}
