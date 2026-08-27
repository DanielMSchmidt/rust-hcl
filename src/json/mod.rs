//! The JSON syntax for HCL (hcl v2: the `json` package).

pub mod ast;
pub mod scanner;
pub mod structure;

use crate::diagnostic::Diagnostics;
use crate::pos::Pos;
use crate::structure::{Body as HclBody, ExprRef, Expression as HclExpression, File};

pub use ast::{Node, ObjectAttr};
pub use scanner::{ScannerPos, Token, TokenType, scan};
pub use structure::{Body, Expression};

/// Parses the given JSON buffer as an HCL file
/// (json: `json.Parse`).
pub fn parse(src: &[u8], filename: &str) -> (File, Diagnostics) {
    todo!()
}

/// Parses the given JSON buffer with its first byte at the given start
/// position (json: `json.ParseWithStartPos`).
pub fn parse_with_start_pos(src: &[u8], filename: &str, start: Pos) -> (File, Diagnostics) {
    todo!()
}

/// Parses the given JSON buffer as a standalone HCL expression
/// (json: `json.ParseExpression`).
pub fn parse_expression(src: &[u8], filename: &str) -> (ExprRef, Diagnostics) {
    todo!()
}

/// Parses the given JSON buffer as an expression, with its first byte at
/// the given start position (json: `json.ParseExpressionWithStartPos`).
pub fn parse_expression_with_start_pos(
    src: &[u8],
    filename: &str,
    start: Pos,
) -> (ExprRef, Diagnostics) {
    todo!()
}

/// Reads and parses the given file as JSON-syntax HCL
/// (json: `json.ParseFile`).
pub fn parse_file(filename: &str) -> (File, Diagnostics) {
    todo!()
}

/// Whether the given expression was produced by this JSON syntax
/// (json: `json.IsJSONExpression`).
pub fn is_json_expression(maybe_json_expr: &dyn HclExpression) -> bool {
    todo!()
}

/// Whether the given body was produced by this JSON syntax
/// (json: `json.IsJSONBody`).
pub fn is_json_body(maybe_json_body: &dyn HclBody) -> bool {
    todo!()
}

/// Parses a whole buffer as a JSON file's content, returning the raw JSON
/// AST (json: unexported `parseFileContent`; exposed for conformance).
pub fn parse_file_content(buf: &[u8], filename: &str, start: Pos) -> (Node, Diagnostics) {
    todo!()
}

/// Parses a buffer as a single JSON value, returning the raw JSON AST
/// (json: unexported `parseExpression`; exposed for conformance as the raw
/// variant of [`parse_expression`]).
pub fn parse_value(buf: &[u8], filename: &str, start: Pos) -> (Node, Diagnostics) {
    todo!()
}

/// Suggests a JSON keyword close to `given`, or the empty string
/// (json: unexported `keywordSuggestion`; exposed for conformance).
pub fn keyword_suggestion(given: &str) -> String {
    todo!()
}
