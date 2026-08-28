//! Wrapping bodies with content transformations (hcl v2: the
//! `ext/transform` package).

use std::sync::Arc;

use crate::diagnostic::Diagnostics;
use crate::structure::BodyRef;

/// A transformation applied to a body (transform:
/// `transform.Transformer`).
pub trait Transformer {
    /// The transformed body; must not mutate the given body
    /// (transform: `Transformer.TransformBody`).
    fn transform_body(&self, body: BodyRef) -> BodyRef;
}

/// A [`Transformer`] from a plain function (transform:
/// `transform.TransformerFunc`).
pub struct TransformerFunc(pub Arc<dyn Fn(BodyRef) -> BodyRef>);

impl Transformer for TransformerFunc {
    fn transform_body(&self, body: BodyRef) -> BodyRef {
        (self.0)(body)
    }
}

/// A transformer applying the given transformers in order (transform:
/// `transform.Chain`).
pub fn chain(c: Vec<Arc<dyn Transformer>>) -> Arc<dyn Transformer> {
    todo!()
}

/// A body whose direct content is the given body's, transformed
/// (transform: `transform.Shallow`).
pub fn shallow(body: BodyRef, transformer: Arc<dyn Transformer>) -> BodyRef {
    todo!()
}

/// A body transformed recursively: the transformer applies to the body and
/// to every block body found under it (transform: `transform.Deep`).
pub fn deep(body: BodyRef, transformer: Arc<dyn Transformer>) -> BodyRef {
    todo!()
}

/// A body with no content that returns the given diagnostics from every
/// content method (transform: `transform.NewErrorBody`).
pub fn new_error_body(diags: Diagnostics) -> BodyRef {
    todo!()
}

/// The given body wrapped so the given diagnostics are appended to every
/// content-method result (transform: `transform.BodyWithDiagnostics`).
pub fn body_with_diagnostics(body: BodyRef, diags: Diagnostics) -> BodyRef {
    todo!()
}
