//! The HCL native syntax: scanner, parser, and expression evaluation
//! (hcl v2: the `hclsyntax` package).

pub mod expression;
pub mod structure;
pub mod token;
pub mod walk;

use crate::diagnostic::Diagnostics;
use crate::pos::{Pos, Range};
use crate::structure::File;
use crate::traversal::Traversal;

pub use expression::{
    AnonSymbolExpr, BinaryOpExpr, ConditionalExpr, ExprSyntaxError, Expression, ForExpr,
    FunctionCallDiagExtra, FunctionCallExpr, FunctionCallUnknownDiagExtra, IndexExpr,
    LiteralValueExpr, ObjectConsExpr, ObjectConsItem, ObjectConsKeyExpr, Operation,
    ParenthesesExpr, RelativeTraversalExpr, ScopeTraversalExpr, SplatExpr, TemplateExpr,
    TemplateJoinExpr, TemplateWrapExpr, TupleConsExpr, UnaryOpExpr,
};
pub use structure::{Attribute, Attributes, Block, Blocks, Body};
pub use token::{Token, TokenType, Tokens, check_invalid_tokens};
pub use walk::{ChildScope, Node, VisitFunc, Walker, variables, visit_all, walk};

/// Parses the given source as a native-syntax configuration file, with the
/// first byte at the given start position (hclsyntax:
/// `hclsyntax.ParseConfig`). The returned file's body downcasts (via
/// `Body::as_any`) to [`structure::Body`].
pub fn parse_config(src: &[u8], filename: &str, start: Pos) -> (File, Diagnostics) {
    todo!()
}

/// Parses the given source as a standalone expression
/// (hclsyntax: `hclsyntax.ParseExpression`).
pub fn parse_expression(src: &[u8], filename: &str, start: Pos) -> (Expression, Diagnostics) {
    todo!()
}

/// Parses the given source as a standalone string template
/// (hclsyntax: `hclsyntax.ParseTemplate`).
pub fn parse_template(src: &[u8], filename: &str, start: Pos) -> (Expression, Diagnostics) {
    todo!()
}

/// Parses the given source as an absolute traversal
/// (hclsyntax: `hclsyntax.ParseTraversalAbs`).
pub fn parse_traversal_abs(src: &[u8], filename: &str, start: Pos) -> (Traversal, Diagnostics) {
    todo!()
}

/// Parses the given source as an absolute traversal that may also use splat
/// syntax (hclsyntax: `hclsyntax.ParseTraversalPartial`).
pub fn parse_traversal_partial(src: &[u8], filename: &str, start: Pos) -> (Traversal, Diagnostics) {
    todo!()
}

/// Scans the given source in configuration mode, also reporting diagnostics
/// for tokens that are never valid in the language
/// (hclsyntax: `hclsyntax.LexConfig`).
pub fn lex_config(src: &[u8], filename: &str, start: Pos) -> (Tokens, Diagnostics) {
    todo!()
}

/// Scans the given source in expression mode
/// (hclsyntax: `hclsyntax.LexExpression`).
pub fn lex_expression(src: &[u8], filename: &str, start: Pos) -> (Tokens, Diagnostics) {
    todo!()
}

/// Scans the given source in template mode
/// (hclsyntax: `hclsyntax.LexTemplate`).
pub fn lex_template(src: &[u8], filename: &str, start: Pos) -> (Tokens, Diagnostics) {
    todo!()
}

/// Whether the given string is a valid HCL identifier
/// (hclsyntax: `hclsyntax.ValidIdentifier`).
pub fn valid_identifier(s: &str) -> bool {
    todo!()
}

/// Decodes a quoted-string-literal token's bytes into the string it
/// represents, processing escapes
/// (hclsyntax: `hclsyntax.ParseStringLiteralToken`).
pub fn parse_string_literal_token(tok: &Token) -> (String, Diagnostics) {
    todo!()
}

/// The scanner's operating mode (hclsyntax: unexported `scanMode`; exposed
/// for conformance because the scanner tests drive it directly).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanMode {
    /// Configuration-file scanning (hclsyntax: `scanNormal`).
    Normal,
    /// Bare-template scanning (hclsyntax: `scanTemplate`).
    Template,
    /// Identifier-only scanning (hclsyntax: `scanIdentOnly`).
    IdentOnly,
}

/// Scans the raw token stream for the given source (hclsyntax: unexported
/// `scanTokens`; exposed for conformance).
pub fn scan_tokens(data: &[u8], filename: &str, start: Pos, mode: ScanMode) -> Tokens {
    todo!()
}

/// Splits the bytes of a string literal into a slice per character or
/// escape sequence (hclsyntax: unexported `scanStringLit`; exposed for
/// conformance).
pub fn scan_string_lit(data: &[u8], quoted: bool) -> Vec<Vec<u8>> {
    todo!()
}

/// Suggests a name from `suggestions` that is close to `given`, or the
/// empty string (hclsyntax: unexported `nameSuggestion`; exposed for
/// conformance).
pub fn name_suggestion(given: &str, suggestions: &[&str]) -> String {
    todo!()
}

/// A window over a token sequence with lookahead, comment skipping, and
/// newline-sensitivity control (hclsyntax: unexported `peeker`; exposed for
/// conformance because the peeker tests drive it directly).
#[derive(Debug)]
pub struct Peeker {
    _priv: (),
}

impl Peeker {
    /// A new peeker over the given tokens (hclsyntax: `newPeeker`).
    pub fn new(tokens: Tokens, include_comments: bool) -> Peeker {
        todo!()
    }

    /// The next significant token, without consuming it
    /// (hclsyntax: `peeker.Peek`).
    pub fn peek(&self) -> Token {
        todo!()
    }

    /// Consumes and returns the next significant token
    /// (hclsyntax: `peeker.Read`).
    pub fn read(&mut self) -> Token {
        todo!()
    }

    /// The range of the next significant token
    /// (hclsyntax: `peeker.NextRange`).
    pub fn next_range(&self) -> Range {
        todo!()
    }

    /// The range of the most recently read token
    /// (hclsyntax: `peeker.PrevRange`).
    pub fn prev_range(&self) -> Range {
        todo!()
    }

    /// Pushes whether newlines are significant onto the peeker's stack
    /// (hclsyntax: `peeker.PushIncludeNewlines`).
    pub fn push_include_newlines(&mut self, include: bool) {
        todo!()
    }

    /// Pops the newline-significance stack, returning the popped value
    /// (hclsyntax: `peeker.PopIncludeNewlines`).
    pub fn pop_include_newlines(&mut self) -> bool {
        todo!()
    }
}
