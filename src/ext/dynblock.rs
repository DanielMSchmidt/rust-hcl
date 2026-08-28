//! `dynamic` block expansion (hcl v2: the `ext/dynblock` package).

use std::sync::Arc;

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::hcldec::Spec;
use crate::schema::BodySchema;
use crate::structure::{BodyRef, ExprRef};
use crate::traversal::Traversal;

/// The callback of [`ExpandOption::CheckForEach`]
/// (dynblock: the parameter of `dynblock.OptCheckForEach`).
pub type ForEachCheckFunc = Arc<dyn Fn(&Value, &ExprRef, Option<&EvalContext>) -> Diagnostics>;

/// An option modifying [`expand`]'s behavior
/// (dynblock: `dynblock.ExpandOption`).
#[derive(Clone)]
pub enum ExpandOption {
    /// Registers a callback consulted for each `for_each` value before it
    /// is expanded (dynblock: `dynblock.OptCheckForEach`).
    CheckForEach(ForEachCheckFunc),
}

/// A body that transparently expands `dynamic` blocks in the given body,
/// evaluating their `for_each` against the given context
/// (dynblock: `dynblock.Expand`; Go's variadic options become the `opts`
/// vector, empty for none).
pub fn expand(body: BodyRef, ctx: Option<Arc<EvalContext>>, opts: Vec<ExpandOption>) -> BodyRef {
    todo!()
}

/// Begins a walk through the body and any bodies it might dynamically
/// generate, for variable detection (dynblock: `dynblock.WalkVariables`).
pub fn walk_variables(body: BodyRef) -> WalkVariablesNode {
    todo!()
}

/// Like [`walk_variables`], but only reports variables needed for the
/// expansion itself (dynblock: `dynblock.WalkExpandVariables`).
pub fn walk_expand_variables(body: BodyRef) -> WalkVariablesNode {
    todo!()
}

/// One node in a variables walk (dynblock: `dynblock.WalkVariablesNode`).
#[derive(Debug, Clone)]
pub struct WalkVariablesNode {
    _priv: (),
}

impl WalkVariablesNode {
    /// The variables this node needs given the schema, plus the child
    /// bodies to descend into (dynblock: `WalkVariablesNode.Visit`).
    pub fn visit(&self, schema: &BodySchema) -> (Vec<Traversal>, Vec<WalkVariablesChild>) {
        todo!()
    }
}

/// One child body discovered during a variables walk
/// (dynblock: `dynblock.WalkVariablesChild`).
#[derive(Debug, Clone)]
pub struct WalkVariablesChild {
    /// The child block's type name
    /// (dynblock: `WalkVariablesChild.BlockTypeName`).
    pub block_type_name: String,
    /// The node to continue the walk with
    /// (dynblock: `WalkVariablesChild.Node`).
    pub node: WalkVariablesNode,
}

impl WalkVariablesChild {
    /// The child body itself, for label inspection
    /// (dynblock: `WalkVariablesChild.Body`).
    pub fn body(&self) -> BodyRef {
        todo!()
    }
}

/// The variables needed to decode the body with the spec, seeing through
/// `dynamic` blocks (dynblock: `dynblock.VariablesHCLDec`).
pub fn variables_hcldec(body: BodyRef, spec: &dyn Spec) -> Vec<Traversal> {
    todo!()
}

/// Like [`variables_hcldec`], but only the variables needed for expansion
/// itself (dynblock: `dynblock.ExpandVariablesHCLDec`).
pub fn expand_variables_hcldec(body: BodyRef, spec: &dyn Spec) -> Vec<Traversal> {
    todo!()
}
