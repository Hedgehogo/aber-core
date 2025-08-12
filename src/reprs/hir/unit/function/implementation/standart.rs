use crate::reprs::hir::{State, Value, WithState};

pub(super) fn one_i32<'input: 'state, 'state>(
    state: &'state mut State<'input>,
) -> WithState<'input, 'state, Result<usize, ()>> {
    let mut value = state.push::<Value>();
    value.set(1);
    value.with_state().map(Ok)
}

pub(super) fn same_i32<'input: 'state, 'state>(
    state: &'state mut State<'input>,
    id: usize,
) -> WithState<'input, 'state, Result<usize, ()>> {
    let value = state.get_mut(id).unwrap().downcast::<Value>().unwrap();
    value.with_state().map(Ok)
}

pub(super) fn add_i32<'input: 'state, 'state>(
    state: &'state mut State<'input>,
    a_id: usize,
    b_id: usize,
) -> WithState<'input, 'state, Result<usize, ()>> {
    let inner = |state: &'state mut State<'input>, id| {
        let value = state.get_mut(id).unwrap().downcast::<Value>().unwrap();
        value
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
                    let mut value = state.push::<Value>();
                    value.set(result);
                    value.with_state()
                })
        })
        .into()
}

pub(super) fn println_i32<'input: 'state, 'state>(
    state: &'state mut State<'input>,
    id: usize,
) -> WithState<'input, 'state, Result<usize, ()>> {
    let value = state.get_mut(id).unwrap().downcast::<Value>().unwrap();
    value.inner().inspect(|value| println!("{}", value));
    value.with_state().map(Ok)
}
