//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   merged_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;
use std::collections::{HashMap, HashSet};

use cty::Value;
use hcl::{
    Attribute, AttributeSchema, Attributes, Block, BlockHeaderSchema, Blocks, Body, BodyContent,
    BodyRef, BodySchema, Diagnostic, DiagnosticSeverity, Diagnostics, EvalContext, ExprRef,
    Expression, Range, Traversal, merge_bodies,
};

/// A zero-value range with only the filename set
/// (Go: `hcl.Range{Filename: name}`).
fn file_range(filename: &str) -> Range {
    Range {
        filename: filename.to_string(),
        ..Default::default()
    }
}

/// Stand-in for a Go nil `hcl.Expression` interface value.
///
/// NOTE(port): the upstream test helper leaves `Attribute.Expr` as its nil
/// zero value; `ExprRef` has no nil, so both the helper body below and the
/// expected literals use this placeholder, which compares equal only to
/// itself.
#[derive(Debug, PartialEq)]
struct NilExpr;

impl Expression for NilExpr {
    fn value(&self, _ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        unreachable!("nil expression stand-in")
    }

    fn variables(&self) -> Vec<Traversal> {
        unreachable!("nil expression stand-in")
    }

    fn range(&self) -> Range {
        unreachable!("nil expression stand-in")
    }

    fn start_range(&self) -> Range {
        unreachable!("nil expression stand-in")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn Expression) -> bool {
        other.as_any().downcast_ref::<NilExpr>().is_some()
    }
}

/// Stand-in for a Go nil `hcl.Body` interface value.
///
/// NOTE(port): the upstream test helper leaves `Block.Body` as its nil zero
/// value; `BodyRef` has no nil, so both the helper body below and the
/// expected literals use this placeholder, which compares equal only to
/// itself.
#[derive(Debug, PartialEq)]
struct NilBody;

impl Body for NilBody {
    fn content(&self, _schema: &BodySchema) -> (BodyContent, Diagnostics) {
        unreachable!("nil body stand-in")
    }

    fn partial_content(&self, _schema: &BodySchema) -> (BodyContent, BodyRef, Diagnostics) {
        unreachable!("nil body stand-in")
    }

    fn just_attributes(&self) -> (Attributes, Diagnostics) {
        unreachable!("nil body stand-in")
    }

    fn missing_item_range(&self) -> Range {
        unreachable!("nil body stand-in")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn Body) -> bool {
        other.as_any().downcast_ref::<NilBody>().is_some()
    }
}

/// Transcription of the upstream unexported test helper
/// `testMergedBodiesVictim` (merged_test.go), a fake `hcl.Body`.
///
/// `has_blocks: None` mirrors the Go nil map (which disables the block
/// handling in `partial_content`), distinct from `Some` of an empty map
/// (Go `map[string]int{}`).
#[derive(Debug, Clone, PartialEq, Default)]
struct TestMergedBodiesVictim {
    name: String,
    has_attributes: Vec<String>,
    has_blocks: Option<HashMap<String, usize>>,
    diag_count: usize,
}

impl Body for TestMergedBodiesVictim {
    fn content(&self, schema: &BodySchema) -> (BodyContent, Diagnostics) {
        let (c, _, d) = self.partial_content(schema);
        (c, d)
    }

    fn partial_content(&self, schema: &BodySchema) -> (BodyContent, BodyRef, Diagnostics) {
        let mut remain = TestMergedBodiesVictim {
            name: self.name.clone(),
            has_attributes: Vec::new(),
            ..Default::default()
        };

        let mut has_attrs: HashSet<&str> = HashSet::new();
        for n in &self.has_attributes {
            has_attrs.insert(n);

            let found = schema.attributes.iter().any(|attr_s| *n == attr_s.name);
            if !found {
                remain.has_attributes.push(n.clone());
            }
        }

        let mut content = BodyContent {
            attributes: Attributes::new(),
            ..Default::default()
        };

        let rng = file_range(&self.name);

        for attr_s in &schema.attributes {
            if has_attrs.contains(attr_s.name.as_str()) {
                content.attributes.insert(
                    attr_s.name.clone(),
                    Attribute {
                        name: attr_s.name.clone(),
                        expr: ExprRef::new(NilExpr),
                        range: Range::default(),
                        name_range: rng.clone(),
                    },
                );
            }
        }

        if let Some(has_blocks) = &self.has_blocks {
            for block_s in &schema.blocks {
                let num = has_blocks.get(&block_s.block_type).copied().unwrap_or(0);
                for _ in 0..num {
                    content.blocks.0.push(Block {
                        block_type: block_s.block_type.clone(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: rng.clone(),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    });
                }
            }

            let mut remain_blocks: HashMap<String, usize> = HashMap::new();
            for (n, count) in has_blocks {
                let found = schema.blocks.iter().any(|block_s| block_s.block_type == *n);
                if !found {
                    remain_blocks.insert(n.clone(), *count);
                }
            }
            remain.has_blocks = Some(remain_blocks);
        }

        let mut diags = Diagnostics::new();
        for i in 0..self.diag_count {
            diags.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: format!("Fake diagnostic {i}"),
                detail: "For testing only.".to_string(),
                context: Some(rng.clone()),
                ..Default::default()
            });
        }

