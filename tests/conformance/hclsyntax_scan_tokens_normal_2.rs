//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/scan_tokens_test.go (TestScanTokens_normal, part 2 of 3)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::{self, ScanMode, Token, TokenType};
use hcl::{Pos, Range};

/// A position literal in upstream field order (Go:
/// `hcl.Pos{Byte: ..., Line: ..., Column: ...}`).
fn pos(byte: usize, line: usize, column: usize) -> Pos {
    Pos { line, column, byte }
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

// Ported from TestScanTokens_normal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/scan_tokens_test.go#L13
// (part 2: cases with opening brace in upstream lines 920-1815)
#[test]
#[ignore = "not yet implemented"]
fn scan_tokens_normal_part2() {
    struct Case {
        input: &'static [u8],
        want: Vec<Token>,
    }

    let tests = [
        // Templates with interpolations and control sequences
        Case {
            input: b"\"${1}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"1".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(4, 1, 5), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(5, 1, 6), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(6, 1, 7), pos(6, 1, 7)),
                },
            ],
        },
        Case {
            input: b"\"%{a}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateControl,
                    bytes: b"%{".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"a".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(4, 1, 5), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(5, 1, 6), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(6, 1, 7), pos(6, 1, 7)),
                },
            ],
        },
        Case {
            input: b"\"${{}}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    range: rng(pos(4, 1, 5), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(5, 1, 6), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(7, 1, 8), pos(7, 1, 8)),
                },
            ],
        },
        Case {
            input: b"\"${\"\"}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(4, 1, 5), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(5, 1, 6), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(7, 1, 8), pos(7, 1, 8)),
                },
            ],
        },
        Case {
            input: b"\"${\"${a}\"}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(4, 1, 5), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"a".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(7, 1, 8), pos(8, 1, 9)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(8, 1, 9), pos(9, 1, 10)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(9, 1, 10), pos(10, 1, 11)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(10, 1, 11), pos(11, 1, 12)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(11, 1, 12), pos(11, 1, 12)),
                },
            ],
        },
        Case {
            input: b"\"${\"${a} foo\"}\"",
            want: vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(3, 1, 4), pos(4, 1, 5)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(4, 1, 5), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"a".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(7, 1, 8), pos(8, 1, 9)),
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b" foo".to_vec(),
                    range: rng(pos(8, 1, 9), pos(12, 1, 13)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(12, 1, 13), pos(13, 1, 14)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(13, 1, 14), pos(14, 1, 15)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(14, 1, 15), pos(15, 1, 16)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(15, 1, 16), pos(15, 1, 16)),
                },
            ],
        },
        // Heredoc Templates
        Case {
            input: b"<<EOT\nhello world\nEOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello world\n".to_vec(),
                    range: rng(pos(6, 2, 1), pos(18, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(18, 3, 1), pos(21, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(21, 3, 4), pos(22, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(22, 4, 1), pos(22, 4, 1)),
                },
            ],
        },
        Case {
            // intentional windows-style line endings
            input: b"<<EOT\r\nhello world\r\nEOT\r\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOT\r\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(7, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello world\r\n".to_vec(),
                    range: rng(pos(7, 2, 1), pos(20, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(20, 3, 1), pos(23, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\r\n".to_vec(),
                    range: rng(pos(23, 3, 4), pos(25, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(25, 4, 1), pos(25, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<EOT\nhello ${name}\nEOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello ".to_vec(),
                    range: rng(pos(6, 2, 1), pos(12, 2, 7)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(12, 2, 7), pos(14, 2, 9)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"name".to_vec(),
                    range: rng(pos(14, 2, 9), pos(18, 2, 13)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(18, 2, 13), pos(19, 2, 14)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(19, 2, 14), pos(20, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(20, 3, 1), pos(23, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(23, 3, 4), pos(24, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(24, 4, 1), pos(24, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<EOT\n${name}EOT\nEOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 2, 1)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(6, 2, 1), pos(8, 2, 3)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"name".to_vec(),
                    range: rng(pos(8, 2, 3), pos(12, 2, 7)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(12, 2, 7), pos(13, 2, 8)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"EOT\n".to_vec(),
                    range: rng(pos(13, 2, 8), pos(17, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(17, 3, 1), pos(20, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(20, 3, 4), pos(21, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(21, 4, 1), pos(21, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<EOT\n  hello world\nEOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"  hello world\n".to_vec(),
                    range: rng(pos(6, 2, 1), pos(20, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(20, 3, 1), pos(23, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(23, 3, 4), pos(24, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(24, 4, 1), pos(24, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<-EOT\n  hello world\nEOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<-EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(7, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"  hello world\n".to_vec(),
                    range: rng(pos(7, 2, 1), pos(21, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOT".to_vec(),
                    range: rng(pos(21, 3, 1), pos(24, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(24, 3, 4), pos(25, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(25, 4, 1), pos(25, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<-EOT\n  hello world\n EOT\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<-EOT\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(7, 2, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"  hello world\n".to_vec(),
                    range: rng(pos(7, 2, 1), pos(21, 3, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b" EOT".to_vec(),
                    range: rng(pos(21, 3, 1), pos(25, 3, 5)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(25, 3, 5), pos(26, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(26, 4, 1), pos(26, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<EOF\n${<<-EOF\nhello\nEOF\n}\nEOF\n",
            want: vec![
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<EOF\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 2, 1)),
                },
                Token {
                    ty: TokenType::TemplateInterp,
                    bytes: b"${".to_vec(),
                    range: rng(pos(6, 2, 1), pos(8, 2, 3)),
                },
                Token {
                    ty: TokenType::OHeredoc,
                    bytes: b"<<-EOF\n".to_vec(),
                    range: rng(pos(8, 2, 3), pos(15, 3, 1)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"hello\n".to_vec(),
                    range: rng(pos(15, 3, 1), pos(21, 4, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(21, 4, 1), pos(24, 4, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(24, 4, 4), pos(25, 5, 1)),
                },
                Token {
                    ty: TokenType::TemplateSeqEnd,
                    bytes: b"}".to_vec(),
                    range: rng(pos(25, 5, 1), pos(26, 5, 2)),
                },
                Token {
                    ty: TokenType::StringLit,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(26, 5, 2), pos(27, 6, 1)),
                },
                Token {
                    ty: TokenType::CHeredoc,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(27, 6, 1), pos(30, 6, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(30, 6, 4), pos(31, 7, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    range: rng(pos(31, 7, 1), pos(31, 7, 1)),
                },
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
            "case {i} (input {:?}): wrong tokens",
            String::from_utf8_lossy(test.input),
        );
    }
}
