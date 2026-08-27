//! Static analysis helpers over expressions (hcl v2:
//! `traversal_for_expr.go`, `expr_call.go`, `expr_list.go`, `expr_map.go`,
//! `expr_unwrap.go`, `static_expr.go`).

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::pos::Range;
use crate::structure::{ExprRef, Expression};
use crate::traversal::Traversal;

/// Interprets the given expression as an absolute traversal, without
/// evaluating it (hcl: `hcl.AbsTraversalForExpr`).
pub fn abs_traversal_for_expr(expr: &dyn Expression) -> (Traversal, Diagnostics) {
    todo!()
}

/// Interprets the given expression as a relative traversal
/// (hcl: `hcl.RelTraversalForExpr`).
pub fn rel_traversal_for_expr(expr: &dyn Expression) -> (Traversal, Diagnostics) {
    todo!()
}

/// The bare keyword the expression consists of, or the empty string if it is
/// not a keyword-shaped expression (hcl: `hcl.ExprAsKeyword`).
pub fn expr_as_keyword(expr: &dyn Expression) -> String {
    todo!()
}

/// A static function call extracted from an expression without evaluating it
/// (hcl: `hcl.StaticCall`).
#[derive(Debug, Clone, PartialEq)]
pub struct StaticCall {
    /// The called function's name (hcl: `StaticCall.Name`).
    pub name: String,
    /// The range of the function name (hcl: `StaticCall.NameRange`).
    pub name_range: Range,
    /// The argument expressions (hcl: `StaticCall.Arguments`).
    pub arguments: Vec<ExprRef>,
    /// The range of the argument list, including parentheses
    /// (hcl: `StaticCall.ArgsRange`).
    pub args_range: Range,
}

/// Interprets the expression as a static function call
/// (hcl: `hcl.ExprCall`).
pub fn expr_call(expr: &dyn Expression) -> (StaticCall, Diagnostics) {
    todo!()
}

/// Interprets the expression as a static list of expressions
/// (hcl: `hcl.ExprList`).
pub fn expr_list(expr: &dyn Expression) -> (Vec<ExprRef>, Diagnostics) {
    todo!()
}

/// One key–value pair from [`expr_map`] (hcl: `hcl.KeyValuePair`).
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValuePair {
    /// The key expression (hcl: `KeyValuePair.Key`).
    pub key: ExprRef,
    /// The value expression (hcl: `KeyValuePair.Value`).
    pub value: ExprRef,
}

/// Interprets the expression as a static map of expressions
/// (hcl: `hcl.ExprMap`).
pub fn expr_map(expr: &dyn Expression) -> (Vec<KeyValuePair>, Diagnostics) {
    todo!()
}

/// Removes one layer of decorator expression, if present
/// (hcl: `hcl.UnwrapExpression`; returns the input when nothing unwraps).
pub fn unwrap_expression(expr: ExprRef) -> ExprRef {
    todo!()
}

/// Unwraps decorator expressions until the predicate matches, or `None` if
/// it never does (hcl: `hcl.UnwrapExpressionUntil`).
pub fn unwrap_expression_until(
    expr: ExprRef,
    until: impl Fn(&dyn Expression) -> bool,
) -> Option<ExprRef> {
    todo!()
}

/// An expression that always evaluates to the given value
/// (hcl: `hcl.StaticExpr`).
pub fn static_expr(val: Value, rng: Range) -> ExprRef {
    todo!()
}
