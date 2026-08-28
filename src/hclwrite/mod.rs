//! Round-trip-preserving construction and rewriting of native-syntax
//! configuration (hcl v2: the `hclwrite` package).
//!
//! Go mutates the write-AST through shared pointers; the Rust node types
//! ([`File`], [`Body`], [`Block`], [`Attribute`], [`Expression`]) are
//! cloneable *handles* to shared mutable nodes, so `&self` methods mutate
//! the underlying node just as the Go pointer methods do.

pub mod generate;

use std::ops::Deref;

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::hclsyntax;
use crate::pos::{Pos, Range};
use crate::traversal::Traversal as HclTraversal;

pub use generate::{
    ObjectAttrTokens, tokens_for_function_call, tokens_for_identifier, tokens_for_object,
    tokens_for_traversal, tokens_for_tuple, tokens_for_value,
};

/// One write-token: a token type, its bytes, and the number of spaces
/// before it (hclwrite: `hclwrite.Token`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Token {
    /// The token's kind (hclwrite: `Token.Type`).
    pub ty: hclsyntax::TokenType,
    /// The token's bytes (hclwrite: `Token.Bytes`).
    pub bytes: Vec<u8>,
    /// How many spaces the formatter places before this token
    /// (hclwrite: `Token.SpacesBefore`).
    pub spaces_before: usize,
}

/// A sequence of write-tokens (hclwrite: `hclwrite.Tokens`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
    /// The bytes of all tokens, with their leading spaces
    /// (hclwrite: `Tokens.Bytes`).
    pub fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl Deref for Tokens {
    type Target = Vec<Token>;

    fn deref(&self) -> &Vec<Token> {
        &self.0
    }
}

impl From<Vec<Token>> for Tokens {
    fn from(v: Vec<Token>) -> Tokens {
        Tokens(v)
    }
}

impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Token> for Tokens {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Tokens {
        Tokens(iter.into_iter().collect())
    }
}

/// A parsed or constructed file, ready for mutation and re-rendering
/// (hclwrite: `hclwrite.File`).
#[derive(Debug, Clone)]
pub struct File {
    _priv: (),
}

impl File {
    /// An empty file, ready to be built up (hclwrite: `hclwrite.NewFile` /
    /// `hclwrite.NewEmptyFile`).
    pub fn new() -> File {
        todo!()
    }

    /// The file's root body (hclwrite: `File.Body`).
    pub fn body(&self) -> Body {
        todo!()
    }

    /// The file rendered back to source bytes (hclwrite: `File.Bytes`).
    pub fn bytes(&self) -> Vec<u8> {
        todo!()
    }

    /// Writes the rendered file to the given writer
    /// (hclwrite: `File.WriteTo`).
    pub fn write_to(&self, wr: &mut dyn std::io::Write) -> std::io::Result<u64> {
        todo!()
    }

    /// The file's tokens, for structural assertions (hclwrite: the
    /// `BuildTokens` mechanism; exposed for conformance).
    pub fn build_tokens(&self) -> Tokens {
        todo!()
    }
}

impl Default for File {
    fn default() -> File {
        File::new()
    }
}

/// A body within a write-AST: a shared, mutable handle
/// (hclwrite: `hclwrite.Body`).
#[derive(Debug, Clone)]
pub struct Body {
    _priv: (),
}

impl Body {
    /// Removes all items from the body (hclwrite: `Body.Clear`).
    pub fn clear(&self) {
        todo!()
    }

    /// Appends raw tokens the writer will not interpret
    /// (hclwrite: `Body.AppendUnstructuredTokens`).
    pub fn append_unstructured_tokens(&self, ts: Tokens) {
        todo!()
    }

    /// The body's attributes by name (hclwrite: `Body.Attributes`).
    pub fn attributes(&self) -> std::collections::HashMap<String, Attribute> {
        todo!()
    }

    /// The body's blocks in order (hclwrite: `Body.Blocks`).
    pub fn blocks(&self) -> Vec<Block> {
        todo!()
    }

    /// The attribute with the given name, if present
    /// (hclwrite: `Body.GetAttribute`; `nil` ⇒ `None`).
    pub fn get_attribute(&self, name: &str) -> Option<Attribute> {
        todo!()
    }

    /// Renames the given attribute, returning whether it was found and no
    /// clash existed (hclwrite: `Body.RenameAttribute`).
    pub fn rename_attribute(&self, from_name: &str, to_name: &str) -> bool {
        todo!()
    }

    /// The first block matching the type name and labels, if any
    /// (hclwrite: `Body.FirstMatchingBlock`; `nil` ⇒ `None`).
    pub fn first_matching_block(&self, type_name: &str, labels: &[&str]) -> Option<Block> {
        todo!()
    }

