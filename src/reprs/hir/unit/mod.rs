pub mod function;
pub mod unit_mut;
pub mod unit_ref;
pub mod value;

use super::state::event::UnitEvent;

pub use function::Function;
pub use unit_mut::UnitMut;
pub use unit_ref::UnitRef;
pub use value::Value;

#[non_exhaustive]
pub enum Unit {
    Value(Value),
    Function(Function),
}

pub trait UnitConv: Into<Unit> {
    type Event: Into<UnitEvent>;

    fn from_unit_ref(unit: &Unit) -> Option<&Self>;

    fn from_unit_mut(unit: &mut Unit) -> Result<&mut Self, &mut Unit>;
}

impl UnitConv for Unit {
    type Event = UnitEvent;

    fn from_unit_ref(unit: &Unit) -> Option<&Self> {
        Some(unit)
    }

    fn from_unit_mut(unit: &mut Unit) -> Result<&mut Self, &mut Unit> {
        Ok(unit)
    }
}

macro_rules! impl_unit_conv {
    ($unit_name:ident, $event_name:ident) => {
        impl From<$event_name> for UnitEvent {
            fn from(value: $event_name) -> Self {
                UnitEvent::$unit_name(value)
            }
        }

        impl UnitConv for $unit_name {
            type Event = $event_name;

            fn from_unit_ref(unit: &Unit) -> Option<&Self> {
                match unit {
                    Unit::$unit_name(unit) => Some(unit),
                    _ => None,
                }
            }

            fn from_unit_mut(unit: &mut Unit) -> Result<&mut Self, &mut Unit> {
                match unit {
                    Unit::$unit_name(unit) => Ok(unit),
                    unit => Err(unit),
                }
            }
        }

        impl From<$unit_name> for Unit {
            fn from(value: $unit_name) -> Self {
                Unit::$unit_name(value)
            }
        }
    };
}

use impl_unit_conv;
