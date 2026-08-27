//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_test.go (TestExpressionErrorMessages,
//!   TestFunctionCallExprValue, TestExpressionAsTraversal,
//!   TestStaticExpressionList, TestParseExpression_incompleteFunctionCall,
//!   TestAllBoolExpressions)
//!   hclsyntax/expression_typeparams_test.go (TestExpressionDiagnosticExtra)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.
//!
//! TestExpressionParseAndValue is ported separately in
//! hclsyntax_expression_1.rs / _2.rs / _3.rs.

use std::any::Any;
use std::sync::Arc;

use cty::function::stdlib::{json_decode_func, strlen_func};
use cty::function::{Function, Spec, static_return_type};
use cty::{Type, Value};
use hcl::hclsyntax::{
    self, Expression, FunctionCallDiagExtra, FunctionCallExpr, FunctionCallUnknownDiagExtra,
    LiteralValueExpr,
};
use hcl::{
    DiagnosticExtra, DiagnosticSeverity, Diagnostics, EvalContext, Pos, Range, Traverser,
    diagnostic_extra,
};

/// An evaluation context with only the given variables
/// (Go: `&hcl.EvalContext{Variables: map[string]cty.Value{...}}`).
fn ctx_with_vars(vars: impl IntoIterator<Item = (&'static str, Value)>) -> EvalContext {
    let mut ctx = EvalContext::new();
    ctx.variables = vars.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    ctx
}

// Ported from TestExpressionErrorMessages:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L2491
#[test]
#[ignore = "not yet implemented"]
fn expression_error_messages() {
    struct Case {
        input: &'static str,
        ctx: Option<EvalContext>,
        want_summary: &'static str,
        want_detail: &'static str,
    }

    let tests = [
        // Error messages describing inconsistent result types for conditional expressions.
        Case {
            input: "true ? 1 : true",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. The 'true' value is number, but the 'false' value is bool.",
        },
        Case {
            input: "true ? [1] : [true]",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. Type mismatch for tuple element 0: The 'true' value is number, but the 'false' value is bool.",
        },
        Case {
            input: "true ? [1] : [1, true]",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. The 'true' tuple has length 1, but the 'false' tuple has length 2.",
        },
        Case {
            input: "true ? { a = 1 } : { a = true }",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. Type mismatch for object attribute \"a\": The 'true' value is number, but the 'false' value is bool.",
        },
        Case {
            input: "true ? { a = true, b = 1 } : { a = true }",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. The 'true' value includes object attribute \"b\", which is absent in the 'false' value.",
        },
        Case {
            input: "true ? { a = true } : { a = true, b = 1 }",
            ctx: None,
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. The 'false' value includes object attribute \"b\", which is absent in the 'true' value.",
        },
        // Failing cases for automatic collection conversions. HCL and cty
        // will attempt to unify tuples into lists. We have to make sure
        // the tuple inner types have no common base type, so we mix and
        // match booleans and numbers and validate the error messages.
        Case {
            input: "true ? listOf2Tuple : listOf1Tuple",
            ctx: Some(ctx_with_vars([
                (
                    "listOf2Tuple",
                    Value::list([Value::tuple([Value::bool(true), Value::zero()])]),
                ),
                (
                    "listOf1Tuple",
                    Value::list([Value::tuple([Value::bool(true)])]),
                ),
            ])),
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. Mismatched list element types: The 'true' tuple has length 2, but the 'false' tuple has length 1.",
        },
        Case {
            input: "true ? setOf2Tuple : setOf1Tuple",
            ctx: Some(ctx_with_vars([
                (
                    "setOf2Tuple",
                    Value::set([Value::tuple([Value::bool(true), Value::zero()])]),
                ),
                (
                    "setOf1Tuple",
                    Value::set([Value::tuple([Value::bool(true)])]),
                ),
            ])),
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. Mismatched set element types: The 'true' tuple has length 2, but the 'false' tuple has length 1.",
        },
        Case {
            input: "true ? mapOf1Tuple : mapOf2Tuple",
            ctx: Some(ctx_with_vars([
                (
                    "mapOf1Tuple",
                    Value::map([("a", Value::tuple([Value::bool(true)]))]),
                ),
                (
                    "mapOf2Tuple",
                    Value::map([("a", Value::tuple([Value::bool(true), Value::zero()]))]),
                ),
            ])),
            want_summary: "Inconsistent conditional result types",
            want_detail: "The true and false result expressions must have consistent types. Mismatched map element types: The 'true' tuple has length 1, but the 'false' tuple has length 2.",
        },
        Case {
            input: "true ? listOfListOf2Tuple : listOfListOf1Tuple",
            ctx: Some(ctx_with_vars([
                (
                    "listOfListOf2Tuple",
                    Value::list([Value::list([Value::tuple([
                        Value::bool(true),
                        Value::zero(),
                    ])])]),
                ),
                (
                    "listOfListOf1Tuple",
                    Value::list([Value::list([Value::tuple([Value::bool(true)])])]),
                ),
            ])),
            want_summary: "Inconsistent conditional result types",
            // This is our totally non-specific last-resort of an error message,
            // for situations that are too complex for any of our rules to
            // describe coherently.
            want_detail: "The true and false result expressions must have consistent types. At least one deeply-nested attribute or element is not compatible across both the 'true' and the 'false' value.",
        },
        // Error messages describing situations where the logical operator
        // short-circuit behavior still found a type error on the RHS that
        // we therefore still report, because the LHS only guards against
        // value-related problems in the RHS.
        Case {
            // It's not valid to access an attribute on a non-object-typed
            // value even if we've proven it isn't null.
            input: "notobj != null && notobj.foo",
            ctx: Some(ctx_with_vars([("notobj", Value::bool(true))])),
            want_summary: "Unsupported attribute",
            want_detail: "Can't access attributes on a primitive-typed value (bool).",
        },
        Case {
            // It's not valid to access an attribute on a non-object-typed
            // value even if we've proven it isn't null.
            input: "notobj == null || notobj.foo",
            ctx: Some(ctx_with_vars([("notobj", Value::bool(true))])),
            want_summary: "Unsupported attribute",
            want_detail: "Can't access attributes on a primitive-typed value (bool).",
        },
        Case {
            // It's not valid to access an index on an unindexable type
            // even if we've proven it isn't null.
            input: "notlist != null && notlist[0]",
            ctx: Some(ctx_with_vars([("notlist", Value::bool(true))])),
            want_summary: "Invalid index",
            want_detail: "This value does not have any indices.",
        },
        Case {
            // Short-circuit can't avoid an error accessing a variable that
            // doesn't exist at all, so we can still report typos.
            input: "value != null && valeu",
            ctx: Some(ctx_with_vars([("value", Value::bool(true))])),
            want_summary: "Unknown variable",
            want_detail: "There is no variable named \"valeu\". Did you mean \"value\"?",
        },
        Case {
            // Short-circuit must still catch type errors on the opposite side
            input: "unknown && \"value\"",
            ctx: Some(ctx_with_vars([("unknown", Value::unknown(Type::bool()))])),
            want_summary: "Invalid operand",
            want_detail: "Unsuitable value for right operand: a bool is required.",
        },
        Case {
            // Short-circuiting must still catch type errors on the opposite side
            input: "value && \"value\"",
            ctx: Some(ctx_with_vars([("value", Value::bool(false))])),
            want_summary: "Invalid operand",
            want_detail: "Unsuitable value for right operand: a bool is required.",
        },
        Case {
            input: "foo(value) && true",
            ctx: Some(EvalContext::new()),
            want_summary: "Function calls not allowed",
            want_detail: "Functions may not be called here.",
        },
        Case {
            input: "map != null || map[\"key\"] == \"value\"",
            ctx: Some(ctx_with_vars([(
                "map",
                Value::null(Type::map(Type::string())),
            )])),
            want_summary: "Attempt to index null value",
            want_detail: "This value is null, so it does not have any indices.",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut diags = Diagnostics::new();
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        diags.extend(parse_diags);
        let (_, val_diags) = expr.value(test.ctx.as_ref());
        diags.extend(val_diags);

        assert!(
            diags.has_errors(),
            "case {i} ({}): unexpected success\nwant error:\n{}; {}",
            test.input,
            test.want_summary,
            test.want_detail,
        );

        let found = diags.iter().any(|diag| {
            diag.severity == DiagnosticSeverity::Error
                && diag.summary == test.want_summary
                && diag.detail == test.want_detail
        });
        // If we get here without finding it then we didn't find the
        // diagnostic we were looking for.
        assert!(
            found,
            "case {i} ({}): missing expected error\ngot:\n{diags}\n\nwant error:\n{}; {}",
            test.input, test.want_summary, test.want_detail,
        );
    }
}

/// A literal-value expression with a zero-value source range
/// (Go: `&LiteralValueExpr{Val: ...}`).
fn lit(val: Value) -> Expression {
    LiteralValueExpr {
        val,
        src_range: Range::default(),
    }
    .into()
}

/// A function-call expression with only name and args populated
/// (Go: `&FunctionCallExpr{Name: ..., Args: ...}`).
fn call_expr(name: &str, args: Vec<Expression>) -> FunctionCallExpr {
    FunctionCallExpr {
        name: name.to_string(),
        args,
        expand_final: false,
        name_range: Range::default(),
        open_paren_range: Range::default(),
        close_paren_range: Range::default(),
    }
}

// Ported from TestFunctionCallExprValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L2707
#[test]
#[ignore = "not yet implemented"]
fn function_call_expr_value() {
    struct Case {
        name: &'static str,
        expr: FunctionCallExpr,
        want: Value,
        diag_count: usize,
    }

    // Every upstream case uses the same `&hcl.EvalContext{Functions: funcs}`.
    let mut ctx = EvalContext::new();
    ctx.functions = [
        ("length".to_string(), strlen_func()),
        ("jsondecode".to_string(), json_decode_func()),
    ]
    .into_iter()
    .collect();

    // NOTE(port): upstream keys this table by a Go map; iteration order is
    // arbitrary there, so source order is preserved here.
    let tests = [
        Case {
            name: "valid call with no conversions",
            expr: call_expr("length", vec![lit(Value::string("hello"))]),
            want: Value::number_int(5),
            diag_count: 0,
        },
        Case {
            name: "valid call with arg conversion",
            expr: call_expr("length", vec![lit(Value::bool(true))]),
            want: Value::number_int(4), // length of string "true"
            diag_count: 0,
        },
        Case {
            name: "valid call with unknown arg",
            expr: call_expr("length", vec![lit(Value::unknown(Type::string()))]),
            want: Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(0), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            name: "valid call with unknown arg needing conversion",
            expr: call_expr("length", vec![lit(Value::unknown(Type::bool()))]),
            want: Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(0), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            name: "valid call with dynamic arg",
            expr: call_expr("length", vec![lit(Value::dynamic())]),
            want: Value::unknown(Type::number())
                .refine()
                .not_null()
                .number_range_lower_bound(Value::number_int(0), true)
                .new_value(),
            diag_count: 0,
        },
        Case {
            name: "invalid arg type",
            expr: call_expr("length", vec![lit(Value::list([Value::string("hello")]))]),
            want: Value::dynamic(),
            diag_count: 1,
        },
        Case {
            name: "function with dynamic return type",
            expr: call_expr("jsondecode", vec![lit(Value::string("\"hello\""))]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            name: "function with dynamic return type unknown arg",
            expr: call_expr("jsondecode", vec![lit(Value::unknown(Type::string()))]),
            want: Value::dynamic(), // type depends on arg value
            diag_count: 0,
        },
        Case {
            name: "error in function",
            expr: call_expr("jsondecode", vec![lit(Value::string("invalid-json"))]),
            want: Value::dynamic(),
            diag_count: 1, // JSON parse error
        },
        Case {
            name: "unknown function",
            expr: call_expr("lenth", vec![]),
            want: Value::dynamic(),
            diag_count: 1,
        },
    ];

    for test in &tests {
        let (got, diags) = Expression::from(test.expr.clone()).value(Some(&ctx));

        assert_eq!(
            diags.len(),
            test.diag_count,
            "{}: wrong number of diagnostics {}; want {}\n{diags}",
            test.name,
            diags.len(),
            test.diag_count,
        );

        assert!(
            got.raw_equals(&test.want),
            "{}: wrong result\ngot:  {got:?}\nwant: {:?}",
            test.name,
            test.want,
        );
    }
}

// Ported from TestExpressionAsTraversal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L2885
#[test]
#[ignore = "not yet implemented"]
fn expression_as_traversal() {
    let (expr, _) = hclsyntax::parse_expression(b"a.b[0][\"c\"]", "", Pos::default());
    let (traversal, diags) = hcl::abs_traversal_for_expr(&expr);
    assert!(diags.is_empty(), "unexpected diagnostics:\n{diags}");
    // Upstream message says "want length 3" while asserting length 4;
    // copied literally.
    assert_eq!(
        traversal.0.len(),
        4,
        "wrong traversal {traversal:?}; want length 3"
    );
    assert_eq!(
        traversal.root_name(),
        "a",
        "wrong root name {:?}; want \"a\"",
        traversal.root_name(),
    );
    match &traversal.0[1] {
        Traverser::Attr { name, .. } => {
            assert_eq!(name, "b", "wrong name {name:?} for step 1; want \"b\"");
        }
        other => panic!("wrong type {other:?} for step 1; want Traverser::Attr"),
    }
    match &traversal.0[2] {
        Traverser::Index { key, .. } => {
            assert!(
                Value::zero().raw_equals(key),
                "wrong name {key:?} for step 2; want cty.Zero"
            );
        }
        other => panic!("wrong type {other:?} for step 2; want Traverser::Index"),
    }
    match &traversal.0[3] {
        Traverser::Index { key, .. } => {
            assert!(
                Value::string("c").raw_equals(key),
                "wrong name {key:?} for step 3; want cty.StringVal(\"c\")"
            );
        }
        other => panic!("wrong type {other:?} for step 3; want Traverser::Index"),
    }
}

// Ported from TestStaticExpressionList:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L2920
#[test]
#[ignore = "not yet implemented"]
fn static_expression_list() {
    let (expr, _) = hclsyntax::parse_expression(b"[0, a, true]", "", Pos::default());
    let (exprs, diags) = hcl::expr_list(&expr);
    assert!(diags.is_empty(), "unexpected diagnostics:\n{diags}");
    assert_eq!(exprs.len(), 3, "wrong result {exprs:?}; want length 3");
    // Go asserts `exprs[0].(*hclsyntax.LiteralValueExpr)`; here the node is
    // the LiteralValue variant of the hclsyntax::Expression enum.
    let first = exprs[0]
        .as_any()
        .downcast_ref::<Expression>()
        .unwrap_or_else(|| {
            panic!(
                "first expr has wrong type {:?}; want hclsyntax::Expression",
                exprs[0]
            )
        });
    let Expression::LiteralValue(first) = first else {
        panic!("first expr has wrong type {first:?}; want hclsyntax::LiteralValueExpr")
    };
    assert!(
        first.val.raw_equals(&Value::zero()),
        "wrong first value {:?}; want cty.Zero",
        first.val,
    );
}

// Check that function call w/ incomplete argument still reports correct range
//
// Ported from TestParseExpression_incompleteFunctionCall:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L2939
#[test]
#[ignore = "not yet implemented"]
fn parse_expression_incomplete_function_call() {
    struct Case {
        cfg: &'static str,
        expected_range: Range,
    }

    let tests = [
        Case {
            cfg: "object({ foo = })",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 1,
                    column: 18,
                    byte: 17,
                },
            },
        },
        Case {
            cfg: "object({\n  foo =\n})",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 3,
                    column: 3,
                    byte: 19,
                },
            },
        },
        Case {
            cfg: "object({ foo = }",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 0,
                    column: 0,
                    byte: 0,
                },
            },
        },
        Case {
            cfg: "object({\n  foo =\n}",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 0,
                    column: 0,
                    byte: 0,
                },
            },
        },
        Case {
            cfg: "object({\n  foo =\n",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 0,
                    column: 0,
                    byte: 0,
                },
            },
        },
        Case {
            cfg: "object({\n  foo =\n)",
            expected_range: Range {
                filename: "test.hcl".to_string(),
                start: Pos::initial(),
                end: Pos {
                    line: 0,
                    column: 0,
                    byte: 0,
                },
            },
        },
    ];

    for (i, tc) in tests.iter().enumerate() {
        let (expr, _) = hclsyntax::parse_expression(tc.cfg.as_bytes(), "test.hcl", Pos::initial());
        assert_eq!(
            expr.range(),
            tc.expected_range,
            "case {i} ({:?}): range mismatch",
            tc.cfg,
        );
    }
}

