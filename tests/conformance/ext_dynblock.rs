//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ext/dynblock/expand_body_test.go
//!   ext/dynblock/variables_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use cty::{Type, Value};
use hcl::ext::dynblock;
use hcl::hcldec::{
    self, AttrSpec, BlockListSpec, BlockMapSpec, BlockSetSpec, BlockTupleSpec, ObjectSpec, SpecRef,
};
use hcl::hclsyntax;
use hcl::hcltest::{
    mock_attrs, mock_body, mock_expr_list, mock_expr_literal, mock_expr_traversal_src,
    mock_expr_variable,
};
use hcl::{
    Attributes, Block, BlockHeaderSchema, Blocks, BodyContent, BodyRef, BodySchema, Diagnostic,
    DiagnosticExtra, DiagnosticSeverity, Diagnostics, EvalContext, ExprRef, Pos, Range,
};

/// Mock attributes from name–expression pairs
/// (Go: `hcltest.MockAttrs(map[string]hcl.Expression{...})`).
fn attrs<const N: usize>(entries: [(&str, ExprRef); N]) -> Attributes {
    mock_attrs(
        entries
            .into_iter()
            .map(|(name, expr)| (name.to_string(), expr))
            .collect(),
    )
}

/// A mock body with the given attributes and blocks and all other fields
/// left as their Go zero values
/// (Go: `hcltest.MockBody(&hcl.BodyContent{Attributes: ..., Blocks: ...})`).
fn mock_content_body(attributes: Attributes, blocks: Vec<Block>) -> BodyRef {
    mock_body(BodyContent {
        attributes,
        blocks: Blocks(blocks),
        missing_item_range: Range::default(),
    })
}

/// A block with the given type, labels, and body, all other fields left as
/// their Go zero values. Upstream pairs each label with one zero `hcl.Range`
/// in `LabelRanges` (Go: `hcl.Block{Type: ..., Labels: ...,
/// LabelRanges: []hcl.Range{{}}, Body: ...}`).
fn block(block_type: &str, labels: &[&str], body: BodyRef) -> Block {
    Block {
        block_type: block_type.to_string(),
        labels: labels.iter().map(|s| s.to_string()).collect(),
        body,
        def_range: Range::default(),
        type_range: Range::default(),
        label_ranges: labels.iter().map(|_| Range::default()).collect(),
    }
}

