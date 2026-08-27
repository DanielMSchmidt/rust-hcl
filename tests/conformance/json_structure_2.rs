//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   json/structure_test.go (part 2: TestBodyContent onward;
//!   TestBodyPartialContent lives in json_structure_1.rs)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::json::{self, Node};
use hcl::{
    Attribute, AttributeSchema, Attributes, BodySchema, EvalContext, ExprRef, Expression as _, Pos,
    Range, Traversal, Traverser,
};

/// `hcl.Pos{Line: line, Column: column, Byte: byte}`.
fn pos(line: usize, column: usize, byte: usize) -> Pos {
    Pos { line, column, byte }
}

/// A range within `test.json`
/// (Go: `hcl.Range{Filename: "test.json", Start: ..., End: ...}`).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: "test.json".to_string(),
        start,
        end,
    }
}

// Ported from TestBodyContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1083
#[test]
#[ignore = "not yet implemented"]
fn body_content() {
    // Upstream: "We test most of the functionality already in
    // TestBodyPartialContent, so this test focuses on the handling of
    // extraneous attributes."
    struct Case {
        src: &'static str,
        schema: BodySchema,
        diag_count: usize,
    }

    let tests = [
        Case {
            src: r#"{"unknown": true}"#,
            schema: BodySchema::default(),
            diag_count: 1,
        },
        Case {
            src: r#"{"//": "comment that should be ignored"}"#,
            schema: BodySchema::default(),
            diag_count: 0,
        },
        Case {
            src: r#"{"unknow": true}"#,
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "unknown".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            diag_count: 1,
        },
        Case {
            src: r#"{"unknow": true, "unnown": true}"#,
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "unknown".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            diag_count: 2,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, diags) = json::parse(test.src.as_bytes(), "test.json");
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): Parse produced diagnostics: {diags}",
            test.src,
        );
        let (_, diags) = file.body.content(&test.schema);
        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i} ({}): wrong number of diagnostics {}; want {}; diags: {diags:#?}",
            test.src,
            diags.len(),
            test.diag_count,
        );
    }
}

// Ported from TestJustAttributes:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1142
#[test]
#[ignore = "not yet implemented"]
fn just_attributes() {
    // Upstream: "We test most of the functionality already in
    // TestBodyPartialContent, so this test focuses on the handling of
    // extraneous attributes."
    struct Case {
        src: &'static str,
        want: Attributes,
        diag_count: usize,
    }

    let tests = [
        Case {
            src: r#"{}"#,
            want: Attributes::new(),
            diag_count: 0,
        },
        Case {
            src: r#"{"foo": true}"#,
            want: Attributes::from([(
                "foo".to_string(),
                Attribute {
                    name: "foo".to_string(),
                    expr: ExprRef::new(json::Expression {
                        src: Node::Boolean {
                            value: true,
                            src_range: rng(pos(1, 9, 8), pos(1, 13, 12)),
                        },
                    }),
                    range: rng(pos(1, 2, 1), pos(1, 13, 12)),
                    name_range: rng(pos(1, 2, 1), pos(1, 7, 6)),
                },
            )]),
            diag_count: 0,
        },
        Case {
            src: r#"{"//": "comment that should be ignored"}"#,
            want: Attributes::new(),
            diag_count: 0,
        },
        Case {
            src: r#"{"foo": true, "foo": true}"#,
            want: Attributes::from([(
                "foo".to_string(),
                Attribute {
                    name: "foo".to_string(),
                    expr: ExprRef::new(json::Expression {
                        src: Node::Boolean {
                            value: true,
                            src_range: rng(pos(1, 9, 8), pos(1, 13, 12)),
                        },
                    }),
                    range: rng(pos(1, 2, 1), pos(1, 13, 12)),
                    name_range: rng(pos(1, 2, 1), pos(1, 7, 6)),
                },
            )]),
            diag_count: 1, // attribute foo was already defined
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (file, diags) = json::parse(test.src.as_bytes(), "test.json");
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): Parse produced diagnostics: {diags}",
            test.src,
        );
        let (got, diags) = file.body.just_attributes();
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

// Ported from TestExpressionVariables:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1240
#[test]
#[ignore = "not yet implemented"]
fn expression_variables() {
    struct Case {
        src: &'static str,
        want: Vec<Traversal>,
    }

    let tests = [
        // NOTE(port): upstream's `Want` is a nil `[]hcl.Traversal` here;
        // Go's reflect.DeepEqual(nil, nil) becomes an empty-vec comparison.
        Case {
            src: r#"{"a":true}"#,
            want: vec![],
        },
        Case {
            src: r#"{"a":"${foo}"}"#,
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(pos(1, 9, 8), pos(1, 12, 11)),
            }])],
        },
        Case {
            src: r#"{"a":["${foo}"]}"#,
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(pos(1, 10, 9), pos(1, 13, 12)),
            }])],
        },
        Case {
            src: r#"{"a":{"b":"${foo}"}}"#,
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(pos(1, 14, 13), pos(1, 17, 16)),
            }])],
        },
        Case {
            src: r#"{"a":{"${foo}":"b"}}"#,
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(pos(1, 10, 9), pos(1, 13, 12)),
            }])],
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (file, diags) = json::parse(test.src.as_bytes(), "test.json");
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): Parse produced diagnostics: {diags}",
            test.src,
        );
        let (attrs, diags) = file.body.just_attributes();
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): JustAttributes produced diagnostics: {diags}",
            test.src,
        );
        let got = attrs["a"].expr.variables();
        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.src);
    }
}

