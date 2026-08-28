//! Capsule types whose values capture expressions for custom decoding
//! (hcl v2: the `ext/customdecode` package).

use std::sync::Arc;

use cty::{Type, Value};

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::structure::ExprRef;

/// A custom decoding callback taking the expression and its context
/// (customdecode: `customdecode.CustomExpressionDecoderFunc`).
pub type CustomExpressionDecoderFunc =
    Arc<dyn Fn(&ExprRef, Option<&EvalContext>) -> (Value, Diagnostics)>;

/// The capsule-ops extension key under which a capsule type can provide a
/// custom expression decoder (customdecode:
/// `customdecode.CustomExpressionDecoder`). Use as the key argument to
/// `cty`'s capsule extension-data mechanism.
pub fn custom_expression_decoder_key() -> &'static str {
    todo!()
}

/// The custom expression decoder of the given capsule type, if any
/// (customdecode: `customdecode.CustomExpressionDecoderForType`).
pub fn custom_expression_decoder_for_type(ty: &Type) -> Option<CustomExpressionDecoderFunc> {
    todo!()
}

/// A capsule type whose values each capture an HCL expression
/// (customdecode: `customdecode.ExpressionType`).
pub fn expression_type() -> Type {
    todo!()
}

/// The given expression wrapped as a value of [`expression_type`]
/// (customdecode: `customdecode.ExpressionVal`).
pub fn expression_val(expr: ExprRef) -> Value {
    todo!()
}

/// The expression wrapped by the given value
/// (customdecode: `customdecode.ExpressionFromVal`).
pub fn expression_from_val(v: &Value) -> ExprRef {
    todo!()
}

/// A capsule type whose values capture an expression together with its
/// evaluation context (customdecode: `customdecode.ExpressionClosureType`).
pub fn expression_closure_type() -> Type {
    todo!()
}

/// An expression bundled with its evaluation context
/// (customdecode: `customdecode.ExpressionClosure`).
#[derive(Debug, Clone)]
pub struct ExpressionClosure {
    /// The captured expression
    /// (customdecode: `ExpressionClosure.Expression`).
    pub expression: ExprRef,
    /// The captured context
    /// (customdecode: `ExpressionClosure.EvalContext`; `nil` ⇒ `None`).
    pub eval_context: Option<Arc<EvalContext>>,
}

impl ExpressionClosure {
    /// Evaluates the closure's expression in its captured context
    /// (customdecode: `ExpressionClosure.Value`).
    pub fn value(&self) -> (Value, Diagnostics) {
        todo!()
    }
}

/// The given closure wrapped as a value of [`expression_closure_type`]
/// (customdecode: `customdecode.ExpressionClosureVal`).
pub fn expression_closure_val(closure: ExpressionClosure) -> Value {
    todo!()
}

/// The closure wrapped by the given value
/// (customdecode: `customdecode.ExpressionClosureFromVal`).
pub fn expression_closure_from_val(v: &Value) -> ExpressionClosure {
    todo!()
}
