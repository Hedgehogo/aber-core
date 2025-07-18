use super::super::state::{State, unit_ref::{UnitRef, FunctionRef}};
use super::super::super::{CompNode, Spanned};

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
        let unit_ref = state.get(self.id).expect("Unit must exist");
        
        #[expect(unreachable_patterns)]
        match unit_ref {
            UnitRef::Function(i) => i,
            _ => panic!("Unit was supposed to be a function"),
        }
    }
}
