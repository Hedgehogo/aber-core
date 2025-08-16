use super::{CompParser, CompParserExtra};
use crate::reprs::{
    hir::{
        node::Call,
        nodes,
        unit::{Id, UnitRef},
        Function, State, WithState,
    },
    span::IntoSpanned,
    CompNode, Hir, Spanned, Wast,
};
use chumsky::{error::Cheap, prelude::*};

struct CallCtx<C> {
    ctx: C,
    function_id: Id<Function>,
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
            .map(|id| id.unit(state))
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
        let id = |state: &State, s| {
            state
                .find(ident(s))
                .unwrap()
                .unit(state)
                .downcast::<Function>()
                .unwrap()
                .id()
        };

        let mut state = State::new();
        state
            .declare::<Function>(ident("two"))
            .unwrap()
            .unit_mut(&mut state)
            .add_arg_count(2);
        state
            .declare::<Function>(ident("one"))
            .unwrap()
            .unit_mut(&mut state)
            .add_arg_count(1);
        state
            .declare::<Function>(ident("zero"))
            .unwrap()
            .unit_mut(&mut state)
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
                id(&state, "two"),
                vec![
                    Call::new(
                        id(&state, "one"),
                        vec![Call::new(id(&state, "zero"), vec![])
                            .into_spanned(8..12)
                            .into_spanned_hir()
                            .into_spanned_node()]
                    )
                    .into_spanned(4..12)
                    .into_spanned_hir()
                    .into_spanned_node(),
                    Call::new(id(&state, "zero"), vec![])
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
        let id = |state: &State, s| {
            state
                .find(ident(s))
                .unwrap()
                .unit(state)
                .downcast::<Function>()
                .unwrap()
                .id()
        };

        let mut state = State::standart();

        let input = [Call::new(
            id(&state, "same"),
            vec![Call::new(id(&state, "one"), vec![])
                .into_spanned(5..8)
                .into_spanned_hir()
                .into_spanned_node()],
        )
        .into_spanned(5..8)
        .into_spanned_hir()
        .into_spanned_node()]
        .into_spanned(0..8);

        let input = nodes(input.as_ref().map(<[_; 1]>::as_slice));

        let result = from_hir(fact::<Extra>())
            .parse_with_state(input, &mut state)
            .into_result()
            .unwrap();

        assert_eq!(result.args.len(), 1);
        assert_eq!(result.result_id(), None);

        let Spanned(arg1, span) = result.args[0]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(span, (5..8).into());
        assert_eq!(arg1.args.len(), 0);
        assert_eq!(arg1.result_id().unwrap().unit(&state).inner(), Some(1));
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

        let result = call(fact::<Extra>())
            .parse_with_state(input, &mut state)
            .into_result()
            .unwrap();

        assert_eq!(result.args.len(), 0);
        assert_eq!(result.result_id().unwrap().unit(&state).inner(), Some(1));
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
        assert_eq!(result.result_id().unwrap().unit(&state).inner(), Some(2));

        let Spanned(arg1, span) = result.args[0]
            .as_ref()
            .map(|node: &CompNode<'_>| node.hir().unwrap().call().unwrap());
        assert_eq!(span, (4..12).into());
        assert_eq!(arg1.args.len(), 1);
        assert_eq!(arg1.result_id().unwrap().unit(&state).inner(), Some(1));

        let Spanned(arg11, span) = arg1.args[0]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(span, (9..12).into());
        assert_eq!(arg11.args.len(), 0);
        assert_eq!(arg11.result_id().unwrap().unit(&state).inner(), Some(1));

        let Spanned(arg2, span) = result.args[1]
            .as_ref()
            .map(|node| node.hir().unwrap().call().unwrap());
        assert_eq!(span, (13..16).into());
        assert_eq!(arg2.args.len(), 0);
        assert_eq!(arg2.result_id().unwrap().unit(&state).inner(), Some(1));
    }
}
