use super::super::{ctx::Ctx, error::Expected};
use super::{GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
use crate::node::wast::character::Character;
use chumsky::{
    error::LabelError,
    prelude::*,
    text::{Char, Grapheme, Graphemes},
};

fn escape_sequence<'input, E>() -> impl GraphemeParser<'input, (), E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    any()
        .validate(|grapheme: &Grapheme, extra, emitter| {
            if grapheme
                .to_ascii()
                .filter(|i| b"\\nt".contains(i))
                .is_none()
            {
                let expected = [Expected::CharEscaped];
                let found = Some(grapheme.into());
                emitter.emit(E::Error::expected_found(expected, found, extra.span()));
            }
        })
        .labelled(Expected::CharEscaped)
}

fn content<'input, E>() -> impl GraphemeParser<'input, &'input str, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let empty = move |expected| {
        custom(move |inp| {
            let found = inp.peek_maybe();
            let span = inp.span_since(&inp.cursor());
            let parser = just("'").then(just("'").not()).rewind();
            match found {
                None => Ok(None),

                Some(_) => match inp.parse(parser) {
                    Ok(_) => Ok(found),
                    Err(_) => Err(E::Error::expected_found([expected], found, span)),
                },
            }
        })
        .validate(move |found, extra, emitter| {
            emitter.emit(E::Error::expected_found([expected], found, extra.span()));
        })
    };

    let unescaped = empty(Expected::CharContent)
        .or(none_of("\\").ignored())
        .to_slice();

    let escaped = just("\\")
        .ignored()
        .ignore_then(empty(Expected::CharEscaped).or(escape_sequence()))
        .to_slice();

    choice((unescaped, escaped))
        .map(Graphemes::as_str)
        .labelled(Expected::CharContent)
}

pub fn character<'input, E>() -> impl GraphemeParser<'input, Character<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let quote = just("'");

    group((
        quote.ignore_then(content()),
        quote
            .to(true)
            .labelled(Expected::CharClose)
            .recover_with(via_parser(empty().to(false))),
    ))
    .map(|(repr, close)| Character::new(repr, close))
    .labelled(Expected::Char)
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
    fn test_character() {
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'m'"))
                .into_result(),
            Ok(Character::from_repr("m"))
        );
    }

    #[test]
    fn test_character_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'m"))
                .into_output_errors(),
            (
                Some(Character::new("m", false)),
                vec![Error::new_expected(
                    Expected::CharClose,
                    None,
                    Span::new(2..2)
                )]
            )
        );
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'"))
                .into_output_errors(),
            (
                Some(Character::new("", false)),
                vec![
                    Error::new(smallvec![Expected::CharContent], None, Span::new(1..1)),
                    Error::new(smallvec![Expected::CharClose], None, Span::new(1..1))
                ]
            )
        );
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'\\"))
                .into_output_errors(),
            (
                Some(Character::new("\\", false)),
                vec![
                    Error::new_expected(Expected::CharEscaped, None, Span::new(2..2)),
                    Error::new_expected(Expected::CharClose, None, Span::new(2..2))
                ]
            )
        );
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'mm"))
                .into_output_errors(),
            (
                None,
                vec![
                    Error::new_expected(Expected::CharClose, Some(grapheme("m")), Span::new(2..3)),
                    Error::new_expected(Expected::Eof, Some(grapheme("m")), Span::new(2..3)),
                ]
            )
        );
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new(smallvec![Expected::Char], None, Span::new(0..0))]
            )
        );
    }
}
