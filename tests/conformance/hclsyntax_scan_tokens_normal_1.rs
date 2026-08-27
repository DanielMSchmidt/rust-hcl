//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/scan_tokens_test.go (TestScanTokens_normal, part 1)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::{self, ScanMode, Token, TokenType};
use hcl::{Pos, Range};

/// `hcl.Pos{Byte: byte, Line: line, Column: column}`.
fn pos(byte: usize, line: usize, column: usize) -> Pos {
    Pos { byte, line, column }
}

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

/// `Token{Type: ty, Bytes: bytes, Range: range}`.
fn tok(ty: TokenType, bytes: &[u8], range: Range) -> Token {
    Token {
        ty,
        bytes: bytes.to_vec(),
        range,
    }
}

// Ported from TestScanTokens_normal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/scan_tokens_test.go#L13
// (part 1: cases with opening brace before upstream line 920)
#[test]
#[ignore = "not yet implemented"]
fn scan_tokens_normal_part1() {
    struct Case {
        input: &'static [u8],
        want: Vec<Token>,
    }

    let tests = [
        // Empty input
        Case {
            input: b"",
            want: vec![tok(TokenType::EOF, b"", rng(pos(0, 1, 1), pos(0, 1, 1)))],
        },
        Case {
            input: b" ",
            want: vec![tok(TokenType::EOF, b"", rng(pos(1, 1, 2), pos(1, 1, 2)))],
        },
        Case {
            input: b"\n\n",
            want: vec![
                tok(TokenType::Newline, b"\n", rng(pos(0, 1, 1), pos(1, 2, 1))),
                tok(TokenType::Newline, b"\n", rng(pos(1, 2, 1), pos(2, 3, 1))),
                tok(TokenType::EOF, b"", rng(pos(2, 3, 1), pos(2, 3, 1))),
            ],
        },
        // Byte-order mark
        Case {
            input: b"\xef\xbb\xbf", // Leading UTF-8 byte-order mark is ignored...
            want: vec![
                // ...but its bytes still count when producing ranges
                tok(TokenType::EOF, b"", rng(pos(3, 1, 1), pos(3, 1, 1))),
            ],
        },
        Case {
            input: b" \xef\xbb\xbf", // Non-leading BOM is invalid
            want: vec![
                tok(
                    TokenType::Invalid,
                    b"\xef\xbb\xbf", // Go: `utf8BOM`
                    rng(pos(1, 1, 2), pos(4, 1, 3)),
                ),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 3), pos(4, 1, 3))),
            ],
        },
        Case {
            input: b"\xfe\xff", // UTF-16 BOM is invalid
            want: vec![
                tok(TokenType::BadUTF8, b"\xfe", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::BadUTF8, b"\xff", rng(pos(1, 1, 2), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        // TokenNumberLit
        Case {
            input: b"1",
            want: vec![
                tok(TokenType::NumberLit, b"1", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::EOF, b"", rng(pos(1, 1, 2), pos(1, 1, 2))),
            ],
        },
        Case {
            input: b"12",
            want: vec![
                tok(TokenType::NumberLit, b"12", rng(pos(0, 1, 1), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: b"12.3",
            want: vec![
                tok(
                    TokenType::NumberLit,
                    b"12.3",
                    rng(pos(0, 1, 1), pos(4, 1, 5)),
                ),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 5), pos(4, 1, 5))),
            ],
        },
        Case {
            input: b"1e2",
            want: vec![
                tok(
                    TokenType::NumberLit,
                    b"1e2",
                    rng(pos(0, 1, 1), pos(3, 1, 4)),
                ),
                tok(TokenType::EOF, b"", rng(pos(3, 1, 4), pos(3, 1, 4))),
            ],
        },
        Case {
            input: b"1e+2",
            want: vec![
                tok(
                    TokenType::NumberLit,
                    b"1e+2",
                    rng(pos(0, 1, 1), pos(4, 1, 5)),
                ),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 5), pos(4, 1, 5))),
            ],
        },
        // TokenIdent
        Case {
            input: b"hello",
            want: vec![
                tok(TokenType::Ident, b"hello", rng(pos(0, 1, 1), pos(5, 1, 6))),
                tok(TokenType::EOF, b"", rng(pos(5, 1, 6), pos(5, 1, 6))),
            ],
        },
        Case {
            input: b"_ello",
            want: vec![
                tok(TokenType::Ident, b"_ello", rng(pos(0, 1, 1), pos(5, 1, 6))),
                tok(TokenType::EOF, b"", rng(pos(5, 1, 6), pos(5, 1, 6))),
            ],
        },
        Case {
            input: b"hel_o",
            want: vec![
                tok(TokenType::Ident, b"hel_o", rng(pos(0, 1, 1), pos(5, 1, 6))),
                tok(TokenType::EOF, b"", rng(pos(5, 1, 6), pos(5, 1, 6))),
            ],
        },
        Case {
            input: b"hel-o",
            want: vec![
                tok(TokenType::Ident, b"hel-o", rng(pos(0, 1, 1), pos(5, 1, 6))),
                tok(TokenType::EOF, b"", rng(pos(5, 1, 6), pos(5, 1, 6))),
            ],
        },
        Case {
            input: b"h3ll0",
            want: vec![
                tok(TokenType::Ident, b"h3ll0", rng(pos(0, 1, 1), pos(5, 1, 6))),
                tok(TokenType::EOF, b"", rng(pos(5, 1, 6), pos(5, 1, 6))),
            ],
        },
        Case {
            input: b"he\xcc\x81llo", // upstream `héllo`: e followed by combining acute accent
            want: vec![
                tok(
                    TokenType::Ident,
                    b"he\xcc\x81llo",
                    rng(pos(0, 1, 1), pos(7, 1, 6)),
                ),
                tok(TokenType::EOF, b"", rng(pos(7, 1, 6), pos(7, 1, 6))),
            ],
        },
        // TokenDoubleColon and associated TokenIdent
        Case {
            input: b"::",
            want: vec![
                tok(
                    TokenType::DoubleColon,
                    b"::",
                    rng(pos(0, 1, 1), pos(2, 1, 3)),
                ),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: b"a::b",
            want: vec![
                tok(TokenType::Ident, b"a", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::DoubleColon,
                    b"::",
                    rng(pos(1, 1, 2), pos(3, 1, 4)),
                ),
                tok(TokenType::Ident, b"b", rng(pos(3, 1, 4), pos(4, 1, 5))),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 5), pos(4, 1, 5))),
            ],
        },
        // Literal-only Templates (string literals, effectively)
        Case {
            input: br#""""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::CQuote, b"\"", rng(pos(1, 1, 2), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: br#""hello""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello",
                    rng(pos(1, 1, 2), pos(6, 1, 7)),
                ),
                tok(TokenType::CQuote, b"\"", rng(pos(6, 1, 7), pos(7, 1, 8))),
                tok(TokenType::EOF, b"", rng(pos(7, 1, 8), pos(7, 1, 8))),
            ],
        },
        Case {
            input: br#""hello, \"world\"!""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    br#"hello, \"world\"!"#, // The escapes are handled by the parser, not the scanner
                    rng(pos(1, 1, 2), pos(18, 1, 19)),
                ),
                tok(
                    TokenType::CQuote,
                    b"\"",
                    rng(pos(18, 1, 19), pos(19, 1, 20)),
                ),
                tok(TokenType::EOF, b"", rng(pos(19, 1, 20), pos(19, 1, 20))),
            ],
        },
        Case {
            input: br#""hello $$""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                // This one scans a little oddly because of how the scanner
                // handles the escaping of the dollar sign, but it's still
                // good enough for the parser since it'll just concatenate
                // these two string literals together anyway.
                tok(TokenType::QuotedLit, b"$", rng(pos(7, 1, 8), pos(8, 1, 9))),
                tok(TokenType::QuotedLit, b"$", rng(pos(8, 1, 9), pos(9, 1, 10))),
                tok(TokenType::CQuote, b"\"", rng(pos(9, 1, 10), pos(10, 1, 11))),
                tok(TokenType::EOF, b"", rng(pos(10, 1, 11), pos(10, 1, 11))),
            ],
        },
        Case {
            input: br#""hello %%""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                // This one scans a little oddly because of how the scanner
                // handles the escaping of the percent sign, but it's still
                // good enough for the parser since it'll just concatenate
                // these two string literals together anyway.
                tok(TokenType::QuotedLit, b"%", rng(pos(7, 1, 8), pos(8, 1, 9))),
                tok(TokenType::QuotedLit, b"%", rng(pos(8, 1, 9), pos(9, 1, 10))),
                tok(TokenType::CQuote, b"\"", rng(pos(9, 1, 10), pos(10, 1, 11))),
                tok(TokenType::EOF, b"", rng(pos(10, 1, 11), pos(10, 1, 11))),
            ],
        },
        Case {
            input: br#""hello $""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                tok(TokenType::QuotedLit, b"$", rng(pos(7, 1, 8), pos(8, 1, 9))),
                tok(TokenType::CQuote, b"\"", rng(pos(8, 1, 9), pos(9, 1, 10))),
                tok(TokenType::EOF, b"", rng(pos(9, 1, 10), pos(9, 1, 10))),
            ],
        },
        Case {
            input: br#""hello %""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                tok(TokenType::QuotedLit, b"%", rng(pos(7, 1, 8), pos(8, 1, 9))),
                tok(TokenType::CQuote, b"\"", rng(pos(8, 1, 9), pos(9, 1, 10))),
                tok(TokenType::EOF, b"", rng(pos(9, 1, 10), pos(9, 1, 10))),
            ],
        },
        Case {
            input: br#""hello $${world}""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                tok(
                    TokenType::QuotedLit,
                    b"$${",
                    rng(pos(7, 1, 8), pos(10, 1, 11)),
                ),
                tok(
                    TokenType::QuotedLit,
                    b"world}",
                    rng(pos(10, 1, 11), pos(16, 1, 17)),
                ),
                tok(
                    TokenType::CQuote,
                    b"\"",
                    rng(pos(16, 1, 17), pos(17, 1, 18)),
                ),
                tok(TokenType::EOF, b"", rng(pos(17, 1, 18), pos(17, 1, 18))),
            ],
        },
        Case {
            input: br#""hello %%{world}""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                tok(
                    TokenType::QuotedLit,
                    b"%%{",
                    rng(pos(7, 1, 8), pos(10, 1, 11)),
                ),
                tok(
                    TokenType::QuotedLit,
                    b"world}",
                    rng(pos(10, 1, 11), pos(16, 1, 17)),
                ),
                tok(
                    TokenType::CQuote,
                    b"\"",
                    rng(pos(16, 1, 17), pos(17, 1, 18)),
                ),
                tok(TokenType::EOF, b"", rng(pos(17, 1, 18), pos(17, 1, 18))),
            ],
        },
        Case {
            input: br#""hello %${world}""#,
            want: vec![
                tok(TokenType::OQuote, b"\"", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(
                    TokenType::QuotedLit,
                    b"hello ",
                    rng(pos(1, 1, 2), pos(7, 1, 8)),
                ),
                tok(TokenType::QuotedLit, b"%", rng(pos(7, 1, 8), pos(8, 1, 9))),
                tok(
                    TokenType::TemplateInterp,
                    b"${",
                    rng(pos(8, 1, 9), pos(10, 1, 11)),
                ),
                tok(
                    TokenType::Ident,
                    b"world",
                    rng(pos(10, 1, 11), pos(15, 1, 16)),
                ),
                tok(
                    TokenType::TemplateSeqEnd,
                    b"}",
                    rng(pos(15, 1, 16), pos(16, 1, 17)),
                ),
                tok(
                    TokenType::CQuote,
                    b"\"",
                    rng(pos(16, 1, 17), pos(17, 1, 18)),
                ),
                tok(TokenType::EOF, b"", rng(pos(17, 1, 18), pos(17, 1, 18))),
            ],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclsyntax::scan_tokens(
            test.input,
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
            ScanMode::Normal,
        );
        assert_eq!(
            got,
            test.want,
            "case {i} (input {:?}): wrong result",
            String::from_utf8_lossy(test.input),
        );
    }
}
