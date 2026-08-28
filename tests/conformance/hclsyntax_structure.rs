//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/structure_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::hclsyntax::{self, Expression, LiteralValueExpr};
use hcl::{
    Attribute, AttributeSchema, Attributes, Block, BlockHeaderSchema, Blocks, Body as _,
    BodyContent, BodyRef, BodySchema, ExprRef, Range,
};

/// Stand-in for a Go nil `hclsyntax.Expression` interface value.
///
/// NOTE(port): upstream leaves `Attribute.Expr` as its nil zero value; the
/// Rust `hclsyntax::Expression` enum has no nil, so this zero-ish literal
/// expression is used on both the input and expected sides (Go compares
/// nil against nil; Rust compares this value against itself).
fn nil_expr() -> Expression {
    Expression::from(LiteralValueExpr {
        val: Value::null(Type::dynamic()),
        src_range: Range::default(),
    })
}

/// Stand-in for Go's `Body: (*Body)(nil)` in expected `hcl.Block`s.
///
/// NOTE(port): `hclsyntax::Block.body` holds a `Body` by value, so the Go
/// nil `*hclsyntax.Body` of an input block's omitted `Body` field becomes
/// the zero-value `hclsyntax::Body` on the input side and this same zero
/// body (wrapped as a handle) on the expected side.
fn nil_body_ref() -> BodyRef {
    BodyRef::new(hclsyntax::Body::default())
}

/// Go: `&hclsyntax.Attribute{Name: name}` — all other fields zero
/// (nil `Expr` becomes [`nil_expr`]).
fn syn_attr(name: &str) -> hclsyntax::Attribute {
    hclsyntax::Attribute {
        name: name.to_string(),
        expr: nil_expr(),
        src_range: Range::default(),
        name_range: Range::default(),
        equals_range: Range::default(),
    }
}

/// Go: `&hcl.Attribute{Name: name}` — all other fields zero
/// (nil `Expr` becomes [`nil_expr`]).
fn hcl_attr(name: &str) -> Attribute {
    Attribute {
        name: name.to_string(),
        expr: ExprRef::new(nil_expr()),
        range: Range::default(),
        name_range: Range::default(),
    }
}

/// Go: `&hclsyntax.Block{Type: ..., Labels: ..., LabelRanges: ...}` — all
/// other fields zero (nil `Body` becomes the zero-value body; see
/// [`nil_body_ref`]).
fn syn_block(block_type: &str, labels: &[&str], label_ranges: Vec<Range>) -> hclsyntax::Block {
    hclsyntax::Block {
        block_type: block_type.to_string(),
        labels: labels.iter().map(|l| l.to_string()).collect(),
        body: hclsyntax::Body::default(),
        type_range: Range::default(),
        label_ranges,
        open_brace_range: Range::default(),
        close_brace_range: Range::default(),
    }
}

/// Go: `hcl.Block{Type: ..., Labels: ..., Body: (*Body)(nil)}` — all other
/// fields zero.
fn want_block(block_type: &str, labels: &[&str]) -> Block {
    Block {
        block_type: block_type.to_string(),
        labels: labels.iter().map(|l| l.to_string()).collect(),
        body: nil_body_ref(),
        def_range: Range::default(),
        type_range: Range::default(),
        label_ranges: vec![],
    }
}

