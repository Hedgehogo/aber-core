use super::super::state::{State, UnitEvent, WithState};
use super::{Function, Id, Unit, UnitConv, Value};
use std::{fmt, marker::PhantomData};

struct UnitMutInner<'state> {
    state: &'state mut State,
    id: usize,
}

#[repr(transparent)]
pub struct UnitMut<'state, T: UnitConv> {
    inner: UnitMutInner<'state>,
    phantom: PhantomData<T>,
}

impl<'state, T: UnitConv> UnitMut<'state, T> {
    pub(in super::super) fn new(state: &'state mut State, id: usize) -> Self {
        Self {
            inner: UnitMutInner { state, id },
            phantom: PhantomData,
        }
    }

    pub fn state(self) -> &'state mut State {
        self.inner.state
    }

    pub fn id(&self) -> Id<T> {
        Id::new(self.inner.id)
    }

    pub fn upcast(self) -> UnitMut<'state, Unit> {
        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner: UnitMutInner = std::mem::transmute(self);

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitMutInner, UnitMut<Unit>>(inner)
        }
    }

    pub fn with_state(self) -> WithState<'state, Id<T>> {
        WithState(self.inner.state, Id::new(self.inner.id))
    }

    pub(super) fn unit(&self) -> &T {
        let unit = self
            .inner
            .state
            .get_unit(self.inner.id)
            .expect("Unit must exist");

        T::from_unit_ref(unit).expect("Different kind of unit was expected")
    }

    pub(super) fn unit_mut(&mut self) -> &mut T {
        let unit = self
            .inner
            .state
            .get_unit_mut(self.inner.id)
            .expect("Unit must exist");

        T::from_unit_mut(unit)
            .ok()
            .expect("Different kind of unit was expected")
    }

    pub(super) fn log(&mut self, event: T::Event) {
        self.inner.state.log(self.id().upcast(), event.into());
    }
}

impl<'state, T: UnitConv> PartialEq for UnitMut<'state, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<'state, T: UnitConv> Eq for UnitMut<'state, T> {}

impl<'state, T> AsRef<UnitMut<'state, Unit>> for UnitMut<'state, T>
where
    T: UnitConv,
{
    fn as_ref(&self) -> &UnitMut<'state, Unit> {
        let ptr = self as *const UnitMut<T>;

        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner = ptr as *const UnitMutInner;

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            &*(inner as *const UnitMut<Unit>)
        }
    }
}

impl<'state, T> AsMut<UnitMut<'state, Unit>> for UnitMut<'state, T>
where
    T: UnitConv,
{
    fn as_mut(&mut self) -> &mut UnitMut<'state, Unit> {
        let ptr = self as *mut UnitMut<T>;

        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner = ptr as *mut UnitMutInner;

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            &mut *(inner as *mut UnitMut<Unit>)
        }
    }
}

impl<'state> UnitMut<'state, Unit> {
    pub fn downcast<T: UnitConv>(self) -> Result<UnitMut<'state, T>, UnitMut<'state, Unit>> {
        match T::from_unit_ref(self.unit()) {
            Some(_) => unsafe {
                // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
                let inner: UnitMutInner = std::mem::transmute(self);

                // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
                Ok(std::mem::transmute::<UnitMutInner, UnitMut<T>>(inner))
            },

            None => Err(self),
        }
    }

    pub fn downcast_ref<T: UnitConv>(&self) -> Result<&UnitMut<'state, T>, &UnitMut<'state, Unit>> {
        match T::from_unit_ref(self.unit()) {
            Some(_) => {
                let ptr = self as *const UnitMut<Unit>;

                unsafe {
                    // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
                    let inner = ptr as *const UnitMutInner;

                    // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
                    Ok(&*(inner as *const UnitMut<T>))
                }
            }

            None => Err(self),
        }
    }

    pub fn downcast_mut<T: UnitConv>(
        &mut self,
    ) -> Result<&mut UnitMut<'state, T>, &mut UnitMut<'state, Unit>> {
        match T::from_unit_ref(self.unit()) {
            Some(_) => {
                let ptr = self as *mut UnitMut<Unit>;

                unsafe {
                    // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
                    let inner = ptr as *mut UnitMutInner;

                    // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
                    Ok(&mut *(inner as *mut UnitMut<T>))
                }
            }

            None => Err(self),
        }
    }

    pub(in super::super) fn rewind(self, event: UnitEvent) {
        match event {
            UnitEvent::Value(event) => self.downcast::<Value>().ok().unwrap().rewind(event),
            UnitEvent::Function(event) => self.downcast::<Function>().ok().unwrap().rewind(event),
        }
    }
}

impl<'state> fmt::Debug for UnitMut<'state, Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit() {
            Unit::Value(_) => f
                .debug_tuple("Value")
                .field(self.downcast_ref::<Value>().ok().unwrap())
                .finish(),

            Unit::Function(_) => f
                .debug_tuple("Function")
                .field(self.downcast_ref::<Function>().ok().unwrap())
                .finish(),
        }
    }
}
