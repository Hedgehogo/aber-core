use super::{
    state::State,
    unit::{Unit, UnitConv, UnitMut, UnitRef},
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub struct Id<T: UnitConv> {
    id: usize,
    phantom: PhantomData<T>,
}

impl<T: UnitConv> Id<T> {
    pub(super) fn new(id: usize) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }

    pub fn unit<'input, 'state>(&self, state: &'state State<'input>) -> UnitRef<'input, 'state, T> {
        UnitRef::new(state, self.id)
    }

    pub fn unit_mut<'input, 'state>(
        &self,
        state: &'state mut State<'input>,
    ) -> UnitMut<'input, 'state, T> {
        UnitMut::new(state, self.id)
    }

    pub fn upcast(self) -> Id<Unit> {
        Id::new(self.id)
    }

    pub(super) fn inner(&self) -> usize {
        self.id
    }
}

impl<T: UnitConv> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl<T: UnitConv> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: UnitConv> Copy for Id<T> {}

impl<T: UnitConv> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: UnitConv> Eq for Id<T> {}

impl<T: UnitConv> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phantom.hash(state);
    }
}
