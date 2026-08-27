//! The JSON scanner (hcl v2: `json/scanner.go`).
//!
//! Go keeps these types unexported; they are public here because the
//! upstream scanner tests drive them directly.

use crate::pos::{Pos, Range};

/// The kind of a JSON [`Token`] (json: unexported `tokenType`; Go's
/// `tokenBraceO` → `BraceO`, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TokenType {
    /// `{` (json: `tokenBraceO`).
    BraceO,
    /// `}` (json: `tokenBraceC`).
    BraceC,
    /// `[` (json: `tokenBrackO`).
    BrackO,
    /// `]` (json: `tokenBrackC`).
    BrackC,
    /// `,` (json: `tokenComma`).
    Comma,
    /// `:` (json: `tokenColon`).
    Colon,
    /// A keyword such as `true`, `false`, or `null`
    /// (json: `tokenKeyword`).
    Keyword,
    /// A string literal (json: `tokenString`).
    String,
    /// A number literal (json: `tokenNumber`).
    Number,
    /// End of input (json: `tokenEOF`).
    #[allow(clippy::upper_case_acronyms)]
    EOF,
    /// An invalid byte sequence (json: `tokenInvalid`).
    #[default]
    Invalid,
    /// `=`, recognized only to remind the user of JSON syntax
    /// (json: `tokenEquals`).
    Equals,
}

/// One JSON token (json: unexported `token`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Token {
    /// The token's kind (json: `token.Type`).
    pub ty: TokenType,
    /// The raw source bytes (json: `token.Bytes`).
    pub bytes: Vec<u8>,
    /// The token's source range (json: `token.Range`).
    pub range: Range,
}

impl Token {
    /// The Go-syntax representation, identical to Go's `token.GoString`,
    /// e.g. `json.token{json.tokenString, []byte("\"hello\""), ...}`
    /// abbreviated form used in test failure output.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

/// A scanner position: the file name plus the position within it
/// (json: unexported `pos`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScannerPos {
    /// The file name (json: `pos.Filename`).
    pub filename: String,
    /// The position (json: `pos.Pos`).
    pub pos: Pos,
}

impl ScannerPos {
    /// The range starting at this position and covering `byte_len` bytes
    /// making up `char_len` grapheme clusters on a single line
    /// (json: `pos.Range`).
    pub fn range(&self, byte_len: usize, char_len: usize) -> Range {
        todo!()
    }
}

/// Scans the primary tokens of a JSON buffer in sequence
/// (json: unexported `scan`; exposed for conformance).
pub fn scan(buf: &[u8], start: ScannerPos) -> Vec<Token> {
    todo!()
}
