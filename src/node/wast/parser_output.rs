use super::Wast;
use super::super::Spanned;

/// Trait realized by the types that the [`fact`](`crate::syntax::parse::fact`) function can 
/// return. It is intended to avoid unnecessary conversion of the returned type into a type with a 
/// larger set of values.
pub trait ParserOutput<'input>: Sized {
    /// Type describing the expression.
    type Expr: Sized;

    /// Creates a node from WAST fact.
    /// 
    /// # Arguments
    /// - `wast` WAST fact.
    fn new_node(wast: Wast<'input, Self>) -> Self;

    /// Creates an expression from a sequence of WAST facts.
    /// 
    /// # Arguments
    /// - `seq` WAST fact sequence with spans.
    fn new_expr(seq: Vec<Spanned<Self>>) -> Self::Expr;
}