// Ported from TestExpressionAsTraversal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1329
#[test]
#[ignore = "not yet implemented"]
fn expression_as_traversal() {
    let e = json::Expression {
        src: Node::String {
            value: "foo.bar[0]".to_string(),
            src_range: Range::default(),
        },
    };
    // NOTE(port): Go's `e.AsTraversal()` returns a nil traversal (length 0)
    // when the expression is not a traversal; `None` maps to length 0 here.
    let traversal = e.as_traversal();
    let len = traversal.as_ref().map_or(0, |t| t.len());
    assert_eq!(len, 3, "incorrect traversal {traversal:?}; want length 3");
}

// Ported from TestStaticExpressionList:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1341
#[test]
#[ignore = "not yet implemented"]
fn static_expression_list() {
    let e = json::Expression {
        src: Node::Array {
            values: vec![Node::String {
                value: "hello".to_string(),
                src_range: Range::default(),
            }],
            src_range: Range::default(),
            open_range: Range::default(),
        },
    };
    let exprs = e.expr_list().unwrap_or_default();
    assert_eq!(exprs.len(), 1, "incorrect exprs {exprs:?}; want length 1");

    let first = exprs[0]
        .as_any()
        .downcast_ref::<json::Expression>()
        .expect("first expr is not a json expression");
    // NOTE(port): upstream compares the `src` node *pointers* for identity
    // (`exprs[0].(*expression).src != e.src.(*arrayVal).Values[0]`); the
    // Rust AST holds nodes by value, so this compares the node values.
    let Node::Array { values, .. } = &e.src else {
        unreachable!("expression src is not an array")
    };
    assert_eq!(&first.src, &values[0], "wrong first expression node");
}

// Ported from TestExpression_Value:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1360
#[test]
#[ignore = "not yet implemented"]
fn expression_value() {
    let src = r#"{
  "string": "string_val",
  "number": 5,
  "bool_true": true,
  "bool_false": false,
  "array": ["a"],
  "object": {"key": "value"},
  "null": null
}"#;
    // NOTE(port): upstream's `expected` is a Go map; ported as pairs in the
    // map literal's source order (iteration order is irrelevant upstream).
    let expected = [
        ("string", Value::string("string_val")),
        ("number", Value::number_int(5)),
        ("bool_true", Value::bool(true)),
        ("bool_false", Value::bool(false)),
        ("array", Value::tuple([Value::string("a")])),
        ("object", Value::object([("key", Value::string("value"))])),
        ("null", Value::null(Type::dynamic())),
    ];

    let (file, diags) = json::parse(src.as_bytes(), "");
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on parse; want 0: {diags}",
        diags.len(),
    );
    // NOTE(port): upstream's nil-`File` and nil-`Body` fatal checks have no
    // analogue: `json::parse` returns `File` (with its `BodyRef`) by value.
    let (attrs, diags) = file.body.just_attributes();
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0: {diags}",
        diags.len(),
    );

    // Go: `attrs[ek].Expr.Value(&hcl.EvalContext{})` — a non-nil empty
    // context.
    let ctx = EvalContext::new();
    for (ek, ev) in &expected {
        let (val, diags) = attrs[*ek].expr.value(Some(&ctx));
        assert_eq!(
            diags.len(),
            0,
            "{ek}: got {} diagnostics on eval; want 0: {diags}",
            diags.len(),
        );
        assert!(
            val.raw_equals(ev),
            "{ek}: wrong result {val:?}; want {ev:?}"
        );
    }
}

