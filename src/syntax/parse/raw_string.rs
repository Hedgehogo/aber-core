use super::super::error::{Error, Expected};
use super::GraphemeParser;
use crate::node::wast::string::String;
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, newline, Graphemes};

pub fn raw_string<'input>() -> impl GraphemeParser<'input, String, Error<'input>> {
    let special = just("\"");

    let start_special = special
        .repeated()
        .at_least(3)
        .count()
        .then_ignore(newline())
        .map_err(|e: Error| e.replace_expected(Expected::RawStringStart));

    let end_special = move |quotes_count| {
        special
            .repeated()
            .at_least(quotes_count)
            .at_most(quotes_count)
            .ignored()
            .map_err(|e: Error| e.replace_expected(Expected::RawStringEnd))
    };

    let line = move |quotes_count| {
        newline()
            .or(end_special(quotes_count))
            .not()
            .then(any())
            .repeated()
            .to_slice()
    };

    let indent = move |quotes| {
        inline_whitespace()
            .to_slice()
            .then_ignore(end_special(quotes))
    };

    custom(move |input| {
        let quotes_count = input.parse(start_special)?;
        let lines_start = input.save();
        let lines_count = input.parse(line(quotes_count).then(newline()).repeated().count())?;
        let indent = input.parse(indent(quotes_count))?;
        let end = input.save();

        if lines_count == 0 {
            return Ok(Default::default());
        }

        let line = just(indent)
            .map_err(|e: Error| e.replace_expected(Expected::RawStringIndent))
            .ignore_then(newline().not().then(any()).repeated().to_slice());

        input.rewind(lines_start);

        let mut result = input
            .parse(line)
            .map(|i: &Graphemes| std::string::String::from(i.as_str()))?;

        for _ in 1..lines_count {
            let (newline, line): (&Graphemes, &Graphemes) =
                input.parse(newline().to_slice().then(line))?;
            result.reserve(newline.as_str().len() + line.as_str().len());
            result.push_str(newline.as_str());
            result.push_str(line.as_str());
        }

        input.rewind(end);

        Ok(String::new(result))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::span::Span;
    use indoc::indoc;

    #[test]
    fn test_raw_string() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        {
            let input = indoc! {r#"
                """
                Hello Aber!
                """"#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = indoc! {r#"
                """
                  Hello Aber!
                  """"#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Ok("Hello Aber!".into())
            );
        }
        {
            let input = indoc! {r#"
                """
                  Hello Aber!
                 """"#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Ok(" Hello Aber!".into())
            );
        }
        {
            let input = indoc! {r#"
                """
                 Hello Aber!
                  """"#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Err(vec![Error::new_expected(
                    Expected::RawStringIndent,
                    Some(grapheme("H")),
                    Span::new(5..6)
                )])
            );
        }
        {
            let input = indoc! {r#"
                """"
                  Hello Aber!
                 """""#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Ok(" Hello Aber!".into())
            );
        }
        {
            let input = indoc! {r#"
                """"
                  Hello Aber!
                 """"#};
            assert_eq!(
                raw_string().parse(Graphemes::new(input)).into_result(),
                Err(vec![Error::new_expected(
                    Expected::RawStringEnd,
                    None,
                    Span::new(23..23)
                )])
            );
        }
    }
}
