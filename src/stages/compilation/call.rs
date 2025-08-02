use super::{CompParser, CompParserExtra};
use crate::reprs::{
    hir::{node::Call, unit::UnitRef, Function, NodesMapper, State},
    CompNode, Spanned, Wast,
};
use chumsky::{error::Cheap, prelude::*};

struct CallCtx<C> {
    ctx: C,
    function_id: usize,
    argument_count: usize,
}

pub fn call<'input, 'comp, E, F, P>(
    fact: P,
) -> impl CompParser<'input, 'comp, Call<'input>, E, F> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp, F>,
    E::Context: Clone,
    F: NodesMapper<'input, 'comp>,
    P: CompParser<'input, 'comp, CompNode<'input>, E, F> + Clone,
{
    select! {
        CompNode::Wast(Wast::Call(call)) => call
    }
    .try_map_with(|call, extra| {
        let ctx: &E::Context = extra.ctx();
        let ctx: E::Context = ctx.clone();
        let state: &mut State = extra.state();

        if let Some(function) = state
            .find(*call.ident.inner())
            .and_then(UnitRef::downcast::<Function>)
        {
            if let Some(argument_count) = function.arg_count() {
                return Ok(CallCtx {
                    ctx,
                    function_id: function.id(),
                    argument_count,
                });
            }
        }

        Err(Cheap::new(extra.span()))
    })
    .then_with_ctx(
        map_ctx(
            |ctx: &CallCtx<E::Context>| ctx.ctx.clone(),
            fact.map_with(|fact, extra| Spanned(fact, extra.span())),
        )
        .repeated()
        .configure(|cfg, ctx: &CallCtx<E::Context>| cfg.exactly(ctx.argument_count))
        .collect(),
    )
    .map(|(ctx, arguments)| Call::new(ctx.function_id, arguments))
}

#[cfg(test)]
mod tests {
    use super::super::fact;
    use super::*;
    use crate::reprs::{
        hir::nodes,
        span::{IntoSpanned, Span},
        wast::call::Ident,
        CompExpr,
    };
    use chumsky::extra::Full;

    pub type Extra<'input> = Full<Cheap<Span>, State<'input>, ()>;

    #[test]
    fn test_call() {
        let ident = |s| Ident::from_repr_unchecked(s);

        let mut state = State::new();
        state
            .declare::<Function>(ident("two"))
            .unwrap()
            .add_arg_count(2);
        state
            .declare::<Function>(ident("one"))
            .unwrap()
            .add_arg_count(1);
        state
            .declare::<Function>(ident("zero"))
            .unwrap()
            .add_arg_count(0);

        let input = [
            ident("two")
                .into_spanned(0..3)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("one")
                .into_spanned(4..7)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("zero")
                .into_spanned(8..12)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("zero")
                .into_spanned(13..17)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
        ]
        .into_spanned(0..17);

        let input = nodes(input.as_ref().map(<[_; 4]>::as_slice));

        assert_eq!(
            call(fact::<Extra, _>())
                .parse_with_state(input, &mut state)
                .into_result(),
            Ok(Call::new(
                0,
                vec![
                    Call::new(
                        1,
                        vec![Call::new(2, vec![])
                            .into_spanned(8..12)
                            .into_spanned_hir()
                            .into_spanned_node()]
                    )
                    .into_spanned(4..12)
                    .into_spanned_hir()
                    .into_spanned_node(),
                    Call::new(2, vec![])
                        .into_spanned(13..17)
                        .into_spanned_hir()
                        .into_spanned_node(),
                ]
            ))
        );
    }
}
