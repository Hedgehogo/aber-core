use super::super::state::{State, UnitEvent};
use super::{Function, Unit, UnitConv, Value};
use std::{fmt, marker::PhantomData};

struct UnitMutInner<'state, 'input> {
    state: &'state mut State<'input>,
    id: usize,
}

#[repr(transparent)]
pub struct UnitMut<'state, 'input, T: UnitConv> {
    inner: UnitMutInner<'state, 'input>,
    phantom: PhantomData<T>,
}

impl<'state, 'input, T: UnitConv> UnitMut<'state, 'input, T> {
    pub(in super::super) fn new(state: &'state mut State<'input>, id: usize) -> Self {
        Self {
            inner: UnitMutInner { state, id },
            phantom: PhantomData,
        }
    }

    pub fn id(&self) -> usize {
        self.inner.id
    }

    pub fn upcast(self) -> UnitMut<'state, 'input, Unit> {
        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner: UnitMutInner = std::mem::transmute(self);

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            std::mem::transmute::<UnitMutInner, UnitMut<Unit>>(inner)
        }
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
        self.inner.state.log(self.id(), event.into());
    }
}

impl<'state, 'input, T: UnitConv> PartialEq for UnitMut<'state, 'input, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl<'state, 'input, T: UnitConv> Eq for UnitMut<'state, 'input, T> {}

impl<'state, 'input, T> AsRef<UnitMut<'state, 'input, Unit>> for UnitMut<'state, 'input, T>
where
    T: UnitConv,
{
    fn as_ref(&self) -> &UnitMut<'state, 'input, Unit> {
        let ptr = self as *const UnitMut<T>;

        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner = ptr as *const UnitMutInner;

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            &*(inner as *const UnitMut<Unit>)
        }
    }
}

impl<'state, 'input, T> AsMut<UnitMut<'state, 'input, Unit>> for UnitMut<'state, 'input, T>
where
    T: UnitConv,
{
    fn as_mut(&mut self) -> &mut UnitMut<'state, 'input, Unit> {
        let ptr = self as *mut UnitMut<T>;

        unsafe {
            // It's safe because `UnitMut<T>` is `#[repr(transparent)]`
            let inner = ptr as *mut UnitMutInner;

            // It's safe because `UnitMut<Unit>` is `#[repr(transparent)]`
            &mut *(inner as *mut UnitMut<Unit>)
        }
    }
}

impl<'state, 'input> UnitMut<'state, 'input, Unit> {
    pub fn downcast<T: UnitConv>(
        self,
    ) -> Result<UnitMut<'state, 'input, T>, UnitMut<'state, 'input, Unit>> {
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

    pub fn downcast_ref<T: UnitConv>(
        &self,
    ) -> Result<&UnitMut<'state, 'input, T>, &UnitMut<'state, 'input, Unit>> {
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
    ) -> Result<&mut UnitMut<'state, 'input, T>, &mut UnitMut<'state, 'input, Unit>> {
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

impl<'state, 'input> fmt::Debug for UnitMut<'state, 'input, Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit() {
            Unit::Value(_) => {
                let unit = self.downcast_ref::<Value>().ok().unwrap();
                write!(f, "Value({unit:?})")
            }

            Unit::Function(_) => {
                let unit = self.downcast_ref::<Function>().ok().unwrap();
                write!(f, "Function({unit:?})")
            }
        }
    }
}
