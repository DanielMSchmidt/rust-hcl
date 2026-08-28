//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   json/structure_test.go (TestBodyPartialContent)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::json::{self, Node};
use hcl::{
    Attribute, AttributeSchema, Attributes, Block, BlockHeaderSchema, Blocks, BodyContent, BodyRef,
    BodySchema, ExprRef, Pos, Range,
};

/// `hcl.Range{Filename: "test.json", Start: ..., End: ...}` with literal
/// `(line, column, byte)` positions.
fn rng(start: (usize, usize, usize), end: (usize, usize, usize)) -> Range {
    Range {
        filename: "test.json".to_string(),
        start: Pos {
            line: start.0,
            column: start.1,
            byte: start.2,
        },
        end: Pos {
            line: end.0,
            column: end.1,
            byte: end.2,
        },
    }
}

/// Go: `&expression{src: &stringVal{Value: ..., SrcRange: ...}}`.
///
/// NOTE(port): upstream's `deep.Equal` does not descend into the json
/// package's unexported `expression.src` field, so some of these upstream
/// literals are internally inconsistent (e.g. the final case's string
/// range); they are copied verbatim per the porting rules.
fn string_expr(value: &str, src_range: Range) -> ExprRef {
    ExprRef::new(json::Expression {
        src: Node::String {
            value: value.to_string(),
            src_range,
        },
    })
}

/// Go: `&body{val: &objectVal{Attrs: []*objectAttr{}, ...}}` — an empty
/// JSON object body, as appears inside expected blocks.
///
/// NOTE(port): upstream's `deep.Equal` does not descend into the json
/// package's unexported `body.val` field, so some of these upstream range
/// literals are internally inconsistent (e.g. the second block of the
/// `{"resource":{"foo_instance":[{"bar":{}}, {"bar":{}}]}}` case reuses the
/// first block's body ranges); they are copied verbatim per the porting
/// rules.
fn empty_object_body(src_range: Range, open_range: Range, close_range: Range) -> BodyRef {
    BodyRef::new(json::Body {
        val: Node::Object {
            attrs: vec![],
            src_range,
            open_range,
            close_range,
        },
    })
}

