//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_template_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value, ValueMarks};
use hcl::hclsyntax;
use hcl::{Diagnostics, EvalContext, Pos};

/// An eval context with the given variables (Go: `&hcl.EvalContext{
/// Variables: map[string]cty.Value{...}}`), wrapped in `Some` for passing
/// where Go passes a non-nil `*hcl.EvalContext`.
fn vars_ctx<const N: usize>(vars: [(&'static str, Value); N]) -> Option<EvalContext> {
    let mut ctx = EvalContext::new();
    ctx.variables = vars.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Some(ctx)
}

// Ported from TestTemplateExprParseAndValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_template_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn template_expr_parse_and_value() {
    // This is a combo test that exercises both the parser and the Value
    // method, with the focus on the latter but indirectly testing the former.
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
            want: Value::string("1"),
            diag_count: 0,
        },
        Case {
            input: r#"(1)"#,
            ctx: None,
            want: Value::string("(1)"),
            diag_count: 0,
        },
        Case {
            input: r#"true"#,
            ctx: None,
            want: Value::string("true"),
            diag_count: 0,
        },
        Case {
            input: "\nhello world\n",
            ctx: None,
            want: Value::string("\nhello world\n"),
            diag_count: 0,
        },
        Case {
            input: r#"hello ${"world"}"#,
            ctx: None,
            want: Value::string("hello world"),
            diag_count: 0,
        },
        Case {
            input: r#"hello\nworld"#, // backslash escapes not supported in bare templates
            ctx: None,
            want: Value::string("hello\\nworld"),
            diag_count: 0,
        },
        Case {
            input: r#"hello ${12.5}"#,
            ctx: None,
            want: Value::string("hello 12.5"),
            diag_count: 0,
        },
        Case {
            input: r#"silly ${"${"nesting"}"}"#,
            ctx: None,
            want: Value::string("silly nesting"),
            diag_count: 0,
        },
        Case {
            input: r#"silly ${"${true}"}"#,
            ctx: None,
            want: Value::string("silly true"),
            diag_count: 0,
        },
        Case {
            input: r#"hello $${escaped}"#,
            ctx: None,
            want: Value::string("hello ${escaped}"),
            diag_count: 0,
        },
        Case {
            input: r#"hello $$nonescape"#,
            ctx: None,
            want: Value::string("hello $$nonescape"),
            diag_count: 0,
        },
        Case {
            input: r#"hello %${"world"}"#,
            ctx: None,
            want: Value::string("hello %world"),
            diag_count: 0,
        },
        Case {
            input: r#"${true}"#,
            ctx: None,
            want: Value::bool(true), // any single expression is unwrapped without stringification
            diag_count: 0,
        },
        Case {
            input: r#"trim ${~ "trim"}"#,
            ctx: None,
            want: Value::string("trimtrim"),
            diag_count: 0,
        },
        Case {
            input: r#"${"trim" ~} trim"#,
            ctx: None,
            want: Value::string("trimtrim"),
            diag_count: 0,
        },
        Case {
            input: "trim\n${~\"trim\"~}\ntrim",
            ctx: None,
            want: Value::string("trimtrimtrim"),
            diag_count: 0,
        },
        Case {
            input: r#" ${~ true ~} "#,
            ctx: None,
            want: Value::string("true"), // can't trim space to reduce to a single expression
            diag_count: 0,
        },
        Case {
            input: r#"${"hello "}${~"trim"~}${" hello"}"#,
            ctx: None,
            want: Value::string("hello trim hello"), // trimming can't reach into a neighboring interpolation
            diag_count: 0,
        },
        Case {
            input: r#"${true}${~"trim"~}${true}"#,
            ctx: None,
            want: Value::string("truetrimtrue"), // trimming is no-op of neighbors aren't literal strings
            diag_count: 0,
        },
        Case {
            input: r#"%{ if true ~} hello %{~ endif }"#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ if false ~} hello %{~ endif}"#,
            ctx: None,
            want: Value::string(""),
            diag_count: 0,
        },
        Case {
            input: r#"%{ if true ~} hello %{~ else ~} goodbye %{~ endif }"#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ if false ~} hello %{~ else ~} goodbye %{~ endif }"#,
            ctx: None,
            want: Value::string("goodbye"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ if true ~} %{~ if false ~} hello %{~ else ~} goodbye %{~ endif ~} %{~ endif }"#,
            ctx: None,
            want: Value::string("goodbye"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ if false ~} %{~ if false ~} hello %{~ else ~} goodbye %{~ endif ~} %{~ endif }"#,
            ctx: None,
            want: Value::string(""),
            diag_count: 0,
        },
        Case {
            input: r#"%{ of true ~} hello %{~ endif}"#,
            ctx: None,
            want: Value::unknown(Type::string()).refine_not_null(),
            diag_count: 2, // "of" is not a valid control keyword, and "endif" is therefore also unexpected
        },
        Case {
            input: r#"%{ for v in ["a", "b", "c"] }${v}%{ endfor }"#,
            ctx: None,
            want: Value::string("abc"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for v in ["a", "b", "c"] } ${v} %{ endfor }"#,
            ctx: None,
            want: Value::string(" a  b  c "),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for v in ["a", "b", "c"] ~} ${v} %{~ endfor }"#,
            ctx: None,
            want: Value::string("abc"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for v in [] }${v}%{ endfor }"#,
            ctx: None,
            want: Value::string(""),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for i, v in ["a", "b", "c"] }${i}${v}%{ endfor }"#,
            ctx: None,
            want: Value::string("0a1b2c"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for k, v in {"A" = "a", "B" = "b", "C" = "c"} }${k}${v}%{ endfor }"#,
            ctx: None,
            want: Value::string("AaBbCc"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ for v in ["a", "b", "c"] }${v}${nl}%{ endfor }"#,
            ctx: vars_ctx([("nl", Value::string("\n"))]),
            want: Value::string("a\nb\nc\n"),
            diag_count: 0,
        },
        Case {
            input: r#"\n"#, // backslash escapes are not interpreted in template literals
            ctx: None,
            want: Value::string("\\n"),
            diag_count: 0,
        },
        Case {
            input: r#"\uu1234"#, // backslash escapes are not interpreted in template literals
            ctx: None, // (this is intentionally an invalid one to ensure we don't produce an error)
            want: Value::string("\\uu1234"),
            diag_count: 0,
        },
        Case {
            input: r#"$"#,
            ctx: None,
            want: Value::string("$"),
            diag_count: 0,
        },
        Case {
            input: r#"$$"#,
            ctx: None,
            want: Value::string("$$"),
            diag_count: 0,
        },
        Case {
            input: r#"%"#,
            ctx: None,
            want: Value::string("%"),
            diag_count: 0,
        },
        Case {
            input: r#"%%"#,
            ctx: None,
            want: Value::string("%%"),
            diag_count: 0,
        },
        Case {
            input: r#"hello %%{ if true }world%%{ endif }"#,
            ctx: None,
            want: Value::string(r#"hello %{ if true }world%{ endif }"#),
            diag_count: 0,
        },
        Case {
            input: r#"hello $%{ if true }world%{ endif }"#,
            ctx: None,
            want: Value::string("hello $world"),
            diag_count: 0,
        },
        Case {
            input: r#"%{ endif }"#,
            ctx: None,
            want: Value::unknown(Type::string()).refine_not_null(),
            diag_count: 1, // Unexpected endif directive
        },
        Case {
            input: r#"%{ endfor }"#,
            ctx: None,
            want: Value::unknown(Type::string()).refine_not_null(),
            diag_count: 1, // Unexpected endfor directive
        },
        Case {
            // can preserve a static prefix as a refinement of an unknown result
            input: r#"test_${unknown}"#,
            ctx: vars_ctx([("unknown", Value::unknown(Type::string()))]),
            want: Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("test_")
                .new_value(),
            diag_count: 0,
        },
        Case {
            // can preserve a dynamic known prefix as a refinement of an unknown result
            input: r#"test_${known}_${unknown}"#,
            ctx: vars_ctx([
                ("known", Value::string("known")),
                ("unknown", Value::unknown(Type::string())),
            ]),
            want: Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("test_known_")
                .new_value(),
            diag_count: 0,
        },
        Case {
            // can preserve a static prefix as a refinement, but the length is limited to 128 B
            // (Go: strings.Repeat("_", 130) + `${unknown}`)
            input: Box::leak(("_".repeat(130) + "${unknown}").into_boxed_str()),
            ctx: vars_ctx([("unknown", Value::unknown(Type::string()))]),
            want: Value::unknown(Type::string())
                .refine()
                .not_null()
                .string_prefix_full("_".repeat(128))
                .new_value(),
            diag_count: 0,
        },
        Case {
            // all marks are passed through to ensure result is always consistent
            input: r#"hello%{ if false } ${target}%{ endif }"#,
            ctx: vars_ctx([("target", Value::string("world").mark("sensitive"))]),
            want: Value::string("hello").mark("sensitive"),
            diag_count: 0,
        },
        Case {
            // marks from interpolated values are passed through
            input: r#"${greeting} ${target}"#,
            ctx: vars_ctx([
                ("greeting", Value::string("hello").mark("english")),
                ("target", Value::string("world").mark("sensitive")),
            ]),
            want: Value::string("hello world")
                .with_marks([ValueMarks::from_marks(["english", "sensitive"])]),
            diag_count: 0,
        },
        Case {
            // can use marks by traversing complex values
            input: r#"Authenticate with "${secrets.passphrase}""#,
            ctx: vars_ctx([(
                "secrets",
                Value::map([(
                    "passphrase",
                    Value::string("my voice is my passport").mark("sensitive"),
                )])
                .mark("sensitive"),
            )]),
            want: Value::string(r#"Authenticate with "my voice is my passport""#)
                .with_marks([ValueMarks::from_marks(["sensitive"])]),
            diag_count: 0,
        },
        Case {
            // can loop over marked collections
            input: r#"%{ for s in secrets }${s}%{ endfor }"#,
            ctx: vars_ctx([(
                "secrets",
                Value::list([
                    Value::string("foo"),
                    Value::string("bar"),
                    Value::string("baz"),
                ])
                .mark("sensitive"),
            )]),
            want: Value::string("foobarbaz").mark("sensitive"),
            diag_count: 0,
        },
        Case {
            // marks on individual elements propagate to the result
            input: r#"%{ for s in secrets }${s}%{ endfor }"#,
            ctx: vars_ctx([(
                "secrets",
                Value::list([
                    Value::string("foo"),
                    Value::string("bar").mark("sensitive"),
                    Value::string("baz"),
                ]),
            )]),
            want: Value::string("foobarbaz").mark("sensitive"),
            diag_count: 0,
        },
        Case {
            // lots of marks!
            input: r#"%{ for s in secrets }${s}%{ endfor }"#,
            ctx: vars_ctx([(
                "secrets",
                Value::list([
                    Value::string("foo").mark("x"),
                    Value::string("bar").mark("y"),
                    Value::string("baz").mark("z"),
                ])
                .mark("x"), // second instance of x
            )]),
            want: Value::string("foobarbaz").with_marks([ValueMarks::from_marks(["x", "y", "z"])]),
            diag_count: 0,
        },
        Case {
            // marks from unknown values are maintained
            input: r#"test_${target}"#,
            ctx: vars_ctx([("target", Value::unknown(Type::string()).mark("sensitive"))]),
            want: Value::unknown(Type::string())
                .mark("sensitive")
                .refine()
                .not_null()
                .string_prefix_full("test_")
                .new_value(),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_template(
            test.input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );

        // We'll skip evaluating if there were parse errors because it
        // isn't reasonable to evaluate a syntactically-invalid template;
        // it'll produce strange results that we don't care about.
        let mut got = test.want.clone();
        let mut val_diags = Diagnostics::default();
        if !parse_diags.has_errors() {
            (got, val_diags) = expr.value(test.ctx.as_ref());
        }

        let diag_count = parse_diags.len() + val_diags.len();
        assert_eq!(
            diag_count, test.diag_count,
            "case {i} ({:?}): wrong number of diagnostics\nparse diags: {:?}\nvalue diags: {:?}",
            test.input, *parse_diags, *val_diags,
        );

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.input);
    }
}

// Ported from TestTemplateExprGracefulValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_template_test.go#L441
#[test]
#[ignore = "not yet implemented"]
fn template_expr_graceful_value() {
    // we don't care about diags since we know it's invalid config
    let (expr, _) = hclsyntax::parse_template(
        b"prefix${provider::}",
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );

    let (got, _) = expr.value(None); // this should not panic

    let want = Value::unknown(Type::string()).refine_not_null();
    assert_eq!(got, want, "wrong result");
}

// Ported from TestTemplateExprWrappedGracefulValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_template_test.go#L452
#[test]
#[ignore = "not yet implemented"]
fn template_expr_wrapped_graceful_value() {
    // we don't care about diags since we know it's invalid config
    let (expr, _) = hclsyntax::parse_template(
        b"${provider::}",
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );

    let (got, _) = expr.value(None); // this should not panic

    assert_eq!(got, Value::dynamic(), "wrong result");
}

// Ported from TestTemplateExprIsStringLiteral:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_template_test.go#L463
#[test]
#[ignore = "not yet implemented"]
fn template_expr_is_string_literal() {
    // NOTE(port): upstream stores the cases in a `map[string]bool` and thus
    // iterates them in nondeterministic order; the cases are independent, so
    // we keep the source-written order here.
    let tests: &[(&str, bool)] = &[
        // A simple string value is a string literal
        ("a", true),
        // Strings containing escape characters or escape sequences are
        // tokenized into multiple string literals, but this should be
        // corrected by the parser
        ("a$b", true),
        ("a%%b", true),
        ("a\nb", true),
        (r#"a$${"b"}"#, true),
        // Wrapped values (HIL-like) are not treated as string literals for
        // legacy reasons
        ("${1}", false),
        (r#"${"b"}"#, false),
        // Even template expressions containing only literal values do not
        // count as string literals
        ("a${1}", false),
        (r#"a${"b"}"#, false),
    ];

    for (i, &(input, want)) in tests.iter().enumerate() {
        let (expr, diags) = hclsyntax::parse_template(input.as_bytes(), "", Pos::initial());
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({input:?}): unexpected diags: {diags}"
        );

        // NOTE(port): upstream only checks IsStringLiteral when the parsed
        // expression type-asserts to *TemplateExpr (`if tmplExpr, ok :=
        // expr.(*TemplateExpr); ok`); other expression types (e.g. a
        // TemplateWrapExpr for the `${...}`-only cases) skip the assertion,
        // and we mirror that with `if let`.
        if let hclsyntax::Expression::Template(tmpl_expr) = &expr {
            let got = tmpl_expr.is_string_literal();
            assert_eq!(got, want, "case {i} ({input:?}): wrong result");
        }
    }
}
