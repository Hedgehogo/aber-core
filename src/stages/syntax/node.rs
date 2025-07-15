//! Module that provides [`Node`].

use super::{EscapedString, Expr, RawString};
use crate::reprs::Wast;

/// Trait realized by the types that the
/// [`fact`](`crate::stages::syntax::parse::fact`) function can return. It is
/// intended to avoid unnecessary conversion of the returned type
/// into a type with a larger set of values.
pub trait Node<'input>: Sized {
    /// Type describing the expression.
    type Expr: Expr<'input, Node = Self>;

    /// Type describing the string.
    type String: EscapedString<'input> + RawString<'input>;

    /// Creates a node from WAST fact.
    ///
    /// # Arguments
    /// - `wast` WAST fact.
    fn from_wast(wast: Wast<'input, Self>) -> Self;
}
