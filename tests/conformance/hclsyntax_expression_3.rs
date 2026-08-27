//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_test.go (TestExpressionParseAndValue, part 3)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value};
use hcl::hclsyntax;
use hcl::{EvalContext, Pos};

struct Case {
    input: &'static str,
    ctx: Option<EvalContext>,
    want: Value,
    diag_count: usize,
}

/// An eval context with the given variables (Go: `&hcl.EvalContext{
/// Variables: map[string]cty.Value{...}}`), wrapped in `Some` for passing
/// where Go passes a non-nil `*hcl.EvalContext`.
fn vars_ctx<const N: usize>(vars: [(&'static str, Value); N]) -> Option<EvalContext> {
    let mut ctx = EvalContext::new();
    ctx.variables = vars.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Some(ctx)
}

// Ported from TestExpressionParseAndValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L17
// (part 3: cases with opening brace at upstream line 1668 or later)
#[test]
#[ignore = "not yet implemented"]
fn expression_parse_and_value_part3() {
    let tests = vec![
        Case {
            input: r#"bar"#,
            ctx: Some(EvalContext::new()),
            want: Value::dynamic(),
            diag_count: 1, // variables not allowed here
        },
        Case {
            input: r#"foo.bar"#,
            ctx: vars_ctx([("foo", Value::string("hello"))]),
            want: Value::dynamic(),
            diag_count: 1, // foo does not have attributes
        },
        Case {
            input: r#"foo.baz"#,
            ctx: vars_ctx([("foo", Value::object([("baz", Value::string("hello"))]))]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"foo["baz"]"#,
            ctx: vars_ctx([("foo", Value::object([("baz", Value::string("hello"))]))]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"foo[true]"#, // key is converted to string
            ctx: vars_ctx([("foo", Value::object([("true", Value::string("hello"))]))]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"foo[0].baz"#,
            ctx: vars_ctx([(
                "foo",
                Value::list([Value::object([("baz", Value::string("hello"))])]),
            )]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: "\n<<EOT\nFoo\nBar\nBaz\nEOT\n",
            ctx: None,
            want: Value::string("Foo\nBar\nBaz\n"),
            diag_count: 0,
        },
        Case {
            input: "\n<<EOT\nFoo\n${bar}\nBaz\nEOT\n",
            ctx: vars_ctx([("bar", Value::string("Bar"))]),
            want: Value::string("Foo\nBar\nBaz\n"),
            diag_count: 0,
        },
        Case {
            input: "\n<<EOT\nFoo\n%{for x in bars}${x}%{endfor}\nBaz\nEOT\n",
            ctx: vars_ctx([(
                "bars",
                Value::list([
                    Value::string("Bar"),
                    Value::string("Bar"),
                    Value::string("Bar"),
                ]),
            )]),
            want: Value::string("Foo\nBarBarBar\nBaz\n"),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<EOT\n  Foo\n  Bar\n  Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("  Foo\n  Bar\n  Baz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<-EOT\n  Foo\n  Bar\n  Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("Foo\nBar\nBaz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<-EOT\n  Foo\n    Bar\n    Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("Foo\n  Bar\n  Baz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<-EOT\n    Foo\n  Bar\n    Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("  Foo\nBar\n  Baz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<-EOT\n    Foo\n  ${bar}\n    Baz\n    EOT\n]\n",
            // Spaces in the interpolation result don't affect the outcome
            ctx: vars_ctx([("bar", Value::string("  Bar"))]),
            want: Value::tuple([Value::string("  Foo\n  Bar\n  Baz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<EOT\n  Foo\n\n  Bar\n\n  Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("  Foo\n\n  Bar\n\n  Baz\n")]),
            diag_count: 0,
        },
        Case {
            input: "[\n  <<-EOT\n  Foo\n\n  Bar\n\n  Baz\n  EOT\n]\n",
            ctx: None,
            want: Value::tuple([Value::string("Foo\n\nBar\n\nBaz\n")]),
            diag_count: 0,
        },
        Case {
            input: r#"unk["baz"]"#,
            ctx: vars_ctx([("unk", Value::unknown(Type::string()))]),
            want: Value::dynamic(),
            diag_count: 1, // value does not have indices (because we know it's a string)
        },
        Case {
            input: r#"unk["boop"]"#,
            ctx: vars_ctx([("unk", Value::unknown(Type::map(Type::string())))]),
            want: Value::unknown(Type::string()), // we know it's a map of string
            diag_count: 0,
        },
        Case {
            input: r#"dyn["boop"]"#,
            ctx: vars_ctx([("dyn", Value::dynamic())]),
            want: Value::dynamic(), // don't know what it is yet
            diag_count: 0,
        },
        Case {
            input: r#"nullstr == "foo""#,
            ctx: vars_ctx([("nullstr", Value::null(Type::string()))]),
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"nullstr == nullstr"#,
            ctx: vars_ctx([("nullstr", Value::null(Type::string()))]),
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#"nullstr == null"#,
            ctx: vars_ctx([("nullstr", Value::null(Type::string()))]),
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#"nullstr == nullnum"#,
            ctx: vars_ctx([
                ("nullstr", Value::null(Type::string())),
                ("nullnum", Value::null(Type::number())),
            ]),
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#""" == nulldyn"#,
            ctx: vars_ctx([("nulldyn", Value::null(Type::dynamic()))]),
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            // Logical AND operator short-circuit behavior
            input: r#"nullobj != null && nullobj.is_thingy"#,
            ctx: vars_ctx([(
                "nullobj",
                Value::null(Type::object([("is_thingy", Type::bool())])),
            )]),
            want: Value::bool(false),
            diag_count: 0, // nullobj != null prevents evaluating nullobj.is_thingy
        },
        Case {
            // Logical AND short-circuit handling of unknown values
            // If the first operand is an unknown bool then we can't know if
            // we will short-circuit or not, and so we must assume we will
            // and wait until the value becomes known before fully evaluating RHS.
            input: r#"unknown < 4 && list[zero]"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::number())),
                ("zero", Value::zero()),
                ("list", Value::list_empty(Type::bool())),
            ]),
            want: Value::unknown(Type::bool()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            // Logical OR operator short-circuit behavior
            input: r#"nullobj == null || nullobj.is_thingy"#,
            ctx: vars_ctx([(
                "nullobj",
                Value::null(Type::object([("is_thingy", Type::bool())])),
            )]),
            want: Value::bool(true),
            diag_count: 0, // nullobj == null prevents evaluating nullobj.is_thingy
        },
        Case {
            // Logical OR short-circuit handling of unknown values
            // If the first operand is an unknown bool then we can't know if
            // we will short-circuit or not, and so we must assume we will
            // and wait until the value becomes known before fully evaluating RHS.
            input: r#"unknown > 4 || list[zero]"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::number())),
                ("zero", Value::zero()),
                ("list", Value::list_empty(Type::bool())),
            ]),
            want: Value::unknown(Type::bool()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            // short circuit calls must still retain marks
            input: r#"lhsTrue || rhsUnknown"#,
            ctx: vars_ctx([
                ("lhsTrue", Value::bool(true).mark("a")),
                ("rhsUnknown", Value::unknown(Type::bool()).mark("b")),
            ]),
            want: Value::bool(true).mark("a").mark("b"),
            diag_count: 0,
        },
        Case {
            // short circuit calls must still retain marks
            input: r#"lhsUnknown || rhsTrue"#,
            ctx: vars_ctx([
                ("rhsTrue", Value::bool(true).mark("a")),
                ("lhsUnknown", Value::unknown(Type::bool()).mark("b")),
            ]),
            want: Value::bool(true).mark("a").mark("b"),
            diag_count: 0,
        },
        Case {
            // short circuit calls must still retain marks
            input: r#"lhsUnknown && rhsFalse"#,
            ctx: vars_ctx([
                ("rhsFalse", Value::bool(false).mark("a")),
                ("lhsUnknown", Value::unknown(Type::bool()).mark("b")),
            ]),
            want: Value::bool(false).mark("a").mark("b"),
            diag_count: 0,
        },
        Case {
            // short circuit calls must still retain marks
            input: r#"lhsFalse && rhsUnknown"#,
            ctx: vars_ctx([
                ("lhsFalse", Value::bool(false).mark("a")),
                ("rhsUnknown", Value::unknown(Type::bool()).mark("b")),
            ]),
            want: Value::bool(false).mark("a").mark("b"),
            diag_count: 0,
        },
        Case {
            input: r#"true ? var : null"#,
            ctx: vars_ctx([("var", Value::object([("a", Value::string("A"))]))]),
            want: Value::object([("a", Value::string("A"))]),
            diag_count: 0,
        },
        Case {
            input: r#"true ? var : null"#,
            ctx: vars_ctx([("var", Value::unknown(Type::dynamic()))]),
            want: Value::unknown(Type::dynamic()),
            diag_count: 0,
        },
        Case {
            input: r#"true ? ["a", "b"] : null"#,
            ctx: None,
            want: Value::tuple([Value::string("a"), Value::string("b")]),
            diag_count: 0,
        },
        Case {
            input: r#"true ? null: ["a", "b"]"#,
            ctx: None,
            want: Value::null(Type::tuple([Type::string(), Type::string()])),
            diag_count: 0,
        },
        Case {
            input: r#"false ? ["a", "b"] : null"#,
            ctx: None,
            want: Value::null(Type::tuple([Type::string(), Type::string()])),
            diag_count: 0,
        },
        Case {
            input: r#"false ? null: ["a", "b"]"#,
            ctx: None,
            want: Value::tuple([Value::string("a"), Value::string("b")]),
            diag_count: 0,
        },
        Case {
            input: r#"false ? null: null"#,
            ctx: None,
            want: Value::null(Type::dynamic()),
            diag_count: 0,
        },
        Case {
            input: r#"false ? var: {a = "b"}"#,
            ctx: vars_ctx([("var", Value::dynamic())]),
            want: Value::object([("a", Value::string("b"))]),
            diag_count: 0,
        },
        Case {
            input: r#"true ? ["a", "b"]: var"#,
            ctx: vars_ctx([("var", Value::unknown(Type::dynamic()))]),
            want: Value::tuple([Value::string("a"), Value::string("b")]),
            diag_count: 0,
        },
        Case {
            input: r#"false ? ["a", "b"]: var"#,
            ctx: vars_ctx([("var", Value::dynamic())]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"false ? ["a", "b"]: var"#,
            ctx: vars_ctx([("var", Value::unknown(Type::dynamic()))]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? 1 : 0"#,
            ctx: vars_ctx([("unknown", Value::unknown(Type::bool()))]),
            want: Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::zero(), true)
                .number_range_upper_bound(Value::number_int(1), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? 0 : 1"#,
            ctx: vars_ctx([("unknown", Value::unknown(Type::bool()))]),
            want: Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::zero(), true)
                .number_range_upper_bound(Value::number_int(1), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? i : j"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("i", Value::null(Type::number())),
                ("j", Value::null(Type::number())),
            ]),
            want: Value::null(Type::number()),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? im : jm"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("im", Value::null(Type::number()).mark("a")),
                ("jm", Value::null(Type::number()).mark("b")),
            ]),
            want: Value::null(Type::number()).mark("a").mark("b"),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? im : jm"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool()).mark("a")),
                ("im", Value::unknown(Type::number())),
                ("jm", Value::unknown(Type::number()).mark("b")),
            ]),
            // the empty refinement may eventually be removed, but does nothing here
            want: Value::unknown(Type::number())
                .refine()
                .new_value()
                .mark("a")
                .mark("b"),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? ix : jx"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("ix", Value::unknown(Type::number())),
                ("jx", Value::unknown(Type::number())),
            ]),
            // the empty refinement may eventually be removed, but does nothing here
            want: Value::unknown(Type::number()).refine().new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? ir : jr"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                (
                    "ir",
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number_int(1), false)
                        .number_range_upper_bound(Value::number_int(3), false)
                        .new_value(),
                ),
                (
                    "jr",
                    Value::unknown(Type::number())
                        .refine()
                        .number_range_lower_bound(Value::number_int(2), true)
                        .number_range_upper_bound(Value::number_int(4), true)
                        .new_value(),
                ),
            ]),
            want: Value::unknown(Type::number())
                .refine()
                .number_range_lower_bound(Value::number_int(1), false)
                .number_range_upper_bound(Value::number_int(4), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? a : b"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("a", Value::unknown(Type::bool()).refine_not_null()),
                ("b", Value::unknown(Type::bool()).refine_not_null()),
            ]),
            want: Value::unknown(Type::bool()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? al : bl"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("al", Value::list_empty(Type::string())),
                ("bl", Value::list_empty(Type::string())),
            ]),
            want: Value::list_empty(Type::string()), // deduced through refinements
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? am : bm"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("am", Value::map_empty(Type::string())),
                ("bm", Value::map_empty(Type::string()).mark("test")),
            ]),
            want: Value::map_empty(Type::string()).mark("test"), // deduced through refinements
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? ar : br"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                (
                    "ar",
                    Value::unknown(Type::set(Type::string()))
                        .refine()
                        .collection_length_lower_bound(1)
                        .collection_length_upper_bound(3)
                        .new_value(),
                ),
                (
                    "br",
                    Value::unknown(Type::set(Type::string()))
                        .refine()
                        .collection_length_lower_bound(2)
                        .collection_length_upper_bound(4)
                        .new_value(),
                ),
            ]),
            // deduced through refinements
            want: Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(4)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? arn : brn"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                (
                    "arn",
                    Value::unknown(Type::set(Type::string()))
                        .refine()
                        .not_null()
                        .collection_length_lower_bound(1)
                        .collection_length_upper_bound(2)
                        .new_value(),
                ),
                (
                    "brn",
                    Value::unknown(Type::set(Type::string()))
                        .refine()
                        .not_null()
                        .collection_length_lower_bound(3)
                        .collection_length_upper_bound(4)
                        .new_value(),
                ),
            ]),
            // deduced through refinements
            want: Value::unknown(Type::set(Type::string()))
                .refine()
                .not_null()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(4)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? amr : bmr"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                (
                    "amr",
                    Value::unknown(Type::set(Type::string()))
                        .mark("test")
                        .refine()
                        .collection_length_lower_bound(1)
                        .collection_length_upper_bound(2)
                        .new_value(),
                ),
                (
                    "bmr",
                    Value::unknown(Type::set(Type::string()))
                        .mark("test")
                        .refine()
                        .collection_length_lower_bound(3)
                        .collection_length_upper_bound(4)
                        .new_value(),
                ),
            ]),
            // deduced through refinements
            want: Value::unknown(Type::set(Type::string()))
                .refine()
                .collection_length_lower_bound(1)
                .collection_length_upper_bound(4)
                .new_value()
                .mark("test"),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? a : b"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("a", Value::list_empty(Type::string())),
                ("b", Value::list([Value::unknown(Type::string())])),
            ]),
            want: Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_upper_bound(1)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unknown ? a : b"#,
            ctx: vars_ctx([
                ("unknown", Value::unknown(Type::bool())),
                ("a", Value::list([Value::string("hello")])),
                ("b", Value::list([Value::unknown(Type::string())])),
            ]),
            // deduced through refinements
            want: Value::list([Value::unknown(Type::string())]),
            diag_count: 0,
        },
        Case {
            // marked conditional
            input: r#"var.foo ? 1 : 0"#,
            ctx: vars_ctx([(
                "var",
                Value::object([("foo", Value::bool(true))]).mark("sensitive"),
            )]),
            want: Value::number_int(1).mark("sensitive"),
            diag_count: 0,
        },
        Case {
            // auto-converts collection types
            input: r#"true ? listOf1Tuple : listOf0Tuple"#,
            ctx: vars_ctx([
                (
                    "listOf1Tuple",
                    Value::list([Value::tuple([Value::bool(true)])]),
                ),
                ("listOf0Tuple", Value::list([Value::empty_tuple()])),
            ]),
            want: Value::list([Value::list([Value::bool(true)])]),
            diag_count: 0,
        },
        Case {
            input: r#"true ? setOf1Tuple : setOf0Tuple"#,
            ctx: vars_ctx([
                (
                    "setOf1Tuple",
                    Value::set([Value::tuple([Value::bool(true)])]),
                ),
                ("setOf0Tuple", Value::set([Value::empty_tuple()])),
            ]),
            want: Value::set([Value::list([Value::bool(true)])]),
            diag_count: 0,
        },
        Case {
            // marked argument expansion
            input: r#"min(xs...)"#,
            ctx: {
                let mut ctx = EvalContext::new();
                ctx.functions = [("min".to_string(), stdlib::min_func())]
                    .into_iter()
                    .collect();
                ctx.variables = [(
                    "xs".to_string(),
                    Value::list([
                        Value::number_int(3),
                        Value::number_int(1),
                        Value::number_int(4),
                    ])
                    .mark("sensitive"),
                )]
                .into_iter()
                .collect();
                Some(ctx)
            },
            want: Value::number_int(1).mark("sensitive"),
            diag_count: 0,
        },
        Case {
            input: r#"test ? sensitiveString : """#,
            // Go: `Functions: map[string]function.Function{}` — an
            // explicitly empty function table, same as the default here.
            ctx: vars_ctx([
                ("test", Value::unknown(Type::bool())),
                ("sensitiveString", Value::string("test").mark("sensitive")),
            ]),
            want: Value::unknown(Type::string())
                .refine_not_null()
                .mark("sensitive"),
            diag_count: 0,
        },
        Case {
            // foo does not exist, but we need to catch the diagnostics when
            // coming out of a ShortCircuit call
            input: "foo(value) && true",
            ctx: Some(EvalContext::new()),
            want: Value::unknown(Type::bool()).refine_not_null(),
            diag_count: 1,
        },
        Case {
            // foo does not exist, but the short-circuit wins
            input: "foo(value) && false",
            ctx: Some(EvalContext::new()),
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            // foo does not exist, but we need to catch the diagnostics when
            // coming out of a ShortCircuit call
            input: "foo(value) || false",
            ctx: Some(EvalContext::new()),
            want: Value::unknown(Type::bool()).refine_not_null(),
            diag_count: 1,
        },
        Case {
            // foo does not exist, but the short-circuit wins
            input: "foo(value) || true",
            ctx: Some(EvalContext::new()),
            want: Value::bool(true),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.input.as_bytes(),
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
        );
        let (got, val_diags) = expr.value(test.ctx.as_ref());

        let diag_count = parse_diags.len() + val_diags.len();
        assert_eq!(
            diag_count, test.diag_count,
            "case {i} ({:?}): wrong number of diagnostics; parse: {:?}, value: {:?}",
            test.input, *parse_diags, *val_diags,
        );

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.input);
    }
}