    /// Removes the given block, returning whether it was found
    /// (hclwrite: `Body.RemoveBlock`).
    pub fn remove_block(&self, block: &Block) -> bool {
        todo!()
    }

    /// Sets an attribute to raw, uninterpreted expression tokens, returning
    /// the attribute (hclwrite: `Body.SetAttributeRaw`).
    pub fn set_attribute_raw(&self, name: &str, tokens: Tokens) -> Attribute {
        todo!()
    }

    /// Sets an attribute to a literal value, returning the attribute
    /// (hclwrite: `Body.SetAttributeValue`).
    pub fn set_attribute_value(&self, name: &str, val: Value) -> Attribute {
        todo!()
    }

    /// Sets an attribute to a traversal expression, returning the attribute
    /// (hclwrite: `Body.SetAttributeTraversal`).
    pub fn set_attribute_traversal(&self, name: &str, traversal: HclTraversal) -> Attribute {
        todo!()
    }

    /// Removes the attribute with the given name, returning it if present
    /// (hclwrite: `Body.RemoveAttribute`; `nil` ⇒ `None`).
    pub fn remove_attribute(&self, name: &str) -> Option<Attribute> {
        todo!()
    }

    /// Appends an existing block to the end of the body
    /// (hclwrite: `Body.AppendBlock`).
    pub fn append_block(&self, block: Block) -> Block {
        todo!()
    }

    /// Appends a new empty block with the given type and labels
    /// (hclwrite: `Body.AppendNewBlock`).
    pub fn append_new_block(&self, type_name: &str, labels: &[&str]) -> Block {
        todo!()
    }

    /// Appends a blank line (hclwrite: `Body.AppendNewline`).
    pub fn append_newline(&self) {
        todo!()
    }

    /// The body's tokens, for structural assertions (hclwrite: the
    /// `BuildTokens` mechanism; exposed for conformance).
    pub fn build_tokens(&self) -> Tokens {
        todo!()
    }
}

/// An attribute within a write-AST: a shared, mutable handle
/// (hclwrite: `hclwrite.Attribute`).
#[derive(Debug, Clone)]
pub struct Attribute {
    _priv: (),
}

impl Attribute {
    /// The comment tokens before the attribute
    /// (hclwrite: `Attribute.LeadComments`).
    pub fn lead_comments(&self) -> Tokens {
        todo!()
    }

    /// The comment tokens after the attribute's expression, before the
    /// newline (hclwrite: `Attribute.LineComments`).
    pub fn line_comments(&self) -> Tokens {
        todo!()
    }

    /// The attribute's value expression (hclwrite: `Attribute.Expr`).
    pub fn expr(&self) -> Expression {
        todo!()
    }

    /// The attribute's tokens, for structural assertions (hclwrite: the
    /// `BuildTokens` mechanism; exposed for conformance).
    pub fn build_tokens(&self) -> Tokens {
        todo!()
    }
}

/// A block within a write-AST: a shared, mutable handle
/// (hclwrite: `hclwrite.Block`).
#[derive(Debug, Clone)]
pub struct Block {
    _priv: (),
}

impl Block {
    /// A new detached block with the given type and labels
    /// (hclwrite: `hclwrite.NewBlock`).
    pub fn new(type_name: &str, labels: &[&str]) -> Block {
        todo!()
    }

    /// The block's body (hclwrite: `Block.Body`).
    pub fn body(&self) -> Body {
        todo!()
    }

    /// The comment tokens before the block
    /// (hclwrite: `Block.LeadComments`).
    pub fn lead_comments(&self) -> Tokens {
        todo!()
    }

    /// The block's type name (hclwrite: `Block.Type`).
    pub fn block_type(&self) -> String {
        todo!()
    }

    /// Changes the block's type name (hclwrite: `Block.SetType`).
    pub fn set_type(&self, type_name: &str) {
        todo!()
    }

    /// The block's labels in order (hclwrite: `Block.Labels`).
    pub fn labels(&self) -> Vec<String> {
        todo!()
    }

    /// Replaces the block's labels (hclwrite: `Block.SetLabels`).
    pub fn set_labels(&self, labels: &[&str]) {
        todo!()
    }

    /// The block's tokens, for structural assertions (hclwrite: the
    /// `BuildTokens` mechanism; exposed for conformance).
    pub fn build_tokens(&self) -> Tokens {
        todo!()
    }
}

/// An expression within a write-AST: a shared, mutable handle
/// (hclwrite: `hclwrite.Expression`).
#[derive(Debug, Clone)]
pub struct Expression {
    _priv: (),
}

