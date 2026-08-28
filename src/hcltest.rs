//! Constructing mock HCL structures for testing (hcl v2: the `hcltest`
//! package).

use std::collections::HashMap;

use cty::Value;

use crate::structure::{Attributes, BodyContent, BodyRef, ExprRef};
use crate::traversal::Traversal;

/// A body with the given fixed content (hcltest: `hcltest.MockBody`).
pub fn mock_body(content: BodyContent) -> BodyRef {
    todo!()
}

/// An expression that always evaluates to the given literal value
/// (hcltest: `hcltest.MockExprLiteral`).
pub fn mock_expr_literal(val: Value) -> ExprRef {
    todo!()
}

/// An expression that evaluates the given variable name in its context
/// (hcltest: `hcltest.MockExprVariable`).
pub fn mock_expr_variable(name: &str) -> ExprRef {
    todo!()
}

/// An expression that evaluates the given traversal
/// (hcltest: `hcltest.MockExprTraversal`).
pub fn mock_expr_traversal(traversal: Traversal) -> ExprRef {
    todo!()
}

/// An expression that evaluates the traversal parsed from the given source
/// (hcltest: `hcltest.MockExprTraversalSrc`).
pub fn mock_expr_traversal_src(src: &str) -> ExprRef {
    todo!()
}

/// An expression that evaluates to a tuple of the given expressions'
/// results (hcltest: `hcltest.MockExprList`).
pub fn mock_expr_list(exprs: Vec<ExprRef>) -> ExprRef {
    todo!()
}

/// Attributes wrapping the given expressions, with synthetic ranges
/// (hcltest: `hcltest.MockAttrs`).
pub fn mock_attrs(exprs: HashMap<String, ExprRef>) -> Attributes {
    todo!()
}
