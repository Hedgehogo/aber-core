//! Module that provides types that describe the compilation level that results in a high intermediate representation (HIR).

pub mod input;
pub mod node;
pub mod state;
pub mod unit;

pub use input::{nodes, Nodes};
pub use node::Mir;
pub use state::{State, WithState};
pub use unit::{Function, Value};
