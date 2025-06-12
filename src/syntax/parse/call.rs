use super::super::{
    ctx::Ctx,
    error::{Error, Expected},
    Node,
};
use super::{
    list::generics, number::digit, spanned, whitespace::whitespaced, GraphemeParser,
    GraphemeParserExtra,
};
use crate::node::{
    wast::{
        call::{Call, Ident},
        number::Radix,
    },
    Spanned, SpannedVec,
};
use chumsky::prelude::*;

pub fn ident<'input, E>() -> impl GraphemeParser<'input, Ident<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let number_start = just("-").or_not().then(digit(Radix::DECIMAL));

    let not_unit = choice((
        one_of(".,;:'\"@(){}[]").ignored(),
        just("//").ignored(),
        just("///").ignored(),
        just("```").ignored(),
        text::whitespace().exactly(1),
    ));

    let unit = not_unit.not().ignore_then(any());

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

pub fn call<'input, N, P, E>(
    expr: P,
) -> impl GraphemeParser<'input, Call<'input, N::Expr>, E> + Clone
where
    N: Node<'input>,
    P: GraphemeParser<'input, Spanned<SpannedVec<N>>, E> + Clone,
    E: GraphemeParserExtra<'input, Error = Error<'input>, Context = Ctx<()>>,
{
    let generics = whitespaced(generics(expr)).or_not();

    spanned(ident().boxed())
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
        wast::{call::Generics, List},
        CompNode,
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_ident() {
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
    }

    #[test]
    fn test_ident_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
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
                    Span::new(0..2)
                )]
            )
        );
    }

    #[test]
    fn test_call() {
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
    }

    #[test]
    fn test_call_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
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
