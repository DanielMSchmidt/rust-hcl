//! Decoding and encoding between HCL bodies and Rust structs (hcl v2: the
//! `gohcl` package).
//!
//! Go drives this with reflection over `hcl:"..."` struct tags; the Rust
//! analogue is the `hcl-derive` crate's `#[derive(FromBody)]` /
//! `#[derive(EncodeBody)]` macros with `#[hcl(...)]` field attributes. See
//! `docs/api-mapping.md` for the attribute grammar.

use cty::Value;

pub use hcl_derive::{EncodeBody, FromBody};

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::hclwrite;
use crate::schema::BodySchema;
use crate::structure::{Body, Expression};

/// A type that can be decoded from an HCL body, normally via
/// `#[derive(FromBody)]` (gohcl: the struct shapes accepted by
/// `gohcl.DecodeBody`).
pub trait FromBody: Sized {
    /// Decodes the body into a new value of this type, alongside any
    /// diagnostics; the partial result is meaningful even with errors
    /// (gohcl: `gohcl.DecodeBody`'s fill-in of the target).
    fn from_body(body: &dyn Body, ctx: Option<&EvalContext>) -> (Self, Diagnostics);

    /// The schema this type implies, and whether it is partial (has a
    /// `remain` field) (gohcl: `gohcl.ImpliedBodySchema`).
    fn implied_body_schema() -> (BodySchema, bool);
}

/// A type that can be encoded into an `hclwrite` body, normally via
/// `#[derive(EncodeBody)]` (gohcl: the struct shapes accepted by
/// `gohcl.EncodeIntoBody`).
pub trait EncodeBody {
    /// Encodes this value's fields into the given body
    /// (gohcl: `gohcl.EncodeIntoBody`).
    fn encode_into_body(&self, dst: &hclwrite::Body);

    /// Encodes this value as a new block of the given type
    /// (gohcl: `gohcl.EncodeAsBlock`).
    fn encode_as_block(&self, block_type: &str) -> hclwrite::Block;
}

/// A type that can be decoded from a single HCL expression (gohcl: the
/// target shapes accepted by `gohcl.DecodeExpression`, via gocty
/// conversion).
pub trait FromExpression: Sized {
    /// Evaluates the expression and converts the result into this type
    /// (gohcl: `gohcl.DecodeExpression`'s fill-in of the target).
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics);
}

impl FromExpression for String {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl FromExpression for i64 {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl FromExpression for f64 {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl FromExpression for bool {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl FromExpression for Value {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl<T: FromExpression> FromExpression for std::collections::HashMap<String, T> {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

impl<T: FromExpression> FromExpression for Vec<T> {
    fn from_expression(expr: &dyn Expression, ctx: Option<&EvalContext>) -> (Self, Diagnostics) {
        todo!()
    }
}

/// Decodes the body into a new `T`, alongside any diagnostics
/// (gohcl: `gohcl.DecodeBody`; the Go out-parameter becomes the first
/// return value).
pub fn decode_body<T: FromBody>(body: &dyn Body, ctx: Option<&EvalContext>) -> (T, Diagnostics) {
    todo!()
}

/// Evaluates the expression and converts the result to a new `T`
/// (gohcl: `gohcl.DecodeExpression`; the Go out-parameter becomes the
/// first return value).
pub fn decode_expression<T: FromExpression>(
    expr: &dyn Expression,
    ctx: Option<&EvalContext>,
) -> (T, Diagnostics) {
    todo!()
}

/// The schema implied by `T`'s field attributes, and whether it is partial
/// (gohcl: `gohcl.ImpliedBodySchema`; the Go value argument becomes a type
/// parameter).
pub fn implied_body_schema<T: FromBody>() -> (BodySchema, bool) {
    todo!()
}

/// Encodes the value's fields into an existing `hclwrite` body
/// (gohcl: `gohcl.EncodeIntoBody`).
pub fn encode_into_body<T: EncodeBody>(val: &T, dst: &hclwrite::Body) {
    todo!()
}

/// Encodes the value as a new `hclwrite` block of the given type
/// (gohcl: `gohcl.EncodeAsBlock`).
pub fn encode_as_block<T: EncodeBody>(val: &T, block_type: &str) -> hclwrite::Block {
    todo!()
}