impl Expression {
    /// A new expression from raw, uninterpreted tokens
    /// (hclwrite: `hclwrite.NewExpressionRaw`).
    pub fn new_raw(tokens: Tokens) -> Expression {
        todo!()
    }

    /// A new expression rendering a literal value
    /// (hclwrite: `hclwrite.NewExpressionLiteral`).
    pub fn new_literal(val: Value) -> Expression {
        todo!()
    }

    /// A new expression rendering an absolute traversal
    /// (hclwrite: `hclwrite.NewExpressionAbsTraversal`).
    pub fn new_abs_traversal(traversal: HclTraversal) -> Expression {
        todo!()
    }

    /// The traversals referenced by the expression
    /// (hclwrite: `Expression.Variables`).
    pub fn variables(&self) -> Vec<TraversalHandle> {
        todo!()
    }

    /// Renames all traversals whose leading names match `search` to use
    /// `replacement` (hclwrite: `Expression.RenameVariablePrefix`).
    pub fn rename_variable_prefix(&self, search: &[&str], replacement: &[&str]) {
        todo!()
    }

    /// The expression's tokens, for structural assertions (hclwrite: the
    /// `BuildTokens` mechanism; exposed for conformance).
    pub fn build_tokens(&self) -> Tokens {
        todo!()
    }
}

/// A traversal within a write-AST: a shared handle
/// (hclwrite: `hclwrite.Traversal`; named to avoid clashing with
/// `hcl::Traversal`).
#[derive(Debug, Clone)]
pub struct TraversalHandle {
    _priv: (),
}

/// Parses the source as native syntax and constructs a writable AST for it
/// (hclwrite: `hclwrite.ParseConfig`).
pub fn parse_config(src: &[u8], filename: &str, start: Pos) -> (File, Diagnostics) {
    todo!()
}

/// Rewrites the source into a canonical layout without changing meaning;
/// invalid input is returned verbatim (hclwrite: `hclwrite.Format`).
pub fn format(src: &[u8]) -> Vec<u8> {
    todo!()
}

/// Adjusts the spacing (`spaces_before`) of the given tokens in place, as
/// the formatter does (hclwrite: unexported `format`; exposed for
/// conformance).
pub fn format_tokens(tokens: &mut Tokens) {
    todo!()
}

/// One line of tokens as partitioned by the formatter, split into its
/// alignment cells (hclwrite: unexported `formatLine`; exposed for
/// conformance).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FormatLine {
    /// The tokens before any assignment or trailing comment
    /// (hclwrite: `formatLine.lead`).
    pub lead: Tokens,
    /// The tokens from the equals sign on, for attribute lines
    /// (hclwrite: `formatLine.assign`).
    pub assign: Tokens,
    /// The trailing single-line comment, if any
    /// (hclwrite: `formatLine.comment`).
    pub comment: Tokens,
}

/// Partitions tokens into the formatter's per-line alignment cells
/// (hclwrite: unexported `linesForFormat`; exposed for conformance).
pub fn lines_for_format(tokens: Tokens) -> Vec<FormatLine> {
    todo!()
}

/// The half-open range of native tokens within the given source range
/// (hclwrite: unexported `partitionTokens`; exposed for conformance).
pub fn partition_tokens(toks: &[hclsyntax::Token], rng: Range) -> (usize, usize) {
    todo!()
}

/// The index splitting leading comment tokens from the rest
/// (hclwrite: unexported `partitionLeadCommentTokens`; exposed for
/// conformance).
pub fn partition_lead_comment_tokens(toks: &[hclsyntax::Token]) -> usize {
    todo!()
}

/// Scans source into write-tokens, with spaces attached
/// (hclwrite: unexported `lexConfig`; exposed for conformance).
pub fn lex_config(src: &[u8]) -> Tokens {
    todo!()
}

/// One node of the structural test tree used by the upstream parser tests
/// (hclwrite: the `TestTreeNode` helper in `ast_test.go`; part of the
/// conformance surface so the ported tests can assert tree shape).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TestTreeNode {
    /// The node's type name, e.g. `Body`, `Attribute`, `identifier`,
    /// `Tokens` (hclwrite: `TestTreeNode.Type`).
    pub node_type: String,
    /// The node's source text, for leaf nodes
    /// (hclwrite: `TestTreeNode.Val`).
    pub val: String,
    /// The node's children in order (hclwrite: `TestTreeNode.Children`).
    pub children: Vec<TestTreeNode>,
}

/// Builds the structural test tree for a parsed file's body
/// (hclwrite: the `makeTestTree` helper in `ast_test.go`; exposed for
/// conformance).
pub fn make_test_tree(file: &File) -> TestTreeNode {
    todo!()
}
