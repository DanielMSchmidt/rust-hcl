//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_test.go (TestExpressionParseAndValue, part 1)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;

use cty::function::{self, Function, Parameter, Spec, stdlib};
use cty::{Type, Value};
use hcl::hclsyntax;
use hcl::{EvalContext, Pos};

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

/// An `impl_fn` for function specs whose upstream Go `Spec.Impl` is left as
/// its nil zero value: the type-check function always errors first, so the
/// implementation is never reached (calling a nil Impl would panic in Go
/// too).
fn nil_impl() -> function::ImplFunc {
    Box::new(|_args, _retty| unreachable!("Impl is nil upstream and never called"))
}

// Ported from TestExpressionParseAndValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L17
// (part 1: cases with opening brace before upstream line 848)
#[test]
#[ignore = "not yet implemented"]
fn expression_parse_and_value_part1() {
    struct Case {
        input: &'static str,
        ctx: Option<EvalContext>,
        want: Value,
        diag_count: usize,
    }

    let tests = vec![
        Case {
            input: r#"1"#,
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            input: r#"(1)"#,
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            input: r#"(2+3)"#,
            ctx: None,
            want: Value::number_int(5),
            diag_count: 0,
        },
        Case {
            input: r#"2*5+1"#,
            ctx: None,
            want: Value::number_int(11),
            diag_count: 0,
        },
        Case {
            input: r#"9%8"#,
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            input: r#"(2+unk)"#,
            ctx: Some(ectx(vec![("unk", Value::unknown(Type::number()))], vec![])),
            want: Value::unknown(Type::number()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"(2+unk)"#,
            ctx: Some(ectx(vec![("unk", Value::dynamic())], vec![])),
            want: Value::unknown(Type::number()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"(unk+unk)"#,
            ctx: Some(ectx(vec![("unk", Value::dynamic())], vec![])),
            want: Value::unknown(Type::number()).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"(2+true)"#,
            ctx: None,
            want: Value::unknown(Type::number()),
            diag_count: 1, // unsuitable type for right operand
        },
        Case {
            input: r#"(false+true)"#,
            ctx: None,
            want: Value::unknown(Type::number()),
            diag_count: 2, // unsuitable type for each operand
        },
        Case {
            input: r#"(5 == 5)"#,
            ctx: None,
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#"(5 == 4)"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"(1 == true)"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"("true" == true)"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"(true == "true")"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"(true != "true")"#,
            ctx: None,
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#"(- 2)"#,
            ctx: None,
            want: Value::number_int(-2),
            diag_count: 0,
        },
        Case {
            input: r#"(! true)"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: "(\n    1\n)",
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            input: r#"(1"#,
            ctx: None,
            want: Value::number_int(1),
            diag_count: 1, // Unbalanced parentheses
        },
        Case {
            input: r#"true"#,
            ctx: None,
            want: Value::bool(true),
            diag_count: 0,
        },
        Case {
            input: r#"false"#,
            ctx: None,
            want: Value::bool(false),
            diag_count: 0,
        },
        Case {
            input: r#"null"#,
            ctx: None,
            want: Value::null(Type::dynamic()),
            diag_count: 0,
        },
        Case {
            input: r#"true true"#,
            ctx: None,
            want: Value::bool(true),
            diag_count: 1, // extra characters after expression
        },
        Case {
            input: r#""hello""#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: "\"hello `backtick` world\"",
            ctx: None,
            want: Value::string("hello `backtick` world"),
            diag_count: 0,
        },
        Case {
            input: r#""hello\nworld""#,
            ctx: None,
            want: Value::string("hello\nworld"),
            diag_count: 0,
        },
        Case {
            input: r#""unclosed"#,
            ctx: None,
            want: Value::string("unclosed"),
            diag_count: 1, // Unterminated template string
        },
        Case {
            input: r#""hello ${"world"}""#,
            ctx: None,
            want: Value::string("hello world"),
            diag_count: 0,
        },
        Case {
            input: r#""hello ${12.5}""#,
            ctx: None,
            want: Value::string("hello 12.5"),
            diag_count: 0,
        },
        Case {
            input: r#""silly ${"${"nesting"}"}""#,
            ctx: None,
            want: Value::string("silly nesting"),
            diag_count: 0,
        },
        Case {
            input: r#""silly ${"${true}"}""#,
            ctx: None,
            want: Value::string("silly true"),
            diag_count: 0,
        },
        Case {
            input: r#""hello $${escaped}""#,
            ctx: None,
            want: Value::string("hello ${escaped}"),
            diag_count: 0,
        },
        Case {
            input: r#""hello $$nonescape""#,
            ctx: None,
            want: Value::string("hello $$nonescape"),
            diag_count: 0,
        },
        Case {
            input: r#""$""#,
            ctx: None,
            want: Value::string("$"),
            diag_count: 0,
        },
        Case {
            input: r#""%""#,
            ctx: None,
            want: Value::string("%"),
            diag_count: 0,
        },
        Case {
            input: r#"upper("foo")"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            input: "\nupper(\n    \"foo\"\n)\n",
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            input: r#"upper(["foo"]...)"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            input: r#"upper("foo", []...)"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            input: r#"upper("foo", "bar")"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::dynamic(),
            diag_count: 1, // too many function arguments
        },
        Case {
            input: r#"upper(["foo", "bar"]...)"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::dynamic(),
            diag_count: 1, // too many function arguments
        },
        Case {
            input: r#"concat([1, null]...)"#,
            ctx: Some(ectx(vec![], vec![("concat", stdlib::concat_func())])),
            want: Value::dynamic(),
            diag_count: 1, // argument cannot be null
        },
        Case {
            input: r#"concat(var.unknownlist...)"#,
            ctx: Some(ectx(
                vec![(
                    "var",
                    Value::object([("unknownlist", Value::unknown(Type::dynamic()))]),
                )],
                vec![("concat", stdlib::concat_func())],
            )),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"foo::upper("foo")"#,
            ctx: Some(ectx(vec![], vec![("foo::upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            // spaces are non-idomatic, but valid
            input: r#"foo :: upper("foo")"#,
            ctx: Some(ectx(vec![], vec![("foo::upper", stdlib::upper_func())])),
            want: Value::string("FOO"),
            diag_count: 0,
        },
        Case {
            // :: is still not a valid identifier
            input: r#"::upper("foo")"#,
            ctx: Some(ectx(vec![], vec![("::upper", stdlib::upper_func())])),
            want: Value::dynamic(),
            diag_count: 1,
        },
        Case {
            // missing name after ::
            input: r#"double::::upper("foo")"#,
            ctx: Some(ectx(
                vec![],
                vec![("double::::upper", stdlib::upper_func())],
            )),
            want: Value::dynamic(),
            diag_count: 2,
        },
        Case {
            // missing name after ::
            input: r#"missing::("foo")"#,
            ctx: Some(ectx(vec![], vec![("missing::", stdlib::upper_func())])),
            want: Value::dynamic(),
            diag_count: 2,
        },
        Case {
            input: r#"misbehave()"#,
            ctx: Some(ectx(
                vec![],
                vec![(
                    "misbehave",
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![],
                        var_param: None,
                        type_fn: Box::new(|_args| {
                            // This function misbehaves by indicating an error
                            // on an argument index that is out of range for
                            // its declared parameters. That would always be
                            // a bug in the function, but we want to avoid
                            // panicking in this case and just behave like it
                            // was a normal (non-arg) error.
                            Err(function::new_arg_error(1, "out of range"))
                        }),
                        refine_result: None,
                        impl_fn: nil_impl(),
                    }),
                )],
            )),
            want: Value::dynamic(),
            diag_count: 1, // Call to function "misbehave" failed: out of range
        },
        Case {
            input: r#"misbehave() /* variadic */"#,
            ctx: Some(ectx(
                vec![],
                vec![(
                    "misbehave",
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![],
                        var_param: Some(Parameter {
                            name: "foo".to_string(),
                            ty: Some(Type::string()),
                            ..Default::default()
                        }),
                        type_fn: Box::new(|_args| {
                            // This function misbehaves by indicating an error
                            // on an argument index that is out of range for
                            // the given arguments. That would always be a
                            // bug in the function, but to avoid panicking we
                            // just treat it like a problem related to the
                            // declared variadic argument.
                            Err(function::new_arg_error(1, "out of range"))
                        }),
                        refine_result: None,
                        impl_fn: nil_impl(),
                    }),
                )],
            )),
            want: Value::dynamic(),
            diag_count: 1, // Invalid value for "foo" parameter: out of range
        },
        Case {
            input: r#"misbehave([]...)"#,
            ctx: Some(ectx(
                vec![],
                vec![(
                    "misbehave",
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![],
                        var_param: Some(Parameter {
                            name: "foo".to_string(),
                            ty: Some(Type::string()),
                            ..Default::default()
                        }),
                        type_fn: Box::new(|_args| {
                            // This function misbehaves by indicating an error
                            // on an argument index that is out of range for
                            // the given arguments. That would always be a
                            // bug in the function, but to avoid panicking we
                            // just treat it like a problem related to the
                            // declared variadic argument.
                            Err(function::new_arg_error(1, "out of range"))
                        }),
                        refine_result: None,
                        impl_fn: nil_impl(),
                    }),
                )],
            )),
            want: Value::dynamic(),
            diag_count: 1, // Invalid value for "foo" parameter: out of range
        },
        Case {
            input: r#"argerrorexpand(["a", "b"]...)"#,
            ctx: Some(ectx(
                vec![],
                vec![(
                    "argerrorexpand",
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![],
                        var_param: Some(Parameter {
                            name: "foo".to_string(),
                            ty: Some(Type::string()),
                            ..Default::default()
                        }),
                        type_fn: Box::new(|_args| {
                            // We should be able to indicate an error in
                            // argument 1 because the indices are into the
                            // arguments _after_ "..." expansion. An earlier
                            // HCL version had a bug where it used the
                            // pre-expansion arguments and would thus panic
                            // in this case.
                            Err(function::new_arg_error(1, "blah blah"))
                        }),
                        refine_result: None,
                        impl_fn: nil_impl(),
                    }),
                )],
            )),
            want: Value::dynamic(),
            diag_count: 1, // Invalid value for "foo" parameter: blah blah
        },
        Case {
            input: r#"[]"#,
            ctx: None,
            want: Value::empty_tuple(),
            diag_count: 0,
        },
        Case {
            input: r#"[1]"#,
            ctx: None,
            want: Value::tuple([Value::number_int(1)]),
            diag_count: 0,
        },
        Case {
            input: r#"[1,]"#,
            ctx: None,
            want: Value::tuple([Value::number_int(1)]),
            diag_count: 0,
        },
        Case {
            input: r#"[1,true]"#,
            ctx: None,
            want: Value::tuple([Value::number_int(1), Value::bool(true)]),
            diag_count: 0,
        },
        Case {
            input: "[\n  1,\n  true\n]",
            ctx: None,
            want: Value::tuple([Value::number_int(1), Value::bool(true)]),
            diag_count: 0,
        },
        Case {
            input: r#"{}"#,
            ctx: None,
            want: Value::empty_object(),
            diag_count: 0,
        },
        Case {
            input: r#"{"hello": "world"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{"hello" = "world"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{hello = "world"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{hello: "world"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{true: "yes"}"#,
            ctx: None,
            want: Value::object([("true", Value::string("yes"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{false: "yes"}"#,
            ctx: None,
            want: Value::object([("false", Value::string("yes"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{null: "yes"}"#,
            ctx: None,
            want: Value::object([("null", Value::string("yes"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{15: "yes"}"#,
            ctx: None,
            want: Value::object([("15", Value::string("yes"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{[]: "yes"}"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // Incorrect key type; Can't use this value as a key: string required
        },
        Case {
            input: r#"{"centos_7.2_ap-south-1" = "ami-abc123"}"#,
            ctx: None,
            want: Value::object([("centos_7.2_ap-south-1", Value::string("ami-abc123"))]),
            diag_count: 0,
        },
        Case {
            // This is syntactically valid (it's similar to foo["bar"])
            // but is rejected during evaluation to force the user to be explicit
            // about which of the following interpretations they mean:
            // -{(foo.bar) = "baz"}
            // -{"foo.bar" = "baz"}
            // naked traversals as keys are allowed when analyzing an expression
            // statically so an application can define object-syntax-based
            // language constructs with looser requirements, but we reject
            // this during normal expression evaluation.
            input: r#"{foo.bar = "ami-abc123"}"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // Ambiguous attribute key; If this expression is intended to be a reference, wrap it in parentheses. If it's instead intended as a literal name containing periods, wrap it in quotes to create a string literal.
        },
        Case {
            // This is a weird variant of the above where a period is followed
            // by a digit, causing the parser to interpret it as an index
            // operator using the legacy HIL/Terraform index syntax.
            // This one _does_ fail parsing, causing it to be subject to
            // parser recovery behavior.
            input: r#"{centos_7.2_ap-south-1 = "ami-abc123"}"#,
            ctx: None,
            want: Value::empty_object(), // (due to parser recovery behavior)
            diag_count: 1, // Missing key/value separator; Expected an equals sign ("=") to mark the beginning of the attribute value. If you intended to given an attribute name containing periods or spaces, write the name in quotes to create a string literal.
        },
        Case {
            input: r#"{var.greeting = "world"}"#,
            ctx: Some(ectx(
                vec![("var", Value::object([("greeting", Value::string("hello"))]))],
                vec![],
            )),
            want: Value::dynamic(),
            diag_count: 1, // Ambiguous attribute key
        },
        Case {
            input: r#"{(var.greeting) = "world"}"#,
            ctx: Some(ectx(
                vec![("var", Value::object([("greeting", Value::string("hello"))]))],
                vec![],
            )),
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            // Marked values as object keys
            input: r#"{(var.greeting) = "world", "goodbye" = "earth"}"#,
            ctx: Some(ectx(
                vec![(
                    "var",
                    Value::object([("greeting", Value::string("hello").mark("marked"))]),
                )],
                vec![],
            )),
            want: Value::object([
                ("hello", Value::string("world")),
                ("goodbye", Value::string("earth")),
            ])
            .mark("marked"),
            diag_count: 0,
        },
        Case {
            input: r#"{"${var.greeting}" = "world"}"#,
            ctx: Some(ectx(
                vec![("var", Value::object([("greeting", Value::string("hello"))]))],
                vec![],
            )),
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{"hello" = "world", "goodbye" = "cruel world"}"#,
            ctx: None,
            want: Value::object([
                ("hello", Value::string("world")),
                ("goodbye", Value::string("cruel world")),
            ]),
            diag_count: 0,
        },
        Case {
            input: "{\n  \"hello\" = \"world\"\n}",
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: "{\n  \"hello\" = \"world\"\n  \"goodbye\" = \"cruel world\"\n}",
            ctx: None,
            want: Value::object([
                ("hello", Value::string("world")),
                ("goodbye", Value::string("cruel world")),
            ]),
            diag_count: 0,
        },
        Case {
            input: "{\n  \"hello\" = \"world\",\n  \"goodbye\" = \"cruel world\"\n}",
            ctx: None,
            want: Value::object([
                ("hello", Value::string("world")),
                ("goodbye", Value::string("cruel world")),
            ]),
            diag_count: 0,
        },
        Case {
            input: "{\n  \"hello\" = \"world\",\n  \"goodbye\" = \"cruel world\",\n}",
            ctx: None,
            want: Value::object([
                ("hello", Value::string("world")),
                ("goodbye", Value::string("cruel world")),
            ]),
            diag_count: 0,
        },
        Case {
            input: "{\n  for k, v in {hello: \"world\"}:\nk => v\n}",
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            // This one is different than the previous because the extra level of
            // object constructor causes the inner for expression to begin parsing
            // in newline-sensitive mode, which it must then properly disable in
            // order to peek the "for" keyword.
            input: "{\n  a = {\n  for k, v in {hello: \"world\"}:\nk => v\n  }\n}",
            ctx: None,
            want: Value::object([("a", Value::object([("hello", Value::string("world"))]))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: k => v if k == "hello"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: upper(k) => upper(v) if k == "hello"}"#,
            ctx: Some(ectx(vec![], vec![("upper", stdlib::upper_func())])),
            want: Value::object([("HELLO", Value::string("WORLD"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in ["world"]: k => v if k == 0}"#,
            ctx: None,
            want: Value::object([("0", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for v in ["world"]: v => v}"#,
            ctx: None,
            want: Value::object([("world", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: k => v if k == "foo"}"#,
            ctx: None,
            want: Value::empty_object(),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: 5 => v}"#,
            ctx: None,
            want: Value::object([("5", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: [] => v}"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // key expression has the wrong type
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: k => k if k == "hello"}"#,
            ctx: None,
            want: Value::object([("hello", Value::string("hello"))]),
            diag_count: 0,
        },
        Case {
            input: r#"{for k, v in {hello: "world"}: k => foo}"#,
            ctx: Some(ectx(vec![("foo", Value::string("foo"))], vec![])),
            want: Value::object([("hello", Value::string("foo"))]),
            diag_count: 0,
        },
        Case {
            input: r#"[for k, v in {hello: "world"}: "${k}=${v}"]"#,
            ctx: None,
            want: Value::tuple([Value::string("hello=world")]),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        // NOTE(port): upstream guards `if expr != nil` before calling Value;
        // parse_expression always returns an Expression here, so the guard is
        // unrepresentable.
        let (got, val_diags) = expr.value(test.ctx.as_ref());

        let diag_count = parse_diags.len() + val_diags.len();
        assert_eq!(
            diag_count, test.diag_count,
            "case {i} ({:?}): wrong number of diagnostics\nparse diags: {:?}\nvalue diags: {:?}",
            test.input, *parse_diags, *val_diags,
        );

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.input,);
    }
}
