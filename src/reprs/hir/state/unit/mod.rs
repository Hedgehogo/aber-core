pub mod function;
pub mod value;

use function::Function;
use value::Value;

pub use function::{FunctionMut, FunctionRef};
pub use value::{ValueMut, ValueRef};

#[non_exhaustive]
pub(super) enum Unit {
    Value(Value),
    Function(Function),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitRef<'state, 'input> {
    Value(ValueRef<'state, 'input>),
    Function(FunctionRef<'state, 'input>),
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitMut<'state, 'input> {
    Value(ValueMut<'state, 'input>),
    Function(FunctionMut<'state, 'input>),
}

macro_rules! make_adapters {
    ($unit_name:ident, $unit_ref_name:ident, $unit_mut_name:ident, $unit_event_name:ident) => {
        use $crate::reprs::hir::state::event::Event;
        use $crate::reprs::hir::state::unit::Unit;
        use $crate::reprs::hir::state::State;

        #[derive(Clone, Copy)]
        pub struct $unit_ref_name<'state, 'input> {
            state: &'state State<'input>,
            id: usize,
        }

        impl<'state, 'input> $unit_ref_name<'state, 'input> {
            pub(in super::super::super) fn new(state: &'state State<'input>, id: usize) -> Self {
                Self { state, id }
            }

            fn unit(&self) -> &'state $unit_name {
                let unit = self.state.get_unit(self.id).expect("Unit must exist");

                match unit {
                    Unit::$unit_name(unit) => unit,
                    _ => panic!("Unit was supposed to be a {}", stringify!($unit_name)),
                }
            }

            pub fn id(&self) -> usize {
                self.id
            }
        }

        impl<'state, 'input> PartialEq for $unit_ref_name<'state, 'input> {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl<'state, 'input> Eq for $unit_ref_name<'state, 'input> {}

        pub struct $unit_mut_name<'state, 'input> {
            state: &'state mut State<'input>,
            id: usize,
        }

        impl<'state, 'input> $unit_mut_name<'state, 'input> {
            pub(in super::super::super) fn new(
                state: &'state mut State<'input>,
                id: usize,
            ) -> Self {
                Self { state, id }
            }

            fn unit(&self) -> &$unit_name {
                let unit = self.state.get_unit(self.id).expect("Unit must exist");

                match unit {
                    Unit::$unit_name(i) => i,
                    _ => panic!("Unit was supposed to be a {}", stringify!($unit_name)),
                }
            }

            fn unit_mut(&mut self) -> &mut $unit_name {
                let unit = self.state.get_unit_mut(self.id).expect("Unit must exist");

                match unit {
                    Unit::$unit_name(i) => i,
                    _ => panic!("Unit was supposed to be a {}", stringify!($unit_name)),
                }
            }

            fn log(&mut self, event: $unit_event_name) {
                let event = Event::Unit(self.id, event.into());
                self.state.log.push(event.into());
            }

            pub fn id(&self) -> usize {
                self.id
            }
        }

        impl<'state, 'input> PartialEq for $unit_mut_name<'state, 'input> {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl<'state, 'input> Eq for $unit_mut_name<'state, 'input> {}
    };
}

use make_adapters;
