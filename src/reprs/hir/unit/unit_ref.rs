use super::super::State;
use super::{Unit, UnitConv, Function, Value};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Copy)]
struct UnitRefInner<'state, 'input> {
    state: &'state State<'input>,
    id: usize,
}

#[repr(transparent)]
pub struct UnitRef<'state, 'input, T: UnitConv> {
    inner: UnitRefInner<'state, 'input>,
    phantom: PhantomData<T>,
}

impl<'state, 'input, T: UnitConv> UnitRef<'state, 'input, T> {
    pub(in super::super) fn new(state: &'state State<'input>, id: usize) -> Self {
        Self {
            inner: UnitRefInner { state, id },
            phantom: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.inner.id
    }

    pub fn upcast(self) -> UnitRef<'state, 'input, Unit> {
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

impl<'state, 'input, T: UnitConv> Clone for UnitRef<'state, 'input, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'state, 'input, T: UnitConv> Copy for UnitRef<'state, 'input, T> {}

impl<'state, 'input, T: UnitConv> PartialEq for UnitRef<'state, 'input, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<'state, 'input, T: UnitConv> Eq for UnitRef<'state, 'input, T> {}

impl<'state, 'input, T> AsRef<UnitRef<'state, 'input, Unit>> for UnitRef<'state, 'input, T>
where
    T: UnitConv,
{
    fn as_ref(&self) -> &UnitRef<'state, 'input, Unit> {
        let ptr = self as *const UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *const UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &*(inner as *const UnitRef<Unit>)
        }
    }
}

impl<'state, 'input, T> AsMut<UnitRef<'state, 'input, Unit>> for UnitRef<'state, 'input, T>
where
    T: UnitConv,
{
    fn as_mut(&mut self) -> &mut UnitRef<'state, 'input, Unit> {
        let ptr = self as *mut UnitRef<T>;

        unsafe {
            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            let inner = ptr as *mut UnitRefInner;

            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            &mut *(inner as *mut UnitRef<Unit>)
        }
    }
}

impl<'state, 'input> UnitRef<'state, 'input, Unit> {
    pub fn downcast<T: UnitConv>(self) -> Option<UnitRef<'state, 'input, T>> {
        T::from_unit_ref(self.unit()).map(|_| unsafe {
            // It's safe because `UnitRef<Unit>` is `#[repr(transparent)]`
            let inner: UnitRefInner = std::mem::transmute(self);

            // It's safe because `UnitRef<T>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitRefInner, UnitRef<T>>(inner)
        })
    }
}

impl<'state, 'input> fmt::Debug for UnitRef<'state, 'input, Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit() {
            Unit::Value(_) => write!(f, "Value({:?})", self.downcast::<Value>().unwrap()),
            Unit::Function(_) => write!(f, "Function({:?})", self.downcast::<Function>().unwrap()),
        }
    }
}