// Ported from TestExpand:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/expand_body_test.go#L19
#[test]
#[ignore = "not yet implemented"]
fn expand() {
    let src_body = mock_content_body(
        Attributes::new(),
        vec![
            block(
                "a",
                &["static0"],
                mock_content_body(
                    attrs([("val", mock_expr_literal(Value::string("static a 0")))]),
                    vec![],
                ),
            ),
            block(
                "b",
                &[],
                mock_content_body(
                    Attributes::new(),
                    vec![
                        block(
                            "c",
                            &[],
                            mock_content_body(
                                attrs([("val0", mock_expr_literal(Value::string("static c 0")))]),
                                vec![],
                            ),
                        ),
                        block(
                            "dynamic",
                            &["c"],
                            mock_content_body(
                                attrs([
                                    (
                                        "for_each",
                                        mock_expr_literal(Value::list([
                                            Value::string("dynamic c 0"),
                                            Value::string("dynamic c 1"),
                                        ])),
                                    ),
                                    ("iterator", mock_expr_variable("dyn_c")),
                                ]),
                                vec![block(
                                    "content",
                                    &[],
                                    mock_content_body(
                                        attrs([("val0", mock_expr_traversal_src("dyn_c.value"))]),
                                        vec![],
                                    ),
                                )],
                            ),
                        ),
                    ],
                ),
            ),
            block(
                "dynamic",
                &["a"],
                mock_content_body(
                    attrs([
                        (
                            "for_each",
                            mock_expr_literal(Value::list([
                                Value::string("dynamic a 0"),
                                Value::string("dynamic a 1"),
                                Value::string("dynamic a 2"),
                            ])),
                        ),
                        (
                            "labels",
                            mock_expr_list(vec![mock_expr_traversal_src("a.key")]),
                        ),
                    ]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("a.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["b"],
                mock_content_body(
                    attrs([
                        (
                            "for_each",
                            mock_expr_literal(Value::list([
                                Value::string("dynamic b 0"),
                                Value::string("dynamic b 1"),
                            ])),
                        ),
                        ("iterator", mock_expr_variable("dyn_b")),
                    ]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            Attributes::new(),
                            vec![
                                block(
                                    "c",
                                    &[],
                                    mock_content_body(
                                        attrs([
                                            (
                                                "val0",
                                                mock_expr_literal(Value::string("static c 1")),
                                            ),
                                            ("val1", mock_expr_traversal_src("dyn_b.value")),
                                        ]),
                                        vec![],
                                    ),
                                ),
                                block(
                                    "dynamic",
                                    &["c"],
                                    mock_content_body(
                                        attrs([(
                                            "for_each",
                                            mock_expr_literal(Value::list([
                                                Value::string("dynamic c 2"),
                                                Value::string("dynamic c 3"),
                                            ])),
                                        )]),
                                        vec![block(
                                            "content",
                                            &[],
                                            mock_content_body(
                                                attrs([
                                                    ("val0", mock_expr_traversal_src("c.value")),
                                                    (
                                                        "val1",
                                                        mock_expr_traversal_src("dyn_b.value"),
                                                    ),
                                                ]),
                                                vec![],
                                            ),
                                        )],
                                    ),
                                ),
                            ],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["b"],
                mock_content_body(
                    attrs([
                        (
                            "for_each",
                            mock_expr_literal(Value::map([(
                                "foo",
                                Value::list([
                                    Value::string("dynamic c nested 0"),
                                    Value::string("dynamic c nested 1"),
                                ]),
                            )])),
                        ),
                        ("iterator", mock_expr_variable("dyn_b")),
                    ]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            Attributes::new(),
                            vec![block(
                                "dynamic",
                                &["c"],
                                mock_content_body(
                                    attrs([("for_each", mock_expr_traversal_src("dyn_b.value"))]),
                                    vec![block(
                                        "content",
                                        &[],
                                        mock_content_body(
                                            attrs([
                                                ("val0", mock_expr_traversal_src("c.value")),
                                                ("val1", mock_expr_traversal_src("dyn_b.key")),
                                            ]),
                                            vec![],
                                        ),
                                    )],
                                ),
                            )],
                        ),
                    )],
                ),
            ),
            block(
                "a",
                &["static1"],
                mock_content_body(
                    attrs([("val", mock_expr_literal(Value::string("static a 1")))]),
                    vec![],
                ),
            ),
        ],
    );

    let dyn_body = dynblock::expand(src_body, None, vec![]);

    // t.Run("PartialDecode")
    let remain = {
        let dec_spec = BlockMapSpec {
            type_name: "a".to_string(),
            label_names: vec!["key".to_string()],
            nested: SpecRef::new(AttrSpec {
                name: "val".to_string(),
                ty: Type::string(),
                required: true,
            }),
        };

        let (got, remain, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            0,
            "PartialDecode: unexpected diagnostics\n{diags:?}",
        );

        let want = Value::map([
            ("static0", Value::string("static a 0")),
            ("static1", Value::string("static a 1")),
            ("0", Value::string("dynamic a 0")),
            ("1", Value::string("dynamic a 1")),
            ("2", Value::string("dynamic a 2")),
        ]);

        assert!(
            got.raw_equals(&want),
            "PartialDecode: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );

        remain
    };

    // t.Run("Decode")
    {
        let dec_spec = BlockListSpec {
            type_name: "b".to_string(),
            nested: SpecRef::new(BlockListSpec {
                type_name: "c".to_string(),
                nested: SpecRef::new(ObjectSpec::from_iter([
                    (
                        "val0",
                        SpecRef::new(AttrSpec {
                            name: "val0".to_string(),
                            ty: Type::string(),
                            required: false,
                        }),
                    ),
                    (
                        "val1",
                        SpecRef::new(AttrSpec {
                            name: "val1".to_string(),
                            ty: Type::string(),
                            required: false,
                        }),
                    ),
                ])),
                min_items: 0,
                max_items: 0,
            }),
            min_items: 0,
            max_items: 0,
        };

        let (got, diags) = hcldec::decode(&*remain, &dec_spec, None);
        assert_eq!(diags.len(), 0, "Decode: unexpected diagnostics\n{diags:?}");

        let want = Value::list([
            Value::list([
                Value::object([
                    ("val0", Value::string("static c 0")),
                    ("val1", Value::null(Type::string())),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 0")),
                    ("val1", Value::null(Type::string())),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 1")),
                    ("val1", Value::null(Type::string())),
                ]),
            ]),
            Value::list([
                Value::object([
                    ("val0", Value::string("static c 1")),
                    ("val1", Value::string("dynamic b 0")),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 2")),
                    ("val1", Value::string("dynamic b 0")),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 3")),
                    ("val1", Value::string("dynamic b 0")),
                ]),
            ]),
            Value::list([
                Value::object([
                    ("val0", Value::string("static c 1")),
                    ("val1", Value::string("dynamic b 1")),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 2")),
                    ("val1", Value::string("dynamic b 1")),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c 3")),
                    ("val1", Value::string("dynamic b 1")),
                ]),
            ]),
            Value::list([
                Value::object([
                    ("val0", Value::string("dynamic c nested 0")),
                    ("val1", Value::string("foo")),
                ]),
                Value::object([
                    ("val0", Value::string("dynamic c nested 1")),
                    ("val1", Value::string("foo")),
                ]),
            ]),
        ]);

        assert!(
            got.raw_equals(&want),
            "Decode: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }
}

/// A bare string stored as a diagnostic extra (Go: the untyped
/// `Extra: "diagnostic extra"` string; Go's `interface{}` field takes the
/// string directly, while `Diagnostic.extra` needs a [`DiagnosticExtra`]
/// impl to carry it).
#[derive(Debug)]
struct StrExtra(&'static str);

impl DiagnosticExtra for StrExtra {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Ported from TestExpandWithForEachCheck:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/expand_body_test.go#L342
#[test]
#[ignore = "not yet implemented"]
fn expand_with_for_each_check() {
    let for_each_expr = mock_expr_literal(Value::map_empty(Type::string()).mark("boop"));
    let eval_ctx = Arc::new(EvalContext::new());
    let src_content = BodyContent {
        blocks: Blocks(vec![block(
            "dynamic",
            &["foo"],
            mock_content_body(
                attrs([("for_each", for_each_expr)]),
                vec![block(
                    "content",
                    &[],
                    mock_content_body(Attributes::new(), vec![]),
                )],
            ),
        )]),
        ..Default::default()
    };
    let src_body = mock_body(src_content);

    let hook_called = Rc::new(Cell::new(false));
    let got_v: Rc<RefCell<Option<Value>>> = Rc::new(RefCell::new(None));
    // NOTE(port): Go stores the `*hcl.EvalContext` the hook received and
    // later compares it by pointer identity against the one passed to
    // `Expand`. The Rust hook receives `Option<&EvalContext>`, whose borrow
    // cannot outlive the call, so the identity comparison happens inside
    // the hook and only its result is recorded.
    let got_eval_ctx_matches = Rc::new(Cell::new(false));

    let exp_body = {
        let hook_called = Rc::clone(&hook_called);
        let got_v = Rc::clone(&got_v);
        let got_eval_ctx_matches = Rc::clone(&got_eval_ctx_matches);
        let hook_ctx = Arc::clone(&eval_ctx);
        // The closure captures test-local Rc state, so it is inherently
        // neither Send nor Sync; `ForEachCheckFunc` is a plain `Arc<dyn Fn>`.
        #[allow(clippy::arc_with_non_send_sync)]
        let check: dynblock::ForEachCheckFunc = Arc::new(move |v, e, ec| {
            hook_called.set(true);
            *got_v.borrow_mut() = Some(v.clone());
            got_eval_ctx_matches.set(ec.is_some_and(|ec| std::ptr::eq(ec, Arc::as_ptr(&hook_ctx))));
            Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Bad for_each".to_string(),
                detail: "I don't like it.".to_string(),
                expression: Some(e.clone()),
                // NOTE(port): Go sets `EvalContext: ec` (the context the
                // hook received); the Rust field wants an owning
                // `Arc<EvalContext>`, so the shared handle passed to
                // `expand` — the same context — stands in.
                eval_context: Some(Arc::clone(&hook_ctx)),
                extra: Some(Arc::new(StrExtra("diagnostic extra"))),
                ..Default::default()
            }])
        });
        dynblock::expand(
            src_body,
            Some(Arc::clone(&eval_ctx)),
            vec![dynblock::ExpandOption::CheckForEach(check)],
        )
    };

    let (_, diags) = exp_body.content(&BodySchema {
        attributes: vec![],
        blocks: vec![BlockHeaderSchema {
            block_type: "foo".to_string(),
            label_names: vec![],
        }],
    });
    assert!(diags.has_errors(), "succeeded; want an error");
    assert_eq!(
        diags.len(),
        1,
        "wrong number of diagnostics; want only one\n{diags:?}",
    );
    assert_eq!(
        diags[0].summary, "Bad for_each",
        "wrong error\ngot:  {}\nwant: Bad for_each\n\n{:?}",
        diags[0].summary, diags[0],
    );
    // This is important to allow the application which provided the
    // hook to pass application-specific extra values through this
    // API in case the hook's diagnostics need some sort of special
    // treatment.
    let got_extra = diags[0]
        .extra
        .as_ref()
        .and_then(|extra| extra.as_any().downcast_ref::<StrExtra>())
        .map(|s| s.0);
    assert_eq!(
        got_extra,
        Some("diagnostic extra"),
        "diagnostic didn't preserve 'extra' field\ngot:  {got_extra:?}\nwant: {:?}\n\n{:?}",
        "diagnostic extra",
        diags[0],
    );

    assert!(hook_called.get(), "check hook wasn't called");
    let got_v = got_v.borrow();
    let got_v = got_v.as_ref().expect("check hook wasn't called");
    assert!(
        got_v.has_mark("boop"),
        "wrong value passed to check hook; want the value marked \"boop\"\n{got_v:?}",
    );
    assert!(
        got_eval_ctx_matches.get(),
        "wrong EvalContext passed to check hook; want the one passed to Expand",
    );
}

// Ported from TestExpandUnknownBodies:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/expand_body_test.go#L425
#[test]
#[ignore = "not yet implemented"]
fn expand_unknown_bodies() {
    let src_content = BodyContent {
        blocks: Blocks(vec![
            block(
                "dynamic",
                &["list"],
                mock_content_body(
                    attrs([(
                        "for_each",
                        mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                    )]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("each.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["tuple"],
                mock_content_body(
                    attrs([(
                        "for_each",
                        mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                    )]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("each.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["set"],
                mock_content_body(
                    attrs([(
                        "for_each",
                        mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                    )]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("each.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["map"],
                mock_content_body(
                    attrs([
                        (
                            "for_each",
                            mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                        ),
                        (
                            "labels",
                            mock_expr_list(vec![mock_expr_literal(Value::string("static"))]),
                        ),
                    ]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("each.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["object"],
                mock_content_body(
                    attrs([
                        (
                            "for_each",
                            mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                        ),
                        (
                            "labels",
                            mock_expr_list(vec![mock_expr_literal(Value::string("static"))]),
                        ),
                    ]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([("val", mock_expr_traversal_src("each.value"))]),
                            vec![],
                        ),
                    )],
                ),
            ),
            block(
                "dynamic",
                &["invalid_list"],
                mock_content_body(
                    attrs([(
                        "for_each",
                        mock_expr_literal(Value::unknown(Type::map(Type::string()))),
                    )]),
                    vec![block(
                        "content",
                        &[],
                        mock_content_body(
                            attrs([
                                ("val", mock_expr_traversal_src("each.value")),
                                // unexpected attributes should still produce an error
                                ("invalid", mock_expr_literal(Value::string("static"))),
                            ]),
                            vec![],
                        ),
                    )],
                ),
            ),
        ]),
        ..Default::default()
    };

    let src_body = mock_body(src_content);
    let dyn_body = dynblock::expand(src_body, None, vec![]);

    // t.Run("DecodeList")
    {
        let dec_spec = BlockListSpec {
            type_name: "list".to_string(),
            nested: SpecRef::new(ObjectSpec::from_iter([(
                "val",
                SpecRef::new(AttrSpec {
                    name: "val".to_string(),
                    ty: Type::string(),
                    required: false,
                }),
            )])),
            min_items: 0,
            max_items: 0,
        };

        let (got, _, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            0,
            "DecodeList: unexpected diagnostics\n{diags:?}",
        );

        let want = Value::unknown(Type::list(Type::object([("val", Type::string())])));

        assert!(
            got.raw_equals(&want),
            "DecodeList: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }

    // t.Run("DecodeTuple")
    {
        let dec_spec = BlockTupleSpec {
            type_name: "tuple".to_string(),
            nested: SpecRef::new(ObjectSpec::from_iter([(
                "val",
                SpecRef::new(AttrSpec {
                    name: "val".to_string(),
                    ty: Type::string(),
                    required: false,
                }),
            )])),
            min_items: 0,
            max_items: 0,
        };

        let (got, _, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            0,
            "DecodeTuple: unexpected diagnostics\n{diags:?}",
        );

        let want = Value::dynamic();

        assert!(
            got.raw_equals(&want),
            "DecodeTuple: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }

    // t.Run("DecodeSet")
    {
        let dec_spec = BlockSetSpec {
            type_name: "tuple".to_string(),
            nested: SpecRef::new(ObjectSpec::from_iter([(
                "val",
                SpecRef::new(AttrSpec {
                    name: "val".to_string(),
                    ty: Type::string(),
                    required: false,
                }),
            )])),
            min_items: 0,
            max_items: 0,
        };

        let (got, _, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            0,
            "DecodeSet: unexpected diagnostics\n{diags:?}",
        );

        let want = Value::unknown(Type::set(Type::object([("val", Type::string())])));

        assert!(
            got.raw_equals(&want),
            "DecodeSet: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }

    // t.Run("DecodeMap")
    {
        let dec_spec = BlockMapSpec {
            type_name: "map".to_string(),
            label_names: vec!["key".to_string()],
            nested: SpecRef::new(ObjectSpec::from_iter([(
                "val",
                SpecRef::new(AttrSpec {
                    name: "val".to_string(),
                    ty: Type::string(),
                    required: false,
                }),
            )])),
        };

        let (got, _, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            0,
            "DecodeMap: unexpected diagnostics\n{diags:?}",
        );

        let want = Value::unknown(Type::map(Type::object([("val", Type::string())])));

        assert!(
            got.raw_equals(&want),
            "DecodeMap: wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }

    // t.Run("DecodeInvalidList")
    {
        let dec_spec = BlockListSpec {
            type_name: "invalid_list".to_string(),
            nested: SpecRef::new(ObjectSpec::from_iter([(
                "val",
                SpecRef::new(AttrSpec {
                    name: "val".to_string(),
                    ty: Type::string(),
                    required: false,
                }),
            )])),
            min_items: 0,
            max_items: 0,
        };

        let (_, _, diags) = hcldec::partial_decode(&*dyn_body, &dec_spec, None);
        assert_eq!(
            diags.len(),
            1,
            "DecodeInvalidList: expected 1 extraneous argument",
        );

        let want = r#"Mock body has extraneous argument "invalid""#;

        assert!(
            format!("{diags}").contains(want),
            "DecodeInvalidList: unexpected diagnostics: {diags:?}",
        );
    }
}

// Ported from TestExpandMarkedForEach:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/expand_body_test.go#L714
#[test]
#[ignore = "not yet implemented"]
fn expand_marked_for_each() {
    let src_body = mock_content_body(
        Attributes::new(),
        vec![block(
            "dynamic",
            &["b"],
            mock_content_body(
                attrs([
                    (
                        "for_each",
                        mock_expr_literal(Value::tuple([Value::string("hey")]).mark("boop")),
                    ),
                    ("iterator", mock_expr_traversal_src("dyn_b")),
                ]),
                vec![block(
                    "content",
                    &[],
                    mock_content_body(
                        attrs([
                            ("val0", mock_expr_literal(Value::string("static c 1"))),
                            ("val1", mock_expr_traversal_src("dyn_b.value")),
                        ]),
                        vec![],
                    ),
                )],
            ),
        )],
    );

    let dyn_body = dynblock::expand(src_body, None, vec![]);

    // t.Run("Decode")
    {
        let dec_spec = BlockListSpec {
            type_name: "b".to_string(),
            nested: SpecRef::new(ObjectSpec::from_iter([
                (
                    "val0",
                    SpecRef::new(AttrSpec {
                        name: "val0".to_string(),
                        ty: Type::string(),
                        required: false,
                    }),
                ),
                (
                    "val1",
                    SpecRef::new(AttrSpec {
                        name: "val1".to_string(),
                        ty: Type::string(),
                        required: false,
                    }),
                ),
            ])),
            min_items: 0,
            max_items: 0,
        };

        let want = Value::list([Value::object([
            ("val0", Value::string("static c 1").mark("boop")),
            ("val1", Value::string("hey").mark("boop")),
        ])
        .mark("boop")]);
        let (got, diags) = hcldec::decode(&*dyn_body, &dec_spec, None);
        assert!(!diags.has_errors(), "unexpected errors\n{diags}");
        // NOTE(port): upstream diffs with `cmp.Diff(want, got,
        // ctydebug.CmpOptions)`, which compares like `RawEquals`.
        assert!(
            got.raw_equals(&want),
            "wrong result\ngot:  {got:?}\nwant: {want:?}",
        );
    }
}

// Ported from TestExpandInvalidIteratorError:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/expand_body_test.go#L777
#[test]
#[ignore = "not yet implemented"]
fn expand_invalid_iterator_error() {
    let src_body = mock_content_body(
        Attributes::new(),
        vec![block(
            "dynamic",
            &["b"],
            mock_content_body(
                attrs([
                    (
                        "for_each",
                        mock_expr_literal(Value::list([
                            Value::string("dynamic b 0"),
                            Value::string("dynamic b 1"),
                        ])),
                    ),
                    ("iterator", mock_expr_literal(Value::string("dyn_b"))),
                ]),
                vec![block(
                    "content",
                    &[],
                    mock_content_body(
                        Attributes::new(),
                        vec![block(
                            "c",
                            &[],
                            mock_content_body(
                                attrs([
                                    ("val0", mock_expr_literal(Value::string("static c 1"))),
                                    ("val1", mock_expr_traversal_src("dyn_b.value")),
                                ]),
                                vec![],
                            ),
                        )],
                    ),
                )],
            ),
        )],
    );

    let dyn_body = dynblock::expand(src_body, None, vec![]);

    // t.Run("Decode")
    {
        let dec_spec = BlockListSpec {
            type_name: "b".to_string(),
            nested: SpecRef::new(BlockListSpec {
                type_name: "c".to_string(),
                nested: SpecRef::new(ObjectSpec::from_iter([
                    (
                        "val0",
                        SpecRef::new(AttrSpec {
                            name: "val0".to_string(),
                            ty: Type::string(),
                            required: false,
                        }),
                    ),
                    (
                        "val1",
                        SpecRef::new(AttrSpec {
                            name: "val1".to_string(),
                            ty: Type::string(),
                            required: false,
                        }),
                    ),
                ])),
                min_items: 0,
                max_items: 0,
            }),
            min_items: 0,
            max_items: 0,
        };

        let (_, diags) = hcldec::decode(&*dyn_body, &dec_spec, None);

        assert!(!diags.is_empty(), "Expected diagnostics, got none");
        assert!(
            diags.len() <= 1,
            "Expected one diagnostic message, got {}\n{diags:?}",
            diags.len(),
        );

        assert_eq!(
            diags[0].summary, "Invalid expression",
            "Expected error subject to be invalid expression, instead it was {:?}",
            diags[0].summary,
        );
    }
}

// Ported from TestVariables:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/dynblock/variables_test.go#L19
#[test]
#[ignore = "not yet implemented"]
fn variables() {
    const SRC: &str = r#"

# We have some references to things inside the "val" attribute inside each
# of our "b" blocks, which should be included in the result of WalkVariables
# but not WalkExpandVariables.

a {
  dynamic "b" {
    for_each = [for i, v in some_list_0: "${i}=${v},${baz}"]
    labels = ["${b.value} ${something_else_0}"]
    content {
      val = "${b.value} ${something_else_1}"
    }
  }
}

dynamic "a" {
  for_each = some_list_1

  content {
    b "foo" {
      val = "${a.value} ${something_else_2}"
    }

    dynamic "b" {
      for_each = some_list_2
      iterator = dyn_b
      labels = ["${a.value} ${dyn_b.value} ${b} ${something_else_3}"]
      content {
        val = "${a.value} ${dyn_b.value} ${something_else_4}"
      }
    }
  }
}

dynamic "a" {
  for_each = some_list_3
  iterator = dyn_a

  content {
    b "foo" {
      val = "${dyn_a.value} ${something_else_5}"
    }

    dynamic "b" {
      for_each = some_list_4
      labels = ["${dyn_a.value} ${b.value} ${a} ${something_else_6}"]
      content {
        val = "${dyn_a.value} ${b.value} ${something_else_7}"
      }
    }
  }
}
"#;

    let (f, diags) = hclsyntax::parse_config(SRC.as_bytes(), "", Pos::default());
    assert_eq!(
        diags.len(),
        0,
        "unexpected diagnostics during parse\n{diags:?}",
    );

    let spec = BlockListSpec {
        type_name: "a".to_string(),
        nested: SpecRef::new(BlockMapSpec {
            type_name: "b".to_string(),
            label_names: vec!["key".to_string()],
            nested: SpecRef::new(AttrSpec {
                name: "val".to_string(),
                ty: Type::string(),
                required: false,
            }),
        }),
        min_items: 0,
        max_items: 0,
    };

    // t.Run("WalkVariables")
    {
        let traversals = dynblock::variables_hcldec(f.body.clone(), &spec);
        let roots: Vec<String> = traversals.iter().map(|t| t.root_name()).collect();
        let got: Vec<&str> = roots.iter().map(String::as_str).collect();

        // The block structure is traversed one level at a time, so the ordering
        // here is reflecting first a pass of the root, then the first child
        // under the root, then the first child under that, etc.
        let want = vec![
            "some_list_1",
            "some_list_3",
            "some_list_0",
            "baz",
            "something_else_0",
            "something_else_1", // Would not be included for WalkExpandVariables because it only appears in content
            "some_list_2",
            "b", // This is correct because it is referenced in a context where the iterator is overridden to be dyn_b
            "something_else_3",
            "something_else_2", // Would not be included for WalkExpandVariables because it only appears in content
            "something_else_4", // Would not be included for WalkExpandVariables because it only appears in content
            "some_list_4",
            "a", // This is correct because it is referenced in a context where the iterator is overridden to be dyn_a
            "something_else_6",
            "something_else_5", // Would not be included for WalkExpandVariables because it only appears in content
            "something_else_7", // Would not be included for WalkExpandVariables because it only appears in content
        ];

        assert_eq!(
            got, want,
            "WalkVariables: wrong result\ngot: {got:?}\nwant: {want:?}",
        );
    }

    // t.Run("WalkExpandVariables")
    {
        let traversals = dynblock::expand_variables_hcldec(f.body.clone(), &spec);
        let roots: Vec<String> = traversals.iter().map(|t| t.root_name()).collect();
        let got: Vec<&str> = roots.iter().map(String::as_str).collect();

        // The block structure is traversed one level at a time, so the ordering
        // here is reflecting first a pass of the root, then the first child
        // under the root, then the first child under that, etc.
        let want = vec![
            "some_list_1",
            "some_list_3",
            "some_list_0",
            "baz",
            "something_else_0",
            "some_list_2",
            "b", // This is correct because it is referenced in a context where the iterator is overridden to be dyn_b
            "something_else_3",
            "some_list_4",
            "a", // This is correct because it is referenced in a context where the iterator is overridden to be dyn_a
            "something_else_6",
        ];

        assert_eq!(
            got, want,
            "WalkExpandVariables: wrong result\ngot: {got:?}\nwant: {want:?}",
        );
    }
}
