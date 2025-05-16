use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
};
use super::{
    list::generics, number::digit, spanned, whitespace::whitespace, GraphemeParser,
    GraphemeParserExtra,
};
use crate::node::{
    wast::{
        call::{Call, Generics, Ident},
        number::Radix,
    },
    Expr, Spanned,
};
use chumsky::prelude::*;

pub fn ident<'input, E>() -> impl GraphemeParser<'input, Ident<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let number_start = just("-").or_not().then(digit(Radix::DECIMAL));

    let unit = whitespace::<(), _, _>(1)
        .not()
        .ignore_then(none_of(".,;:'\"@(){}[]"));

    number_start
        .not()
        .ignore_then(unit.repeated().at_least(1))
        .to_slice()
        .try_map(|i, span| {
            if i.as_str() != "=" {
                Ok(i)
            } else {
                Err(Error::new_expected(Expected::ValidIdent, None, span.into()))
            }
        })
        .map_err(|e: Error| e.replace_expected(Expected::Ident))
        .map(|i| Ident::from_repr_unchecked(i.as_str()))
}

pub fn call<'input, X, P, E>(expr: P) -> impl GraphemeParser<'input, Call<'input, X>, E> + Clone
where
    X: Expr<'input>,
    P: GraphemeParser<'input, Spanned<X>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let generics = whitespace(0)
        .then(spanned(generics::<X, _, _>(expr)).map(Spanned::from))
        .map(|(whitespace, args)| Generics::new(whitespace, args))
        .or_not();

    spanned(ident())
        .map(Spanned::from)
        .then(generics)
        .map(|(ident, generics)| Call::new(ident, generics))
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::{expr::expr, fact::fact, tests::Extra};
    use crate::node::{
        span::{IntoSpanned, Span},
        wast::List,
        CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_ident() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            ident::<Extra>()
                .parse(Graphemes::new("hello"))
                .into_result(),
            Ok(Ident::from_repr_unchecked("hello"))
        );
        assert_eq!(
            ident::<Extra>()
                .parse(Graphemes::new("-hello"))
                .into_result(),
            Ok(Ident::from_repr_unchecked("-hello"))
        );
        assert_eq!(
            ident::<Extra>()
                .parse(Graphemes::new("9hello"))
                .into_output_errors(),
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
            ident::<Extra>()
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
            ident::<Extra>()
                .parse(Graphemes::new("@hello"))
                .into_output_errors(),
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
            ident::<Extra>()
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
            call(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("hello"))
                .into_result(),
            Ok(Call::new(
                (Ident::from_repr_unchecked("hello"), 0..5).into(),
                None
            ))
        );
        assert_eq!(
            call(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("hello[]"))
                .into_result(),
            Ok(Call::new(
                Ident::from_repr_unchecked("hello").into_spanned(0..5),
                Some(Generics::new((), List::new(vec![], ()).into_spanned(5..7)))
            ))
        );
        assert_eq!(
            call(expr(fact::<CompNode, Extra>()))
                .parse(Graphemes::new("hello //hello\n []"))
                .into_result(),
            Ok(Call::new(
                Ident::from_repr_unchecked("hello").into_spanned(0..5),
                Some(Generics::new(
                    (),
                    List::new(vec![], ()).into_spanned(15..17)
                ))
            ))
        );
        assert_eq!(
            call(expr(fact::<CompNode, Extra>()))
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
