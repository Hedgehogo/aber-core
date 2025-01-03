//! Module that provides types for describing expressions

use super::Node;
use super::span::Spanned;

/// Type that describes an expression.
pub type Expr<'input> = Vec<Spanned<Node<'input>>>;

/// Type that describes a sequence of expressions.
pub type ExprVec<'input> = Vec<Spanned<Expr<'input>>>;
