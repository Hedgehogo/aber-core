use super::State;

pub struct WithState<'input, 'state, T>(pub &'state mut State<'input>, pub T);

impl<'input, 'state, T> WithState<'input, 'state, T> {
    pub fn map<U, F>(self, f: F) -> WithState<'input, 'state, U>
    where
        F: FnOnce(T) -> U,
    {
        let Self(state, inner) = self;
        WithState(state, f(inner))
    }

    pub fn map_with<U, F>(self, f: F) -> WithState<'input, 'state, U>
    where
        F: FnOnce(&'state mut State<'input>, T) -> (&'state mut State<'input>, U),
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

impl<'input, 'state, T, E> WithState<'input, 'state, Result<T, E>> {
    pub fn from_result(
        result: Result<WithState<'input, 'state, T>, WithState<'input, 'state, E>>,
    ) -> Self {
        match result {
            Ok(WithState(state, ok)) => Self(state, Ok(ok)),
            Err(WithState(state, err)) => Self(state, Err(err)),
        }
    }

    pub fn into_result(self) -> Result<WithState<'input, 'state, T>, WithState<'input, 'state, E>> {
        let Self(state, inner) = self;
        match inner {
            Ok(ok) => Ok(WithState(state, ok)),
            Err(err) => Err(WithState(state, err)),
        }
    }
}

impl<'input, 'state, T, E> From<Result<WithState<'input, 'state, T>, WithState<'input, 'state, E>>>
    for WithState<'input, 'state, Result<T, E>>
{
    fn from(value: Result<WithState<'input, 'state, T>, WithState<'input, 'state, E>>) -> Self {
        Self::from_result(value)
    }
}

impl<'input, 'state, T, E> From<WithState<'input, 'state, Result<T, E>>>
    for Result<WithState<'input, 'state, T>, WithState<'input, 'state, E>>
{
    fn from(value: WithState<'input, 'state, Result<T, E>>) -> Self {
        value.into_result()
    }
}
