//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/scan_tokens_test.go (TestScanTokens_template only)
//!   hclsyntax/scan_string_lit_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.
//!
//! `TestScanTokens_normal` from scan_tokens_test.go is ported separately in
//! the `hclsyntax_scan_tokens_normal_*` targets.

use hcl::hclsyntax::{self, ScanMode, Token, TokenType};
use hcl::{Pos, Range};

/// Shorthand for `hcl.Pos{Byte: ..., Line: ..., Column: ...}`.
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

// Ported from TestScanTokens_template:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/scan_tokens_test.go#L2664
#[test]
#[ignore = "not yet implemented"]
fn scan_tokens_template() {
    struct Case {
        input: &'static str,
        want: Vec<Token>,
    }

    let tests = [
        // Empty input
        Case {
            input: "",
            want: vec![Token {
                ty: TokenType::EOF,
                bytes: vec![],
                range: rng(pos(0, 1, 1), pos(0, 1, 1)),
            }],
        },
        // Simple literals
        Case {
            input: " hello ",
            want: vec![
                Token {
                    ty: TokenType::StringLit,
                    bytes: b" hello ".to_vec(),
                    range: rng(pos(0, 1, 1), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(7, 1, 8), pos(7, 1, 8)),
                },
            ],
        },
        Case {
            input: "\nhello\n",
            want: vec![
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello\n".to_vec(),
                    range: rng(pos(1, 2, 1), pos(7, 3, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(7, 3, 1), pos(7, 3, 1)),
                },
            ],
        },
        Case {
            input: "hello ${foo} hello",
            want: vec![
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello ".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(6, 1, 7), pos(8, 1, 9)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    range: rng(pos(8, 1, 9), pos(11, 1, 12)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(11, 1, 12), pos(12, 1, 13)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b" hello".to_vec(),
                    range: rng(pos(12, 1, 13), pos(18, 1, 19)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(18, 1, 19), pos(18, 1, 19)),
                },
            ],
        },
        Case {
            input: "hello ${~foo~} hello",
            want: vec![
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello ".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${~".to_vec(),
                    range: rng(pos(6, 1, 7), pos(9, 1, 10)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    range: rng(pos(9, 1, 10), pos(12, 1, 13)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"~}".to_vec(),
                    range: rng(pos(12, 1, 13), pos(14, 1, 15)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b" hello".to_vec(),
                    range: rng(pos(14, 1, 15), pos(20, 1, 21)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(20, 1, 21), pos(20, 1, 21)),
                },
            ],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclsyntax::scan_tokens(
            test.input.as_bytes(),
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
            ScanMode::Template,
        );
        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.input,);
    }
}

// Ported from TestScanStringLit:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/scan_string_lit_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn scan_string_lit() {
    struct Case {
        input: &'static str,
        want_quoted: Vec<&'static str>,
        want_unquoted: Vec<&'static str>,
    }

    let tests = [
        Case {
            input: "",
            want_quoted: vec![],
            want_unquoted: vec![],
        },
        Case {
            input: r"hello",
            want_quoted: vec![r"hello"],
            want_unquoted: vec![r"hello"],
        },
        Case {
            input: r"hello world",
            want_quoted: vec![r"hello world"],
            want_unquoted: vec![r"hello world"],
        },
        Case {
            input: r"hello\nworld",
            want_quoted: vec![r"hello", r"\n", r"world"],
            want_unquoted: vec![r"hello\nworld"],
        },
        Case {
            input: r"hello\🥁world",
            want_quoted: vec![r"hello", r"\🥁", r"world"],
            want_unquoted: vec![r"hello\🥁world"],
        },
        Case {
            input: r"hello\uabcdworld",
            want_quoted: vec![r"hello", r"\uabcd", r"world"],
            want_unquoted: vec![r"hello\uabcdworld"],
        },
        Case {
            input: r"hello\uabcdabcdworld",
            want_quoted: vec![r"hello", r"\uabcd", r"abcdworld"],
            want_unquoted: vec![r"hello\uabcdabcdworld"],
        },
        Case {
            input: r"hello\uabcworld",
            want_quoted: vec![r"hello", r"\uabc", r"world"],
            want_unquoted: vec![r"hello\uabcworld"],
        },
        Case {
            input: r"hello\U01234567world",
            want_quoted: vec![r"hello", r"\U01234567", r"world"],
            want_unquoted: vec![r"hello\U01234567world"],
        },
        Case {
            input: r"hello\U012345670123world",
            want_quoted: vec![r"hello", r"\U01234567", r"0123world"],
            want_unquoted: vec![r"hello\U012345670123world"],
        },
        Case {
            input: r"hello\Uabcdworld",
            want_quoted: vec![r"hello", r"\Uabcd", r"world"],
            want_unquoted: vec![r"hello\Uabcdworld"],
        },
        Case {
            input: r"hello\Uabcworld",
            want_quoted: vec![r"hello", r"\Uabc", r"world"],
            want_unquoted: vec![r"hello\Uabcworld"],
        },
        Case {
            input: r"hello\uworld",
            want_quoted: vec![r"hello", r"\u", r"world"],
            want_unquoted: vec![r"hello\uworld"],
        },
        Case {
            input: r"hello\Uworld",
            want_quoted: vec![r"hello", r"\U", r"world"],
            want_unquoted: vec![r"hello\Uworld"],
        },
        Case {
            input: r"hello\u",
            want_quoted: vec![r"hello", r"\u"],
            want_unquoted: vec![r"hello\u"],
        },
        Case {
            input: r"hello\U",
            want_quoted: vec![r"hello", r"\U"],
            want_unquoted: vec![r"hello\U"],
        },
        Case {
            input: r"hello\",
            want_quoted: vec![r"hello", r"\"],
            want_unquoted: vec![r"hello\"],
        },
        Case {
            input: r"hello$${world}",
            want_quoted: vec![r"hello", r"$${", r"world}"],
            want_unquoted: vec![r"hello", r"$${", r"world}"],
        },
        Case {
            input: r"hello$$world",
            want_quoted: vec![r"hello", r"$$", r"world"],
            want_unquoted: vec![r"hello", r"$$", r"world"],
        },
        Case {
            input: r"hello$world",
            want_quoted: vec![r"hello", r"$", r"world"],
            want_unquoted: vec![r"hello", r"$", r"world"],
        },
        Case {
            input: r"hello$",
            want_quoted: vec![r"hello", r"$"],
            want_unquoted: vec![r"hello", r"$"],
        },
        Case {
            input: r"hello$${",
            want_quoted: vec![r"hello", r"$${"],
            want_unquoted: vec![r"hello", r"$${"],
        },
        Case {
            input: r"hello%%{world}",
            want_quoted: vec![r"hello", r"%%{", r"world}"],
            want_unquoted: vec![r"hello", r"%%{", r"world}"],
        },
        Case {
            input: r"hello%%world",
            want_quoted: vec![r"hello", r"%%", r"world"],
            want_unquoted: vec![r"hello", r"%%", r"world"],
        },
        Case {
            input: r"hello%world",
            want_quoted: vec![r"hello", r"%", r"world"],
            want_unquoted: vec![r"hello", r"%", r"world"],
        },
        Case {
            input: r"hello%",
            want_quoted: vec![r"hello", r"%"],
            want_unquoted: vec![r"hello", r"%"],
        },
        Case {
            input: r"hello%%{",
            want_quoted: vec![r"hello", r"%%{"],
            want_unquoted: vec![r"hello", r"%%{"],
        },
        Case {
            input: r"hello\${world}",
            want_quoted: vec![r"hello", r"\$", r"{world}"],
            want_unquoted: vec![r"hello\", r"$", r"{world}"],
        },
        Case {
            input: r"hello\%{world}",
            want_quoted: vec![r"hello", r"\%", r"{world}"],
            want_unquoted: vec![r"hello\", r"%", r"{world}"],
        },
        Case {
            input: "hello\nworld",
            want_quoted: vec!["hello", "\n", "world"],
            want_unquoted: vec!["hello", "\n", "world"],
        },
        Case {
            input: "hello\rworld",
            want_quoted: vec!["hello", "\r", "world"],
            want_unquoted: vec!["hello", "\r", "world"],
        },
        Case {
            input: "hello\r\nworld",
            want_quoted: vec!["hello", "\r\n", "world"],
            want_unquoted: vec!["hello", "\r\n", "world"],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        // Upstream runs each input as two subtests, "quoted" and
        // "unquoted", against the respective want tables.
        let got = hclsyntax::scan_string_lit(test.input.as_bytes(), true);
        let want: Vec<Vec<u8>> = test
            .want_quoted
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        assert_eq!(
            got, want,
            "case {i} ({:?}) quoted: wrong result",
            test.input,
        );

        let got = hclsyntax::scan_string_lit(test.input.as_bytes(), false);
        let want: Vec<Vec<u8>> = test
            .want_unquoted
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        assert_eq!(
            got, want,
            "case {i} ({:?}) unquoted: wrong result",
            test.input,
        );
    }
}
