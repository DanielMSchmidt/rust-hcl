//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/parser_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::{self, TokenType};
use hcl::hclwrite::{self, TestTreeNode, Token, Tokens};
use hcl::{Pos, Range};

/// A [`TestTreeNode`] literal (Go: a `TestTreeNode{Type: ..., Val: ...,
/// Children: ...}` struct literal with zero-value fields omitted).
fn tn(node_type: &str, val: &str, children: Vec<TestTreeNode>) -> TestTreeNode {
    TestTreeNode {
        node_type: node_type.to_string(),
        val: val.to_string(),
        children,
    }
}

/// A range carrying only byte offsets (Go: `hcl.Range{Start:
/// hcl.Pos{Byte: ...}, End: hcl.Pos{Byte: ...}}` with `Filename` and the
/// other `Pos` fields left as their zero values).
fn byte_range(start_byte: usize, end_byte: usize) -> Range {
    Range {
        filename: String::new(),
        start: Pos {
            line: 0,
            column: 0,
            byte: start_byte,
        },
        end: Pos {
            line: 0,
            column: 0,
            byte: end_byte,
        },
    }
}

// Ported from TestParse:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/parser_test.go#L17
//
// NOTE(port): upstream drives the unexported `parse` function directly and
// calls `makeTestTree(file.body)` on the resulting file's root body node
// (`file.body` is a `*node` whose content is the `*hclwrite.Body`).
// `hclwrite.ParseConfig` is a thin wrapper around `parse`, so per
// docs/api-mapping.md the Rust port drives `hclwrite::parse_config` and
// `hclwrite::make_test_tree(&file)`, which builds the tree from the file's
// root body — the roots are identical (`node_type: "Body"`).
//
// NOTE(port): `makeTestTree` (hclwrite/ast_test.go#L18) derives each
// node's `Type` from the Go dynamic type name of the node's content with
// the `hclwrite.`/`*hclwrite.` prefix stripped (hence "Body", "Attribute",
// "Tokens", and lowercase names like "identifier", "comments",
// "blockLabels", "quoted", "number" for unexported types), takes `Val`
// from the content's `testValue()` when implemented, and otherwise — for
// childless leaves — from the raw bytes of `BuildTokens(nil)`. The Rust
// `make_test_tree` mirrors those semantics, so the `node_type`/`val`
// strings below are ported byte-for-byte.
#[test]
#[ignore = "not yet implemented"]
fn parse() {
    struct Case {
        src: &'static str,
        want: TestTreeNode,
    }

    let tests = [
        Case {
            src: "",
            want: tn("Body", "", vec![]),
        },
        Case {
            src: "a = 1\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn("Expression", "", vec![tn("Tokens", " 1", vec![])]),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "# aye aye aye\na = 1\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "# aye aye aye\n", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn("Expression", "", vec![tn("Tokens", " 1", vec![])]),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = 1 # because it is\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn("Expression", "", vec![tn("Tokens", " 1", vec![])]),
                        tn("comments", " # because it is\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            // two newlines separate the comment from the attribute
            src: "# bee bee bee\n\nb = 1\n",
            want: tn(
                "Body",
                "",
                vec![
                    // Only lead/line comments attached to an object have
                    // type "comments"
                    tn("Tokens", "# bee bee bee\n\n", vec![]),
                    tn(
                        "Attribute",
                        "",
                        vec![
                            tn("comments", "", vec![]),
                            tn("identifier", "b", vec![]),
                            tn("Tokens", " =", vec![]),
                            tn("Expression", "", vec![tn("Tokens", " 1", vec![])]),
                            tn("comments", "", vec![]),
                            tn("Tokens", "\n", vec![]),
                        ],
                    ),
                ],
            ),
        },
        Case {
            src: "a = (\n  1 + 2\n)\nb = 3\n",
            want: tn(
                "Body",
                "",
                vec![
                    tn(
                        "Attribute",
                        "",
                        vec![
                            tn("comments", "", vec![]),
                            tn("identifier", "a", vec![]),
                            tn("Tokens", " =", vec![]),
                            tn(
                                "Expression",
                                "",
                                vec![tn("Tokens", " (\n  1 + 2\n)", vec![])],
                            ),
                            tn("comments", "", vec![]),
                            tn("Tokens", "\n", vec![]),
                        ],
                    ),
                    tn(
                        "Attribute",
                        "",
                        vec![
                            tn("comments", "", vec![]),
                            tn("identifier", "b", vec![]),
                            tn("Tokens", " =", vec![]),
                            tn("Expression", "", vec![tn("Tokens", " 3", vec![])]),
                            tn("comments", "", vec![]),
                            tn("Tokens", "\n", vec![]),
                        ],
                    ),
                ],
            ),
        },
        Case {
            src: "b {}\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Block",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "b", vec![]),
                        tn("blockLabels", "", vec![]),
                        tn("Tokens", " {", vec![]),
                        tn("Body", "", vec![]),
                        tn("Tokens", "}", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "b label {}\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Block",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "b", vec![]),
                        tn("blockLabels", "", vec![tn("identifier", " label", vec![])]),
                        tn("Tokens", " {", vec![]),
                        tn("Body", "", vec![]),
                        tn("Tokens", "}", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "b \"label\" {}\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Block",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "b", vec![]),
                        tn("blockLabels", "", vec![tn("quoted", " \"label\"", vec![])]),
                        tn("Tokens", " {", vec![]),
                        tn("Body", "", vec![]),
                        tn("Tokens", "}", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "b \"label1\" /* foo */ \"label2\" {}\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Block",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "b", vec![]),
                        tn(
                            "blockLabels",
                            "",
                            vec![
                                tn("quoted", " \"label1\"", vec![]),
                                // The comment between the labels just
                                // becomes an "unstructured tokens"
                                // node, because this isn't a place
                                // where we expect comments to attach
                                // to a particular object as
                                // documentation.
                                tn("Tokens", " /* foo */", vec![]),
                                tn("quoted", " \"label2\"", vec![]),
                            ],
                        ),
                        tn("Tokens", " {", vec![]),
                        tn("Body", "", vec![]),
                        tn("Tokens", "}", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "b {\n  a = 1\n}\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Block",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "b", vec![]),
                        tn("blockLabels", "", vec![]),
                        tn("Tokens", " {", vec![]),
                        tn(
                            "Body",
                            "",
                            vec![
                                tn("Tokens", "\n", vec![]),
                                tn(
                                    "Attribute",
                                    "",
                                    vec![
                                        tn("comments", "", vec![]),
                                        tn("identifier", "  a", vec![]),
                                        tn("Tokens", " =", vec![]),
                                        tn("Expression", "", vec![tn("Tokens", " 1", vec![])]),
                                        tn("comments", "", vec![]),
                                        tn("Tokens", "\n", vec![]),
                                    ],
                                ),
                            ],
                        ),
                        tn("Tokens", "}", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![tn(
                                "Traversal",
                                "",
                                vec![tn(
                                    "TraverseName",
                                    "",
                                    vec![tn("identifier", " foo", vec![])],
                                )],
                            )],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo.bar\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![tn(
                                "Traversal",
                                "",
                                vec![
                                    tn("TraverseName", "", vec![tn("identifier", " foo", vec![])]),
                                    tn(
                                        "TraverseName",
                                        "",
                                        vec![
                                            tn("Tokens", ".", vec![]),
                                            tn("identifier", "bar", vec![]),
                                        ],
                                    ),
                                ],
                            )],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[0]\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![tn(
                                "Traversal",
                                "",
                                vec![
                                    tn("TraverseName", "", vec![tn("identifier", " foo", vec![])]),
                                    tn(
                                        "TraverseIndex",
                                        "",
                                        vec![
                                            tn("Tokens", "[", vec![]),
                                            tn("number", "0", vec![]),
                                            tn("Tokens", "]", vec![]),
                                        ],
                                    ),
                                ],
                            )],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo.0\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![tn(
                                "Traversal",
                                "",
                                vec![
                                    tn("TraverseName", "", vec![tn("identifier", " foo", vec![])]),
                                    tn(
                                        "TraverseIndex",
                                        "",
                                        vec![tn("Tokens", ".", vec![]), tn("number", "0", vec![])],
                                    ),
                                ],
                            )],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo.*\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", ".*", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo.*.bar\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", ".*.bar", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[*]\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", "[*]", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[*].bar\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", "[*].bar", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[bar]\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", "[", vec![]),
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", "bar", vec![])],
                                    )],
                                ),
                                tn("Tokens", "]", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[bar.baz]\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", "[", vec![]),
                                tn(
                                    "Traversal",
                                    "",
                                    vec![
                                        tn(
                                            "TraverseName",
                                            "",
                                            vec![tn("identifier", "bar", vec![])],
                                        ),
                                        tn(
                                            "TraverseName",
                                            "",
                                            vec![
                                                tn("Tokens", ".", vec![]),
                                                tn("identifier", "baz", vec![]),
                                            ],
                                        ),
                                    ],
                                ),
                                tn("Tokens", "]", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
        Case {
            src: "a = foo[bar].baz\n",
            want: tn(
                "Body",
                "",
                vec![tn(
                    "Attribute",
                    "",
                    vec![
                        tn("comments", "", vec![]),
                        tn("identifier", "a", vec![]),
                        tn("Tokens", " =", vec![]),
                        tn(
                            "Expression",
                            "",
                            vec![
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", " foo", vec![])],
                                    )],
                                ),
                                tn("Tokens", "[", vec![]),
                                tn(
                                    "Traversal",
                                    "",
                                    vec![tn(
                                        "TraverseName",
                                        "",
                                        vec![tn("identifier", "bar", vec![])],
                                    )],
                                ),
                                tn("Tokens", "].baz", vec![]),
                            ],
                        ),
                        tn("comments", "", vec![]),
                        tn("Tokens", "\n", vec![]),
                    ],
                )],
            ),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i} ({:?}): unexpected diagnostics: {diags}",
            test.src,
        );

        let got = hclwrite::make_test_tree(&file);

        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ninput:\n{}",
            test.src,
        );
    }
}

