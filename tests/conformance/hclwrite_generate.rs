//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/generate_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::hclsyntax::TokenType;
use hcl::hclwrite::{self, ObjectAttrTokens, Token, Tokens};
use hcl::{Range, Traversal, Traverser};

// Ported from TestTokensForValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L18
#[test]
#[ignore = "not yet implemented"]
fn tokens_for_value() {
    struct Case {
        val: Value,
        want: Tokens,
    }

    let tests = [
        Case {
            val: Value::null(Type::dynamic()),
            want: Tokens(vec![Token {
                ty: TokenType::Ident,
                bytes: b"null".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            val: Value::bool(true),
            want: Tokens(vec![Token {
                ty: TokenType::Ident,
                bytes: b"true".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            val: Value::bool(false),
            want: Tokens(vec![Token {
                ty: TokenType::Ident,
                bytes: b"false".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            val: Value::number_int(0),
            want: Tokens(vec![Token {
                ty: TokenType::NumberLit,
                bytes: b"0".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            val: Value::number_float(0.5),
            want: Tokens(vec![Token {
                ty: TokenType::NumberLit,
                bytes: b"0.5".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            // NOTE(port): upstream constructs this number as a 512-bit
            // precision big.Float product 40000000 * 2000000 via
            // cty.NumberVal; rust-cty has no big-float constructor, so we
            // parse the same value at full cty precision instead.
            val: Value::parse_number("80000000000000").unwrap(),
            want: Tokens(vec![Token {
                ty: TokenType::NumberLit,
                bytes: b"80000000000000".to_vec(),
                spaces_before: 0,
            }]),
        },
        Case {
            val: Value::string(""),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string("foo"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string(r#""foo""#),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: br#"\"foo\""#.to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string("hello\nworld\n"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: br"hello\nworld\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string("hello\r\nworld\r\n"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: br"hello\r\nworld\r\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string(r"what\what"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: br"what\\what".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string("𝄞"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: "𝄞".as_bytes().to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::string("👩🏾"),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: "👩🏾".as_bytes().to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::empty_tuple(),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::tuple([Value::empty_tuple()]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::list_empty(Type::string()),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::set_empty(Type::bool()),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::tuple([Value::bool(true)]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::tuple([Value::bool(true), Value::number_int(0)]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Comma,
                    bytes: b",".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"0".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::empty_object(),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::map_empty(Type::bool()),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::object([("foo", Value::bool(true))]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::object([("foo", Value::bool(true)), ("bar", Value::number_int(0))]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"bar".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::NumberLit,
                    bytes: b"0".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            val: Value::object([("foo bar", Value::bool(true))]),
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"foo bar".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"true".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclwrite::tokens_for_value(&test.val);
        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}

// Ported from TestTokensForTraversal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L508
#[test]
#[ignore = "not yet implemented"]
fn tokens_for_traversal() {
    struct Case {
        val: Traversal,
        want: Tokens,
    }

    let tests = [Case {
        val: Traversal(vec![
            Traverser::Root {
                name: "root".to_string(),
                src_range: Range::default(),
            },
            Traverser::Attr {
                name: "attr".to_string(),
                src_range: Range::default(),
            },
            Traverser::Index {
                key: Value::string("index"),
                src_range: Range::default(),
            },
        ]),
        want: Tokens(vec![
            Token {
                ty: TokenType::Ident,
                bytes: b"root".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::Dot,
                bytes: b".".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::Ident,
                bytes: b"attr".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::OBrack,
                bytes: b"[".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::OQuote,
                bytes: b"\"".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::QuotedLit,
                bytes: b"index".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::CQuote,
                bytes: b"\"".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::CBrack,
                bytes: b"]".to_vec(),
                spaces_before: 0,
            },
        ]),
    }];

    for (i, test) in tests.iter().enumerate() {
        let got = hclwrite::tokens_for_traversal(&test.val);
        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}

// Ported from TestTokensForTuple:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L558
#[test]
#[ignore = "not yet implemented"]
fn tokens_for_tuple() {
    struct Case {
        name: &'static str,
        val: Vec<Tokens>,
        want: Tokens,
    }

    let tests = [
        Case {
            name: "no elements",
            val: vec![],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "one element",
            val: vec![hclwrite::tokens_for_value(&Value::string("foo"))],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "two elements",
            val: vec![
                hclwrite::tokens_for_traversal(&Traversal(vec![
                    Traverser::Root {
                        name: "root".to_string(),
                        src_range: Range::default(),
                    },
                    Traverser::Attr {
                        name: "attr".to_string(),
                        src_range: Range::default(),
                    },
                ])),
                hclwrite::tokens_for_value(&Value::string("foo")),
            ],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrack,
                    bytes: b"[".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"root".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Dot,
                    bytes: b".".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"attr".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Comma,
                    bytes: b",".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrack,
                    bytes: b"]".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
    ];

    for test in &tests {
        let got = hclwrite::tokens_for_tuple(test.val.clone());
        assert_eq!(got, test.want, "case {}: wrong result", test.name);
    }
}

// Ported from TestTokensForObject:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L631
#[test]
#[ignore = "not yet implemented"]
fn tokens_for_object() {
    struct Case {
        name: &'static str,
        val: Vec<ObjectAttrTokens>,
        want: Tokens,
    }

    let tests = [
        Case {
            name: "no attributes",
            val: vec![],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "one attribute",
            val: vec![ObjectAttrTokens {
                name: hclwrite::tokens_for_traversal(&Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: Range::default(),
                }])),
                value: hclwrite::tokens_for_value(&Value::string("baz")),
            }],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"bar".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"baz".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "two attributes",
            val: vec![
                ObjectAttrTokens {
                    name: hclwrite::tokens_for_traversal(&Traversal(vec![Traverser::Root {
                        name: "foo".to_string(),
                        src_range: Range::default(),
                    }])),
                    value: hclwrite::tokens_for_traversal(&Traversal(vec![
                        Traverser::Root {
                            name: "root".to_string(),
                            src_range: Range::default(),
                        },
                        Traverser::Attr {
                            name: "attr".to_string(),
                            src_range: Range::default(),
                        },
                    ])),
                },
                ObjectAttrTokens {
                    name: hclwrite::tokens_for_traversal(&Traversal(vec![Traverser::Root {
                        name: "bar".to_string(),
                        src_range: Range::default(),
                    }])),
                    value: hclwrite::tokens_for_value(&Value::string("baz")),
                },
            ],
            want: Tokens(vec![
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"root".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Dot,
                    bytes: b".".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"attr".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"bar".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::Equal,
                    bytes: b"=".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"baz".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
    ];

    for test in &tests {
        let got = hclwrite::tokens_for_object(test.val.clone());
        assert_eq!(got, test.want, "case {}: wrong result", test.name);
    }
}

// Ported from TestTokensForFunctionCall:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L729
#[test]
#[ignore = "not yet implemented"]
fn tokens_for_function_call() {
    struct Case {
        name: &'static str,
        func_name: &'static str,
        val: Vec<Tokens>,
        want: Tokens,
    }

    let tests = [
        Case {
            name: "no arguments",
            func_name: "uuid",
            val: vec![],
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"uuid".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OParen,
                    bytes: b"(".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CParen,
                    bytes: b")".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "one argument",
            func_name: "strlen",
            val: vec![hclwrite::tokens_for_value(&Value::string("hello"))],
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"strlen".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OParen,
                    bytes: b"(".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"hello".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CParen,
                    bytes: b")".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            name: "two arguments",
            func_name: "list",
            val: vec![
                hclwrite::tokens_for_identifier("string"),
                hclwrite::tokens_for_identifier("int"),
            ],
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"list".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OParen,
                    bytes: b"(".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"string".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Comma,
                    bytes: b",".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"int".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CParen,
                    bytes: b")".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
    ];

    for test in &tests {
        let got = hclwrite::tokens_for_function_call(test.func_name, test.val.clone());
        assert_eq!(got, test.want, "case {}: wrong result", test.name);
    }
}

// Ported from TestTokenGenerateConsistency:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/generate_test.go#L802
//
// This test verifies that different ways of generating equivalent token
// sequences all generate identical tokens, to help us keep them all in
// sync under future maintenance.
#[test]
#[ignore = "not yet implemented"]
fn token_generate_consistency() {
    // Subtest "tuple constructor".
    {
        let tests: [(&str, Vec<Value>); 3] = [
            ("no elements", vec![]),
            ("one element", vec![Value::string("hello")]),
            (
                "two elements",
                vec![Value::string("hello"), Value::string("world")],
            ),
        ];

        for (name, elems) in &tests {
            let list_val = if !elems.is_empty() {
                Value::list(elems.clone())
            } else {
                Value::list_empty(Type::dynamic())
            };
            let from_list_value = hclwrite::tokens_for_value(&list_val);
            let from_tuple_value = hclwrite::tokens_for_value(&Value::tuple(elems.clone()));
            let elem_tokens: Vec<Tokens> = elems.iter().map(hclwrite::tokens_for_value).collect();
            let from_tuple_tokens = hclwrite::tokens_for_tuple(elem_tokens);

            assert_eq!(
                from_list_value, from_tuple_tokens,
                "tuple constructor case {name}: inconsistency between tokens_for_value(list) and tokens_for_tuple",
            );
            assert_eq!(
                from_tuple_value, from_tuple_tokens,
                "tuple constructor case {name}: inconsistency between tokens_for_value(tuple) and tokens_for_tuple",
            );
        }
    }

    // Subtest "object constructor".
    {
        let tests: [(&str, Vec<(&str, Value)>); 3] = [
            ("no elements", vec![]),
            ("one element", vec![("greeting", Value::string("hello"))]),
            (
                "two elements",
                vec![
                    ("greeting1", Value::string("hello")),
                    ("greeting2", Value::string("world")),
                ],
            ),
        ];

        for (name, attrs) in &tests {
            let map_val = if !attrs.is_empty() {
                Value::map(attrs.clone())
            } else {
                Value::map_empty(Type::dynamic())
            };
            let from_map_value = hclwrite::tokens_for_value(&map_val);
            let from_object_value = hclwrite::tokens_for_value(&Value::object(attrs.clone()));

            // TokensForValue always writes the keys/attributes in cty's
            // standard iteration order, but TokensForObject gives the
            // caller direct control of the ordering. The result is
            // therefore consistent only if the given attributes are
            // pre-sorted into the same iteration order, which is a lexical
            // sort by attribute name.
            let mut keys: Vec<&str> = attrs.iter().map(|(k, _)| *k).collect();
            keys.sort_unstable();
            let mut attr_tokens: Vec<ObjectAttrTokens> = Vec::with_capacity(attrs.len());
            for k in keys {
                let v = &attrs.iter().find(|(ak, _)| *ak == k).unwrap().1;
                attr_tokens.push(ObjectAttrTokens {
                    name: hclwrite::tokens_for_identifier(k),
                    value: hclwrite::tokens_for_value(v),
                });
            }
            let from_object_tokens = hclwrite::tokens_for_object(attr_tokens);

            assert_eq!(
                from_map_value, from_object_tokens,
                "object constructor case {name}: inconsistency between tokens_for_value(map) and tokens_for_object",
            );
            assert_eq!(
                from_object_value, from_object_tokens,
                "object constructor case {name}: inconsistency between tokens_for_value(object) and tokens_for_object",
            );
        }
    }
}