// Ported from TestExpressionValue_Diags:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/structure_test.go#L1421
//
// Upstream: "TestExpressionValue_Diags asserts that Value() returns
// diagnostics from nested evaluations for complex objects (e.g. ObjectVal,
// ArrayVal)".
#[test]
#[ignore = "not yet implemented"]
fn expression_value_diags() {
    struct Case {
        name: &'static str,
        src: &'static str,
        expected: Value,
        error: &'static str,
    }

    let cases = [
        Case {
            name: "string: happy",
            src: r#"{"v": "happy ${VAR1}"}"#,
            expected: Value::string("happy case"),
            error: "",
        },
        Case {
            name: "string: unhappy",
            src: r#"{"v": "happy ${UNKNOWN}"}"#,
            expected: Value::unknown(Type::string()).refine_not_null(),
            error: "Unknown variable",
        },
        Case {
            name: "object_val: happy",
            src: r#"{"v": {"key": "happy ${VAR1}"}}"#,
            expected: Value::object([("key", Value::string("happy case"))]),
            error: "",
        },
        Case {
            name: "object_val: unhappy",
            src: r#"{"v": {"key": "happy ${UNKNOWN}"}}"#,
            expected: Value::object([("key", Value::unknown(Type::string()).refine_not_null())]),
            error: "Unknown variable",
        },
        Case {
            name: "object_key: happy",
            src: r#"{"v": {"happy ${VAR1}": "val"}}"#,
            expected: Value::object([("happy case", Value::string("val"))]),
            error: "",
        },
        Case {
            name: "object_key: unhappy",
            src: r#"{"v": {"happy ${UNKNOWN}": "val"}}"#,
            expected: Value::dynamic(),
            error: "Unknown variable",
        },
        Case {
            name: "array: happy",
            src: r#"{"v": ["happy ${VAR1}"]}"#,
            expected: Value::tuple([Value::string("happy case")]),
            error: "",
        },
        Case {
            name: "array: unhappy",
            src: r#"{"v": ["happy ${UNKNOWN}"]}"#,
            expected: Value::tuple([Value::unknown(Type::string()).refine_not_null()]),
            error: "Unknown variable",
        },
    ];

    let mut ctx = EvalContext::new();
    ctx.variables
        .insert("VAR1".to_string(), Value::string("case"));

    for (i, c) in cases.iter().enumerate() {
        let (file, diags) = json::parse(c.src.as_bytes(), "");
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): got {} diagnostics on parse; want 0: {diags}",
            c.name,
            diags.len(),
        );
        let (attrs, diags) = file.body.just_attributes();
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({}): got {} diagnostics on decode; want 0: {diags}",
            c.name,
            diags.len(),
        );

        let (val, diags) = attrs["v"].expr.value(Some(&ctx));
        if c.error.is_empty() {
            assert_eq!(
                diags.len(),
                0,
                "case {i} ({}): got {} diagnostics on eval; want 0: {diags}",
                c.name,
                diags.len(),
            );
        } else {
            assert!(
                !diags.is_empty(),
                "case {i} ({}): got 0 diagnostics on eval, want 1 with {}",
                c.name,
                c.error,
            );
            // Go: `strings.Contains(diags[0].Error(), c.error)`.
            let err = diags[0].to_string();
            assert!(
                err.contains(c.error),
                "case {i} ({}): found error: {err}; want {}",
                c.name,
                c.error,
            );
        }

        assert!(
            val.raw_equals(&c.expected),
            "case {i} ({}): wrong result {val:?}; want {:?}",
            c.name,
            c.expected,
        );
    }
}
