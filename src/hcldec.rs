//! Spec-driven decoding of bodies to `cty` values (hcl v2: the `hcldec`
//! package).

use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use cty::function::Function;
use cty::{RefinementBuilder, Type, Value};

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::pos::Range;
use crate::schema::BodySchema;
use crate::structure::{Body, BodyRef, ExprRef};
use crate::traversal::Traversal;

/// A single spec in the hcldec language (hcldec: `hcldec.Spec`).
///
/// The concrete spec types ([`AttrSpec`], [`BlockSpec`], …) implement this
/// trait; compose them via [`SpecRef`] handles.
pub trait Spec: Any + fmt::Debug {
    /// This spec as [`Any`], for downcasting (Go: implicit via interface
    /// type assertions).
    fn as_any(&self) -> &dyn Any;
}

/// A shared handle to a [`Spec`] (Go: the `hcldec.Spec` interface value).
#[derive(Debug, Clone)]
pub struct SpecRef(pub Arc<dyn Spec>);

impl SpecRef {
    /// Wraps a concrete spec in a shared handle.
    pub fn new(spec: impl Spec) -> SpecRef {
        SpecRef(Arc::new(spec))
    }
}

impl Deref for SpecRef {
    type Target = dyn Spec;

    fn deref(&self) -> &(dyn Spec + 'static) {
        &*self.0
    }
}

/// Decodes the given body according to the spec
/// (hcldec: `hcldec.Decode`).
pub fn decode(body: &dyn Body, spec: &dyn Spec, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
    todo!()
}

/// Partially decodes the body, returning the leftover body alongside
/// (hcldec: `hcldec.PartialDecode`).
pub fn partial_decode(
    body: &dyn Body,
    spec: &dyn Spec,
    ctx: Option<&EvalContext>,
) -> (Value, BodyRef, Diagnostics) {
    todo!()
}

/// The `cty` type the spec would produce when decoding
/// (hcldec: `hcldec.ImpliedType`).
pub fn implied_type(spec: &dyn Spec) -> Type {
    todo!()
}

/// The approximate source range of the value the spec would decode
/// (hcldec: `hcldec.SourceRange`).
pub fn source_range(body: &dyn Body, spec: &dyn Spec) -> Range {
    todo!()
}

/// The specs for the child block types the spec would decode, by type name
/// (hcldec: `hcldec.ChildBlockTypes`).
pub fn child_block_types(spec: &dyn Spec) -> HashMap<String, SpecRef> {
    todo!()
}

/// The body schema the spec implies (hcldec: `hcldec.ImpliedSchema`).
pub fn implied_schema(spec: &dyn Spec) -> BodySchema {
    todo!()
}

/// The variables needed to decode the body with the spec
/// (hcldec: `hcldec.Variables`).
pub fn variables(body: &dyn Body, spec: &dyn Spec) -> Vec<Traversal> {
    todo!()
}

/// Specs by attribute name, producing an object value
/// (hcldec: `hcldec.ObjectSpec`).
#[derive(Debug, Clone, Default)]
pub struct ObjectSpec(pub HashMap<String, SpecRef>);