// Ported from TestPartitionTokens:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/parser_test.go#L1231
#[test]
#[ignore = "not yet implemented"]
fn partition_tokens() {
    struct Case {
        tokens: hclsyntax::Tokens,
        rng: Range,
        want_start: usize,
        want_end: usize,
    }

    let tests = [
        Case {
            tokens: vec![],
            rng: byte_range(0, 0),
            want_start: 0,
            want_end: 0,
        },
        Case {
            tokens: vec![hclsyntax::Token {
                ty: TokenType::Ident,
                range: byte_range(0, 4),
                ..Default::default()
            }],
            rng: byte_range(0, 4),
            want_start: 0,
            want_end: 1,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(0, 4),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(4, 8),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(8, 12),
                    ..Default::default()
                },
            ],
            rng: byte_range(4, 8),
            want_start: 1,
            want_end: 2,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(0, 4),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(4, 8),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(8, 12),
                    ..Default::default()
                },
            ],
            rng: byte_range(0, 8),
            want_start: 0,
            want_end: 2,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(0, 4),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(4, 8),
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Ident,
                    range: byte_range(8, 12),
                    ..Default::default()
                },
            ],
            rng: byte_range(4, 12),
            want_start: 1,
            want_end: 3,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got_start, got_end) = hclwrite::partition_tokens(&test.tokens, test.rng.clone());

        assert!(
            got_start == test.want_start && got_end == test.want_end,
            "case {i}: wrong result\ntokens: {:?}\nrange: {:?}\ngot:   {got_start}, {got_end}\nwant:  {}, {}",
            test.tokens,
            test.rng,
            test.want_start,
            test.want_end,
        );
    }
}

