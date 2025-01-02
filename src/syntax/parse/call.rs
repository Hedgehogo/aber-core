use super::super::error::{Error, Expected};
use super::{list::generics, number::digit, spanned, whitespace::whitespace, GraphemeParser};
use crate::node::{
    wast::{
        call::{Call, Ident},
        number::Radix,
    },
    Expr, Spanned,
};
use chumsky::prelude::*;

pub fn ident<'input>() -> impl GraphemeParser<'input, Ident<'input>, Error<'input>> + Copy {
    let number_start = just("-").or_not().then(digit(Radix::DECIMAL));

    let unit = whitespace()
        .at_least(1)
        .not()
        .ignore_then(none_of(".,;:'\"@(){}[]"));

    number_start
        .not()
        .ignore_then(unit.repeated().at_least(1))
        .to_slice()
        .filter(|i| i.as_str() != "=")
        .map_err(|e: Error| e.replace_expected(Expected::Ident))
        .map(|i| Ident::new(i.as_str()))
}

pub fn call<'input, E>(
    expression: E,
) -> impl GraphemeParser<'input, Call<'input>, Error<'input>> + Clone
where
    E: GraphemeParser<'input, Spanned<Expr<'input>>, Error<'input>> + Clone,
{
    let generics = whitespace().ignore_then(spanned(
        generics(expression).or_not().map(Option::unwrap_or_default),
    ));

    spanned(ident())
        .map(Spanned::from)
        .then(generics.map(Spanned::from))
        .map(|(ident, generics)| Call::new(ident, generics))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{expression::expression, meaningful_unit::meaningful_unit};
    use crate::node::span::Span;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_ident() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            ident().parse(Graphemes::new("hello")).into_result(),
            Ok(Ident::new("hello"))
        );
        assert_eq!(
            ident().parse(Graphemes::new("-hello")).into_result(),
            Ok(Ident::new("-hello"))
        );
        assert_eq!(
            ident().parse(Graphemes::new("9hello")).into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Ident,
                    Some(grapheme("9")),
                    Span::new(0..1)
                )]
            )
        );
        assert_eq!(
            ident()
                .parse(Graphemes::new("-9hello"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Ident,
                    Some(grapheme("-")),
                    Span::new(0..2)
                )]
            )
        );
        assert_eq!(
            ident().parse(Graphemes::new("@hello")).into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Ident,
                    Some(grapheme("@")),
                    Span::new(0..1)
                )]
            )
        );
        assert_eq!(
            ident()
                .parse(Graphemes::new("//hello"))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(
                    Expected::Ident,
                    Some(grapheme("/")),
                    Span::new(0..7)
                )]
            )
        );
    }

    #[test]
    fn test_call() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            call(expression(meaningful_unit()))
                .parse(Graphemes::new("hello"))
                .into_result(),
            Ok(Call::new(
                (Ident::new("hello"), 0..5).into(),
                (vec![], 5..5).into()
            ))
        );
        assert_eq!(
            call(expression(meaningful_unit()))
                .parse(Graphemes::new("hello[]"))
                .into_result(),
            Ok(Call::new(
                (Ident::new("hello"), 0..5).into(),
                (vec![], 5..7).into()
            ))
        );
        assert_eq!(
            call(expression(meaningful_unit()))
                .parse(Graphemes::new("hello //hello\n []"))
                .into_result(),
            Ok(Call::new(
                (Ident::new("hello"), 0..5).into(),
                (vec![], 15..17).into()
            ))
        );
        assert_eq!(
            call(expression(meaningful_unit()))
                .parse(Graphemes::new("hello,[]"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Generics, Expected::Eof],
                    Some(grapheme(",")),
                    Span::new(5..6)
                )]
            )
        );
    }
}
