use super::super::error::{Error, Expected};
use super::{spanned, GraphemeParser};
use crate::node::wast::number::{Digit, Digits, Number, Radix};
use chumsky::prelude::*;
use chumsky::text::{unicode::Grapheme, Char};

pub fn digit<'input>(radix: Radix) -> impl GraphemeParser<'input, Digit, Error<'input>> + Copy {
    let error = move |found, span| Error::new_expected(Expected::Digit(radix), found, span);
    any()
        .map_err(move |e: Error| error(None, e.span()))
        .try_map(move |i: &Grapheme, span: SimpleSpan| {
            i.to_ascii()
                .and_then(|i| Digit::from_ascii(i, radix))
                .ok_or_else(|| error(Some(i), span.into()))
        })
}

pub fn digits<'input>(
    radix: Radix,
) -> impl GraphemeParser<'input, Digits<'input>, Error<'input>> + Copy {
    digit(radix)
        .then(digit(radix).ignored().or(just("_").ignored()).repeated())
        .to_slice()
        .map(|i| unsafe { Digits::from_str_unchecked(i.as_str()) })
}

pub fn number<'input>() -> impl GraphemeParser<'input, Number<'input>, Error<'input>> {
    let frac = move |radix| digits(radix).or(empty().map(|_| Digits::default()));

    let unsigned = custom(move |input| {
        let (radix_or_int, span) = input.parse(spanned(digits(Radix::DECIMAL)))?;

        let (radix, int) = match input.parse(
            just("'")
                .map_err(|e: Error| e.replace_expected(Expected::RadixSpecial))
                .or_not(),
        )? {
            Some(_) => radix_or_int
                .as_str()
                .parse::<u8>()
                .ok()
                .and_then(Radix::new)
                .ok_or_else(|| Error::new_expected(Expected::Radix, None, span.into()))
                .and_then(|radix| Ok((radix, input.parse(digits(radix))?)))?,

            None => (Radix::DECIMAL, radix_or_int),
        };

        input
            .parse(
                just(".")
                    .map_err(|e: Error| e.replace_expected(Expected::NumberDot))
                    .ignore_then(frac(radix))
                    .or_not(),
            )
            .map(|frac| (radix, int, frac))
    });

    just("-")
        .map_err(move |e: Error| e.replace_expected(Expected::Minus))
        .or_not()
        .then(unsigned)
        .map(|(sign, (radix, int, frac))| Number::new(sign.is_none(), radix, int, frac))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::Span;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_number() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        let digits = |s| unsafe { Digits::from_str_unchecked(s) };
        assert_eq!(
            number().parse(Graphemes::new("10")).into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("10"), None))
        );
        assert_eq!(
            number().parse(Graphemes::new("10A")).into_result(),
            Err(vec![Error::new(
                smallvec![Expected::NumberDot, Expected::Eof],
                Some(grapheme("A")),
                Span::new(2..3)
            )])
        );
        assert_eq!(
            number().parse(Graphemes::new("36_000")).into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("36_000"), None))
        );
        assert_eq!(
            number().parse(Graphemes::new("_1")).into_result(),
            Err(vec![Error::new(
                smallvec![Expected::Minus, Expected::Digit(Radix::DECIMAL)],
                Some(grapheme("_")),
                Span::new(0..1)
            )])
        );
        assert_eq!(
            number().parse(Graphemes::new("10.05")).into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                Some(digits("05"))
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("10.")).into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                Some(digits(""))
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new(".1")).into_result(),
            Err(vec![Error::new(
                smallvec![Expected::Minus, Expected::Digit(Radix::DECIMAL)],
                Some(grapheme(".")),
                Span::new(0..1)
            )])
        );
        assert_eq!(
            number().parse(Graphemes::new("4'13.02")).into_result(),
            Ok(Number::new(
                true,
                Radix::new(4).unwrap(),
                digits("13"),
                Some(digits("02"))
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("2'2")).into_result(),
            Err(vec![Error::new_expected(
                Expected::Digit(Radix::BINARY),
                Some(grapheme("2")),
                Span::new(2..3)
            )])
        );
        assert_eq!(
            number().parse(Graphemes::new("36'ABER.")).into_result(),
            Ok(Number::new(
                true,
                Radix::MAX,
                digits("ABER"),
                Some(digits(""))
            ))
        );
        assert_eq!(
            number().parse(Graphemes::new("1'0")).into_result(),
            Err(vec![Error::new_expected(
                Expected::Radix,
                None,
                Span::new(0..1)
            )])
        );
        assert_eq!(
            number().parse(Graphemes::new("60'15")).into_result(),
            Err(vec![Error::new_expected(
                Expected::Radix,
                None,
                Span::new(0..2)
            )])
        );
    }
}
