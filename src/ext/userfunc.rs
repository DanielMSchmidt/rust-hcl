//! User-defined functions declared in configuration (hcl v2: the
//! `ext/userfunc` package).

use std::collections::HashMap;
use std::sync::Arc;

use cty::function::Function;

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::structure::BodyRef;

/// A callback providing the context in which a user function's result
/// expression is evaluated, called once per call
/// (userfunc: `userfunc.ContextFunc`; a Go nil context ⇒ `None`).
pub type ContextFunc = Arc<dyn Fn() -> Option<Arc<EvalContext>>>;

/// Decodes the blocks of the given type into function definitions,
/// returning the functions by name and the remaining body
/// (userfunc: `userfunc.DecodeUserFunctions`).
pub fn decode_user_functions(
    body: BodyRef,
    block_type: &str,
    context: Option<ContextFunc>,
) -> (HashMap<String, Function>, BodyRef, Diagnostics) {
    todo!()
}
