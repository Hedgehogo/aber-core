use super::super::error::{Error, Expected};
use super::GraphemeParser;
use crate::node::wast::character::Character;
use chumsky::prelude::*;
use chumsky::text::unicode::Grapheme;
use text::{Char, Graphemes};

fn escape_sequence<'input>() -> impl GraphemeParser<'input, &'input Grapheme, Error<'input>> + Copy
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
                .ok_or_else(|| Error::new_expected(Expected::CharEscaped, Some(i), span.into()))
        })
        .map_err(|e: Error| e.replace_expected(Expected::CharEscaped))
}

pub fn character<'input>() -> impl GraphemeParser<'input, Character<'input>, Error<'input>> + Copy {
    let quote = just("'").map_err(|e: Error| e.replace_expected(Expected::CharSpecial));
    let escape = just("\\").map_err(|e: Error| e.replace_expected(Expected::CharEscape));
    let escaped = escape.ignore_then(escape_sequence());
    let unescaped = none_of("\\").map_err(|e: Error| e.replace_expected(Expected::CharUnescaped));

    let character = escaped.or(unescaped).recover_with(via_parser(
        any()
            .or_not()
            .map(|_| Graphemes::new("\u{FFFD}").iter().next().unwrap()),
    ));

    quote
        .ignore_then(character)
        .then_ignore(quote.ignored().recover_with(via_parser(empty())))
        .map(Character::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::Span;
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_character() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            character().parse(Graphemes::new("'m'")).into_result(),
            Ok(Character::new(grapheme("m")))
        );
        assert_eq!(
            character().parse(Graphemes::new("'m")).into_output_errors(),
            (
                Some(Character::new(grapheme("m"))),
                vec![Error::new_expected(
                    Expected::CharSpecial,
                    None,
                    Span::new(2..2)
                )]
            )
        );
        assert_eq!(
            character().parse(Graphemes::new("'")).into_output_errors(),
            (
                Some(Character::new(grapheme("\u{FFFD}"))),
                vec![
                    Error::new(
                        smallvec![Expected::CharUnescaped, Expected::CharEscape],
                        None,
                        Span::new(1..1)
                    ),
                    Error::new(smallvec![Expected::CharSpecial], None, Span::new(1..1))
                ]
            )
        );
        assert_eq!(
            character()
                .parse(Graphemes::new("'\\"))
                .into_output_errors(),
            (
                Some(Character::new(grapheme("\u{FFFD}"))),
                vec![
                    Error::new_expected(Expected::CharEscaped, None, Span::new(2..2)),
                    Error::new_expected(Expected::CharSpecial, None, Span::new(2..2))
                ]
            )
        );
        assert_eq!(
            character()
                .parse(Graphemes::new("'mm"))
                .into_output_errors(),
            (
                None,
                vec![
                    Error::new_expected(Expected::CharSpecial, None, Span::new(2..2)),
                    Error::new_expected(Expected::Eof, Some(grapheme("m")), Span::new(2..3)),
                ]
            )
        );
        assert_eq!(
            character().parse(Graphemes::new("")).into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::CharSpecial],
                    None,
                    Span::new(0..0)
                )]
            )
        );
    }
}
