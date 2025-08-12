pub mod event;
pub mod with_state;

use super::super::wast::call::Ident;
use super::input::Nodes;
use super::unit::{unit_mut::UnitMut, unit_ref::UnitRef, Unit, UnitConv};
use chumsky::{
    input::{self, Cursor, Input},
    inspector::Inspector,
};
use event::{Event, EventZipped};
use std::collections::hash_map::{Entry, HashMap};

pub use event::UnitEvent;
pub use with_state::WithState;

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

    pub fn standart() -> Self {
        use super::unit::function::{impls, Function, Time};

        let ident = |s| Ident::from_repr_unchecked(s);

        let mut state = Self::new();

        state
            .declare::<Function>(ident("one"))
            .unwrap()
            .add_impl(impls::OneI32.into());
        state
            .declare::<Function>(ident("same"))
            .unwrap()
            .add_impl(impls::SameI32.into());
        state
            .declare::<Function>(ident("add"))
            .unwrap()
            .add_impl(impls::AddI32.into());
        state
            .declare::<Function>(ident("println"))
            .unwrap()
            .add_impl(impls::PrintlnI32.into());

        let mut run = state.declare::<Function>(ident("run")).unwrap();
        run.add_impl(impls::RunI32.into());
        run.specify_time(Time::Runtime);

        state
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

                Event::Push(_) => {}

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

    pub fn get<'state>(&'state self, id: usize) -> Option<UnitRef<'input, 'state, Unit>> {
        (self.units.len() > id).then(|| UnitRef::new(self, id))
    }

    pub fn get_mut<'state>(&'state mut self, id: usize) -> Option<UnitMut<'input, 'state, Unit>> {
        (self.units.len() > id).then(|| UnitMut::new(self, id))
    }

    pub fn find<'state>(
        &'state self,
        ident: Ident<'input>,
    ) -> Option<UnitRef<'input, 'state, Unit>> {
        let id = self.idents.get(&ident).copied()?;
        self.get(id)
    }

    pub fn find_mut<'state>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Option<UnitMut<'input, 'state, Unit>> {
        let id = self.idents.get(&ident).copied()?;
        self.get_mut(id)
    }

    pub fn declare<'state, T: UnitConv + Default>(
        &'state mut self,
        ident: Ident<'input>,
    ) -> Result<UnitMut<'input, 'state, T>, UnitMut<'input, 'state, Unit>> {
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

    pub(super) fn push<'state, T: UnitConv + Default>(
        &'state mut self,
    ) -> UnitMut<'input, 'state, T> {
        let id = self.units.len();
        self.units.push(T::default().into());
        self.log.push(Event::Push(id).into());
        UnitMut::new(self, id)
    }
}

impl Default for State<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'input: 'comp, 'comp> Inspector<'comp, Nodes<'input, 'comp>> for State<'input> {
    type Checkpoint = Checkpoint;

    fn on_token(&mut self, _token: &<Nodes<'input, 'comp> as Input<'comp>>::Token) {}

    fn on_save<'parse>(
        &self,
        _cursor: &Cursor<'comp, 'parse, Nodes<'input, 'comp>>,
    ) -> Self::Checkpoint {
        self.save()
    }

    fn on_rewind<'parse>(
        &mut self,
        marker: &input::Checkpoint<'comp, 'parse, Nodes<'input, 'comp>, Checkpoint>,
    ) {
        self.rewind(marker.inspector())
    }
}