// Ported from TestBodyContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_test.go#L18
#[test]
#[ignore = "not yet implemented"]
fn body_content() {
    struct Case {
        body: hclsyntax::Body,
        schema: BodySchema,
        partial: bool,
        want: BodyContent,
        diag_count: usize,
    }

    let tests = [
        Case {
            body: hclsyntax::Body::default(),
            schema: BodySchema::default(),
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 0,
        },
        // Attributes
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([("foo".to_string(), syn_attr("foo"))]),
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "foo".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::from([("foo".to_string(), hcl_attr("foo"))]),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([("foo".to_string(), syn_attr("foo"))]),
                ..Default::default()
            },
            schema: BodySchema::default(),
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // attribute "foo" is not expected
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([("foo".to_string(), syn_attr("foo"))]),
                ..Default::default()
            },
            schema: BodySchema::default(),
            partial: true,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 0, // in partial mode, so extra "foo" is acceptable
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::new(),
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "foo".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 0, // "foo" not required, so no error
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::new(),
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "foo".to_string(),
                    required: true,
                }],
                blocks: vec![],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // "foo" is required
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([("foo".to_string(), syn_attr("foo"))]),
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // attribute "foo" not expected (it's defined as a block)
        },
        // Blocks
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &[], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![want_block("foo", &[])]),
                missing_item_range: Range::default(),
            },
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &[], vec![]), syn_block("foo", &[], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![want_block("foo", &[]), want_block("foo", &[])]),
                missing_item_range: Range::default(),
            },
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &[], vec![]), syn_block("bar", &[], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![want_block("foo", &[])]),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // blocks of type "bar" not expected
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &[], vec![]), syn_block("bar", &[], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: true,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![want_block("foo", &[])]),
                missing_item_range: Range::default(),
            },
            diag_count: 0, // extra "bar" allowed because we're in partial mode
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &["bar"], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![want_block("foo", &["bar"])]),
                missing_item_range: Range::default(),
            },
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &[], vec![])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // missing label "name"
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block("foo", &["bar"], vec![Range::default()])],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // no labels expected
        },
        Case {
            body: hclsyntax::Body {
                blocks: vec![syn_block(
                    "foo",
                    &["bar", "baz"],
                    vec![Range::default(), Range::default()],
                )],
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // too many labels
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([("foo".to_string(), syn_attr("foo"))]),
                ..Default::default()
            },
            schema: BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "foo".to_string(),
                    label_names: vec![],
                }],
            },
            partial: false,
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks::default(),
                missing_item_range: Range::default(),
            },
            diag_count: 1, // should've been a block, not an attribute
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (got, diags) = if test.partial {
            let (got, _, diags) = test.body.partial_content(&test.schema);
            (got, diags)
        } else {
            test.body.content(&test.schema)
        };

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i}: wrong number of diagnostics {}; want {}; diags: {diags:#?}",
            diags.len(),
            test.diag_count,
        );

        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}

// Ported from TestBodyJustAttributes:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_test.go#L422
#[test]
#[ignore = "not yet implemented"]
fn body_just_attributes() {
    struct Case {
        body: hclsyntax::Body,
        want: Attributes,
        diag_count: usize,
    }

    let tests = [
        Case {
            body: hclsyntax::Body::default(),
            want: Attributes::new(),
            diag_count: 0,
        },
        // NOTE(port): upstream distinguishes a nil `Attributes` map (previous
        // case) from an empty one (this case); Rust's `HashMap` has a single
        // empty representation, so both cases construct the same body.
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::new(),
                ..Default::default()
            },
            want: Attributes::new(),
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([(
                    "foo".to_string(),
                    hclsyntax::Attribute {
                        name: "foo".to_string(),
                        expr: Expression::from(LiteralValueExpr {
                            val: Value::string("bar"),
                            src_range: Range::default(),
                        }),
                        src_range: Range::default(),
                        name_range: Range::default(),
                        equals_range: Range::default(),
                    },
                )]),
                ..Default::default()
            },
            want: Attributes::from([(
                "foo".to_string(),
                Attribute {
                    name: "foo".to_string(),
                    expr: ExprRef::new(Expression::from(LiteralValueExpr {
                        val: Value::string("bar"),
                        src_range: Range::default(),
                    })),
                    range: Range::default(),
                    name_range: Range::default(),
                },
            )]),
            diag_count: 0,
        },
        Case {
            body: hclsyntax::Body {
                attributes: hclsyntax::Attributes::from([(
                    "foo".to_string(),
                    hclsyntax::Attribute {
                        name: "foo".to_string(),
                        expr: Expression::from(LiteralValueExpr {
                            val: Value::string("bar"),
                            src_range: Range::default(),
                        }),
                        src_range: Range::default(),
                        name_range: Range::default(),
                        equals_range: Range::default(),
                    },
                )]),
                blocks: vec![syn_block("foo", &[], vec![])],
                ..Default::default()
            },
            want: Attributes::from([(
                "foo".to_string(),
                Attribute {
                    name: "foo".to_string(),
                    expr: ExprRef::new(Expression::from(LiteralValueExpr {
                        val: Value::string("bar"),
                        src_range: Range::default(),
                    })),
                    range: Range::default(),
                    name_range: Range::default(),
                },
            )]),
            diag_count: 1, // blocks are not allowed here
        },
        // NOTE(port): upstream's final case sets the unexported `hiddenAttrs`
        // field (a "foo" attribute hidden by a previous PartialContent call)
        // and expects JustAttributes to return no attributes. The Rust
        // `hclsyntax::Body` exposes no way to construct hidden attributes
        // from outside the crate, so that case cannot be transcribed:
        // https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_test.go#L487-L503
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (got, diags) = test.body.just_attributes();

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i}: wrong number of diagnostics {}; want {}; diags: {diags:#?}",
            diags.len(),
            test.diag_count,
        );

        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}
