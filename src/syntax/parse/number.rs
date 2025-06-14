use super::super::error::{Error, Expected};
use super::{spanned, GraphemeParser, GraphemeParserExtra};
use crate::node::wast::number::{Digit, Digits, Number, Radix};
use chumsky::prelude::*;
use chumsky::text::{unicode::Grapheme, Char};

pub fn digit<'input, E>(radix: Radix) -> impl GraphemeParser<'input, Digit, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    let error = move |found, span| Error::new_expected(Expected::Digit(radix), found, span);
    any()
        .map_err(move |e: Error| error(None, e.span()))
        .try_map(move |i: &Grapheme, span: SimpleSpan| {
            i.to_ascii()
                .and_then(|i| Digit::from_ascii(i, radix))
                .ok_or_else(|| error(Some(i), span.into()))
        })
}

pub fn digits<'input, E>(
    radix: Radix,
    expected: Expected,
) -> impl GraphemeParser<'input, Digits<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    let spacer = just("_").map_err(|e: Error| e.replace_expected(Expected::NumberSpacer));
    digit(radix)
        .map_err(move |e: Error| e.replace_expected(expected))
        .then(digit(radix).ignored().or(spacer.ignored()).repeated())
        .to_slice()
        .map(|i| Digits::from_repr_unchecked(i.as_str()))
}

pub fn number<'input, E>() -> impl GraphemeParser<'input, Number<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>>,
{
    let frac =
        move |radix| digits(radix, Expected::Digit(radix)).or(empty().map(|_| Digits::default()));

    let unsigned = custom(move |input| {
        let (radix_or_int, span) =
            input.parse(spanned(digits(Radix::DECIMAL, Expected::Number)))?;

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
                .and_then(|radix| {
                    Ok((radix, input.parse(digits(radix, Expected::Digit(radix)))?))
                })?,

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
        .map_err(move |e: Error| e.replace_expected(Expected::Number))
        .or_not()
        .then(unsigned)
        .map(|(sign, (radix, int, frac))| Number::new(sign.is_none(), radix, int, frac))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::tests::Extra;
    use crate::node::span::Span;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_number() {
        let digits = |s| Digits::from_repr_unchecked(s);
        assert_eq!(
            number::<Extra>().parse(Graphemes::new("10")).into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("10"), None))
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("36_000"))
                .into_result(),
            Ok(Number::new(true, Radix::DECIMAL, digits("36_000"), None))
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("10.05"))
                .into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                Some(digits("05"))
            ))
        );
        assert_eq!(
            number::<Extra>().parse(Graphemes::new("10.")).into_result(),
            Ok(Number::new(
                true,
                Radix::DECIMAL,
                digits("10"),
                Some(digits(""))
            ))
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("4'13.02"))
                .into_result(),
            Ok(Number::new(
                true,
                Radix::new(4).unwrap(),
                digits("13"),
                Some(digits("02"))
            ))
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("36'ABER."))
                .into_result(),
            Ok(Number::new(
                true,
                Radix::MAX,
                digits("ABER"),
                Some(digits(""))
            ))
        );
    }

    #[test]
    fn test_number_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("10A"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![
                        Expected::Digit(Radix::DECIMAL),
                        Expected::NumberSpacer,
                        Expected::RadixSpecial,
                        Expected::NumberDot,
                        Expected::Eof
                    ],
                    Some(grapheme("A")),
                    Span::new(2..3)
                )]
            )
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("_1"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Number,
                    Some(grapheme("_")),
                    Span::new(0..1)
                )]
            )
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new(".1"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Number,
                    Some(grapheme(".")),
                    Span::new(0..1)
                )]
            )
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("2'2"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Digit(Radix::BINARY),
                    Some(grapheme("2")),
                    Span::new(2..3)
                )]
            )
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("1'0"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(Expected::Radix, None, Span::new(0..1))]
            )
        );
        assert_eq!(
            number::<Extra>()
                .parse(Graphemes::new("60'15"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(Expected::Radix, None, Span::new(0..2))]
            )
        );
    }
}
