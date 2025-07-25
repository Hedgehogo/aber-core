pub mod unit;

use super::super::wast::call::Ident;
use super::input::{Nodes, NodesMapper};
use chumsky::{
    input::{self, Cursor, Input},
    inspector::Inspector,
};
use std::collections::hash_map::{Entry, HashMap};
use unit::{function::Function, Unit};

pub use unit::{
    function::{FunctionMut, FunctionRef},
    UnitMut, UnitRef,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
enum Event<'input> {
    Declare(Ident<'input>),
}

pub struct State<'input> {
    units: Vec<Unit>,
    idents: HashMap<Ident<'input>, usize>,
    log: Vec<Event<'input>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checkpoint {
    units_len: usize,
    log_len: usize,
}

impl<'input> State<'input> {
    pub fn new() -> Self {
        let units = Vec::new();
        let idents = HashMap::new();
        let log = Vec::new();
        Self { units, idents, log }
    }

    pub fn save(&self) -> Checkpoint {
        Checkpoint {
            units_len: self.units.len(),
            log_len: self.log.len(),
        }
    }

    pub fn rewind(&mut self, marker: &Checkpoint) {
        let (_, rest) = self.log.split_at(marker.log_len);
        for event in rest.iter().rev() {
            match event {
                Event::Declare(ident) => {
                    self.idents.remove(ident);
                }
            }
        }

        self.units.truncate(marker.units_len);
        self.log.truncate(marker.log_len);
    }

    fn get_unit(&self, id: usize) -> Option<&Unit> {
        self.units.get(id)
    }

    fn get_unit_mut(&mut self, id: usize) -> Option<&mut Unit> {
        self.units.get_mut(id)
    }

    pub fn get<'state>(&'state self, id: usize) -> Option<UnitRef<'state, 'input>> {
        let unit = self.units.get(id)?;

        let unit_ref = match unit {
            Unit::Function(_) => UnitRef::Function(FunctionRef::new(self, id)),
        };

        Some(unit_ref)
    }

    pub fn get_mut<'state>(&'state mut self, id: usize) -> Option<UnitMut<'state, 'input>> {
        let unit = self.units.get_mut(id)?;

        let unit_ref = match unit {
            Unit::Function(_) => UnitMut::Function(FunctionMut::new(self, id)),
        };

        Some(unit_ref)
    }

    pub fn find<'state>(&'state self, ident: Ident<'input>) -> Option<UnitRef<'state, 'input>> {
        let id = self.idents.get(&ident).copied()?;
        self.get(id)
    }

    pub fn find_mut<'state>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Option<UnitMut<'state, 'input>> {
        let id = self.idents.get(&ident).copied()?;
        self.get_mut(id)
    }

    pub fn declare<'state>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Result<FunctionMut<'state, 'input>, UnitMut<'state, 'input>> {
        match self.idents.entry(ident) {
            Entry::Vacant(vacant) => {
                let id = self.units.len();
                let function = Function { arguments: None };

                vacant.insert(id);
                self.log.push(Event::Declare(ident));
                self.units.push(Unit::Function(function));
                Ok(FunctionMut::new(self, id))
            }

            Entry::Occupied(occupied) => {
                let id = *occupied.get();
                #[expect(unreachable_patterns)]
                match self.get_mut(id).unwrap() {
                    UnitMut::Function(function) => Ok(function),
                    unit => Err(unit),
                }
            }
        }
    }
}

impl Default for State<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'input: 'comp, 'comp, F> Inspector<'comp, Nodes<'input, 'comp, F>> for State<'input>
where
    F: NodesMapper<'input, 'comp>,
{
    type Checkpoint = Checkpoint;

    fn on_token(&mut self, _token: &<Nodes<'input, 'comp, F> as Input<'comp>>::Token) {}

    fn on_save<'parse>(
        &self,
        _cursor: &Cursor<'comp, 'parse, Nodes<'input, 'comp, F>>,
    ) -> Self::Checkpoint {
        self.save()
    }

    fn on_rewind<'parse>(
        &mut self,
        marker: &input::Checkpoint<'comp, 'parse, Nodes<'input, 'comp, F>, Checkpoint>,
    ) {
        self.rewind(marker.inspector())
    }
}