// Ported from TestBodyPartialContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L18
#[test]
#[ignore = "not yet implemented"]
fn body_partial_content() {
    struct Case {
        src: &'static str,
        schema: BodySchema,
        want: BodyContent,
        diag_count: usize,
    }

    // NOTE(port): upstream leaves `BodyContent.Blocks` nil (or sets it to
    // `nil` explicitly) where no blocks are expected; the Rust `Blocks` has
    // a single empty representation, `Blocks::default()`.
    let tests = [
        Case {
            src: "{}",
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 2, 1), (1, 3, 2)),
            },
            diag_count: 0,
        },
        Case {
            src: "[]",
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 1, 0), (1, 2, 1)),
            },
            diag_count: 0,
        },
        Case {
            src: "[{}]",
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 1, 0), (1, 2, 1)),
            },
            diag_count: 0,
        },
        Case {
            src: "[[]]",
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 1, 0), (1, 2, 1)),
            },
            diag_count: 1, // elements of root array must be objects
        },
        Case {
            src: r#"{"//": "comment that should be ignored"}"#,
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 40, 39), (1, 41, 40)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"//": "comment that should be ignored", "//": "another comment"}"#,
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 65, 64), (1, 66, 65)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"name":"Ermintrude"}"#,
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: string_expr("Ermintrude", rng((1, 9, 8), (1, 21, 20))),
                        range: rng((1, 2, 1), (1, 21, 20)),
                        name_range: rng((1, 2, 1), (1, 8, 7)),
                    },
                )]),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 21, 20), (1, 22, 21)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"[{"name":"Ermintrude"}]"#,
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: string_expr("Ermintrude", rng((1, 10, 9), (1, 22, 21))),
                        range: rng((1, 3, 2), (1, 22, 21)),
                        name_range: rng((1, 3, 2), (1, 9, 8)),
                    },
                )]),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 1, 0), (1, 2, 1)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"name":"Ermintrude"}"#,
            schema: BodySchema {
                attributes: vec![
                    AttributeSchema {
                        name: "name".to_string(),
                        required: true,
                    },
                    AttributeSchema {
                        name: "age".to_string(),
                        required: true,
                    },
                ],
                blocks: vec![],
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: string_expr("Ermintrude", rng((1, 9, 8), (1, 21, 20))),
                        range: rng((1, 2, 1), (1, 21, 20)),
                        name_range: rng((1, 2, 1), (1, 8, 7)),
                    },
                )]),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 21, 20), (1, 22, 21)),
            },
            diag_count: 1,
        },
        Case {
            src: r#"{"resource": null}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec![],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                // We don't find any blocks if the value is json null.
                blocks: Blocks::default(),
                missing_item_range: rng((1, 18, 17), (1, 19, 18)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"resource": { "nested": null }}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 32, 31), (1, 33, 32)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"resource":{}}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec![],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![Block {
                    block_type: "resource".to_string(),
                    labels: vec![],
                    body: empty_object_body(
                        rng((1, 13, 12), (1, 15, 14)),
                        rng((1, 13, 12), (1, 14, 13)),
                        rng((1, 14, 13), (1, 15, 14)),
                    ),
                    def_range: rng((1, 13, 12), (1, 14, 13)),
                    type_range: rng((1, 2, 1), (1, 12, 11)),
                    label_ranges: vec![],
                }]),
                missing_item_range: rng((1, 15, 14), (1, 16, 15)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"resource":[{},{}]}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec![],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "resource".to_string(),
                        labels: vec![],
                        body: empty_object_body(
                            rng((1, 14, 13), (1, 16, 15)),
                            rng((1, 14, 13), (1, 15, 14)),
                            rng((1, 15, 14), (1, 16, 15)),
                        ),
                        def_range: rng((1, 13, 12), (1, 14, 13)),
                        type_range: rng((1, 2, 1), (1, 12, 11)),
                        label_ranges: vec![],
                    },
                    Block {
                        block_type: "resource".to_string(),
                        labels: vec![],
                        body: empty_object_body(
                            rng((1, 17, 16), (1, 19, 18)),
                            rng((1, 17, 16), (1, 18, 17)),
                            rng((1, 18, 17), (1, 19, 18)),
                        ),
                        def_range: rng((1, 13, 12), (1, 14, 13)),
                        type_range: rng((1, 2, 1), (1, 12, 11)),
                        label_ranges: vec![],
                    },
                ]),
                missing_item_range: rng((1, 20, 19), (1, 21, 20)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"resource":{"foo_instance":{"bar":{}}}}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec!["type".to_string(), "name".to_string()],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![Block {
                    block_type: "resource".to_string(),
                    labels: vec!["foo_instance".to_string(), "bar".to_string()],
                    body: empty_object_body(
                        rng((1, 36, 35), (1, 38, 37)),
                        rng((1, 36, 35), (1, 37, 36)),
                        rng((1, 37, 36), (1, 38, 37)),
                    ),
                    def_range: rng((1, 36, 35), (1, 37, 36)),
                    type_range: rng((1, 2, 1), (1, 12, 11)),
                    label_ranges: vec![
                        rng((1, 14, 13), (1, 28, 27)),
                        rng((1, 30, 29), (1, 35, 34)),
                    ],
                }]),
                missing_item_range: rng((1, 40, 39), (1, 41, 40)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"resource":{"foo_instance":[{"bar":{}}, {"bar":{}}]}}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "resource".to_string(),
                    label_names: vec!["type".to_string(), "name".to_string()],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "resource".to_string(),
                        labels: vec!["foo_instance".to_string(), "bar".to_string()],
                        body: empty_object_body(
                            rng((1, 37, 36), (1, 39, 38)),
                            rng((1, 37, 36), (1, 38, 37)),
                            rng((1, 38, 37), (1, 39, 38)),
                        ),
                        def_range: rng((1, 37, 36), (1, 38, 37)),
                        type_range: rng((1, 2, 1), (1, 12, 11)),
                        label_ranges: vec![
                            rng((1, 14, 13), (1, 28, 27)),
                            rng((1, 31, 30), (1, 36, 35)),
                        ],
                    },
                    // NOTE(port): this second block's body ranges repeat the
                    // first block's, verbatim from upstream (see the note on
                    // `empty_object_body`).
                    Block {
                        block_type: "resource".to_string(),
                        labels: vec!["foo_instance".to_string(), "bar".to_string()],
                        body: empty_object_body(
                            rng((1, 37, 36), (1, 39, 38)),
                            rng((1, 37, 36), (1, 38, 37)),
                            rng((1, 38, 37), (1, 39, 38)),
                        ),
                        def_range: rng((1, 49, 48), (1, 50, 49)),
                        type_range: rng((1, 2, 1), (1, 12, 11)),
                        label_ranges: vec![
                            rng((1, 14, 13), (1, 28, 27)),
                            rng((1, 43, 42), (1, 48, 47)),
                        ],
                    },
                ]),
                missing_item_range: rng((1, 54, 53), (1, 55, 54)),
            },
            diag_count: 0,
        },
        Case {
            src: r#"{"name":"Ermintrude"}"#,
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "name".to_string(),
                    label_names: vec![],
                }],
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 21, 20), (1, 22, 21)),
            },
            diag_count: 1, // name is supposed to be a block
        },
        Case {
            src: r#"[{"name":"Ermintrude"},{"name":"Ermintrude"}]"#,
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        // NOTE(port): this string range is upstream's
                        // literal, even though the string actually starts at
                        // byte 9 in this source (see the note on
                        // `string_expr`).
                        expr: string_expr("Ermintrude", rng((1, 9, 8), (1, 21, 20))),
                        range: rng((1, 3, 2), (1, 22, 21)),
                        name_range: rng((1, 3, 2), (1, 9, 8)),
                    },
                )]),
                blocks: Blocks::default(),
                missing_item_range: rng((1, 1, 0), (1, 2, 1)),
            },
            diag_count: 1, // "name" attribute is defined twice
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (file, diags) = json::parse(test.src.as_bytes(), "test.json");
        assert!(
            diags.is_empty(),
            "case {i} ({}): Parse produced diagnostics: {diags:#?}",
            test.src,
        );

        let (got, _, diags) = file.body.partial_content(&test.schema);
        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i} ({}): wrong number of diagnostics {}; want {}; diags: {diags:#?}",
            test.src,
            diags.len(),
            test.diag_count,
        );

        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.src);
    }
}
