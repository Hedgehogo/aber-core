use super::super::super::{
    span::{IntoSpanned, Spanned},
    CompNode,
};
use super::super::{
    state::{State, WithState},
    unit::{
        function::{implementation::impl_mut::ComptimeImplMut, Function},
        Id,
    },
    Value,
};
use super::Mir;

/// Type that describes the *call* construct from MIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'input> {
    id: Id<Function>,
    result_id: Option<Id<Value>>,
    pub args: Vec<Spanned<CompNode<'input>>>,
}

impl<'input> Call<'input> {
    pub fn new(id: Id<Function>, args: Vec<Spanned<CompNode<'input>>>) -> Self {
        Self {
            id,
            result_id: None,
            args,
        }
    }

    pub(crate) fn id(&self) -> Id<Function> {
        self.id
    }

    pub(crate) fn result_id(&self) -> Option<Id<Value>> {
        self.result_id
    }

    pub(crate) fn set_result_id(&mut self, id: Id<Value>) {
        self.result_id = Some(id);
    }

    pub(crate) fn comptime<'state, 'call>(
        &'call mut self,
        state: &'state mut State<'input>,
    ) -> Result<ComptimeCallMut<'input, 'state, 'call>, &'state mut State<'input>> {
        self.id
            .unit_mut(state)
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
    pub fn into_spanned_mir(self) -> Spanned<Mir<'input>> {
        let Spanned(call, span) = self;
        Mir::Call(call).into_spanned(span)
    }
}

pub struct ComptimeCallMut<'input, 'state, 'call> {
    args: &'call Vec<Spanned<CompNode<'input>>>,
    result_id: &'call mut Option<Id<Value>>,
    implementation: ComptimeImplMut<'input, 'state>,
}

impl<'input, 'state, 'call> ComptimeCallMut<'input, 'state, 'call> {
    pub(crate) fn state(self) -> &'state mut State<'input> {
        self.implementation.state()
    }

    pub(crate) fn execute(self) -> WithState<'input, 'state, Result<Id<Value>, ()>> {
        let ok = self
            .args
            .iter()
            .map(|item| item.inner())
            .all(|node| match node {
                CompNode::Mir(Mir::Call(call)) => call.result_id.is_some(),
                _ => false,
            });

        let arg_ids = if ok {
            self.args.iter().map(|item| {
                match item.inner() {
                    CompNode::Mir(Mir::Call(call)) => call.result_id,
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
