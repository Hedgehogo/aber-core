use super::{CompParser, CompParserExtra};
use crate::node::{
    hir::call::Call,
    span::Span,
    state::{State, UnitRef},
    CompNode, Spanned, Wast,
};
use chumsky::{error::Cheap, prelude::*};

struct CallCtx<C> {
    ctx: C,
    function_id: usize,
    argument_count: usize,
    span: Span,
}

pub fn call<'input, E, P>(fact: P) -> impl CompParser<'input, Spanned<Call<'input>>, E> + Clone
where
    E: CompParserExtra<'input>,
    E::Context: Clone,
    P: CompParser<'input, Spanned<CompNode<'input>>, E> + Clone,
{
    select! {
        Spanned(CompNode::Wast(Wast::Call(call)), span) => Spanned(call, span)
    }
    .try_map_with(|call, extra| {
        let Spanned(call, span) = call;
        let ctx: &E::Context = extra.ctx();
        let ctx: E::Context = ctx.clone();
        let state: &mut State = extra.state();

        if let Some(UnitRef::Function(function)) = state.find(call.ident.0) {
            if let Some(argument_count) = function.argument_count() {
                return Ok(CallCtx {
                    ctx,
                    function_id: function.id(),
                    argument_count,
                    span,
                });
            }
        }

        Err(Cheap::new(span.into()))
    })
    .then_with_ctx(
        map_ctx(|ctx: &CallCtx<E::Context>| ctx.ctx.clone(), fact)
            .repeated()
            .configure(|cfg, ctx: &CallCtx<E::Context>| cfg.exactly(ctx.argument_count))
            .collect(),
    )
    .map(|(ctx, arguments)| Spanned(Call::new(ctx.function_id, arguments), ctx.span))
}

#[cfg(test)]
mod tests {
    use super::super::fact;
    use super::*;
    use crate::node::{
        hir::Hir,
        span::IntoSpanned,
        wast::{self, call::Ident},
    };
    use chumsky::extra::Full;

    pub type Extra<'input> = Full<Cheap, State<'input>, ()>;

    #[test]
    fn test_call() {
        let hir_call = |id, args| CompNode::Hir(Hir::Call(Call::new(id, args)));

        let wast_call = |s| {
            CompNode::Wast(Wast::Call(wast::Call::new(
                Ident::from_repr_unchecked(s).into_spanned(0..0),
                None,
            )))
        };

        let mut state = State::new();
        state.declare(Ident::from_repr_unchecked("two"));
        state.add_argument_count(Ident::from_repr_unchecked("two"), 2);
        state.declare(Ident::from_repr_unchecked("one"));
        state.add_argument_count(Ident::from_repr_unchecked("one"), 1);
        state.declare(Ident::from_repr_unchecked("zero"));
        state.add_argument_count(Ident::from_repr_unchecked("zero"), 0);

        let input = [
            wast_call("two").into_spanned(0..3),
            wast_call("one").into_spanned(4..7),
            wast_call("zero").into_spanned(8..12),
            wast_call("zero").into_spanned(13..17),
        ];

        assert_eq!(
            call(fact::<Extra>())
                .parse_with_state(&input, &mut state)
                .into_result(),
            Ok(Call::new(
                0,
                vec![
                    hir_call(1, vec![hir_call(2, vec![]).into_spanned(8..12)]).into_spanned(4..7),
                    hir_call(2, vec![]).into_spanned(13..17),
                ]
            )
            .into_spanned(0..3))
        );
    }
}