// Ported from TestPartitionLeadCommentTokens:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/parser_test.go#L1375
#[test]
#[ignore = "not yet implemented"]
fn partition_lead_comment_tokens() {
    struct Case {
        tokens: hclsyntax::Tokens,
        want_start: usize,
    }

    let tests = [
        Case {
            tokens: vec![],
            want_start: 0,
        },
        Case {
            tokens: vec![hclsyntax::Token {
                ty: TokenType::Comment,
                ..Default::default()
            }],
            want_start: 0,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Comment,
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Comment,
                    ..Default::default()
                },
            ],
            want_start: 0,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Comment,
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Newline,
                    ..Default::default()
                },
            ],
            want_start: 2,
        },
        Case {
            tokens: vec![
                hclsyntax::Token {
                    ty: TokenType::Comment,
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Newline,
                    ..Default::default()
                },
                hclsyntax::Token {
                    ty: TokenType::Comment,
                    ..Default::default()
                },
            ],
            want_start: 2,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got_start = hclwrite::partition_lead_comment_tokens(&test.tokens);

        assert_eq!(
            got_start, test.want_start,
            "case {i}: wrong result\ntokens: {:?}",
            test.tokens,
        );
    }
}

// Ported from TestLexConfig:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/parser_test.go#L1445
#[test]
#[ignore = "not yet implemented"]
fn lex_config() {
    struct Case {
        input: &'static str,
        want: Tokens,
    }

    let tests = [
        Case {
            input: "a  b ",
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"a".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"b".to_vec(),
                    spaces_before: 2,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    spaces_before: 1,
                },
            ]),
        },
        Case {
            input: "\nfoo \"bar\" \"baz\" {\n    pizza = \" cheese \"\n}\n",
            want: Tokens(vec![
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"bar".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
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
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::Ident,
                    bytes: b"pizza".to_vec(),
                    spaces_before: 4,
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
                    bytes: b" cheese ".to_vec(),
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
                Token {
                    ty: TokenType::Newline,
                    bytes: b"\n".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: vec![],
                    spaces_before: 0,
                },
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclwrite::lex_config(test.input.as_bytes());

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.input,);
    }
}
