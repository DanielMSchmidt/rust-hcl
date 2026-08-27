//! Expression evaluation contexts (hcl v2: `eval_context.go`).

use std::collections::HashMap;
use std::sync::Arc;

use cty::Value;
use cty::function::Function;

/// The variables and functions in scope for expression evaluation
/// (hcl: `hcl.EvalContext`).
///
/// Contexts form a tree: `EvalContext::new_child(&parent)` mirrors Go's
/// `ctx.NewChild()`, taking the parent as a shared handle so the child can
/// keep a reference up the chain.
#[derive(Debug, Default)]
pub struct EvalContext {
    /// The variables in scope (hcl: `EvalContext.Variables`).
    pub variables: HashMap<String, Value>,
    /// The functions in scope (hcl: `EvalContext.Functions`).
    pub functions: HashMap<String, Function>,
    // Stub phase: read only once `parent()` and lookup are implemented.
    #[allow(dead_code)]
    parent: Option<Arc<EvalContext>>,
}

impl EvalContext {
    /// An empty root context (Go: `&hcl.EvalContext{}`).
    pub fn new() -> EvalContext {
        EvalContext::default()
    }

    /// A new empty child of the given context; definitions in the child
    /// shadow the parent's on lookup (hcl: `EvalContext.NewChild`).
    pub fn new_child(parent: &Arc<EvalContext>) -> EvalContext {
        todo!()
    }

    /// The parent of this context, if it is not a root
    /// (hcl: `EvalContext.Parent`; `nil` ⇒ `None`).
    pub fn parent(&self) -> Option<&EvalContext> {
        todo!()
    }
}
