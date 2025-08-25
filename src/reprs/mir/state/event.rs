use super::super::super::hir::Ident;
use super::super::unit::{function::FunctionEvent, value::ValueEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitEvent {
    Value(ValueEvent),
    Function(FunctionEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum Event {
    Declare(Ident),
    Push(usize),
    Unit(usize, UnitEvent),
}

impl From<EventZipped> for Event {
    fn from(value: EventZipped) -> Self {
        match value {
            EventZipped::Declare(ident) => Event::Declare(ident),
            EventZipped::Push(id) => Event::Push(id),
            EventZipped::ValueSet(id) => Event::Unit(id, ValueEvent::Set.into()),
            EventZipped::FunctionAddArgCount(id) => {
                Event::Unit(id, FunctionEvent::AddArgCount.into())
            }
            EventZipped::FunctionAddImpl(id) => Event::Unit(id, FunctionEvent::AddImpl.into()),
            EventZipped::FunctionSpecifyTime(id) => {
                Event::Unit(id, FunctionEvent::SpecifyTime.into())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum EventZipped {
    Declare(Ident),
    Push(usize),
    ValueSet(usize),
    FunctionAddArgCount(usize),
    FunctionAddImpl(usize),
    FunctionSpecifyTime(usize),
}

impl EventZipped {
    pub(super) fn unzip(self) -> Event {
        self.into()
    }
}

impl From<Event> for EventZipped {
    fn from(value: Event) -> Self {
        match value {
            Event::Declare(ident) => EventZipped::Declare(ident),

            Event::Push(id) => EventZipped::Push(id),

            Event::Unit(id, event) => match event {
                UnitEvent::Value(event) => match event {
                    ValueEvent::Set => EventZipped::ValueSet(id),
                },

                UnitEvent::Function(event) => match event {
                    FunctionEvent::AddArgCount => EventZipped::FunctionAddArgCount(id),
                    FunctionEvent::AddImpl => EventZipped::FunctionAddImpl(id),
                    FunctionEvent::SpecifyTime => EventZipped::FunctionSpecifyTime(id),
                },
            },
        }
    }
}