impl Spec for ObjectSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<K: Into<String>> FromIterator<(K, SpecRef)> for ObjectSpec {
    fn from_iter<I: IntoIterator<Item = (K, SpecRef)>>(iter: I) -> ObjectSpec {
        ObjectSpec(iter.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

/// Specs in order, producing a tuple value (hcldec: `hcldec.TupleSpec`).
#[derive(Debug, Clone, Default)]
pub struct TupleSpec(pub Vec<SpecRef>);

impl Spec for TupleSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes one attribute's expression to a value of a given type
/// (hcldec: `hcldec.AttrSpec`).
#[derive(Debug, Clone)]
pub struct AttrSpec {
    /// The attribute name (hcldec: `AttrSpec.Name`).
    pub name: String,
    /// The type to convert the value to (hcldec: `AttrSpec.Type`).
    pub ty: Type,
    /// Whether omitting the attribute is an error
    /// (hcldec: `AttrSpec.Required`).
    pub required: bool,
}

impl Spec for AttrSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Produces a fixed literal value (hcldec: `hcldec.LiteralSpec`).
#[derive(Debug, Clone)]
pub struct LiteralSpec {
    /// The value to produce (hcldec: `LiteralSpec.Value`).
    pub value: Value,
}

impl Spec for LiteralSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Evaluates a fixed expression (hcldec: `hcldec.ExprSpec`).
#[derive(Debug, Clone)]
pub struct ExprSpec {
    /// The expression to evaluate (hcldec: `ExprSpec.Expr`).
    pub expr: ExprRef,
}

impl Spec for ExprSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes at most one nested block of a given type
/// (hcldec: `hcldec.BlockSpec`).
#[derive(Debug, Clone)]
pub struct BlockSpec {
    /// The block type name (hcldec: `BlockSpec.TypeName`).
    pub type_name: String,
    /// The spec for the block's contents (hcldec: `BlockSpec.Nested`).
    pub nested: SpecRef,
    /// Whether omitting the block is an error
    /// (hcldec: `BlockSpec.Required`).
    pub required: bool,
}

impl Spec for BlockSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes zero or more nested blocks into a list
/// (hcldec: `hcldec.BlockListSpec`).
#[derive(Debug, Clone)]
pub struct BlockListSpec {
    /// The block type name (hcldec: `BlockListSpec.TypeName`).
    pub type_name: String,
    /// The spec for each block's contents
    /// (hcldec: `BlockListSpec.Nested`).
    pub nested: SpecRef,
    /// The minimum number of blocks; 0 for no minimum
    /// (hcldec: `BlockListSpec.MinItems`).
    pub min_items: usize,
    /// The maximum number of blocks; 0 for no maximum
    /// (hcldec: `BlockListSpec.MaxItems`).
    pub max_items: usize,
}

impl Spec for BlockListSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes zero or more nested blocks into a tuple
/// (hcldec: `hcldec.BlockTupleSpec`).
#[derive(Debug, Clone)]
pub struct BlockTupleSpec {
    /// The block type name (hcldec: `BlockTupleSpec.TypeName`).
    pub type_name: String,
    /// The spec for each block's contents
    /// (hcldec: `BlockTupleSpec.Nested`).
    pub nested: SpecRef,
    /// The minimum number of blocks; 0 for no minimum
    /// (hcldec: `BlockTupleSpec.MinItems`).
    pub min_items: usize,
    /// The maximum number of blocks; 0 for no maximum
    /// (hcldec: `BlockTupleSpec.MaxItems`).
    pub max_items: usize,
}

impl Spec for BlockTupleSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes zero or more nested blocks into a set
/// (hcldec: `hcldec.BlockSetSpec`).
#[derive(Debug, Clone)]
pub struct BlockSetSpec {
    /// The block type name (hcldec: `BlockSetSpec.TypeName`).
    pub type_name: String,
    /// The spec for each block's contents
    /// (hcldec: `BlockSetSpec.Nested`).
    pub nested: SpecRef,
    /// The minimum number of blocks; 0 for no minimum
    /// (hcldec: `BlockSetSpec.MinItems`).
    pub min_items: usize,
    /// The maximum number of blocks; 0 for no maximum
    /// (hcldec: `BlockSetSpec.MaxItems`).
    pub max_items: usize,
}

impl Spec for BlockSetSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes labelled blocks into nested maps keyed by their labels
/// (hcldec: `hcldec.BlockMapSpec`).
#[derive(Debug, Clone)]
pub struct BlockMapSpec {
    /// The block type name (hcldec: `BlockMapSpec.TypeName`).
    pub type_name: String,
    /// The label names, defining the nesting depth
    /// (hcldec: `BlockMapSpec.LabelNames`).
    pub label_names: Vec<String>,
    /// The spec for each block's contents
    /// (hcldec: `BlockMapSpec.Nested`).
    pub nested: SpecRef,
}

impl Spec for BlockMapSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes labelled blocks into nested objects keyed by their labels
/// (hcldec: `hcldec.BlockObjectSpec`).
#[derive(Debug, Clone)]
pub struct BlockObjectSpec {
    /// The block type name (hcldec: `BlockObjectSpec.TypeName`).
    pub type_name: String,
    /// The label names, defining the nesting depth
    /// (hcldec: `BlockObjectSpec.LabelNames`).
    pub label_names: Vec<String>,
    /// The spec for each block's contents
    /// (hcldec: `BlockObjectSpec.Nested`).
    pub nested: SpecRef,
}

impl Spec for BlockObjectSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Decodes a single block's arbitrary attributes into a map
/// (hcldec: `hcldec.BlockAttrsSpec`).
#[derive(Debug, Clone)]
pub struct BlockAttrsSpec {
    /// The block type name (hcldec: `BlockAttrsSpec.TypeName`).
    pub type_name: String,
    /// The type of each attribute value
    /// (hcldec: `BlockAttrsSpec.ElementType`).
    pub element_type: Type,
    /// Whether omitting the block is an error
    /// (hcldec: `BlockAttrsSpec.Required`).
    pub required: bool,
}

impl Spec for BlockAttrsSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Produces the value of one of the current block's labels
/// (hcldec: `hcldec.BlockLabelSpec`).
#[derive(Debug, Clone)]
pub struct BlockLabelSpec {
    /// The label index, 0-based (hcldec: `BlockLabelSpec.Index`).
    pub index: usize,
    /// The label's name, for diagnostics
    /// (hcldec: `BlockLabelSpec.Name`).
    pub name: String,
}

impl Spec for BlockLabelSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Evaluates a primary spec, falling back to a default spec when the
/// primary produces a null value (hcldec: `hcldec.DefaultSpec`).
#[derive(Debug, Clone)]
pub struct DefaultSpec {
    /// The primary spec (hcldec: `DefaultSpec.Primary`).
    pub primary: SpecRef,
    /// The fallback spec (hcldec: `DefaultSpec.Default`).
    pub default: SpecRef,
}

impl Spec for DefaultSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Passes a wrapped spec's result through a transform expression, in which
/// the result appears as a variable (hcldec: `hcldec.TransformExprSpec`).
#[derive(Debug, Clone)]
pub struct TransformExprSpec {
    /// The wrapped spec (hcldec: `TransformExprSpec.Wrapped`).
    pub wrapped: SpecRef,
    /// The transform expression (hcldec: `TransformExprSpec.Expr`).
    pub expr: ExprRef,
    /// The context to evaluate the transform in
    /// (hcldec: `TransformExprSpec.TransformCtx`).
    pub transform_ctx: Arc<EvalContext>,
    /// The variable name the wrapped result is bound to
    /// (hcldec: `TransformExprSpec.VarName`).
    pub var_name: String,
}

impl Spec for TransformExprSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Passes a wrapped spec's result through a function
/// (hcldec: `hcldec.TransformFuncSpec`).
#[derive(Debug)]
pub struct TransformFuncSpec {
    /// The wrapped spec (hcldec: `TransformFuncSpec.Wrapped`).
    pub wrapped: SpecRef,
    /// The single-argument transform function
    /// (hcldec: `TransformFuncSpec.Func`).
    pub func: Function,
}

impl Spec for TransformFuncSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The refinement callback of a [`RefineValueSpec`]
/// (hcldec: the `RefineValueSpec.Refine` field's type).
pub type RefineFunc = Box<dyn Fn(RefinementBuilder) -> RefinementBuilder>;

/// Refines the wrapped spec's result with unknown-value refinements
/// (hcldec: `hcldec.RefineValueSpec`).
pub struct RefineValueSpec {
    /// The wrapped spec (hcldec: `RefineValueSpec.Wrapped`).
    pub wrapped: SpecRef,
    /// The refinement callback (hcldec: `RefineValueSpec.Refine`).
    pub refine: RefineFunc,
}

impl fmt::Debug for RefineValueSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefineValueSpec")
            .field("wrapped", &self.wrapped)
            .finish_non_exhaustive()
    }
}

impl Spec for RefineValueSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The validation callback of a [`ValidateSpec`]
/// (hcldec: the `ValidateSpec.Func` field's type).
pub type ValidateFunc = Box<dyn Fn(&Value) -> Diagnostics>;

/// Validates the wrapped spec's result with a custom callback
/// (hcldec: `hcldec.ValidateSpec`).
pub struct ValidateSpec {
    /// The wrapped spec (hcldec: `ValidateSpec.Wrapped`).
    pub wrapped: SpecRef,
    /// The validation callback (hcldec: `ValidateSpec.Func`).
    pub func: ValidateFunc,
}

impl fmt::Debug for ValidateSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidateSpec")
            .field("wrapped", &self.wrapped)
            .finish_non_exhaustive()
    }
}

impl Spec for ValidateSpec {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
