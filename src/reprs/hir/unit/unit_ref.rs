use super::super::{Id, State};
use super::{Function, Unit, UnitConv, Value};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Copy)]
struct UnitRefInner<'input, 'state> {
    state: &'state State<'input>,
    id: usize,
}

#[repr(transparent)]
pub struct UnitRef<'input, 'state, T: UnitConv> {
    inner: UnitRefInner<'input, 'state>,
    phantom: PhantomData<T>,
}

impl<'input, 'state, T: UnitConv> UnitRef<'input, 'state, T> {
    pub(in super::super) fn new(state: &'state State<'input>, id: usize) -> Self {
        Self {
            inner: UnitRefInner { state, id },
            phantom: PhantomData,
        }
    }

    pub fn state(&self) -> &'state State<'input> {
        self.inner.state
    }

    pub fn id(&self) -> Id<T> {
        Id::new(self.inner.id)
    }

    pub fn upcast(self) -> UnitRef<'input, 'state, Unit> {
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

impl<'input, 'state, T: UnitConv> Clone for UnitRef<'input, 'state, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'input, 'state, T: UnitConv> Copy for UnitRef<'input, 'state, T> {}

impl<'input, 'state, T: UnitConv> PartialEq for UnitRef<'input, 'state, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<'input, 'state, T: UnitConv> Eq for UnitRef<'input, 'state, T> {}

impl<'input, 'state, T> AsRef<UnitRef<'input, 'state, Unit>> for UnitRef<'input, 'state, T>
where
    T: UnitConv,
{
    fn as_ref(&self) -> &UnitRef<'input, 'state, Unit> {
        let ptr = self as *const UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *const UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &*(inner as *const UnitRef<Unit>)
        }
    }
}

impl<'input, 'state, T> AsMut<UnitRef<'input, 'state, Unit>> for UnitRef<'input, 'state, T>
where
    T: UnitConv,
{
    fn as_mut(&mut self) -> &mut UnitRef<'input, 'state, Unit> {
        let ptr = self as *mut UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *mut UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &mut *(inner as *mut UnitRef<Unit>)
        }
    }
}

impl<'input, 'state> UnitRef<'input, 'state, Unit> {
    pub fn downcast<T: UnitConv>(self) -> Option<UnitRef<'input, 'state, T>> {
        T::from_unit_ref(self.unit()).map(|_| unsafe {
            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            let inner: UnitRefInner = std::mem::transmute(self);

            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitRefInner, UnitRef<T>>(inner)
        })
    }
}

impl<'input, 'state> fmt::Debug for UnitRef<'input, 'state, Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit() {
            Unit::Value(_) => write!(f, "Value({:?})", self.downcast::<Value>().unwrap()),
            Unit::Function(_) => write!(f, "Function({:?})", self.downcast::<Function>().unwrap()),
        }
    }
}
