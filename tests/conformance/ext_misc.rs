//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ext/tryfunc/tryfunc_test.go
//!   ext/transform/transform_test.go
//!   ext/userfunc/decode_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;
use std::sync::Arc;

use cty::function::Function;
use cty::{Type, Value};
use hcl::ext::transform::{self, Transformer, TransformerFunc};
use hcl::ext::{tryfunc, userfunc};
use hcl::hclsyntax;
use hcl::hcltest::{mock_attrs, mock_body, mock_expr_literal};
use hcl::{
    AttributeSchema, Attributes, Block, BlockHeaderSchema, Blocks, BodyContent, BodyRef,
    BodySchema, EvalContext, Pos, Range,
};

/// An evaluation context with the given variables and functions
/// (Go: `&hcl.EvalContext{Variables: ..., Functions: ...}`).
fn ectx(
    variables: Vec<(&'static str, Value)>,
    functions: Vec<(&'static str, Function)>,
) -> EvalContext {
    let mut ctx = EvalContext::new();
    ctx.variables = variables
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<HashMap<_, _>>();
    ctx.functions = functions
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<HashMap<_, _>>();
    ctx
}

// Ported from TestTryFunc:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/tryfunc/tryfunc_test.go#L15
#[test]
#[ignore = "not yet implemented"]
fn try_func() {
    struct Case {
        name: &'static str,
        expr: &'static str,
        vars: Vec<(&'static str, Value)>,
        // NOTE(port): Go's `cty.NilVal` (the Value zero value, used when
        // only an error is expected) becomes `None`; it is never compared
        // against the result because those cases return early on the error
        // assertion, exactly as upstream does.
        want: Option<Value>,
        want_err: &'static str,
    }

    // NOTE(port): upstream's cases live in a Go map keyed by name (random
    // iteration order); they are kept here in source order.
    let tests = [
        Case {
            name: "one argument succeeds",
            expr: r#"try(1)"#,
            vars: vec![],
            want: Some(Value::number_int(1)),
            want_err: "",
        },
        Case {
            name: "one marked argument succeeds",
            expr: r#"try(sensitive)"#,
            vars: vec![("sensitive", Value::string("secret").mark("porpoise"))],
            want: Some(Value::string("secret").mark("porpoise")),
            want_err: "",
        },
        Case {
            name: "two arguments, first succeeds",
            expr: r#"try(1, 2)"#,
            vars: vec![],
            want: Some(Value::number_int(1)),
            want_err: "",
        },
        Case {
            name: "two arguments, first fails",
            expr: r#"try(nope, 2)"#,
            vars: vec![],
            want: Some(Value::number_int(2)),
            want_err: "",
        },
        Case {
            name: "two arguments, first depends on unknowns",
            expr: r#"try(unknown, 2)"#,
            vars: vec![("unknown", Value::unknown(Type::number()))],
            want: Some(Value::dynamic()), // can't proceed until first argument is known
            want_err: "",
        },
        Case {
            name: "two arguments, first succeeds and second depends on unknowns",
            expr: r#"try(1, unknown)"#,
            vars: vec![("unknown", Value::unknown(Type::number()))],
            // we know 1st succeeds, so it doesn't matter that 2nd is unknown
            want: Some(Value::number_int(1)),
            want_err: "",
        },
        Case {
            name: "two arguments, first depends on unknowns deeply",
            expr: r#"try(has_unknowns, 2)"#,
            vars: vec![("has_unknowns", Value::list([Value::unknown(Type::bool())]))],
            // can't proceed until first argument is wholly known
            want: Some(Value::dynamic()),
            want_err: "",
        },
        Case {
            name: "two arguments, first traverses through an unkown",
            expr: r#"try(unknown.baz, 2)"#,
            vars: vec![("unknown", Value::unknown(Type::map(Type::string())))],
            // can't proceed until first argument is wholly known
            want: Some(Value::dynamic()),
            want_err: "",
        },
        Case {
            name: "two arguments, both marked, first succeeds",
            expr: r#"try(sensitive, other)"#,
            vars: vec![
                ("sensitive", Value::string("secret").mark("porpoise")),
                ("other", Value::string("that").mark("a")),
            ],
            want: Some(Value::string("secret").mark("porpoise")),
            want_err: "",
        },
        Case {
            name: "two arguments, both marked, second succeeds",
            expr: r#"try(sensitive, other)"#,
            vars: vec![("other", Value::string("that").mark("a"))],
            want: Some(Value::string("that").mark("a")),
            want_err: "",
        },
        Case {
            name: "two arguments, result is element of marked list ",
            expr: r#"try(sensitive[0], other)"#,
            vars: vec![
                (
                    "sensitive",
                    Value::list([
                        Value::string("list"),
                        Value::string("of "),
                        Value::string("secrets"),
                    ])
                    .mark("secret"),
                ),
                ("other", Value::string("not")),
            ],
            want: Some(Value::string("list").mark("secret")),
            want_err: "",
        },
        Case {
            name: "nested known expression from unknown",
            // this expression contains an unknown, but will always return in
            // "bar"
            expr: r#"try({u: false ? unknown : "bar"}, other)"#,
            vars: vec![
                ("unknown", Value::unknown(Type::string())),
                ("other", Value::map([("v", Value::string("oops"))])),
            ],
            want: Some(Value::object([("u", Value::string("bar"))])),
            want_err: "",
        },
        Case {
            name: "nested index op on unknown",
            // unknown and other have identical types, but we must return a
            // dynamic value since v could change within the final result value
            // after the first argument becomes known.
            expr: r#"try({u: unknown["foo"], v: "orig"}, other)"#,
            vars: vec![
                ("unknown", Value::unknown(Type::map(Type::string()))),
                (
                    "other",
                    Value::map([("u", Value::string("oops")), ("v", Value::string("oops"))]),
                ),
            ],
            want: Some(Value::dynamic()),
            want_err: "",
        },
        Case {
            name: "three arguments, all fail",
            expr: r#"try(this, that, this_thing_in_particular)"#,
            vars: vec![],
            want: Some(Value::number_int(2)),
            // The grammar of this stringification of the message is unfortunate,
            // but caller can type-assert our result to get the original
            // diagnostics directly in order to produce a better result.
            want_err: r#"test.hcl:1,1-5: Error in function call; Call to function "try" failed: no expression succeeded:
- Variables not allowed (at test.hcl:1,5-9)
  Variables may not be used here.
- Variables not allowed (at test.hcl:1,11-15)
  Variables may not be used here.
- Variables not allowed (at test.hcl:1,17-41)
  Variables may not be used here.

At least one expression must produce a successful result."#,
        },
        Case {
            name: "no arguments",
            expr: r#"try()"#,
            vars: vec![],
            want: None,
            want_err: r#"test.hcl:1,1-5: Error in function call; Call to function "try" failed: at least one argument is required."#,
        },
    ];

    for test in tests {
        let (expr, diags) = hclsyntax::parse_expression(
            test.expr.as_bytes(),
            "test.hcl",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            !diags.has_errors(),
            "case {:?}: unexpected problems: {diags}",
            test.name,
        );

        let ctx = ectx(test.vars, vec![("try", tryfunc::try_func())]);

        let (got, err) = expr.value(Some(&ctx));

        if !err.is_empty() {
            if !test.want_err.is_empty() {
                assert_eq!(
                    format!("{err}"),
                    test.want_err,
                    "case {:?}: wrong error",
                    test.name,
                );
            } else {
                panic!(
                    "case {:?}: unexpected error\ngot:  {err}\nwant: <nil>",
                    test.name,
                );
            }
            continue;
        }
        assert!(
            test.want_err.is_empty(),
            "case {:?}: wrong error\ngot:  <nil>\nwant: {}",
            test.name,
            test.want_err,
        );

        let want = test
            .want
            .as_ref()
            .expect("want is NilVal only in error cases");
        assert!(
            want.raw_equals(&got),
            "case {:?}: wrong result\ngot:  {got:?}\nwant: {want:?}",
            test.name,
        );
    }
}

// Ported from TestCanFunc:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/tryfunc/tryfunc_test.go#L202
#[test]
#[ignore = "not yet implemented"]
fn can_func() {
    struct Case {
        name: &'static str,
        expr: &'static str,
        vars: Vec<(&'static str, Value)>,
        want: Value,
    }

    // NOTE(port): upstream's cases live in a Go map keyed by name (random
    // iteration order); they are kept here in source order.
    let tests = [
        Case {
            name: "succeeds",
            expr: r#"can(1)"#,
            vars: vec![],
            want: Value::bool(true),
        },
        Case {
            name: "fails",
            expr: r#"can(nope)"#,
            vars: vec![],
            want: Value::bool(false),
        },
        Case {
            name: "simple unknown",
            expr: r#"can(unknown)"#,
            vars: vec![("unknown", Value::unknown(Type::number()))],
            want: Value::unknown(Type::bool()),
        },
        Case {
            name: "traversal through unknown",
            expr: r#"can(unknown.foo)"#,
            vars: vec![("unknown", Value::unknown(Type::map(Type::number())))],
            want: Value::unknown(Type::bool()),
        },
        Case {
            name: "deep unknown",
            expr: r#"can(has_unknown)"#,
            vars: vec![("has_unknown", Value::list([Value::unknown(Type::bool())]))],
            want: Value::unknown(Type::bool()),
        },
    ];

    for test in tests {
        let (expr, diags) = hclsyntax::parse_expression(
            test.expr.as_bytes(),
            "test.hcl",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            !diags.has_errors(),
            "case {:?}: unexpected problems: {diags}",
            test.name,
        );

        let ctx = ectx(test.vars, vec![("can", tryfunc::can_func())]);

        let (got, err) = expr.value(Some(&ctx));
        assert!(
            err.is_empty(),
            "case {:?}: unexpected error\ngot:  {err}\nwant: <nil>",
            test.name,
        );
        assert!(
            test.want.raw_equals(&got),
            "case {:?}: wrong result\ngot:  {got:?}\nwant: {:?}",
            test.name,
            test.want,
        );
    }
}

// NOTE(port): upstream's compile-time interface assertion
// (`var _ hcl.Body = deepWrapper{}`, transform_test.go line 17) has no
// direct analogue: `deepWrapper` is private to `hcl::ext::transform`, and
// `transform::deep` returning `BodyRef` already requires the `Body` impl at
// compile time.

// Ported from TestDeep:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/transform/transform_test.go#L19
#[test]
#[ignore = "not yet implemented"]
fn deep() {
    #[allow(clippy::arc_with_non_send_sync)]
    let test_transform: Arc<dyn Transformer> =
        Arc::new(TransformerFunc(Arc::new(|body: BodyRef| -> BodyRef {
            let (_, remain, diags) = body.partial_content(&BodySchema {
                attributes: Vec::new(),
                blocks: vec![BlockHeaderSchema {
                    block_type: "remove".to_string(),
                    label_names: Vec::new(),
                }],
            });

            transform::body_with_diagnostics(remain, diags)
        })));

    // NOTE(port): Go leaves `Block.Body` as a nil interface for the nested
    // "remove" block and the remaining `Block`/`BodyContent` fields as zero
    // values; `BodyRef` cannot be nil, so `hcl::empty_body()` stands in (the
    // block is filtered out by the transform, so its body is never read).
    let src = mock_body(BodyContent {
        attributes: mock_attrs(HashMap::from([(
            "true".to_string(),
            mock_expr_literal(Value::bool(true)),
        )])),
        blocks: Blocks(vec![
            Block {
                block_type: "remove".to_string(),
                labels: Vec::new(),
                body: hcl::empty_body(),
                def_range: Range::default(),
                type_range: Range::default(),
                label_ranges: Vec::new(),
            },
            Block {
                block_type: "child".to_string(),
                labels: Vec::new(),
                body: mock_body(BodyContent {
                    attributes: Attributes::new(),
                    blocks: Blocks(vec![Block {
                        block_type: "remove".to_string(),
                        labels: Vec::new(),
                        body: hcl::empty_body(),
                        def_range: Range::default(),
                        type_range: Range::default(),
                        label_ranges: Vec::new(),
                    }]),
                    missing_item_range: Range::default(),
                }),
                def_range: Range::default(),
                type_range: Range::default(),
                label_ranges: Vec::new(),
            },
        ]),
        missing_item_range: Range::default(),
    });

    let wrapped = transform::deep(src, test_transform);

    let (root_content, diags) = wrapped.content(&BodySchema {
        attributes: vec![AttributeSchema {
            name: "true".to_string(),
            required: false,
        }],
        blocks: vec![BlockHeaderSchema {
            block_type: "child".to_string(),
            label_names: Vec::new(),
        }],
    });
    assert!(
        diags.is_empty(),
        "unexpected diagnostics for root content:\n{diags:?}",
    );

    let want_attrs = mock_attrs(HashMap::from([(
        "true".to_string(),
        mock_expr_literal(Value::bool(true)),
    )]));
    assert_eq!(
        root_content.attributes, want_attrs,
        "wrong root attributes\ngot:  {:?}\nwant: {want_attrs:?}",
        root_content.attributes,
    );

    assert_eq!(
        root_content.blocks.len(),
        1,
        "wrong number of root blocks {}; want 1",
        root_content.blocks.len(),
    );
    assert_eq!(
        root_content.blocks[0].block_type, "child",
        "wrong block type {}; want child",
        root_content.blocks[0].block_type,
    );

    let child_block = &root_content.blocks[0];
    let (child_content, diags) = child_block.body.content(&BodySchema::default());
    assert!(
        diags.is_empty(),
        "unexpected diagnostics for child content:\n{diags:?}",
    );

    assert!(
        child_content.attributes.is_empty(),
        "unexpected attributes in child content; want empty content",
    );
    assert!(
        child_content.blocks.is_empty(),
        "unexpected blocks in child content; want empty content",
    );
}

// Ported from TestDecodeUserFunctions:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/userfunc/decode_test.go#L15
#[test]
#[ignore = "not yet implemented"]
fn decode_user_functions() {
    struct Case {
        src: &'static str,
        test_expr: &'static str,
        base_ctx: Option<EvalContext>,
        want: Value,
        diag_count: usize,
    }

    let tests = [
        Case {
            src: r#"
function "greet" {
  params = [name]
  result = "Hello, ${name}."
}
"#,
            test_expr: r#"greet("Ermintrude")"#,
            base_ctx: None,
            want: Value::string("Hello, Ermintrude."),
            diag_count: 0,
        },
        Case {
            src: r#"
function "greet" {
  params = [name]
  result = "Hello, ${name}."
}
"#,
            test_expr: r#"greet()"#,
            base_ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // missing value for "name"
        },
        Case {
            src: r#"
function "greet" {
  params = [name]
  result = "Hello, ${name}."
}
"#,
            test_expr: r#"greet("Ermintrude", "extra")"#,
            base_ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // too many arguments
        },
        Case {
            src: r#"
function "add" {
  params = [a, b]
  result = a + b
}
"#,
            test_expr: r#"add(1, 5)"#,
            base_ctx: None,
            want: Value::number_int(6),
            diag_count: 0,
        },
        Case {
            src: r#"
function "argstuple" {
  params = []
  variadic_param = args
  result = args
}
"#,
            test_expr: r#"argstuple("a", true, 1)"#,
            base_ctx: None,
            want: Value::tuple([Value::string("a"), Value::bool(true), Value::number_int(1)]),
            diag_count: 0,
        },
        Case {
            src: r#"
function "missing_var" {
  params = []
  result = nonexist
}
"#,
            test_expr: r#"missing_var()"#,
            base_ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // no variable named "nonexist"
        },
        Case {
            src: r#"
function "closure" {
  params = []
  result = upvalue
}
"#,
            test_expr: r#"closure()"#,
            base_ctx: Some(ectx(vec![("upvalue", Value::bool(true))], vec![])),
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            src: r#"
function "neg" {
  params = [val]
  result = -val
}
function "add" {
  params = [a, b]
  result = a + b
}
"#,
            test_expr: r#"neg(add(1, 3))"#,
            base_ctx: None,
            want: Value::number_int(-4),
            diag_count: 0,
        },
        Case {
            src: r#"
function "neg" {
  parrams = [val]
  result = -val
}
"#,
            test_expr: r#"null"#,
            base_ctx: None,
            want: Value::null(Type::dynamic()),
            diag_count: 2, // missing attribute "params", and unknown attribute "parrams"
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let (f, mut diags) = hclsyntax::parse_config(
            test.src.as_bytes(),
            "config",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        // NOTE(port): upstream guards `f == nil || f.Body == nil`;
        // parse_config always returns a File whose `body` is a non-nil
        // `BodyRef`, so the guard is unrepresentable.

        // NOTE(port): upstream passes the unexported `decodeUserFunctions`
        // a contextFunc closure returning test.baseCtx (possibly nil); the
        // public DecodeUserFunctions is a thin wrapper over it with the
        // same behavior.
        let base_ctx = test.base_ctx.map(Arc::new);
        let (funcs, _, funcs_diags) = userfunc::decode_user_functions(
            f.body.clone(),
            "function",
            Some(Arc::new(move || base_ctx.clone())),
        );
        diags.extend(funcs_diags);

        let (expr, expr_parse_diags) = hclsyntax::parse_expression(
            test.test_expr.as_bytes(),
            "testexpr",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        diags.extend(expr_parse_diags);
        // NOTE(port): upstream guards `expr == nil`; parse_expression always
        // returns an Expression, so the guard is unrepresentable.

        let mut ctx = EvalContext::new();
        ctx.functions = funcs;
        let (got, expr_diags) = expr.value(Some(&ctx));
        diags.extend(expr_diags);

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i}: wrong number of diagnostics {}; want {}\n{:?}",
            diags.len(),
            test.diag_count,
            *diags,
        );

        assert!(
            got.raw_equals(&test.want),
            "case {i}: wrong result\ngot:  {got:?}\nwant: {:?}",
            test.want,
        );
    }
}
