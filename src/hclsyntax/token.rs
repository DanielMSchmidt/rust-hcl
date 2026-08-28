//! Lexical tokens of the native syntax (hcl v2: `hclsyntax/token.go`).

use crate::diagnostic::Diagnostics;
use crate::pos::Range;

/// The kind of a [`Token`] (hclsyntax: `hclsyntax.TokenType`).
///
/// Variant names drop the Go `Token` prefix: `TokenOBrace` → `OBrace`, etc.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TokenType {
    /// `{` (hclsyntax: `TokenOBrace`).
    OBrace,
    /// `}` (hclsyntax: `TokenCBrace`).
    CBrace,
    /// `[` (hclsyntax: `TokenOBrack`).
    OBrack,
    /// `]` (hclsyntax: `TokenCBrack`).
    CBrack,
    /// `(` (hclsyntax: `TokenOParen`).
    OParen,
    /// `)` (hclsyntax: `TokenCParen`).
    CParen,
    /// An opening quote (hclsyntax: `TokenOQuote`).
    OQuote,
    /// A closing quote (hclsyntax: `TokenCQuote`).
    CQuote,
    /// A heredoc introducer (hclsyntax: `TokenOHeredoc`).
    OHeredoc,
    /// A heredoc terminator (hclsyntax: `TokenCHeredoc`).
    CHeredoc,
    /// `*` (hclsyntax: `TokenStar`).
    Star,
    /// `/` (hclsyntax: `TokenSlash`).
    Slash,
    /// `+` (hclsyntax: `TokenPlus`).
    Plus,
    /// `-` (hclsyntax: `TokenMinus`).
    Minus,
    /// `%` (hclsyntax: `TokenPercent`).
    Percent,
    /// `=` (hclsyntax: `TokenEqual`).
    Equal,
    /// `==` (hclsyntax: `TokenEqualOp`).
    EqualOp,
    /// `!=` (hclsyntax: `TokenNotEqual`).
    NotEqual,
    /// `<` (hclsyntax: `TokenLessThan`).
    LessThan,
    /// `<=` (hclsyntax: `TokenLessThanEq`).
    LessThanEq,
    /// `>` (hclsyntax: `TokenGreaterThan`).
    GreaterThan,
    /// `>=` (hclsyntax: `TokenGreaterThanEq`).
    GreaterThanEq,
    /// `&&` (hclsyntax: `TokenAnd`).
    And,
    /// `||` (hclsyntax: `TokenOr`).
    Or,
    /// `!` (hclsyntax: `TokenBang`).
    Bang,
    /// `.` (hclsyntax: `TokenDot`).
    Dot,
    /// `,` (hclsyntax: `TokenComma`).
    Comma,
    /// `::` (hclsyntax: `TokenDoubleColon`).
    DoubleColon,
    /// `...` (hclsyntax: `TokenEllipsis`).
    Ellipsis,
    /// `=>` (hclsyntax: `TokenFatArrow`).
    FatArrow,
    /// `?` (hclsyntax: `TokenQuestion`).
    Question,
    /// `:` (hclsyntax: `TokenColon`).
    Colon,
    /// `${` (hclsyntax: `TokenTemplateInterp`).
    TemplateInterp,
    /// `%{` (hclsyntax: `TokenTemplateControl`).
    TemplateControl,
    /// `}` ending a template sequence (hclsyntax: `TokenTemplateSeqEnd`).
    TemplateSeqEnd,
    /// A string literal that might contain backslash escapes
    /// (hclsyntax: `TokenQuotedLit`).
    QuotedLit,
    /// A string literal with no backslash escapes
    /// (hclsyntax: `TokenStringLit`).
    StringLit,
    /// A number literal (hclsyntax: `TokenNumberLit`).
    NumberLit,
    /// An identifier (hclsyntax: `TokenIdent`).
    Ident,
    /// A comment (hclsyntax: `TokenComment`).
    Comment,
    /// A newline (hclsyntax: `TokenNewline`).
    Newline,
    /// End of file (hclsyntax: `TokenEOF`).
    EOF,
    /// `&`, recognized only for diagnostics (hclsyntax: `TokenBitwiseAnd`).
    BitwiseAnd,
    /// `|`, recognized only for diagnostics (hclsyntax: `TokenBitwiseOr`).
    BitwiseOr,
    /// `~`, recognized only for diagnostics (hclsyntax: `TokenBitwiseNot`).
    BitwiseNot,
    /// `^`, recognized only for diagnostics (hclsyntax: `TokenBitwiseXor`).
    BitwiseXor,
    /// `**`, recognized only for diagnostics (hclsyntax: `TokenStarStar`).
    StarStar,
    /// `'`, recognized only for diagnostics (hclsyntax: `TokenApostrophe`).
    Apostrophe,
    /// A backtick, recognized only for diagnostics
    /// (hclsyntax: `TokenBacktick`).
    Backtick,
    /// `;`, recognized only for diagnostics (hclsyntax: `TokenSemicolon`).
    Semicolon,
    /// Tab characters, recognized only for diagnostics
    /// (hclsyntax: `TokenTabs`).
    Tabs,
    /// An invalid character (hclsyntax: `TokenInvalid`).
    Invalid,
    /// A byte sequence that is not valid UTF-8
    /// (hclsyntax: `TokenBadUTF8`).
    BadUTF8,
    /// A literal newline inside a quoted string
    /// (hclsyntax: `TokenQuotedNewline`).
    QuotedNewline,
    /// Placeholder when a token is required but none is available; never
    /// produced by the scanner (hclsyntax: `TokenNil`).
    #[default]
    Nil,
}

impl TokenType {
    /// The Go-syntax representation, identical to Go's
    /// `TokenType.GoString`, e.g. `hclsyntax.TokenOBrace`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

/// One lexical token of native syntax (hclsyntax: `hclsyntax.Token`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Token {
    /// The token's kind (hclsyntax: `Token.Type`).
    pub ty: TokenType,
    /// The raw source bytes of the token (hclsyntax: `Token.Bytes`).
    pub bytes: Vec<u8>,
    /// The token's source range (hclsyntax: `Token.Range`).
    pub range: Range,
}

/// A sequence of [`Token`]s (hclsyntax: `hclsyntax.Tokens`).
pub type Tokens = Vec<Token>;

/// Checks a token sequence for tokens that are valid to scan but never
/// valid in the language, producing helpful diagnostics
/// (hclsyntax: unexported `checkInvalidTokens`; exposed for conformance).
pub fn check_invalid_tokens(tokens: &[Token]) -> Diagnostics {
    todo!()
}
