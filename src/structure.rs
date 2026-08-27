//! The information model: files, bodies, blocks, attributes, and
//! expressions (hcl v2: `structure.go`, `structure_at_pos.go`, `file.go`
//! parts of the root package).

use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use cty::{Value, ValueMarks};

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::expr_helpers::{KeyValuePair, StaticCall};
use crate::pos::{Pos, Range};
use crate::schema::BodySchema;
use crate::traversal::Traversal;

/// The top-level node of a parsed file (hcl: `hcl.File`).
#[derive(Debug, Clone)]
pub struct File {
    /// The file's content (hcl: `File.Body`).
    pub body: BodyRef,
    /// The raw source bytes (hcl: `File.Bytes`).
    pub bytes: Vec<u8>,
    /// Editor-integration hook used by `hcled` and diagnostic formatters
    /// (hcl: `File.Nav`; `nil` ⇒ `None`).
    pub nav: Option<Arc<dyn FileNav>>,
}

impl File {
    /// The blocks whose ranges contain the given position, outermost first
    /// (hcl: `File.BlocksAtPos`).
    pub fn blocks_at_pos(&self, pos: Pos) -> Vec<Block> {
        todo!()
    }

    /// The outermost block containing the given position, if any
    /// (hcl: `File.OutermostBlockAtPos`).
    pub fn outermost_block_at_pos(&self, pos: Pos) -> Option<Block> {
        todo!()
    }

    /// The innermost block containing the given position, if any
    /// (hcl: `File.InnermostBlockAtPos`).
    pub fn innermost_block_at_pos(&self, pos: Pos) -> Option<Block> {
        todo!()
    }

    /// The outermost expression containing the given position, if any
    /// (hcl: `File.OutermostExprAtPos`).
    pub fn outermost_expr_at_pos(&self, pos: Pos) -> Option<ExprRef> {
        todo!()
    }

    /// The attribute whose range contains the given position, if any
    /// (hcl: `File.AttributeAtPos`).
    pub fn attribute_at_pos(&self, pos: Pos) -> Option<Attribute> {
        todo!()
    }
}

/// Editor-navigation support attached to a [`File`] (hcl: the `File.Nav`
/// contract consumed by `hcled` — the unexported `contextStringer` and
/// `contextDefRanger` optional interfaces).
pub trait FileNav: fmt::Debug {
    /// A human-readable description of the context at the given byte offset
    /// (hcled: `contextStringer.ContextString`).
    fn context_string(&self, offset: usize) -> String;

    /// The definition range of the context at the given byte offset, if the
    /// implementation supports it (hcled: `contextDefRanger.ContextDefRange`;
    /// `None` means unsupported).
    fn context_def_range(&self, offset: usize) -> Option<Range> {
        None
    }
}

/// A nested configuration block (hcl: `hcl.Block`).
#[derive(Debug, Clone)]
pub struct Block {
    /// The block type name (hcl: `Block.Type`).
    pub block_type: String,
    /// The block's labels, in order (hcl: `Block.Labels`).
    pub labels: Vec<String>,
    /// The block's content (hcl: `Block.Body`).
    pub body: BodyRef,
    /// Range for seeking to the block's definition in an editor
    /// (hcl: `Block.DefRange`).
    pub def_range: Range,
    /// Range of the block type declaration specifically
    /// (hcl: `Block.TypeRange`).
    pub type_range: Range,
    /// Ranges of the label values specifically (hcl: `Block.LabelRanges`).
    pub label_ranges: Vec<Range>,
}

/// Deep equality, matching what upstream tests compare with
/// `reflect.DeepEqual`; bodies compare via [`Body::eq_dyn`].
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// A sequence of [`Block`]s (hcl: `hcl.Blocks`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Blocks(pub Vec<Block>);

impl Blocks {
    /// Only the blocks of the given type name, preserving order
    /// (hcl: `Blocks.OfType`).
    pub fn of_type(&self, type_name: &str) -> Blocks {
        todo!()
    }

