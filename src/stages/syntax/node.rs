//! Module that provides [`Node`].

use super::Expr;
use crate::reprs::Wast;

/// Trait realized by the types that the
/// [`fact`](`crate::stages::syntax::parse::fact`) function can return. It is
/// intended to avoid unnecessary conversion of the returned type
/// into a type with a larger set of values.
pub trait Node: Sized {
    /// Type describing the expression.
    type Expr: Expr<Node = Self>;

    /// Type describing the identifier.
    type Ident;

    /// Type describing the digit sequence.
    type Digits;

    /// Type describing the character literal.
    type Character;

    /// Type describing the string literal.
    type String;

    /// Creates a node from WAST fact.
    ///
    /// # Arguments
    /// - `wast` WAST fact.
    fn from_wast(wast: Wast<Self>) -> Self;
}
