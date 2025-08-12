use crate::reprs::hir::unit::UnitRef;

use super::super::super::{
    span::{IntoSpanned, Spanned},
    CompNode,
};
use super::super::{
    state::{State, WithState},
    unit::function::{implementation::impl_mut::ComptimeImplMut, FunctionMut, FunctionRef},
    Value,
};
use super::Hir;

/// Type that describes the *call* construct from MIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    id: usize,
    result_id: Option<usize>,
    pub args: Vec<Spanned<CompNode<'input>>>,
}

impl<'input> Call<'input> {
    pub fn new(id: usize, args: Vec<Spanned<CompNode<'input>>>) -> Self {
        Self {
            id,
            result_id: None,
            args,
        }
    }

    pub fn result<'state>(
        &self,
        state: &'state State<'input>,
    ) -> Option<UnitRef<'input, 'state, Value>> {
        self.result_id
            .map(|id| state.get(id).unwrap())
            .map(|unit| unit.downcast::<Value>().unwrap())
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn result_id(&self) -> Option<usize> {
        self.result_id
    }

    pub(crate) fn set_result_id(&mut self, id: usize) {
        self.result_id = Some(id);
    }

    #[cfg(test)]
    pub(crate) fn with_result(mut self, id: usize) -> Self {
        self.result_id = Some(id);
        self
    }

    #[expect(dead_code)]
    pub(crate) fn function<'state>(
        &self,
        state: &'state State<'input>,
    ) -> FunctionRef<'input, 'state> {
        FunctionRef::new(state, self.id)
    }

    pub(crate) fn function_mut<'state>(
        &self,
        state: &'state mut State<'input>,
    ) -> FunctionMut<'input, 'state> {
        FunctionMut::new(state, self.id)
    }

    pub(crate) fn comptime<'state, 'call>(
        &'call mut self,
        state: &'state mut State<'input>,
    ) -> Result<ComptimeCallMut<'input, 'state, 'call>, &'state mut State<'input>> {
        self.function_mut(state)
            .implementation()
            .map_err(|err| err.state())
            .and_then(|implementation| implementation.comptime().map_err(|err| err.state()))
            .map(|implementation| ComptimeCallMut {
                args: &self.args,
                result_id: &mut self.result_id,
                implementation,
            })
    }
}

impl<'input> Spanned<Call<'input>> {
    pub fn into_spanned_hir(self) -> Spanned<Hir<'input>> {
        let Spanned(call, span) = self;
        Hir::Call(call).into_spanned(span)
    }
}

pub struct ComptimeCallMut<'input, 'state, 'call> {
    args: &'call Vec<Spanned<CompNode<'input>>>,
    result_id: &'call mut Option<usize>,
    implementation: ComptimeImplMut<'input, 'state>,
}

impl<'input, 'state, 'call> ComptimeCallMut<'input, 'state, 'call> {
    pub(crate) fn state(self) -> &'state mut State<'input> {
        self.implementation.state()
    }

    pub(crate) fn execute(self) -> WithState<'input, 'state, Result<usize, ()>> {
        let ok = self
            .args
            .iter()
            .map(|item| item.inner())
            .all(|node| match node {
                CompNode::Hir(Hir::Call(call)) => call.result_id.is_some(),
                _ => false,
            });

        let arg_ids = if ok {
            self.args.iter().map(|item| {
                match item.inner() {
                    CompNode::Hir(Hir::Call(call)) => call.result_id,
                    _ => None,
                }
                .unwrap()
            })
        } else {
            return WithState(self.state(), Err(()));
        };

        self.implementation
            .execute(arg_ids)
            .into_result()
            .inspect(|id| *self.result_id = Some(*id.inner()))
            .into()
    }
}
