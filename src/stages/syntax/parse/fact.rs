use super::super::{
    ctx::Ctx, error::Expected, Character, Digits, EscapedString, Expr, Ident, Node, RawString,
    Whitespace,
};
use super::{
    block::block, call::call, character::character, escaped_string::escaped_string, expr::expr,
    list::tuple, number::number, raw_string::raw_string, spanned, whitespace::whitespace,
    GraphemeLabelError, GraphemeParser, GraphemeParserExtra,
};
use crate::reprs::{
    wast::{Pair, Wast},
    Spanned,
};
use chumsky::prelude::*;

pub fn fact<'input, N, E>() -> impl GraphemeParser<'input, Spanned<N>, E> + Clone
where
    N: Node + 'input,
    N::Ident: Ident<'input, E::State>,
    N::Digits: Digits<'input>,
    N::Character: Character<'input>,
    N::String: EscapedString<'input> + RawString<'input>,
    <N::Expr as Expr>::Whitespace: Whitespace<'input>,
    E: GraphemeParserExtra<'input, Context = Ctx<()>>,
    E::Error: GraphemeLabelError<'input, Expected>,
{
    recursive(|fact| {
        let choice = choice((
            number().map(Wast::Number).map(N::from_wast),
            character().map(Wast::Character).map(N::from_wast),
            escaped_string().map(Wast::String).map(N::from_wast),
            raw_string().map(Wast::String).map(N::from_wast),
            call(expr(fact.clone())).map(Wast::Call).map(N::from_wast),
            tuple(expr(fact.clone())).map(Wast::Tuple).map(N::from_wast),
            block(expr(fact)).map(Wast::Block).map(N::from_wast),
        ))
        .boxed();

        let pair_special = just(":")
            .then(just(":").not())
            .labelled(Expected::PairSpecial);

        spanned(choice)
            .map(Spanned::from)
            .then(whitespace().then_ignore(pair_special).or_not())
            .map_with(|(node, pair), extra| match pair {
                Some(whitespace) => Wast::Pair(Pair::new(Box::new(node), whitespace))
                    .into_spanned_node(extra.span()),

                None => node,
            })
            .labelled(Expected::Fact)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::{
        error::Error,
        string::{self, StringData},
    };
    use super::super::tests::Extra;
    use crate::reprs::{
        span::Span,
        wast::{
            self, escaped_string,
            number::{Digits, Number, Radix},
            wast_node::WastNode,
            Character, Whitespace,
        },
    };
    use smallvec::smallvec;
    use text::Graphemes;

    fn new_escaped<'input>(
        sections: Vec<&'input str>,
        inner_repr: &'input str,
    ) -> wast::String<'input> {
        wast::String::Escaped(string::EscapedStringSealed::from_data_unchecked(
            {
                let mut data: escaped_string::EscapedStringData = StringData::with_capacity(0);
                for section in sections {
                    data = data.with_next_section(section);
                }
                data
            },
            inner_repr,
            &Ctx::default(),
        ))
    }

    fn new_raw<'input>(
        inner_repr: &'input str,
        quotes_count: usize,
        capacity: usize,
        line_break_count: usize,
        indent: &'input str,
    ) -> wast::String<'input> {
        wast::String::Raw(string::RawStringSealed::from_data_unchecked(
            (),
            inner_repr,
            &Ctx::new_raw(
                Default::default(),
                quotes_count,
                capacity,
                line_break_count,
                indent,
            ),
        ))
    }

    #[test]
    fn test_fact() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        let digits = |s| Digits::from_repr_unchecked(s);
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new("10"))
                .into_result(),
            Ok(
                Wast::Number(Number::new(true, Radix::DECIMAL, digits("10"), None))
                    .into_spanned_node(0..2)
            )
        );
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new("'m'"))
                .into_result(),
            Ok(Wast::Character(grapheme("m").into()).into_spanned_node(0..3))
        );
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new("\"Hello\""))
                .into_result(),
            Ok(Wast::String(new_escaped(vec!["Hello"], "Hello")).into_spanned_node(0..7))
        );
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new("\"\"\"\nHello\n\"\"\""))
                .into_result(),
            Ok(Wast::String(new_raw("Hello", 3, 5, 0, "")).into_spanned_node(0..13))
        );
    }

    #[test]
    fn test_fact_erroneous() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new(""))
                .into_output_errors(),
            (
                None,
                vec![Error::new_expected(Expected::Fact, None, Span::new(0..0))]
            )
        );
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new("'g:"))
                .into_output_errors(),
            (
                Some(
                    Wast::Pair(Pair::new(
                        Box::new(
                            Wast::Character(Character::new("g", false)).into_spanned_node(0..2)
                        ),
                        Whitespace::from_repr_unchecked("")
                    ))
                    .into_spanned_node(0..3)
                ),
                vec![Error::new_expected(
                    Expected::CharClose,
                    Some(grapheme(":")),
                    Span::new(2..3)
                )]
            )
        );
        assert_eq!(
            fact::<WastNode, Extra>()
                .parse(Graphemes::new(":"))
                .into_output_errors(),
            (
                None,
                vec![Error::new(
                    smallvec![Expected::Fact],
                    Some(grapheme(":")),
                    Span::new(0..1)
                )]
            )
        );
    }
}