    /// The blocks grouped by their type name (hcl: `Blocks.ByType`).
    pub fn by_type(&self) -> HashMap<String, Blocks> {
        todo!()
    }
}

impl Deref for Blocks {
    type Target = Vec<Block>;

    fn deref(&self) -> &Vec<Block> {
        &self.0
    }
}

impl From<Vec<Block>> for Blocks {
    fn from(v: Vec<Block>) -> Blocks {
        Blocks(v)
    }
}

impl IntoIterator for Blocks {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Attributes by name (hcl: `hcl.Attributes`, a `map[string]*Attribute`).
pub type Attributes = HashMap<String, Attribute>;

/// A name–expression pair within a body (hcl: `hcl.Attribute`).
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The attribute name (hcl: `Attribute.Name`).
    pub name: String,
    /// The value expression (hcl: `Attribute.Expr`).
    pub expr: ExprRef,
    /// The range of the whole attribute definition (hcl: `Attribute.Range`).
    pub range: Range,
    /// The range of the attribute name (hcl: `Attribute.NameRange`).
    pub name_range: Range,
}

/// Deep equality, matching what upstream tests compare with
/// `reflect.DeepEqual`; expressions compare via [`Expression::eq_dyn`].
impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// The result of applying a [`BodySchema`] to a [`Body`]
/// (hcl: `hcl.BodyContent`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BodyContent {
    /// The attributes requested by the schema (hcl:
    /// `BodyContent.Attributes`).
    pub attributes: Attributes,
    /// The blocks requested by the schema (hcl: `BodyContent.Blocks`).
    pub blocks: Blocks,
    /// Where a missing item would be reported (hcl:
    /// `BodyContent.MissingItemRange`).
    pub missing_item_range: Range,
}

/// A container for definitions in the language: attributes and blocks
/// (hcl: `hcl.Body`).
///
/// The `unknown`/`body_value_marks` methods mirror Go's optional
/// `hcldec.UnknownBody` / `hcldec.MarkedBody` interface assertions; the
/// defaults report "not implemented by this body".
pub trait Body: Any + fmt::Debug {
    /// The content of the body, requiring that the schema is matched
    /// exhaustively (hcl: `Body.Content`).
    fn content(&self, schema: &BodySchema) -> (BodyContent, Diagnostics);

    /// The content matched by the schema plus a "remainder" body holding
    /// everything else (hcl: `Body.PartialContent`).
    fn partial_content(&self, schema: &BodySchema) -> (BodyContent, BodyRef, Diagnostics);

    /// All attributes, for bodies in "attribute syntax only" position
    /// (hcl: `Body.JustAttributes`).
    fn just_attributes(&self) -> (Attributes, Diagnostics);

    /// The range to report when a required item is missing
    /// (hcl: `Body.MissingItemRange`).
    fn missing_item_range(&self) -> Range;

    /// This body as [`Any`], for downcasting (Go: implicit via interface
    /// type assertions).
    fn as_any(&self) -> &dyn Any;

    /// Deep equality against another body, used by conformance asserts where
    /// upstream compared with `reflect.DeepEqual` (Go: no direct analogue).
    fn eq_dyn(&self, other: &dyn Body) -> bool {
        todo!()
    }

    /// Whether the body's content is wholly unknown, for bodies that track
    /// that (hcldec: `UnknownBody.Unknown`; `None` means the body does not
    /// implement the extension).
    fn unknown(&self) -> Option<bool> {
        None
    }

    /// Marks to apply to values decoded from this body, for bodies that
    /// track them (hcldec: `MarkedBody.BodyValueMarks`; `None` means the
    /// body does not implement the extension).
    fn body_value_marks(&self) -> Option<ValueMarks> {
        None
    }
}

