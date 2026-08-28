//! The raw JSON AST (hcl v2: `json/ast.go`).
//!
//! Go keeps these node types unexported; they are public here because the
//! upstream parser tests compare against them directly. Go's `*objectVal` →
//! `Node::Object { .. }`, etc.

use cty::Value;

use crate::pos::Range;

/// One node of raw JSON structure (json: the unexported `node` interface
/// over `objectVal`, `arrayVal`, `booleanVal`, `numberVal`, `stringVal`,
/// `nullVal`, and `invalidVal`).
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// A JSON object (json: `*objectVal`).
    Object {
        /// The object's attributes, in source order including duplicates
        /// (json: `objectVal.Attrs`).
        attrs: Vec<ObjectAttr>,
        /// The range of the whole object (json: `objectVal.SrcRange`).
        src_range: Range,
        /// The range of the opening brace (json: `objectVal.OpenRange`).
        open_range: Range,
        /// The range of the closing brace (json: `objectVal.CloseRange`).
        close_range: Range,
    },
    /// A JSON array (json: `*arrayVal`).
    Array {
        /// The element values (json: `arrayVal.Values`).
        values: Vec<Node>,
        /// The range of the whole array (json: `arrayVal.SrcRange`).
        src_range: Range,
        /// The range of the opening bracket (json: `arrayVal.OpenRange`).
        open_range: Range,
    },
    /// A JSON boolean (json: `*booleanVal`).
    Boolean {
        /// The value (json: `booleanVal.Value`).
        value: bool,
        /// The source range (json: `booleanVal.SrcRange`).
        src_range: Range,
    },
    /// A JSON number (json: `*numberVal`).
    Number {
        /// The parsed value (json: `numberVal.Value`, a `*big.Float`; here a
        /// `cty` number value).
        value: Value,
        /// The source range (json: `numberVal.SrcRange`).
        src_range: Range,
    },
    /// A JSON string (json: `*stringVal`).
    String {
        /// The decoded string value (json: `stringVal.Value`).
        value: String,
        /// The source range (json: `stringVal.SrcRange`).
        src_range: Range,
    },
    /// A JSON null (json: `*nullVal`).
    Null {
        /// The source range (json: `nullVal.SrcRange`).
        src_range: Range,
    },
    /// A placeholder for invalid input (json: `invalidVal`).
    Invalid {
        /// The source range (json: `invalidVal.SrcRange`).
        src_range: Range,
    },
}

impl Node {
    /// The node's full source range (json: `node.Range`).
    pub fn range(&self) -> Range {
        todo!()
    }

    /// A shorter range at the start of the node (json: `node.StartRange`).
    pub fn start_range(&self) -> Range {
        todo!()
    }
}

/// One attribute of a JSON object node (json: unexported `objectAttr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectAttr {
    /// The attribute name (json: `objectAttr.Name`).
    pub name: String,
    /// The attribute value (json: `objectAttr.Value`).
    pub value: Node,
    /// The range of the name string (json: `objectAttr.NameRange`).
    pub name_range: Range,
}
