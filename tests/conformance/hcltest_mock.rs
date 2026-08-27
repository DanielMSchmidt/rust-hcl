//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hcltest/mock_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;

use cty::Value;
use hcl::hcltest::{mock_attrs, mock_body, mock_expr_literal};
use hcl::{
    AttributeSchema, Attributes, Block, BlockHeaderSchema, Blocks, BodyContent, BodySchema,
    ExprRef, KeyValuePair, Range,
};

// NOTE(port): upstream's compile-time interface assertions
// (`var _ hcl.Body = mockBody{}`, `var _ hcl.Expression = mockExprLiteral{}`,
// `var _ hcl.Expression = mockExprVariable("")`) have no direct analogue:
// the mock types are private to `hcl::hcltest`, and the `mock_body` /
// `mock_expr_literal` / `mock_expr_variable` signatures returning
// `BodyRef` / `ExprRef` already require the trait impls at compile time.

/// A `BodyContent` with the given attributes and blocks and all other fields
/// left as their Go zero values (Go: `&hcl.BodyContent{Attributes: ...,
/// Blocks: ...}`).
fn body_content(attributes: Attributes, blocks: Blocks) -> BodyContent {
    BodyContent {
        attributes,
        blocks,
        missing_item_range: Range::default(),
    }
}

/// A block with the given type and labels and all other fields left as their
/// Go zero values (Go: `hcl.Block{Type: ..., Labels: ...}`).
///
/// NOTE(port): Go leaves `Block.Body` as a nil interface; `BodyRef` cannot be
/// nil, so an empty mock body stands in on both the input and expectation
/// sides, which compare equal under deep equality the same way Go's nil
/// bodies do under `reflect.DeepEqual`.
fn block(block_type: &str, labels: &[&str]) -> Block {
    Block {
        block_type: block_type.to_string(),
        labels: labels.iter().map(|s| s.to_string()).collect(),
        body: mock_body(BodyContent::default()),
        def_range: Range::default(),
        type_range: Range::default(),
        label_ranges: Vec::new(),
    }
}

// Ported from TestMockBodyPartialContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcltest/mock_test.go#L21
#[test]
#[ignore = "not yet implemented"]
fn mock_body_partial_content() {
    struct Case {
        name: &'static str,
        input: BodyContent,
        schema: BodySchema,
        want: BodyContent,
        remain: BodyContent,
        diag_count: usize,
    }

    let tests = vec![
        Case {
            name: "empty",
            input: body_content(Attributes::new(), Blocks::default()),
            schema: BodySchema::default(),
            want: body_content(Attributes::new(), Blocks::default()),
            remain: body_content(Attributes::new(), Blocks::default()),
            diag_count: 0,
        },
        Case {
            name: "attribute requested",
            input: body_content(
                mock_attrs(HashMap::from([(
                    "name".to_string(),
                    mock_expr_literal(Value::string("Ermintrude")),
                )])),
                Blocks::default(),
            ),
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: false,
                }],
                blocks: Vec::new(),
            },
            want: body_content(
                mock_attrs(HashMap::from([(
                    "name".to_string(),
                    mock_expr_literal(Value::string("Ermintrude")),
                )])),
                Blocks::default(),
            ),
            remain: body_content(Attributes::new(), Blocks::default()),
            diag_count: 0,
        },
        Case {
            name: "attribute remains",
            input: body_content(
                mock_attrs(HashMap::from([(
                    "name".to_string(),
                    mock_expr_literal(Value::string("Ermintrude")),
                )])),
                Blocks::default(),
            ),
            schema: BodySchema::default(),
            want: body_content(Attributes::new(), Blocks::default()),
            remain: body_content(
                mock_attrs(HashMap::from([(
                    "name".to_string(),
                    mock_expr_literal(Value::string("Ermintrude")),
                )])),
                Blocks::default(),
            ),
            diag_count: 0,
        },
        Case {
            name: "attribute missing",
            input: body_content(Attributes::new(), Blocks::default()),
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: true,
                }],
                blocks: Vec::new(),
            },
            want: body_content(Attributes::new(), Blocks::default()),
            remain: body_content(Attributes::new(), Blocks::default()),
            diag_count: 1, // missing attribute "name"
        },
        Case {
            name: "block requested, no labels",
            input: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            schema: BodySchema {
                attributes: Vec::new(),
                blocks: vec![BlockHeaderSchema {
                    block_type: "baz".to_string(),
                    label_names: Vec::new(),
                }],
            },
            want: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            remain: body_content(Attributes::new(), Blocks::default()),
            diag_count: 0,
        },
        Case {
            name: "block requested, wrong labels",
            input: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            schema: BodySchema {
                attributes: Vec::new(),
                blocks: vec![BlockHeaderSchema {
                    block_type: "baz".to_string(),
                    label_names: vec!["foo".to_string()],
                }],
            },
            want: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            remain: body_content(Attributes::new(), Blocks::default()),
            diag_count: 1, // "baz" requires 1 label
        },
        Case {
            name: "block remains",
            input: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            schema: BodySchema::default(),
            want: body_content(Attributes::new(), Blocks::default()),
            remain: body_content(Attributes::new(), Blocks(vec![block("baz", &[])])),
            diag_count: 0,
        },
        Case {
            name: "various",
            input: body_content(
                mock_attrs(HashMap::from([
                    (
                        "name".to_string(),
                        mock_expr_literal(Value::string("Ermintrude")),
                    ),
                    ("age".to_string(), mock_expr_literal(Value::number_int(32))),
                ])),
                Blocks(vec![
                    block("baz", &[]),
                    block("bar", &["foo1"]),
                    block("bar", &["foo2"]),
                ]),
            ),
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: false,
                }],
                blocks: vec![BlockHeaderSchema {
                    block_type: "bar".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            want: body_content(
                mock_attrs(HashMap::from([(
                    "name".to_string(),
                    mock_expr_literal(Value::string("Ermintrude")),
                )])),
                Blocks(vec![block("bar", &["foo1"]), block("bar", &["foo2"])]),
            ),
            remain: body_content(
                mock_attrs(HashMap::from([(
                    "age".to_string(),
                    mock_expr_literal(Value::number_int(32)),
                )])),
                Blocks(vec![block("baz", &[])]),
            ),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let name = test.name;
        let in_body = mock_body(test.input);
        let (got, remain_body, diags) = in_body.partial_content(&test.schema);
        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i} ({name}): wrong number of diagnostics: {diags}",
        );

        assert_eq!(got, test.want, "case {i} ({name}): wrong result");

        // NOTE(port): Go downcasts with `remainBody.(mockBody).C` and
        // compares the content with reflect.DeepEqual; the Rust mock body
        // type is private, so compare the remain handle against a fresh mock
        // body wrapping the expected content via BodyRef's deep equality,
        // which checks the same type-and-content property.
        assert_eq!(
            remain_body,
            mock_body(test.remain),
            "case {i} ({name}): wrong remain",
        );
    }
}