/// A shared handle to a [`Body`] (Go: the `hcl.Body` interface value).
#[derive(Debug, Clone)]
pub struct BodyRef(pub Arc<dyn Body>);

impl BodyRef {
    /// Wraps a concrete body in a shared handle.
    pub fn new(body: impl Body) -> BodyRef {
        BodyRef(Arc::new(body))
    }
}

impl Deref for BodyRef {
    type Target = dyn Body;

    fn deref(&self) -> &(dyn Body + 'static) {
        &*self.0
    }
}

/// Deep equality via [`Body::eq_dyn`].
impl PartialEq for BodyRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_dyn(&*other.0)
    }
}

/// A single expression to be evaluated (hcl: `hcl.Expression`).
///
/// The `as_traversal`/`expr_call`/`expr_list`/`expr_map`/
/// `unwrap_expression` methods mirror Go's optional interface assertions
/// used by `hcl.AbsTraversalForExpr`, `hcl.ExprCall`, `hcl.ExprList`,
/// `hcl.ExprMap`, and `hcl.UnwrapExpression`; the defaults report "not
/// supported by this expression".
pub trait Expression: Any + fmt::Debug {
    /// The value of the expression in the given context; `None` mirrors a
    /// nil `*hcl.EvalContext` (hcl: `Expression.Value`).
    fn value(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics);

    /// The variables needed to evaluate the expression
    /// (hcl: `Expression.Variables`).
    fn variables(&self) -> Vec<Traversal>;

    /// The range of the whole expression (hcl: `Expression.Range`).
    fn range(&self) -> Range;

    /// A shorter range at the start of the expression, for compact
    /// diagnostics (hcl: `Expression.StartRange`).
    fn start_range(&self) -> Range;

    /// This expression as [`Any`], for downcasting (Go: implicit via
    /// interface type assertions).
    fn as_any(&self) -> &dyn Any;

    /// Deep equality against another expression, used by conformance
    /// asserts where upstream compared with `reflect.DeepEqual` (Go: no
    /// direct analogue).
    fn eq_dyn(&self, other: &dyn Expression) -> bool {
        todo!()
    }

    /// The expression as an absolute traversal, if it is one
    /// (hclsyntax: `Expression.AsTraversal`; `nil` ⇒ `None`).
    fn as_traversal(&self) -> Option<Traversal> {
        None
    }

    /// The expression as a static function call, if it is one
    /// (hcl: the `exprCall` optional interface behind `hcl.ExprCall`).
    fn expr_call(&self) -> Option<StaticCall> {
        None
    }

    /// The expression as a static list of expressions, if it is one
    /// (hcl: the `exprList` optional interface behind `hcl.ExprList`).
    fn expr_list(&self) -> Option<Vec<ExprRef>> {
        None
    }

    /// The expression as static key–value pairs, if it is one
    /// (hcl: the `exprMap` optional interface behind `hcl.ExprMap`).
    fn expr_map(&self) -> Option<Vec<KeyValuePair>> {
        None
    }

    /// The wrapped expression, for decorator expressions
    /// (hcl: the `unwrapExpression` optional interface behind
    /// `hcl.UnwrapExpression`).
    fn unwrap_expression(&self) -> Option<ExprRef> {
        None
    }
}

/// A shared handle to an [`Expression`] (Go: the `hcl.Expression` interface
/// value).
#[derive(Debug, Clone)]
pub struct ExprRef(pub Arc<dyn Expression>);

impl ExprRef {
    /// Wraps a concrete expression in a shared handle.
    pub fn new(expr: impl Expression) -> ExprRef {
        ExprRef(Arc::new(expr))
    }
}

impl Deref for ExprRef {
    type Target = dyn Expression;

    fn deref(&self) -> &(dyn Expression + 'static) {
        &*self.0
    }
}

/// Deep equality via [`Expression::eq_dyn`].
impl PartialEq for ExprRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_dyn(&*other.0)
    }
}
