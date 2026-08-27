//! The JSON syntax's implementation of the HCL information model
//! (hcl v2: `json/structure.go`).
//!
//! Go keeps these types unexported; they are public here because the
//! upstream structure tests compare against them directly.

use std::any::Any;

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::pos::Range;
use crate::schema::BodySchema;
use crate::structure as hcl_structure;
use crate::structure::{BodyContent, BodyRef};
use crate::traversal::Traversal;

use super::ast::Node;

/// A JSON-syntax HCL body, wrapping a raw JSON value
/// (json: the unexported `body` struct).
///
/// Go's unexported `body.hiddenAttrs` bookkeeping field (attributes hidden
/// by a previous `PartialContent` call) has no public analogue here.
#[derive(Debug, Clone)]
pub struct Body {
    /// The raw JSON value this body wraps (json: `body.val`).
    pub val: Node,
}

impl hcl_structure::Body for Body {
    fn content(&self, schema: &BodySchema) -> (BodyContent, Diagnostics) {
        todo!()
    }

    fn partial_content(&self, schema: &BodySchema) -> (BodyContent, BodyRef, Diagnostics) {
        todo!()
    }

    fn just_attributes(&self) -> (hcl_structure::Attributes, Diagnostics) {
        todo!()
    }

    fn missing_item_range(&self) -> Range {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A JSON-syntax HCL expression, wrapping a raw JSON value
/// (json: the unexported `expression` struct).
#[derive(Debug, Clone)]
pub struct Expression {
    /// The raw JSON value this expression wraps (json: `expression.src`).
    pub src: Node,
}

impl hcl_structure::Expression for Expression {
    fn value(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        todo!()
    }

    fn variables(&self) -> Vec<Traversal> {
        todo!()
    }

    fn range(&self) -> Range {
        todo!()
    }

    fn start_range(&self) -> Range {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
