use super::super::super::wast::call::Ident;
use super::unit::function::FunctionEvent;
use super::unit::value::ValueEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum Event<'input> {
    Declare(Ident<'input>),
    Unit(usize, UnitEvent),
}

impl<'input> From<EventZipped<'input>> for Event<'input> {
    fn from(value: EventZipped<'input>) -> Self {
        match value {
            EventZipped::Declare(ident) => Event::Declare(ident),
            EventZipped::ValueSet(id) => Event::Unit(id, ValueEvent::Set.into()),
            EventZipped::FunctionAddArgCount(id) => {
                Event::Unit(id, FunctionEvent::AddArgCount.into())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnitEvent {
    Value(ValueEvent),
    Function(FunctionEvent),
}

impl From<ValueEvent> for UnitEvent {
    fn from(value: ValueEvent) -> Self {
        UnitEvent::Value(value)
    }
}

impl From<FunctionEvent> for UnitEvent {
    fn from(value: FunctionEvent) -> Self {
        UnitEvent::Function(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum EventZipped<'input> {
    Declare(Ident<'input>),
    ValueSet(usize),
    FunctionAddArgCount(usize),
}

impl<'input> EventZipped<'input> {
    pub(super) fn into_event(self) -> Event<'input> {
        self.into()
    }
}

impl<'input> From<Event<'input>> for EventZipped<'input> {
    fn from(value: Event<'input>) -> Self {
        match value {
            Event::Declare(ident) => EventZipped::Declare(ident),

            Event::Unit(id, event) => match event {
                UnitEvent::Value(event) => match event {
                    ValueEvent::Set => EventZipped::ValueSet(id),
                },

                UnitEvent::Function(event) => match event {
                    FunctionEvent::AddArgCount => EventZipped::FunctionAddArgCount(id),
                },
            },
        }
    }
}
