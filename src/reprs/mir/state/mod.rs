pub mod event;
pub mod with_state;

use super::super::{hir::Ident, wast::call::Ident as WastIdent};
use super::{
    input::Nodes,
    unit::{Id, Unit, UnitConv, UnitMut},
};
use chumsky::{
    input::{self, Cursor, Input},
    inspector::Inspector,
};
use event::{Event, EventZipped};
use std::collections::hash_map::{Entry, HashMap};
use string_interner::DefaultStringInterner;

pub use event::UnitEvent;
pub use with_state::WithState;

pub struct State {
    units: Vec<Unit>,
    interner: DefaultStringInterner,
    names: HashMap<Ident, usize>,
    log: Vec<EventZipped>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checkpoint {
    units_len: usize,
    log_len: usize,
}

impl State {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            units: Default::default(),
            interner: Default::default(),
            names: Default::default(),
            log: Default::default(),
        }
    }

    pub fn standart() -> Self {
        use super::unit::function::{impls, Function, Time};

        let mut interner = DefaultStringInterner::new();
        let mut ident = |s| Ident::from_repr_unchecked(&mut interner, s);

        let mut state = Self::new();

        state
            .declare::<Function>(ident("one"))
            .unwrap()
            .unit_mut(&mut state)
            .add_impl(impls::OneI32.into());
        state
            .declare::<Function>(ident("same"))
            .unwrap()
            .unit_mut(&mut state)
            .add_impl(impls::SameI32.into());
        state
            .declare::<Function>(ident("add"))
            .unwrap()
            .unit_mut(&mut state)
            .add_impl(impls::AddI32.into());
        state
            .declare::<Function>(ident("println"))
            .unwrap()
            .unit_mut(&mut state)
            .add_impl(impls::PrintlnI32.into());

        let mut run = state
            .declare::<Function>(ident("run"))
            .unwrap()
            .unit_mut(&mut state);
        run.add_impl(impls::RunI32.into());
        run.specify_time(Time::Runtime);

        state.interner = interner;

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
                    self.names.remove(&ident);
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

    pub fn find(&self, ident: Ident) -> Option<Id<Unit>> {
        self.names.get(&ident).copied().map(Id::new)
    }

    pub fn declare<T: UnitConv + Default>(&mut self, ident: Ident) -> Option<Id<T>> {
        match self.names.entry(ident) {
            Entry::Vacant(vacant) => {
                let id = self.units.len();
                vacant.insert(id);
                self.log.push(EventZipped::Declare(ident));
                self.units.push(T::default().into());
                Some(Id::new(id))
            }

            Entry::Occupied(occupied) => {
                let id = *occupied.get();
                match T::from_unit_mut(&mut self.units[id]) {
                    Ok(_) => Some(Id::new(id)),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn push<T: UnitConv + Default>(&mut self) -> Id<T> {
        let id = self.units.len();
        self.units.push(T::default().into());
        self.log.push(Event::Push(id).into());
        Id::new(id)
    }

    pub fn add_ident<'input>(&mut self, ident: WastIdent<'input>) -> Ident {
        Ident::from_repr_unchecked(&mut self.interner, ident.as_str())
    }

    pub(super) fn get_unit(&self, id: usize) -> Option<&Unit> {
        self.units.get(id)
    }

    pub(super) fn get_unit_mut(&mut self, id: usize) -> Option<&mut Unit> {
        self.units.get_mut(id)
    }

    pub(super) fn log(&mut self, id: Id<Unit>, event: UnitEvent) {
        self.log.push(Event::Unit(id.inner(), event).into());
    }
}

impl<'comp> Inspector<'comp, Nodes<'comp>> for State {
    type Checkpoint = Checkpoint;

    fn on_token(&mut self, _token: &<Nodes<'comp> as Input<'comp>>::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'comp, 'parse, Nodes<'comp>>) -> Self::Checkpoint {
        self.save()
    }

    fn on_rewind<'parse>(
        &mut self,
        marker: &input::Checkpoint<'comp, 'parse, Nodes<'comp>, Checkpoint>,
    ) {
        self.rewind(marker.inspector())
    }
}
