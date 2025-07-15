pub mod unit;
pub mod unit_ref;

use super::super::{wast::call::Ident, CompNode, Spanned};
use chumsky::{
    input::{self, Cursor, Input},
    inspector::Inspector,
};
use std::collections::hash_map::{Entry, HashMap};
use unit::{Function, Unit};
pub use unit_ref::{FunctionRef, UnitRef};

pub type Nodes<'input> = &'input [Spanned<CompNode<'input>>];

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

    pub fn declare(&mut self, ident: Ident<'input>) {
        if let Entry::Vacant(vacant) = self.idents.entry(ident) {
            let id = self.units.len();
            let function = Function { arguments: None };

            vacant.insert(id);
            self.log.push(Event::Declare(ident));
            self.units.push(Unit::Function(function));
        }
    }

    pub fn add_argument_count(&mut self, ident: Ident<'input>, argument_count: usize) {
        match self.idents.entry(ident) {
            Entry::Occupied(occupied) => {
                let unit = &mut self.units[*occupied.get()];
                match unit {
                    Unit::Function(function) => function.arguments = Some(argument_count),
                }
            }

            Entry::Vacant(_) => todo!(),
        }
    }
}

impl Default for State<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'input> Inspector<'input, Nodes<'input>> for State<'input> {
    type Checkpoint = Checkpoint;

    fn on_token(&mut self, _token: &<Nodes<'input> as Input<'input>>::Token) {}

    fn on_save<'parse>(&self, _cursor: &Cursor<'input, 'parse, Nodes<'input>>) -> Self::Checkpoint {
        self.save()
    }

    fn on_rewind<'parse>(
        &mut self,
        marker: &input::Checkpoint<'input, 'parse, Nodes<'input>, Checkpoint>,
    ) {
        self.rewind(marker.inspector())
    }
}
