//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   json/scanner_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::json::scanner::{self, ScannerPos, Token, TokenType};
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

/// `token{Type: ty, Bytes: bytes, Range: range}`. Upstream leaves `Bytes`
/// as its nil zero value on EOF tokens; pass `b""` for those.
fn tok(ty: TokenType, bytes: &[u8], range: Range) -> Token {
    Token {
        ty,
        bytes: bytes.to_vec(),
        range,
    }
}

// Ported from TestScan:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/scanner_test.go#L15
#[test]
#[ignore = "not yet implemented"]
fn scan() {
    struct Case {
        input: &'static [u8],
        want: Vec<Token>,
    }

    let tests = [
        Case {
            input: b"",
            want: vec![tok(TokenType::EOF, b"", rng(pos(0, 1, 1), pos(0, 1, 1)))],
        },
        Case {
            input: b"   ",
            want: vec![tok(TokenType::EOF, b"", rng(pos(3, 1, 4), pos(3, 1, 4)))],
        },
        Case {
            input: b"{}",
            want: vec![
                tok(TokenType::BraceO, b"{", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::BraceC, b"}", rng(pos(1, 1, 2), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: b"][",
            want: vec![
                tok(TokenType::BrackC, b"]", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::BrackO, b"[", rng(pos(1, 1, 2), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: b":,",
            want: vec![
                tok(TokenType::Colon, b":", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::Comma, b",", rng(pos(1, 1, 2), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: b"1",
            want: vec![
                tok(TokenType::Number, b"1", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::EOF, b"", rng(pos(1, 1, 2), pos(1, 1, 2))),
            ],
        },
        Case {
            input: b"  1",
            want: vec![
                tok(TokenType::Number, b"1", rng(pos(2, 1, 3), pos(3, 1, 4))),
                tok(TokenType::EOF, b"", rng(pos(3, 1, 4), pos(3, 1, 4))),
            ],
        },
        Case {
            input: b"  12",
            want: vec![
                tok(TokenType::Number, b"12", rng(pos(2, 1, 3), pos(4, 1, 5))),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 5), pos(4, 1, 5))),
            ],
        },
        Case {
            input: b"1 2",
            want: vec![
                tok(TokenType::Number, b"1", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::Number, b"2", rng(pos(2, 1, 3), pos(3, 1, 4))),
                tok(TokenType::EOF, b"", rng(pos(3, 1, 4), pos(3, 1, 4))),
            ],
        },
        Case {
            input: b"\n1\n 2",
            want: vec![
                tok(TokenType::Number, b"1", rng(pos(1, 2, 1), pos(2, 2, 2))),
                tok(TokenType::Number, b"2", rng(pos(4, 3, 2), pos(5, 3, 3))),
                tok(TokenType::EOF, b"", rng(pos(5, 3, 3), pos(5, 3, 3))),
            ],
        },
        Case {
            input: b"-1 2.5",
            want: vec![
                tok(TokenType::Number, b"-1", rng(pos(0, 1, 1), pos(2, 1, 3))),
                tok(TokenType::Number, b"2.5", rng(pos(3, 1, 4), pos(6, 1, 7))),
                tok(TokenType::EOF, b"", rng(pos(6, 1, 7), pos(6, 1, 7))),
            ],
        },
        Case {
            input: b"true",
            want: vec![
                tok(TokenType::Keyword, b"true", rng(pos(0, 1, 1), pos(4, 1, 5))),
                tok(TokenType::EOF, b"", rng(pos(4, 1, 5), pos(4, 1, 5))),
            ],
        },
        Case {
            input: b"[true]",
            want: vec![
                tok(TokenType::BrackO, b"[", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::Keyword, b"true", rng(pos(1, 1, 2), pos(5, 1, 6))),
                tok(TokenType::BrackC, b"]", rng(pos(5, 1, 6), pos(6, 1, 7))),
                tok(TokenType::EOF, b"", rng(pos(6, 1, 7), pos(6, 1, 7))),
            ],
        },
        Case {
            input: br#""""#,
            want: vec![
                tok(TokenType::String, br#""""#, rng(pos(0, 1, 1), pos(2, 1, 3))),
                tok(TokenType::EOF, b"", rng(pos(2, 1, 3), pos(2, 1, 3))),
            ],
        },
        Case {
            input: br#""hello""#,
            want: vec![
                tok(
                    TokenType::String,
                    br#""hello""#,
                    rng(pos(0, 1, 1), pos(7, 1, 8)),
                ),
                tok(TokenType::EOF, b"", rng(pos(7, 1, 8), pos(7, 1, 8))),
            ],
        },
        Case {
            input: br#""he\"llo""#,
            want: vec![
                tok(
                    TokenType::String,
                    br#""he\"llo""#,
                    rng(pos(0, 1, 1), pos(9, 1, 10)),
                ),
                tok(TokenType::EOF, b"", rng(pos(9, 1, 10), pos(9, 1, 10))),
            ],
        },
        Case {
            input: br#""hello\\" 1"#,
            want: vec![
                tok(
                    TokenType::String,
                    br#""hello\\""#,
                    rng(pos(0, 1, 1), pos(9, 1, 10)),
                ),
                tok(TokenType::Number, b"1", rng(pos(10, 1, 11), pos(11, 1, 12))),
                tok(TokenType::EOF, b"", rng(pos(11, 1, 12), pos(11, 1, 12))),
            ],
        },
        Case {
            // upstream `"🇬🇧"`: two regional-indicator symbols forming one
            // grapheme cluster (10 bytes with the quotes)
            input: b"\"\xf0\x9f\x87\xac\xf0\x9f\x87\xa7\"",
            want: vec![
                tok(
                    TokenType::String,
                    b"\"\xf0\x9f\x87\xac\xf0\x9f\x87\xa7\"",
                    rng(pos(0, 1, 1), pos(10, 1, 4)),
                ),
                tok(TokenType::EOF, b"", rng(pos(10, 1, 4), pos(10, 1, 4))),
            ],
        },
        Case {
            // upstream `"á́́́́́́́"`: `a` followed by eight combining acute
            // accents (19 bytes with the quotes)
            input: b"\"a\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\"",
            want: vec![
                tok(
                    TokenType::String,
                    b"\"a\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\xcc\x81\"",
                    rng(pos(0, 1, 1), pos(19, 1, 4)),
                ),
                tok(TokenType::EOF, b"", rng(pos(19, 1, 4), pos(19, 1, 4))),
            ],
        },
        Case {
            input: b"&",
            want: vec![
                tok(TokenType::Invalid, b"&", rng(pos(0, 1, 1), pos(1, 1, 2))),
                tok(TokenType::EOF, b"", rng(pos(1, 1, 2), pos(1, 1, 2))),
            ],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let start = ScannerPos {
            filename: String::new(),
            pos: Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
        };
        let got = scanner::scan(test.input, start);
        assert_eq!(
            got,
            test.want,
            "case {i} (input {:?}): wrong result",
            String::from_utf8_lossy(test.input),
        );
    }
}
