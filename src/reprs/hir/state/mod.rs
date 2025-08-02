pub mod event;

use super::super::wast::call::Ident;
use super::input::{Nodes, NodesMapper};
use super::unit::{unit_mut::UnitMut, unit_ref::UnitRef, Unit, UnitConv};
use chumsky::{
    input::{self, Cursor, Input},
    inspector::Inspector,
};
use event::{Event, EventZipped};
use std::collections::hash_map::{Entry, HashMap};

pub use event::UnitEvent;

pub struct State<'input> {
    units: Vec<Unit>,
    idents: HashMap<Ident<'input>, usize>,
    log: Vec<EventZipped<'input>>,
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
        let log = std::mem::take(&mut self.log);
        let (_, rest) = log.split_at(marker.log_len);
        for event in rest.iter().rev() {
            match event.unzip() {
                Event::Declare(ident) => {
                    self.idents.remove(&ident);
                }

                Event::Unit(id, event) => {
                    if id <= marker.units_len {
                        UnitMut::<Unit>::new(self, id).rewind(event);
                    }
                }
            }
        }

        self.log = log;

        self.units.truncate(marker.units_len);
        self.log.truncate(marker.log_len);
    }

    pub fn get<'state>(&'state self, id: usize) -> Option<UnitRef<'state, 'input, Unit>> {
        Some(UnitRef::new(self, id))
    }

    pub fn get_mut<'state>(&'state mut self, id: usize) -> Option<UnitMut<'state, 'input, Unit>> {
        Some(UnitMut::new(self, id))
    }

    pub fn find<'state>(
        &'state self,
        ident: Ident<'input>,
    ) -> Option<UnitRef<'state, 'input, Unit>> {
        let id = self.idents.get(&ident).copied()?;
        self.get(id)
    }

    pub fn find_mut<'state>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Option<UnitMut<'state, 'input, Unit>> {
        let id = self.idents.get(&ident).copied()?;
        self.get_mut(id)
    }

    pub fn declare<'state, T: UnitConv + Default>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Result<UnitMut<'state, 'input, T>, UnitMut<'state, 'input, Unit>> {
        match self.idents.entry(ident) {
            Entry::Vacant(vacant) => {
                let id = self.units.len();
                vacant.insert(id);
                self.log.push(EventZipped::Declare(ident));
                self.units.push(T::default().into());
                Ok(UnitMut::new(self, id))
            }

            Entry::Occupied(occupied) => {
                let id = *occupied.get();
                match T::from_unit_mut(&mut self.units[id]) {
                    Ok(_) => Ok(UnitMut::new(self, id)),
                    Err(_) => Err(UnitMut::new(self, id)),
                }
            }
        }
    }

    pub(super) fn get_unit(&self, id: usize) -> Option<&Unit> {
        self.units.get(id)
    }

    pub(super) fn get_unit_mut(&mut self, id: usize) -> Option<&mut Unit> {
        self.units.get_mut(id)
    }

    pub(super) fn log(&mut self, id: usize, event: UnitEvent) {
        self.log.push(Event::Unit(id, event).into());
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
