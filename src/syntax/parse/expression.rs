use super::super::error::Error;
use super::spanned;
use super::{whitespace::whitespace, GraphemeParser};
use crate::node::wast::Wast;
use crate::node::{Expr, Node, Spanned};
use chumsky::prelude::*;

pub fn expression<'input, M>(
    meaningful_unit: M,
    at_least: usize,
) -> impl GraphemeParser<'input, Expr<'input>, Error<'input>> + Clone
where
    M: GraphemeParser<'input, Spanned<Node<'input>>, Error<'input>> + Clone,
{
    meaningful_unit
        .separated_by(whitespace())
        .at_least(at_least)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::Expected;
    use super::super::meaningful_unit::meaningful_unit;
    use crate::node::{
        span::Span,
        wast::{
            character::Character,
            number::{Digits, Number, Radix},
        },
    };
    use smallvec::smallvec;
    use text::Graphemes;

    #[test]
    fn test_expression() {
        let grapheme = |s| Graphemes::new(s).iter().next().unwrap();
        assert_eq!(
            expression(meaningful_unit(), 0)
                .parse(Graphemes::new("\"hello\" //hello\n 'h"))
                .into_output_errors(),
            (
                Some(vec![
                    Wast::String("hello".into()).into_node().into_spanned(0..7),
                    Wast::Character(grapheme("h").into())
                        .into_node()
                        .into_spanned(17..19),
                ]),
                vec![Error::new_expected(
                    Expected::CharSpecial,
                    None,
                    Span::new(19..19)
                )]
            )
        );
    }
}
