use super::{CompParser, CompParserExtra};
use crate::reprs::{
    hir::{node::Call, nodes, unit::UnitRef, Function, State, WithState},
    span::IntoSpanned,
    CompNode, Hir, Spanned, Wast,
};
use chumsky::{error::Cheap, prelude::*};

struct CallCtx<C> {
    ctx: C,
    function_id: usize,
    argument_count: usize,
}

fn from_wast<'input, 'comp, E, P>(
    fact: P,
) -> impl CompParser<'input, 'comp, Call<'input>, E> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
    P: CompParser<'input, 'comp, CompNode<'input>, E> + Clone,
{
    select_ref! {
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
            fact.clone()
                .map_with(|fact, extra| Spanned(fact, extra.span())),
        )
        .repeated()
        .configure(|cfg, ctx: &CallCtx<E::Context>| cfg.exactly(ctx.argument_count))
        .collect(),
    )
    .map(|(ctx, arguments)| Call::new(ctx.function_id, arguments))
}

fn from_hir<'input, 'comp, E, P>(fact: P) -> impl CompParser<'input, 'comp, Call<'input>, E> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
    P: CompParser<'input, 'comp, CompNode<'input>, E> + Clone,
{
    fact
        .map_with(|fact, extra| fact.into_spanned(extra.span()))
        .repeated()
        .collect()
        .nested_in(select_ref! {
            CompNode::Hir(Hir::Call(call)) = extra => nodes(call.args.as_slice().into_spanned(extra.span()))
        })
        .rewind()
        .then(select_ref! {
            CompNode::Hir(Hir::Call(call)) => (call.id(), call.result_id())
        }).map(|(args, (id, result_id))| {
            let mut call = Call::new(id, args);
            if let Some(result_id) = result_id {
                call.set_result_id(result_id);
            }
            call
        })
}

pub fn call<'input, 'comp, E, P>(fact: P) -> impl CompParser<'input, 'comp, Call<'input>, E> + Clone
where
    'input: 'comp,
    E: CompParserExtra<'input, 'comp>,
    E::Context: Clone,
    P: CompParser<'input, 'comp, CompNode<'input>, E> + Clone,
{
    from_wast(fact.clone())
        .or(from_hir(fact))
        .try_map_with(|mut call: Call, extra| match call.result_id() {
            Some(_) => Ok(call),

            None => match call.comptime(extra.state()) {
                Ok(comptime) => {
                    let WithState(_, id) = comptime.execute();
                    id.map(|_| call).map_err(|_| {
                        println!("call");
                        Cheap::new(extra.span())
                    })
                }

                Err(_) => Ok(call),
            },
        })
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
    fn test_from_wast() {
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
            from_wast(fact::<Extra>())
                .parse_with_state(input, &mut state)
                .into_result(),
            Ok(Call::new(
                state.find(ident("two")).unwrap().id(),
                vec![
                    Call::new(
                        state.find(ident("one")).unwrap().id(),
                        vec![Call::new(state.find(ident("zero")).unwrap().id(), vec![])
                            .into_spanned(8..12)
                            .into_spanned_hir()
                            .into_spanned_node()]
                    )
                    .into_spanned(4..12)
                    .into_spanned_hir()
                    .into_spanned_node(),
                    Call::new(state.find(ident("zero")).unwrap().id(), vec![])
                        .into_spanned(13..17)
                        .into_spanned_hir()
                        .into_spanned_node(),
                ]
            ))
        );
    }

    #[test]
    fn test_from_hir() {
        let ident = |s| Ident::from_repr_unchecked(s);

        let mut state = State::standart();

        let input = [Call::new(
            state.find(ident("same")).unwrap().id(),
            vec![Call::new(state.find(ident("one")).unwrap().id(), vec![])
                .into_spanned(0..3)
                .into_spanned_hir()
                .into_spanned_node()],
        )
        .into_spanned(4..8)
        .into_spanned_hir()
        .into_spanned_node()]
        .into_spanned(0..8);

        let input = nodes(input.as_ref().map(<[_; 1]>::as_slice));

        assert_eq!(
            from_hir(fact::<Extra>())
                .parse_with_state(input, &mut state)
                .into_result(),
            Ok(Call::new(
                state.find(ident("same")).unwrap().id(),
                vec![Call::new(state.find(ident("one")).unwrap().id(), vec![])
                    .with_result(5)
                    .into_spanned(0..3)
                    .into_spanned_hir()
                    .into_spanned_node()],
            ))
        );
    }

    #[test]
    fn test_call() {
        let ident = |s| Ident::from_repr_unchecked(s);

        let mut state = State::standart();

        let input = [ident("one")
            .into_spanned(0..3)
            .into_spanned_call::<CompExpr>()
            .into_spanned_wast()
            .into_spanned_node()]
        .into_spanned(0..3);

        let input = nodes(input.as_ref().map(<[_; 1]>::as_slice));

        assert_eq!(
            call(fact::<Extra>())
                .parse_with_state(input, &mut state)
                .into_result(),
            Ok(Call::new(state.find(ident("one")).unwrap().id(), vec![]).with_result(5))
        );
    }

    #[test]
    fn test_standart() {
        let ident = |s| Ident::from_repr_unchecked(s);

        let mut state = State::standart();

        let input = [
            ident("add")
                .into_spanned(0..3)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("same")
                .into_spanned(4..8)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("one")
                .into_spanned(9..12)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
            ident("one")
                .into_spanned(13..16)
                .into_spanned_call::<CompExpr>()
                .into_spanned_wast()
                .into_spanned_node(),
        ]
        .into_spanned(0..16);

        let input = nodes(input.as_ref().map(<[_; 4]>::as_slice));

        let result = call(fact::<Extra>())
            .parse_with_state(input, &mut state)
            .into_result()
            .unwrap();

        assert_eq!(result.args.len(), 2);
        assert_eq!(result.result(&state).unwrap().inner(), Some(2));

        let arg1 = result.args[0]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(arg1.span(), (4..12).into());
        assert_eq!(arg1.inner().args.len(), 1);
        assert_eq!(arg1.inner().result(&state).unwrap().inner(), Some(1));

        let arg11 = arg1.inner().args[0]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(arg11.span(), (9..12).into());
        assert_eq!(arg11.inner().args.len(), 0);
        assert_eq!(arg11.inner().result(&state).unwrap().inner(), Some(1));

        let arg2 = result.args[1]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(arg2.span(), (13..16).into());
        assert_eq!(arg2.inner().args.len(), 0);
        assert_eq!(arg2.inner().result(&state).unwrap().inner(), Some(1));
    }
}