// Ported from TestExprList:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcltest/mock_test.go#L275
#[test]
#[ignore = "not yet implemented"]
fn expr_list() {
    struct Case {
        name: &'static str,
        input: ExprRef,
        want: Vec<ExprRef>,
        diags: &'static str,
    }

    let tests = vec![
        Case {
            name: "as list",
            input: mock_expr_literal(Value::list([Value::string("foo"), Value::string("bar")])),
            want: vec![
                mock_expr_literal(Value::string("foo")),
                mock_expr_literal(Value::string("bar")),
            ],
            diags: "",
        },
        Case {
            name: "as tuple",
            input: mock_expr_literal(Value::tuple([Value::string("foo"), Value::string("bar")])),
            want: vec![
                mock_expr_literal(Value::string("foo")),
                mock_expr_literal(Value::string("bar")),
            ],
            diags: "",
        },
        Case {
            name: "not list",
            input: mock_expr_literal(Value::object([
                ("a", Value::string("foo")),
                ("b", Value::string("bar")),
            ])),
            // NOTE(port): Go's `Want: nil` (nil slice) maps to an empty Vec.
            want: Vec::new(),
            diags: "list expression is required",
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let name = test.name;
        let (got, diags) = hcl::expr_list(&*test.input);
        if !test.diags.is_empty() {
            if diags.has_errors() {
                assert!(
                    format!("{diags}").contains(test.diags),
                    "case {i} ({name}): expected error {:?}, got {diags:?}",
                    test.diags,
                );
            }
            assert!(
                diags.has_errors(),
                "case {i} ({name}): expected diagnostic message {:?}",
                test.diags,
            );
        } else {
            assert!(
                !diags.has_errors(),
                "case {i} ({name}): unexpected diagnostics: {diags}",
            );
        }

        assert_eq!(got, test.want, "case {i} ({name}): incorrect expression");
    }
}

// Ported from TestExprMap:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcltest/mock_test.go#L332
#[test]
#[ignore = "not yet implemented"]
fn expr_map() {
    struct Case {
        name: &'static str,
        input: ExprRef,
        want: Vec<KeyValuePair>,
        diags: &'static str,
    }

    let tests = vec![
        Case {
            name: "as object",
            input: mock_expr_literal(Value::object([
                ("name", Value::string("test")),
                ("count", Value::number_int(2)),
            ])),
            want: vec![
                KeyValuePair {
                    key: mock_expr_literal(Value::string("count")),
                    value: mock_expr_literal(Value::number_int(2)),
                },
                KeyValuePair {
                    key: mock_expr_literal(Value::string("name")),
                    value: mock_expr_literal(Value::string("test")),
                },
            ],
            diags: "",
        },
        Case {
            name: "as map",
            input: mock_expr_literal(Value::map([
                ("name", Value::string("test")),
                ("version", Value::string("2.0.0")),
            ])),
            want: vec![
                KeyValuePair {
                    key: mock_expr_literal(Value::string("name")),
                    value: mock_expr_literal(Value::string("test")),
                },
                KeyValuePair {
                    key: mock_expr_literal(Value::string("version")),
                    value: mock_expr_literal(Value::string("2.0.0")),
                },
            ],
            diags: "",
        },
        Case {
            name: "not map",
            input: mock_expr_literal(Value::list([Value::string("foo"), Value::string("bar")])),
            // NOTE(port): Go's `Want: nil` (nil slice) maps to an empty Vec.
            want: Vec::new(),
            diags: "map expression is required",
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let name = test.name;
        let (got, diags) = hcl::expr_map(&*test.input);
        if !test.diags.is_empty() {
            if diags.has_errors() {
                assert!(
                    format!("{diags}").contains(test.diags),
                    "case {i} ({name}): expected error {:?}, got {diags:?}",
                    test.diags,
                );
            }
            assert!(
                diags.has_errors(),
                "case {i} ({name}): expected diagnostic message {:?}",
                test.diags,
            );
        } else {
            assert!(
                !diags.has_errors(),
                "case {i} ({name}): unexpected diagnostics: {diags}",
            );
        }

        assert_eq!(got, test.want, "case {i} ({name}): incorrect expression");
    }
}
