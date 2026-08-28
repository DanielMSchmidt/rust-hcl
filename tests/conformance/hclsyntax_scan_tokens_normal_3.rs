//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/scan_tokens_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::{self, ScanMode, Token, TokenType};
use hcl::{Pos, Range};

/// A position literal (Go: `hcl.Pos{Byte: ..., Line: ..., Column: ...}`),
/// arguments in upstream field order.
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

// Ported from TestScanTokens_normal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/scan_tokens_test.go#L13
// (part 3: cases with opening brace at upstream line 1816 or later)
#[test]
#[ignore = "not yet implemented"]
fn scan_tokens_normal_part3() {
    struct Case {
        input: &'static [u8],
        want: Vec<Token>,
    }

    let tests = [
        Case {
            input: b"<<EOF \nhello\nEOF\n",
            // `EOF ` is not a valid identifier
            // so `<<EOF ` is not a valid TokenOHeredoc
            want: vec![
                Token {
                    ty: TokenType::LessThan,
                    bytes: b"<".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::LessThan,
                    bytes: b"<".to_vec(),
                    range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(2, 1, 3), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"hello".to_vec(),
                    range: rng(pos(7, 2, 1), pos(12, 2, 6)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(12, 2, 6), pos(13, 3, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(13, 3, 1), pos(16, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(16, 3, 4), pos(17, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(17, 4, 1), pos(17, 4, 1)),
                },
            ],
        },
        Case {
            input: b"<<EOF \nhello\nEOF \n",
            // `EOF ` is not a valid identifier
            // so `<<EOF ` is not a valid TokenOHeredoc
            want: vec![
                Token {
                    ty: TokenType::LessThan,
                    bytes: b"<".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::LessThan,
                    bytes: b"<".to_vec(),
                    range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(2, 1, 3), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"hello".to_vec(),
                    range: rng(pos(7, 2, 1), pos(12, 2, 6)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(12, 2, 6), pos(13, 3, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"EOF".to_vec(),
                    range: rng(pos(13, 3, 1), pos(16, 3, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(17, 3, 5), pos(18, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(18, 4, 1), pos(18, 4, 1)),
                },
            ],
        },
        // Combinations
        Case {
            input: b" (1 + 2) * 3 ",
            want: vec![
                Token {
                    ty: TokenType::OParen,
                    bytes: b"(".to_vec(),
                    range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"1".to_vec(),
                    range: rng(pos(2, 1, 3), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::Plus,
                    bytes: b"+".to_vec(),
                    range: rng(pos(4, 1, 5), pos(5, 1, 6)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"2".to_vec(),
                    range: rng(pos(6, 1, 7), pos(7, 1, 8)),
                },
                Token {
                    ty: TokenType::CParen,
                    bytes: b")".to_vec(),
                    range: rng(pos(7, 1, 8), pos(8, 1, 9)),
                },
                Token {
                    ty: TokenType::Star,
                    bytes: b"*".to_vec(),
                    range: rng(pos(9, 1, 10), pos(10, 1, 11)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"3".to_vec(),
                    range: rng(pos(11, 1, 12), pos(12, 1, 13)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(13, 1, 14), pos(13, 1, 14)),
                },
            ],
        },
        Case {
            input: b"9%8",
            want: vec![
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"9".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::Percent,
                    bytes: b"%".to_vec(),
                    range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"8".to_vec(),
                    range: rng(pos(2, 1, 3), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(3, 1, 4), pos(3, 1, 4)),
                },
            ],
        },
        Case {
            input: b"\na = 1\n",
            want: vec![
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"a".to_vec(),
                    range: rng(pos(1, 2, 1), pos(2, 2, 2)),
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    range: rng(pos(3, 2, 3), pos(4, 2, 4)),
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"1".to_vec(),
                    range: rng(pos(5, 2, 5), pos(6, 2, 6)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(6, 2, 6), pos(7, 3, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(7, 3, 1), pos(7, 3, 1)),
                },
            ],
        },
        // Comments
        Case {
            input: b"# hello\n",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"# hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(8, 2, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(8, 2, 1), pos(8, 2, 1)),
                },
            ],
        },
        Case {
            input: b"// hello\n",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"// hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(9, 2, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(9, 2, 1), pos(9, 2, 1)),
                },
            ],
        },
        Case {
            input: b"// hello\n// hello",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"// hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(9, 2, 1)),
                },
                Token {
                    ty: TokenType::Comment,
                    bytes: b"// hello".to_vec(),
                    range: rng(pos(9, 2, 1), pos(17, 2, 9)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(17, 2, 9), pos(17, 2, 9)),
                },
            ],
        },
        Case {
            input: b"// hello\nfoo\n// hello",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"// hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(9, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    range: rng(pos(9, 2, 1), pos(12, 2, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(12, 2, 4), pos(13, 3, 1)),
                },
                Token {
                    ty: TokenType::Comment,
                    bytes: b"// hello".to_vec(),
                    range: rng(pos(13, 3, 1), pos(21, 3, 9)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(21, 3, 9), pos(21, 3, 9)),
                },
            ],
        },
        Case {
            input: b"# hello\n# hello",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"# hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(8, 2, 1)),
                },
                Token {
                    ty: TokenType::Comment,
                    bytes: b"# hello".to_vec(),
                    range: rng(pos(8, 2, 1), pos(15, 2, 8)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(15, 2, 8), pos(15, 2, 8)),
                },
            ],
        },
        Case {
            input: b"# hello\nfoo\n# hello",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"# hello\n".to_vec(),
                    range: rng(pos(0, 1, 1), pos(8, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    range: rng(pos(8, 2, 1), pos(11, 2, 4)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(11, 2, 4), pos(12, 3, 1)),
                },
                Token {
                    ty: TokenType::Comment,
                    bytes: b"# hello".to_vec(),
                    range: rng(pos(12, 3, 1), pos(19, 3, 8)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(19, 3, 8), pos(19, 3, 8)),
                },
            ],
        },
        Case {
            input: b"/* hello */",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"/* hello */".to_vec(),
                    range: rng(pos(0, 1, 1), pos(11, 1, 12)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(11, 1, 12), pos(11, 1, 12)),
                },
            ],
        },
        Case {
            input: b"/* hello */ howdy /* hey */",
            want: vec![
                Token {
                    ty: TokenType::Comment,
                    bytes: b"/* hello */".to_vec(),
                    range: rng(pos(0, 1, 1), pos(11, 1, 12)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"howdy".to_vec(),
                    range: rng(pos(12, 1, 13), pos(17, 1, 18)),
                },
                Token {
                    ty: TokenType::Comment,
                    bytes: b"/* hey */".to_vec(),
                    range: rng(pos(18, 1, 19), pos(27, 1, 28)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(27, 1, 28), pos(27, 1, 28)),
                },
            ],
        },
        // Invalid things
        Case {
            // Go: `🌻` (U+1F33B, four UTF-8 bytes)
            input: b"\xf0\x9f\x8c\xbb",
            want: vec![
                Token {
                    ty: TokenType::Invalid,
                    bytes: b"\xf0\x9f\x8c\xbb".to_vec(),
                    range: rng(pos(0, 1, 1), pos(4, 1, 2)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(4, 1, 2), pos(4, 1, 2)),
                },
            ],
        },
        Case {
            input: b"|",
            want: vec![
                Token {
                    ty: TokenType::BitwiseOr,
                    bytes: b"|".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(1, 1, 2), pos(1, 1, 2)),
                },
            ],
        },
        Case {
            input: b"\x80", // UTF-8 continuation without an introducer
            want: vec![
                Token {
                    ty: TokenType::BadUTF8,
                    bytes: b"\x80".to_vec(),
                    range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(1, 1, 2), pos(1, 1, 2)),
                },
            ],
        },
        Case {
            input: b" \x80\x80", // UTF-8 continuation without an introducer
            want: vec![
                Token {
                    ty: TokenType::BadUTF8,
                    bytes: b"\x80".to_vec(),
                    range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                },
                Token {
                    ty: TokenType::BadUTF8,
                    bytes: b"\x80".to_vec(),
                    range: rng(pos(2, 1, 3), pos(3, 1, 4)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(3, 1, 4), pos(3, 1, 4)),
                },
            ],
        },
        Case {
            input: b"\t\t",
            want: vec![Token {
                ty: TokenType::EOF,
                bytes: b"".to_vec(),
                range: rng(pos(2, 1, 3), pos(2, 1, 3)),
            }],
        },
        // Misc combinations that have come up in bug reports, etc.
        Case {
            input: b"locals {\n  is_percent = percent_sign == \"%\" ? true : false\n}\n",
            want: vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"locals".to_vec(),
                    range: rng(pos(0, 1, 1), pos(6, 1, 7)),
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    range: rng(pos(7, 1, 8), pos(8, 1, 9)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(8, 1, 9), pos(9, 2, 1)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"is_percent".to_vec(),
                    range: rng(pos(11, 2, 3), pos(21, 2, 13)),
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    range: rng(pos(22, 2, 14), pos(23, 2, 15)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"percent_sign".to_vec(),
                    range: rng(pos(24, 2, 16), pos(36, 2, 28)),
                },
                Token {
                    ty: TokenType::EqualOp,
                    bytes: b"==".to_vec(),
                    range: rng(pos(37, 2, 29), pos(39, 2, 31)),
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(40, 2, 32), pos(41, 2, 33)),
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"%".to_vec(),
                    range: rng(pos(41, 2, 33), pos(42, 2, 34)),
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    range: rng(pos(42, 2, 34), pos(43, 2, 35)),
                },
                Token {
                    ty: TokenType::Question,
                    bytes: b"?".to_vec(),
                    range: rng(pos(44, 2, 36), pos(45, 2, 37)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    range: rng(pos(46, 2, 38), pos(50, 2, 42)),
                },
                Token {
                    ty: TokenType::Colon,
                    bytes: b":".to_vec(),
                    range: rng(pos(51, 2, 43), pos(52, 2, 44)),
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"false".to_vec(),
                    range: rng(pos(53, 2, 45), pos(58, 2, 50)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(58, 2, 50), pos(59, 3, 1)),
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    range: rng(pos(59, 3, 1), pos(60, 3, 2)),
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    range: rng(pos(60, 3, 2), pos(61, 4, 1)),
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    range: rng(pos(61, 4, 1), pos(61, 4, 1)),
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
            "case {i} (input {:?}): wrong result",
            String::from_utf8_lossy(test.input),
        );
    }
}
