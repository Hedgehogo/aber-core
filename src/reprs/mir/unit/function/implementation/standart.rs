use crate::reprs::mir::{unit::Id, State, Value, WithState};

pub(super) fn one_i32<'state>(
    state: &'state mut State,
) -> WithState<'state, Result<Id<Value>, ()>> {
    let value = state.push::<Value>();
    value.unit_mut(state).set(1);
    WithState(state, Ok(value))
}

pub(super) fn same_i32<'state>(
    state: &'state mut State,
    id: Id<Value>,
) -> WithState<'state, Result<Id<Value>, ()>> {
    WithState(state, Ok(id))
}

pub(super) fn add_i32<'state>(
    state: &'state mut State,
    a_id: Id<Value>,
    b_id: Id<Value>,
) -> WithState<'state, Result<Id<Value>, ()>> {
    let inner = |state, id: Id<Value>| {
        id.unit_mut(state)
            .into_inner()
            .map(|inner| inner.ok_or(()))
            .into_result()
    };

    inner(state, a_id)
        .and_then(|WithState(state, a)| {
            inner(state, b_id).map(move |with_state| with_state.map(|b| (a, b)))
        })
        .and_then(|with_state| {
            with_state
                .map(|(a, b)| a.checked_add(b).ok_or(()))
                .into_result()
                .map(|WithState(state, result)| {
                    let value = state.push::<Value>();
                    value.unit_mut(state).set(result);
                    WithState(state, value)
                })
        })
        .into()
}

pub(super) fn println_i32<'state>(
    state: &'state mut State,
    id: Id<Value>,
) -> WithState<'state, Result<Id<Value>, ()>> {
    let value = id.unit_mut(state);
    value.inner().inspect(|value| println!("{}", value));
    value.with_state().map(Ok)
}
