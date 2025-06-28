use super::super::{ctx::Ctx, error::Expected};
use super::{GraphemeLabelError, GraphemeParser, GraphemeParserExtra};
use crate::node::wast::character::Character;
use chumsky::{
    error::LabelError,
    prelude::*,
    text::{Char, Grapheme, Graphemes},
};

fn escape_sequence<'input, E>() -> impl GraphemeParser<'input, &'input Grapheme, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    any()
        .try_map(|i: &Grapheme, span: SimpleSpan| {
            i.to_ascii()
                .and_then(|i| match i {
                    b'\\' => Some("\\"),
                    b'n' => Some("\n"),
                    b't' => Some("\t"),
                    _ => None,
                })
                .map(|i| Graphemes::new(i).iter().next().unwrap())
                .ok_or_else(|| {
                    E::Error::expected_found([Expected::CharEscaped], Some(i.into()), span)
                })
        })
        .labelled(Expected::CharEscaped)
}

pub fn character<'input, E>() -> impl GraphemeParser<'input, Character<'input>, E> + Copy
where
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    let quote = just("'");
    let unescaped = none_of("\\").labelled(Expected::CharUnescaped);
    let escaped = just("\\")
        .ignore_then(escape_sequence())
        .labelled(Expected::CharEscape);

    let character = escaped.or(unescaped).recover_with(via_parser(
        any()
            .or_not()
            .map(|_| Graphemes::new("\u{FFFD}").iter().next().unwrap()),
    ));

    quote
        .ignore_then(character)
        .then_ignore(
            quote
                .labelled(Expected::CharClose)
                .ignored()
                .recover_with(via_parser(empty())),
        )
        .map(Character::new)
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
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'m'"))
                .into_result(),
            Ok(Character::new(grapheme("m")))
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
                Some(Character::new(grapheme("m"))),
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
                Some(Character::new(grapheme("\u{FFFD}"))),
                vec![
                    Error::new(
                        smallvec![Expected::CharUnescaped, Expected::CharEscape],
                        None,
                        Span::new(1..1)
                    ),
                    Error::new(smallvec![Expected::CharClose], None, Span::new(1..1))
                ]
            )
        );
        assert_eq!(
            character::<Extra>()
                .parse(Graphemes::new("'\\"))
                .into_output_errors(),
            (
                Some(Character::new(grapheme("\u{FFFD}"))),
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
