//! The native-syntax body AST (hcl v2: `hclsyntax/structure.go`,
//! `hclsyntax/file.go`).

use std::any::Any;
use std::collections::HashMap;

use crate::diagnostic::Diagnostics;
use crate::hclsyntax::expression::Expression;
use crate::pos::{Pos, Range};
use crate::schema::BodySchema;
use crate::structure::{self as hcl_structure, BodyContent, BodyRef, ExprRef};

/// A top-level parsed native-syntax file
/// (hclsyntax: `hclsyntax.File`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct File {
    /// The file's body (hclsyntax: `File.Body`).
    pub body: Body,
    /// The raw source bytes (hclsyntax: `File.Bytes`).
    pub bytes: Vec<u8>,
}

impl File {
    /// The file wrapped in the syntax-agnostic `hcl::File` type
    /// (hclsyntax: `File.AsHCLFile`).
    pub fn as_hcl_file(&self) -> hcl_structure::File {
        todo!()
    }
}

/// Attributes by name (hclsyntax: `hclsyntax.Attributes`).
pub type Attributes = HashMap<String, Attribute>;

/// A sequence of blocks (hclsyntax: `hclsyntax.Blocks`).
pub type Blocks = Vec<Block>;

/// The content of a native-syntax file or block
/// (hclsyntax: `hclsyntax.Body`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Body {
    /// The body's attributes (hclsyntax: `Body.Attributes`).
    pub attributes: Attributes,
    /// The body's blocks (hclsyntax: `Body.Blocks`).
    pub blocks: Blocks,
    /// The range of the whole body (hclsyntax: `Body.SrcRange`).
    pub src_range: Range,
    /// The zero-length range at the final token of the body
    /// (hclsyntax: `Body.EndRange`).
    pub end_range: Range,
}

impl Body {
    /// The blocks whose ranges contain the given position, outermost first,
    /// as syntax-agnostic blocks (hclsyntax: `Body.BlocksAtPos`).
    pub fn blocks_at_pos(&self, pos: Pos) -> Vec<hcl_structure::Block> {
        todo!()
    }

    /// The innermost block containing the given position, if any
    /// (hclsyntax: `Body.InnermostBlockAtPos`).
    pub fn innermost_block_at_pos(&self, pos: Pos) -> Option<hcl_structure::Block> {
        todo!()
    }

    /// The outermost block containing the given position, if any
    /// (hclsyntax: `Body.OutermostBlockAtPos`).
    pub fn outermost_block_at_pos(&self, pos: Pos) -> Option<hcl_structure::Block> {
        todo!()
    }

    /// The attribute whose range contains the given position, if any
    /// (hclsyntax: `Body.AttributeAtPos`).
    pub fn attribute_at_pos(&self, pos: Pos) -> Option<hcl_structure::Attribute> {
        todo!()
    }

    /// The outermost expression containing the given position, if any
    /// (hclsyntax: `Body.OutermostExprAtPos`).
    pub fn outermost_expr_at_pos(&self, pos: Pos) -> Option<ExprRef> {
        todo!()
    }
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

/// A name–expression pair in a native-syntax body
/// (hclsyntax: `hclsyntax.Attribute`).
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// The attribute name (hclsyntax: `Attribute.Name`).
    pub name: String,
    /// The value expression (hclsyntax: `Attribute.Expr`).
    pub expr: Expression,
    /// The range of the whole attribute (hclsyntax: `Attribute.SrcRange`).
    pub src_range: Range,
    /// The range of the name (hclsyntax: `Attribute.NameRange`).
    pub name_range: Range,
    /// The range of the equals sign (hclsyntax: `Attribute.EqualsRange`).
    pub equals_range: Range,
}

impl Attribute {
    /// The attribute as the syntax-agnostic `hcl::Attribute`
    /// (hclsyntax: `Attribute.AsHCLAttribute`).
    pub fn as_hcl_attribute(&self) -> hcl_structure::Attribute {
        todo!()
    }
}

/// A nested block in a native-syntax body (hclsyntax: `hclsyntax.Block`).
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The block type name (hclsyntax: `Block.Type`).
    pub block_type: String,
    /// The block labels, in order (hclsyntax: `Block.Labels`).
    pub labels: Vec<String>,
    /// The block's body (hclsyntax: `Block.Body`).
    pub body: Body,
    /// The range of the type name (hclsyntax: `Block.TypeRange`).
    pub type_range: Range,
    /// The ranges of the labels (hclsyntax: `Block.LabelRanges`).
    pub label_ranges: Vec<Range>,
    /// The range of the opening brace
    /// (hclsyntax: `Block.OpenBraceRange`).
    pub open_brace_range: Range,
    /// The range of the closing brace
    /// (hclsyntax: `Block.CloseBraceRange`).
    pub close_brace_range: Range,
}

impl Block {
    /// The block as the syntax-agnostic `hcl::Block`
    /// (hclsyntax: `Block.AsHCLBlock`).
    pub fn as_hcl_block(&self) -> hcl_structure::Block {
        todo!()
    }

    /// The block's "definition" range for editor navigation
    /// (hclsyntax: `Block.DefRange`).
    pub fn def_range(&self) -> Range {
        todo!()
    }
}
