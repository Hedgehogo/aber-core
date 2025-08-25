use super::super::State;
use super::{Function, Id, Unit, UnitConv, Value};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Copy)]
struct UnitRefInner<'state> {
    state: &'state State,
    id: usize,
}

#[repr(transparent)]
pub struct UnitRef<'state, T: UnitConv> {
    inner: UnitRefInner<'state>,
    phantom: PhantomData<T>,
}

impl<'state, T: UnitConv> UnitRef<'state, T> {
    pub(in super::super) fn new(state: &'state State, id: usize) -> Self {
        Self {
            inner: UnitRefInner { state, id },
            phantom: PhantomData,
        }
    }

    pub fn state(&self) -> &'state State {
        self.inner.state
    }

    pub fn id(&self) -> Id<T> {
        Id::new(self.inner.id)
    }

    pub fn upcast(self) -> UnitRef<'state, Unit> {
        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner: UnitRefInner = std::mem::transmute(self);

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitRefInner, UnitRef<Unit>>(inner)
        }
    }

    pub(super) fn unit(&self) -> &'state T {
        let unit = self
            .inner
            .state
            .get_unit(self.inner.id)
            .expect("Unit must exist");

        T::from_unit_ref(unit).expect("Different kind of unit was expected")
    }
}

impl<'state, T: UnitConv> Clone for UnitRef<'state, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'state, T: UnitConv> Copy for UnitRef<'state, T> {}

impl<'state, T: UnitConv> PartialEq for UnitRef<'state, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<'state, T: UnitConv> Eq for UnitRef<'state, T> {}

impl<'state, T> AsRef<UnitRef<'state, Unit>> for UnitRef<'state, T>
where
    T: UnitConv,
{
    fn as_ref(&self) -> &UnitRef<'state, Unit> {
        let ptr = self as *const UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *const UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &*(inner as *const UnitRef<Unit>)
        }
    }
}

impl<'state, T> AsMut<UnitRef<'state, Unit>> for UnitRef<'state, T>
where
    T: UnitConv,
{
    fn as_mut(&mut self) -> &mut UnitRef<'state, Unit> {
        let ptr = self as *mut UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *mut UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &mut *(inner as *mut UnitRef<Unit>)
        }
    }
}

impl<'state> UnitRef<'state, Unit> {
    pub fn downcast<T: UnitConv>(self) -> Option<UnitRef<'state, T>> {
        T::from_unit_ref(self.unit()).map(|_| unsafe {
            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            let inner: UnitRefInner = std::mem::transmute(self);

            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitRefInner, UnitRef<T>>(inner)
        })
    }
}

impl<'state> fmt::Debug for UnitRef<'state, Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit() {
            Unit::Value(_) => f
                .debug_tuple("Value")
                .field(&self.downcast::<Value>().unwrap())
                .finish(),

            Unit::Function(_) => f
                .debug_tuple("Function")
                .field(&self.downcast::<Function>().unwrap())
                .finish(),
        }
    }
}
