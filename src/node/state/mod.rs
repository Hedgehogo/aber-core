pub mod unit;
pub mod unit_ref;

use crate::node::wast::call::Ident;
use std::collections::HashMap;
use unit::Unit;
use unit_ref::{FunctionRef, UnitRef};

pub struct State<'input> {
    units: Vec<Unit>,
    idents: HashMap<Ident<'input>, usize>,
}

impl<'input> State<'input> {
    pub fn new() -> Self {
        let units = Vec::new();
        let idents = HashMap::new();
        Self { units, idents }
    }

    fn get_unit<'state>(&'state self, id: usize) -> Option<&'state Unit> {
        self.units.get(id)
    }

    pub fn get<'state>(&'state self, id: usize) -> Option<UnitRef<'state, 'input>> {
        let unit = self.units.get(id)?;

        let unit_ref = match unit {
            Unit::Function(_) => UnitRef::Function(FunctionRef::new(self, id)),
        };

        Some(unit_ref)
    }

    pub fn find<'state>(&'state self, ident: Ident<'input>) -> Option<UnitRef<'state, 'input>> {
        let id = self.idents.get(&ident).copied()?;
        self.get(id)
    }
}