// Ported from TestAllBoolExpressions:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L3012
#[test]
#[ignore = "not yet implemented"]
fn all_bool_expressions() {
    // NOTE(port): upstream keys this table by a Go map; iteration order is
    // arbitrary there, so source order is preserved here.
    let inputs = [
        // truth table for all boolean expressions
        ("true && true", Value::bool(true)),
        ("true || true", Value::bool(true)),
        ("true && false", Value::bool(false)),
        ("true || false", Value::bool(true)),
        ("true && unknown", Value::dynamic()),
        ("true || unknown", Value::bool(true)),
        ("false && true", Value::bool(false)),
        ("false || true", Value::bool(true)),
        ("false && false", Value::bool(false)),
        ("false || false", Value::bool(false)),
        ("false && unknown", Value::bool(false)),
        ("false || unknown", Value::dynamic()),
        ("unknown && true", Value::dynamic()),
        ("unknown || true", Value::bool(true)),
        ("unknown && false", Value::bool(false)),
        ("unknown || false", Value::dynamic()),
        ("unknown && unknown", Value::dynamic()),
        ("unknown || unknown", Value::dynamic()),
        // Truth table for all possible combinations of 3 part boolean
        // expressions. Also added equivalent parenthesized versions for when
        // the operator precedense affects the result.
        ("true && true && true", Value::bool(true)),
        ("true || true && true", Value::bool(true)),
        ("true || true || true", Value::bool(true)),
        ("true && true || true", Value::bool(true)),
        ("true && true && false", Value::bool(false)),
        ("true || true && false", Value::bool(true)),
        ("true || true || false", Value::bool(true)),
        ("true && true || false", Value::bool(true)),
        ("true && true && unknown", Value::dynamic()),
        ("true || true && unknown", Value::bool(true)),
        ("true || true || unknown", Value::bool(true)),
        ("true && true || unknown", Value::bool(true)),
        ("true && false && true", Value::bool(false)),
        ("true || false && true", Value::bool(true)),
        ("true || false || true", Value::bool(true)),
        ("true && false || true", Value::bool(true)),
        ("true && false && false", Value::bool(false)),
        ("true || false && false", Value::bool(true)),
        ("true || false || false", Value::bool(true)),
        ("true && false || false", Value::bool(false)),
        ("true && false && unknown", Value::bool(false)),
        ("true || false && unknown", Value::bool(true)),
        ("true || false || unknown", Value::bool(true)),
        ("true && false || unknown", Value::dynamic()),
        ("true && unknown && true", Value::dynamic()),
        ("true || unknown && true", Value::bool(true)),
        ("true || unknown || true", Value::bool(true)),
        ("true && unknown || true", Value::bool(true)),
        ("true && unknown && false", Value::bool(false)),
        ("true || unknown && false", Value::bool(true)),
        ("true || unknown || false", Value::bool(true)),
        ("true && unknown || false", Value::dynamic()),
        ("true && unknown && unknown", Value::dynamic()),
        ("true || unknown && unknown", Value::bool(true)),
        ("true || unknown || unknown", Value::bool(true)),
        ("true && unknown || unknown", Value::dynamic()),
        ("false && true && true", Value::bool(false)),
        ("false || true && true", Value::bool(true)),
        ("false || true || true", Value::bool(true)),
        ("false && true || true", Value::bool(true)),
        ("(false && true) || true", Value::bool(true)),
        ("false && true && false", Value::bool(false)),
        ("false || true && false", Value::bool(false)),
        ("false || true || false", Value::bool(true)),
        ("false && true || false", Value::bool(false)),
        ("false && true && unknown", Value::bool(false)),
        ("false || true && unknown", Value::dynamic()),
        ("false || true || unknown", Value::bool(true)),
        ("false && true || unknown", Value::dynamic()),
        ("(false && true) || unknown", Value::dynamic()),
        ("false && false && true", Value::bool(false)),
        ("false || false && true", Value::bool(false)),
        ("false || false || true", Value::bool(true)),
        ("false && false || true", Value::bool(true)),
        ("false && false && false", Value::bool(false)),
        ("false || false && false", Value::bool(false)),
        ("false || false || false", Value::bool(false)),
        ("false && false || false", Value::bool(false)),
        ("false && false && unknown", Value::bool(false)),
        ("false || false && unknown", Value::bool(false)),
        ("false || false || unknown", Value::dynamic()),
        ("false && false || unknown", Value::dynamic()),
        ("(false && false) || unknown", Value::dynamic()),
        ("false && unknown && true", Value::bool(false)),
        ("false || unknown && true", Value::dynamic()),
        ("false || unknown || true", Value::bool(true)),
        ("false && unknown || true", Value::bool(true)),
        ("(false && unknown) || true", Value::bool(true)),
        ("false && unknown && false", Value::bool(false)),
        ("false || unknown && false", Value::bool(false)),
        ("false || unknown || false", Value::dynamic()),
        ("false && unknown || false", Value::bool(false)),
        ("false && unknown && unknown", Value::bool(false)),
        ("false || unknown && unknown", Value::dynamic()),
        ("false || unknown || unknown", Value::dynamic()),
        ("false && unknown || unknown", Value::dynamic()),
        ("(false && unknown) || unknown", Value::dynamic()),
        ("unknown && true && true", Value::dynamic()),
        ("unknown || true && true", Value::bool(true)),
        ("unknown || true || true", Value::bool(true)),
        ("unknown && true || true", Value::bool(true)),
        ("unknown && true && false", Value::bool(false)),
        ("unknown || true && false", Value::dynamic()),
        ("unknown || (true && false)", Value::dynamic()),
        ("unknown || true || false", Value::bool(true)),
        ("unknown && true || false", Value::dynamic()),
        ("unknown && true && unknown", Value::dynamic()),
        ("unknown || true && unknown", Value::dynamic()),
        ("unknown || true || unknown", Value::bool(true)),
        ("unknown && true || unknown", Value::dynamic()),
        ("unknown && false && true", Value::bool(false)),
        ("unknown || false && true", Value::dynamic()),
        ("unknown || false || true", Value::bool(true)),
        ("unknown && false || true", Value::bool(true)),
        ("(unknown && false) || true", Value::bool(true)),
        ("unknown && false && false", Value::bool(false)),
        ("unknown || false && false", Value::dynamic()),
        ("unknown || false || false", Value::dynamic()),
        ("unknown && false || false", Value::bool(false)),
        ("unknown && false && unknown", Value::bool(false)),
        ("unknown || false && unknown", Value::dynamic()),
        ("unknown || false || unknown", Value::dynamic()),
        ("unknown && false || unknown", Value::dynamic()),
        ("unknown && unknown && true", Value::dynamic()),
        ("unknown || unknown && true", Value::dynamic()),
        ("unknown || unknown || true", Value::bool(true)),
        ("unknown && unknown || true", Value::bool(true)),
        ("unknown && unknown && false", Value::bool(false)),
        ("unknown || unknown && false", Value::dynamic()),
        ("unknown || unknown || false", Value::dynamic()),
        ("unknown && unknown || false", Value::dynamic()),
        ("unknown && unknown && unknown", Value::dynamic()),
        ("unknown || unknown && unknown", Value::dynamic()),
        ("unknown || unknown || unknown", Value::dynamic()),
        ("unknown && unknown || unknown", Value::dynamic()),
    ];

    for (i, (input, want)) in inputs.iter().enumerate() {
        let mut want = want.clone();
        if !want.is_known() {
            want = Value::unknown(Type::bool()).refine_not_null();
        }
        let mut ctx = EvalContext::new();
        ctx.variables
            .insert("unknown".to_string(), Value::unknown(Type::dynamic()));

        let (expr, diags) = hclsyntax::parse_expression(
            input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(!diags.has_errors(), "case {i} ({input}): {diags}");
        let (got, diags) = expr.value(Some(&ctx));
        assert!(!diags.has_errors(), "case {i} ({input}): {diags}");

        assert_eq!(
            got.is_known(),
            want.is_known(),
            "case {i}: {input:?} resulted in {got:?}, wanted {want:?}",
        );
        if !got.is_known() {
            // this validates that the uknown refinements are correct too
            assert!(
                got.raw_equals(&want),
                "case {i} ({input}): wrong unknown, got:{got:?}, want:{want:?}",
            );
            // covered in known comparison
            continue;
        }

        assert!(
            !got.equals(&want).is_false(),
            "case {i}: {input:?} resulted in {got:?}, wanted {want:?}",
        );
    }
}

/// A function whose implementation always fails with "the expected error"
/// (Go: `function.New(&function.Spec{Type: function.StaticReturnType(
/// cty.String), Impl: ...})`).
///
/// NOTE(port): the Go `Impl` returns `(cty.DynamicVal, error)`; the Rust
/// `ImplFunc` returns `Result`, so only the error is carried.
fn erroring_fn() -> Function {
    Function::new(Spec {
        description: String::new(),
        params: vec![],
        var_param: None,
        type_fn: static_return_type(Type::string()),
        refine_result: None,
        impl_fn: Box::new(|_args, _ret_type| Err(cty::Error::new("the expected error"))),
    })
}

/// An evaluation context whose functions map holds only `name`
/// (Go: `&hcl.EvalContext{Functions: map[string]function.Function{...}}`).
fn ctx_with_fn(name: &str) -> EvalContext {
    let mut ctx = EvalContext::new();
    ctx.functions.insert(name.to_string(), erroring_fn());
    ctx
}

/// Wraps another extra value, exposing it through the unwrapping hook
/// (Go: the test-local `diagnosticExtraWrapper` implementing
/// `hcl.DiagnosticExtraUnwrapper`).
#[derive(Debug)]
struct DiagnosticExtraWrapper {
    wrapped: Option<Arc<dyn DiagnosticExtra>>,
}

impl DiagnosticExtra for DiagnosticExtraWrapper {
    fn unwrap_diagnostic_extra(&self) -> Option<&dyn DiagnosticExtra> {
        self.wrapped.as_deref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Ported from TestExpressionDiagnosticExtra:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_typeparams_test.go#L20
#[test]
#[ignore = "not yet implemented"]
fn expression_diagnostic_extra() {
    struct Case {
        input: &'static str,
        ctx: EvalContext,
        assert: fn(Diagnostics),
    }

    // Errors for unknown function calls
    fn assert_unknown_global(diags: Diagnostics) {
        for diag in &diags {
            let Some(extra) = diagnostic_extra::<dyn FunctionCallUnknownDiagExtra>(diag) else {
                continue;
            };

            assert_eq!(
                extra.called_function_name(),
                "boop",
                "wrong called function name {:?}; want \"boop\"",
                extra.called_function_name(),
            );
            let ns = extra.called_function_namespace();
            assert_eq!(ns, "", "expected no namespace, got {ns}");
            return;
        }
        panic!("None of the returned diagnostics implement FunctionCallUnknownDiagExtra\n{diags}");
    }

    fn assert_unknown_namespaced(diags: Diagnostics) {
        for diag in &diags {
            let Some(extra) = diagnostic_extra::<dyn FunctionCallUnknownDiagExtra>(diag) else {
                continue;
            };

            assert_eq!(
                extra.called_function_name(),
                "boop",
                "wrong called function name {:?}; want \"boop\"",
                extra.called_function_name(),
            );
            let ns = extra.called_function_namespace();
            assert_eq!(
                ns, "ns::source::",
                "expected namespace ns::source::, got {ns}"
            );
            return;
        }
        panic!("None of the returned diagnostics implement FunctionCallUnknownDiagExtra\n{diags}");
    }

    // Error from within the function itself, exposed both directly and
    // through a wrapper extra.
    fn assert_call_error(mut diags: Diagnostics) {
        fn try_diags(diags: &Diagnostics) {
            for diag in diags {
                let Some(extra) = diagnostic_extra::<dyn FunctionCallDiagExtra>(diag) else {
                    continue;
                };

                assert_eq!(
                    extra.called_function_name(),
                    "boop",
                    "wrong called function name {:?}; want \"boop\"",
                    extra.called_function_name(),
                );
                let err = extra
                    .function_call_error()
                    .expect("FunctionCallError returned None");
                assert_eq!(
                    err.to_string(),
                    "the expected error",
                    "wrong error message\ngot:  {:?}\nwant: \"the expected error\"",
                    err.to_string(),
                );
                return;
            }
            // Upstream message names "FunctionCallDiagError"; copied literally.
            panic!("None of the returned diagnostics implement FunctionCallDiagError\n{diags}");
        }

        // unwrapped
        try_diags(&diags);

        // It should also work if we wrap up the "extras" in wrapper types.
        // `Diagnostic.extra` is `Option<Arc<dyn DiagnosticExtra>>` and the
        // trait is not Send + Sync, so the lint is inherent to the API.
        #[allow(clippy::arc_with_non_send_sync)]
        for diag in diags.iter_mut() {
            diag.extra = Some(Arc::new(DiagnosticExtraWrapper {
                wrapped: diag.extra.clone(),
            }));
        }
        // wrapped
        try_diags(&diags);
    }

    let tests = [
        Case {
            input: "boop()",
            ctx: ctx_with_fn("zap"),
            assert: assert_unknown_global,
        },
        Case {
            input: "ns::source::boop()",
            ctx: ctx_with_fn("zap"),
            assert: assert_unknown_namespaced,
        },
        // Error messages describing inconsistent result types for conditional expressions.
        Case {
            input: "boop()",
            ctx: ctx_with_fn("boop"),
            assert: assert_call_error,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let mut diags = Diagnostics::new();
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        diags.extend(parse_diags);
        let (_, val_diags) = expr.value(Some(&test.ctx));
        diags.extend(val_diags);

        assert!(
            diags.has_errors(),
            "case {i} ({}): unexpected success",
            test.input,
        );

        (test.assert)(diags);
    }
}