        (content, BodyRef::new(remain), diags)
    }

    fn just_attributes(&self) -> (Attributes, Diagnostics) {
        let mut attrs = Attributes::new();

        let rng = file_range(&self.name);

        for name in &self.has_attributes {
            attrs.insert(
                name.clone(),
                Attribute {
                    name: name.clone(),
                    expr: ExprRef::new(NilExpr),
                    range: Range::default(),
                    name_range: rng.clone(),
                },
            );
        }

        let mut diags = Diagnostics::new();
        for i in 0..self.diag_count {
            diags.push(Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: format!("Fake diagnostic {i}"),
                detail: "For testing only.".to_string(),
                context: Some(rng.clone()),
                ..Default::default()
            });
        }

        (attrs, diags)
    }

    fn missing_item_range(&self) -> Range {
        file_range(&self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn Body) -> bool {
        other
            .as_any()
            .downcast_ref::<TestMergedBodiesVictim>()
            .is_some_and(|o| o == self)
    }
}

// Ported from TestMergedBodiesContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/merged_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn merged_bodies_content() {
    struct Case {
        bodies: Vec<BodyRef>,
        schema: BodySchema,
        want: BodyContent,
        diag_count: usize,
    }

    let tests = [
        Case {
            bodies: vec![],
            schema: BodySchema::default(),
            want: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    required: true,
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            diag_count: 1,
        },
        Case {
            bodies: vec![BodyRef::new(TestMergedBodiesVictim {
                has_attributes: vec!["name".to_string()],
                ..Default::default()
            })],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: ExprRef::new(NilExpr),
                        range: Range::default(),
                        name_range: Range::default(),
                    },
                )]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["name".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["name".to_string()],
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: ExprRef::new(NilExpr),
                        range: Range::default(),
                        name_range: file_range("first"),
                    },
                )]),
                ..Default::default()
            },
            diag_count: 1,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["name".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["age".to_string()],
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                attributes: vec![
                    AttributeSchema {
                        name: "name".to_string(),
                        ..Default::default()
                    },
                    AttributeSchema {
                        name: "age".to_string(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::from([
                    (
                        "name".to_string(),
                        Attribute {
                            name: "name".to_string(),
                            expr: ExprRef::new(NilExpr),
                            range: Range::default(),
                            name_range: file_range("first"),
                        },
                    ),
                    (
                        "age".to_string(),
                        Attribute {
                            name: "age".to_string(),
                            expr: ExprRef::new(NilExpr),
                            range: Range::default(),
                            name_range: file_range("second"),
                        },
                    ),
                ]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![BodyRef::new(TestMergedBodiesVictim {
                has_blocks: Some(HashMap::from([("pizza".to_string(), 1)])),
                ..Default::default()
            })],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![Block {
                    block_type: "pizza".to_string(),
                    labels: Vec::new(),
                    body: BodyRef::new(NilBody),
                    def_range: Range::default(),
                    type_range: Range::default(),
                    label_ranges: Vec::new(),
                }]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![BodyRef::new(TestMergedBodiesVictim {
                has_blocks: Some(HashMap::from([("pizza".to_string(), 2)])),
                ..Default::default()
            })],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: Range::default(),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: Range::default(),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                ]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_blocks: Some(HashMap::from([("pizza".to_string(), 1)])),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_blocks: Some(HashMap::from([("pizza".to_string(), 1)])),
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("first"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("second"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                ]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_blocks: Some(HashMap::from([("pizza".to_string(), 2)])),
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("second"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("second"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                ]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_blocks: Some(HashMap::from([("pizza".to_string(), 2)])),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("first"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("first"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                ]),
                ..Default::default()
            },
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            diag_count: 0,
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let merged = merge_bodies(test.bodies);
        let (got, diags) = merged.content(&test.schema);

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i}: wrong number of diagnostics; got: {diags:#?}"
        );

        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}

// Ported from TestMergeBodiesPartialContent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/merged_test.go#L357
#[test]
#[ignore = "not yet implemented"]
fn merge_bodies_partial_content() {
    struct Case {
        bodies: Vec<BodyRef>,
        schema: BodySchema,
        want_content: BodyContent,
        // NOTE(port): upstream constructs the unexported `mergedBodies`
        // type directly (e.g. `mergedBodies{...}`); `merge_bodies` is its
        // public constructor, so the expected remain is built through it.
        want_remain: BodyRef,
        diag_count: usize,
    }

    let tests = [
        Case {
            bodies: vec![],
            schema: BodySchema::default(),
            want_content: BodyContent {
                attributes: Attributes::new(),
                ..Default::default()
            },
            want_remain: merge_bodies(vec![]),
            diag_count: 0,
        },
        Case {
            bodies: vec![BodyRef::new(TestMergedBodiesVictim {
                name: "first".to_string(),
                has_attributes: vec!["name".to_string(), "age".to_string()],
                ..Default::default()
            })],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want_content: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: ExprRef::new(NilExpr),
                        range: Range::default(),
                        name_range: file_range("first"),
                    },
                )]),
                ..Default::default()
            },
            want_remain: merge_bodies(vec![BodyRef::new(TestMergedBodiesVictim {
                name: "first".to_string(),
                has_attributes: vec!["age".to_string()],
                ..Default::default()
            })]),
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["name".to_string(), "age".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["name".to_string(), "pizza".to_string()],
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                attributes: vec![AttributeSchema {
                    name: "name".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want_content: BodyContent {
                attributes: Attributes::from([(
                    "name".to_string(),
                    Attribute {
                        name: "name".to_string(),
                        expr: ExprRef::new(NilExpr),
                        range: Range::default(),
                        name_range: file_range("first"),
                    },
                )]),
                ..Default::default()
            },
            want_remain: merge_bodies(vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["age".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["pizza".to_string()],
                    ..Default::default()
                }),
            ]),
            diag_count: 1,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["name".to_string(), "age".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["pizza".to_string(), "soda".to_string()],
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                attributes: vec![
                    AttributeSchema {
                        name: "name".to_string(),
                        ..Default::default()
                    },
                    AttributeSchema {
                        name: "soda".to_string(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            want_content: BodyContent {
                attributes: Attributes::from([
                    (
                        "name".to_string(),
                        Attribute {
                            name: "name".to_string(),
                            expr: ExprRef::new(NilExpr),
                            range: Range::default(),
                            name_range: file_range("first"),
                        },
                    ),
                    (
                        "soda".to_string(),
                        Attribute {
                            name: "soda".to_string(),
                            expr: ExprRef::new(NilExpr),
                            range: Range::default(),
                            name_range: file_range("second"),
                        },
                    ),
                ]),
                ..Default::default()
            },
            want_remain: merge_bodies(vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec!["age".to_string()],
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec!["pizza".to_string()],
                    ..Default::default()
                }),
            ]),
            diag_count: 0,
        },
        Case {
            bodies: vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_blocks: Some(HashMap::from([("pizza".to_string(), 1)])),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_blocks: Some(HashMap::from([
                        ("pizza".to_string(), 1),
                        ("soda".to_string(), 2),
                    ])),
                    ..Default::default()
                }),
            ],
            schema: BodySchema {
                blocks: vec![BlockHeaderSchema {
                    block_type: "pizza".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            want_content: BodyContent {
                attributes: Attributes::new(),
                blocks: Blocks(vec![
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("first"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                    Block {
                        block_type: "pizza".to_string(),
                        labels: Vec::new(),
                        body: BodyRef::new(NilBody),
                        def_range: file_range("second"),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    },
                ]),
                ..Default::default()
            },
            want_remain: merge_bodies(vec![
                BodyRef::new(TestMergedBodiesVictim {
                    name: "first".to_string(),
                    has_attributes: vec![],
                    has_blocks: Some(HashMap::new()),
                    ..Default::default()
                }),
                BodyRef::new(TestMergedBodiesVictim {
                    name: "second".to_string(),
                    has_attributes: vec![],
                    has_blocks: Some(HashMap::from([("soda".to_string(), 2)])),
                    ..Default::default()
                }),
            ]),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let merged = merge_bodies(test.bodies);
        let (got, got_remain, diags) = merged.partial_content(&test.schema);

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i}: wrong number of diagnostics; got: {diags:#?}"
        );

        assert_eq!(got, test.want_content, "case {i}: wrong content result");

        assert_eq!(
            got_remain, test.want_remain,
            "case {i}: wrong remaining result"
        );
    }
}
