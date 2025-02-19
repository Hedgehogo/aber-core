use super::{Error, NextStage, SResult};
use crate::node::state::State;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default)]
pub struct Or<'input, 'state, I, O, E, F, A>
where
    F: NextStage<'input, 'state, I, O, E>,
    A: NextStage<'input, 'state, I, O, E>,
    E: Error,
{
    first: F,
    alternative: A,
    phantom: PhantomData<(&'state mut State<'input>, I, O, E)>,
}

impl<'input, 'state, E, I, O, F, A> NextStage<'input, 'state, I, O, E>
    for Or<'input, 'state, I, O, E, F, A>
where
    F: NextStage<'input, 'state, I, O, E>,
    A: NextStage<'input, 'state, I, O, E>,
    E: Error,
{
    fn next_stage(
        self,
        node: I,
        state: &'state mut State<'input>,
    ) -> SResult<'input, 'state, I, O, E> {
        match self.first.next_stage(node, state) {
            Ok(i) => Ok(i),
            Err((i, state, err)) => self
                .alternative
                .next_stage(i, state)
                .map_err(move |(i, state, e)| (i, state, err.merge(e))),
        }
    }
}

pub fn or<'input, 'state, I, O, E, F, A>(first: F, alternative: A) -> Or<'input, 'state, I, O, E, F, A>
where
    F: NextStage<'input, 'state, I, O, E>,
    A: NextStage<'input, 'state, I, O, E>,
    E: Error,
{
    Or {
        first,
        alternative,
        phantom: PhantomData,
    }
}
