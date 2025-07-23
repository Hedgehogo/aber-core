use super::super::super::{
    span::{IntoSpanned, Spanned},
    CompNode,
};
use super::super::state::{
    unit::function::{FunctionMut, FunctionRef},
    State,
};
use super::Hir;

/// Type that describes the *call* construct from MIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    id: usize,
    args: Vec<Spanned<CompNode<'input>>>,
}

impl<'input> Call<'input> {
    pub fn new(id: usize, args: Vec<Spanned<CompNode<'input>>>) -> Self {
        Self { id, args }
    }

    pub fn function<'state>(&self, state: &'state State<'input>) -> FunctionRef<'state, 'input> {
        FunctionRef::new(state, self.id)
    }

    pub fn function_mut<'state>(
        &self,
        state: &'state mut State<'input>,
    ) -> FunctionMut<'state, 'input> {
        FunctionMut::new(state, self.id)
    }
}

impl<'input> Spanned<Call<'input>> {
    pub fn into_spanned_hir(self) -> Spanned<Hir<'input>> {
        let Spanned(call, span) = self;
        Hir::Call(call).into_spanned(span)
    }
}
