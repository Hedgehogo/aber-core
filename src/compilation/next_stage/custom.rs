use super::{Error, NextStage, SResult};
use crate::node::state::State;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Custom<'input, 'state, I, O, E, F>
where
    F: FnOnce(I, &'state mut State<'input>) -> SResult<'input, 'state, I, O, E>,
    E: Error,
{
    f: F,
    phantom: PhantomData<(&'state mut State<'input>, I, O, E)>,
}

impl<'input, 'state, E, I, O, F> NextStage<'input, 'state, I, O, E>
    for Custom<'input, 'state, I, O, E, F>
where
    F: FnOnce(I, &'state mut State<'input>) -> SResult<'input, 'state, I, O, E>,
    E: Error,
{
    fn next_stage(
        self,
        node: I,
        state: &'state mut State<'input>,
    ) -> SResult<'input, 'state, I, O, E> {
        (self.f)(node, state)
    }
}

pub fn custom<'input, 'state, I, O, E, F>(f: F) -> Custom<'input, 'state, I, O, E, F>
where
    F: FnOnce(I, &'state mut State<'input>) -> SResult<'input, 'state, I, O, E>,
    E: Error,
{
    Custom {
        f,
        phantom: PhantomData,
    }
}
