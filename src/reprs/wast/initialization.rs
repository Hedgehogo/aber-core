//! Module that provides [`Initialization`].

use super::{whitespaced::Whitespaced, List, Spanned};
use crate::stages::syntax::{Expr, Node};
use std::fmt;

type Name<N, W> = (Whitespaced<<N as Node>::Expr, <N as Node>::Ident>, W);

/// Type describing sequence I from the initialization syntax specification.
pub struct Argument<X: Expr> {
    pub name: Option<Name<X::Node, X::Whitespace>>,
    pub expr: Spanned<X>,
}

impl<X: Expr> Argument<X> {
    /// Creates a new `Argument`.
    pub fn new(name: Option<Name<X::Node, X::Whitespace>>, expr: Spanned<X>) -> Self {
        Self { name, expr }
    }
}

impl<X> fmt::Debug for Argument<X>
where
    X: Expr + fmt::Debug,
    X::Whitespace: fmt::Debug,
    <<X as Expr>::Node as Node>::Ident: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Argument")
            .field("name", &self.name)
            .field("expr", &self.expr)
            .finish()
    }
}

impl<X> Clone for Argument<X>
where
    X: Expr + Clone,
    X::Whitespace: Clone,
    <<X as Expr>::Node as Node>::Ident: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            expr: self.expr.clone(),
        }
    }
}

impl<X> PartialEq for Argument<X>
where
    X: Expr + PartialEq,
    X::Whitespace: PartialEq,
    <<X as Expr>::Node as Node>::Ident: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.expr == other.expr
    }
}

impl<X> Eq for Argument<X>
where
    X: Expr + Eq,
    X::Whitespace: Eq,
    <<X as Expr>::Node as Node>::Ident: Eq,
{
}

/// Type describing elements of initialization syntax except expression.
pub type Arguments<X> = Whitespaced<X, List<Argument<X>, X>>;

/// Type describing syntactic construct *initialization*.
///
/// # Fields
/// - `expr` Expression before the operator.
/// - `args` Items listed comma-separately.
pub struct Initialization<X: Expr> {
    pub expr: Spanned<X>,
    pub args: Arguments<X>,
}

impl<X: Expr> Initialization<X> {
    /// Creates a new `Initialization`.
    pub fn new(expr: Spanned<X>, args: Arguments<X>) -> Self {
        Self { expr, args }
    }
}

impl<X> fmt::Debug for Initialization<X>
where
    X: Expr + fmt::Debug,
    X::Whitespace: fmt::Debug,
    <<X as Expr>::Node as Node>::Ident: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Initialization")
            .field("expr", &self.expr)
            .field("args", &self.args)
            .finish()
    }
}

impl<X> Clone for Initialization<X>
where
    X: Expr + Clone,
    X::Whitespace: Clone,
    <<X as Expr>::Node as Node>::Ident: Clone,
{
    fn clone(&self) -> Self {
        Self {
            expr: self.expr.clone(),
            args: self.args.clone(),
        }
    }
}

impl<X> PartialEq for Initialization<X>
where
    X: Expr + PartialEq,
    X::Whitespace: PartialEq,
    <<X as Expr>::Node as Node>::Ident: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr && self.args == other.args
    }
}

impl<X> Eq for Initialization<X>
where
    X: Expr + Eq,
    X::Whitespace: Eq,
    <<X as Expr>::Node as Node>::Ident: Eq,
{
}
