use super::super::error::Expected;
use super::{spanned, GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
use crate::node::wast::number::{Digit, Digits, Number, Radix};
use chumsky::{
    label::LabelError,
    prelude::*,
    text::{Char, Grapheme},
};

pub fn digit<'input, E>() -> impl GraphemeParser<'input, Digit, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Radix>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    custom(|input| {
        let before = input.cursor();
        let found = input.parse(any()).ok();
        let span: SimpleSpan = input.span_since(&before);
        found
            .and_then(|grapheme: &Grapheme| {
                grapheme
                    .to_ascii()
                    .and_then(|ascii| Digit::from_ascii(ascii, *input.ctx()))
            })
            .ok_or(LabelError::expected_found(
                [Expected::Digit(*input.ctx())],
                found.map(Into::into),
                span,
            ))
    })
}

pub fn digits<'input, E>() -> impl GraphemeParser<'input, Digits<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Radix>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let spacer = just("_").labelled(Expected::NumberSpacer);
    digit()
        .then(digit().ignored().or(spacer.ignored()).repeated())
        .to_slice()
        .map(|i| Digits::from_repr_unchecked(i.as_str()))
}

pub fn number<'input, E>() -> impl GraphemeParser<'input, Number<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let frac = digits().or(empty().map(|_| Digits::default()));

    let unsigned = custom(move |input| {
        let (radix_or_int, span) = input.parse(spanned(digits().with_ctx(Radix::DECIMAL)))?;

        let (radix, int) = match input.parse(just("'").labelled(Expected::RadixSpecial).or_not())? {
            Some(_) => radix_or_int
                .as_str()
                .parse::<u8>()
                .ok()
                .and_then(Radix::new)
                .ok_or_else(|| LabelError::expected_found([Expected::Radix], None, span))
                .and_then(|radix| Ok((radix, input.parse(digits().with_ctx(radix))?)))?,

            None => (Radix::DECIMAL, radix_or_int),
        };

        input
            .parse(
                just(".")
                    .labelled(Expected::NumberDot)
                    .ignore_then(frac.with_ctx(radix))
                    .or_not(),
            )
            .map(|frac| (radix, int, frac))
    });

    just("-")
        .or_not()
        .then(unsigned)
        .map(|(sign, (radix, int, frac))| Number::new(sign.is_none(), radix, int, frac))
        .labelled(Expected::Number)
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Error;
    use super::super::tests::Extra;
    use crate::node::span::Span;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test() {
        use chumsky::extra::Err;

        pub fn parser<'src>() -> impl Parser<'src, &'src str, (), Err<Rich<'src, char, SimpleSpan>>>
        {
            let b = just("_").then(just("a").labelled("al")).ignored();
            let custom = custom(move |input| input.parse(b));
            custom.labelled("bl")
        }

        assert_eq!(
            parser().parse("_c").into_output_errors(),
            (
                None,
                vec![{
                    let mut err = LabelError::<&str, _>::expected_found(
                        ["al"],
                        Some('c'.into()),
                        SimpleSpan::new((), 1..2),
                    );
                    LabelError::<&str, _>::in_context(&mut err, "bl", SimpleSpan::new((), 0..2));
                    err
                }]
            )
        );
    }

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
