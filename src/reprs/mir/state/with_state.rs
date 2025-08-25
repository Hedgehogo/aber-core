use super::State;

pub struct WithState<'state, T>(pub &'state mut State, pub T);

impl<'state, T> WithState<'state, T> {
    pub fn map<U, F>(self, f: F) -> WithState<'state, U>
    where
        F: FnOnce(T) -> U,
    {
        let Self(state, inner) = self;
        WithState(state, f(inner))
    }

    pub fn map_with<U, F>(self, f: F) -> WithState<'state, U>
    where
        F: FnOnce(&'state mut State, T) -> (&'state mut State, U),
    {
        let Self(state, inner) = self;
        let (state, inner) = f(state, inner);
        WithState(state, inner)
    }

    pub fn inner(&self) -> &T {
        let Self(_, inner) = self;
        inner
    }
}

impl<'state, T, E> WithState<'state, Result<T, E>> {
    pub fn from_result(result: Result<WithState<'state, T>, WithState<'state, E>>) -> Self {
        match result {
            Ok(WithState(state, ok)) => Self(state, Ok(ok)),
            Err(WithState(state, err)) => Self(state, Err(err)),
        }
    }

    pub fn into_result(self) -> Result<WithState<'state, T>, WithState<'state, E>> {
        let Self(state, inner) = self;
        match inner {
            Ok(ok) => Ok(WithState(state, ok)),
            Err(err) => Err(WithState(state, err)),
        }
    }
}

impl<'state, T, E> From<Result<WithState<'state, T>, WithState<'state, E>>>
    for WithState<'state, Result<T, E>>
{
    fn from(value: Result<WithState<'state, T>, WithState<'state, E>>) -> Self {
        Self::from_result(value)
    }
}

impl<'state, T, E> From<WithState<'state, Result<T, E>>>
    for Result<WithState<'state, T>, WithState<'state, E>>
{
    fn from(value: WithState<'state, Result<T, E>>) -> Self {
        value.into_result()
    }
}
