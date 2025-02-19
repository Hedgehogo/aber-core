use super::{
    error::Error,
    or::{or, Or},
};
use crate::node::state::State;

pub type SResult<'input, 'state, I, O, E> = Result<O, (I, &'state mut State<'input>, E)>;

pub trait NextStage<'input, 'state, I, O, E>: Sized
where
    E: Error,
{
    fn next_stage(
        self,
        node: I,
        state: &'state mut State<'input>,
    ) -> SResult<'input, 'state, I, O, E>;

    fn or<T>(self, alt: T) -> Or<'input, 'state, I, O, E, Self, T>
    where
        T: NextStage<'input, 'state, I, O, E>,
    {
        or(self, alt)
    }
}
