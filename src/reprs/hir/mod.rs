//! Module that provides types that describe the compilation level that results in a high intermediate representation (HIR).

pub mod input;
pub mod node;
pub mod state;

pub use node::Hir;
pub use input::{nodes, Nodes, NodesMapper};
pub use state::{FunctionMut, FunctionRef, State, UnitMut, UnitRef};
