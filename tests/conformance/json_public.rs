//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   json/public_test.go
//!   json/navigation_test.go
//!   json/didyoumean_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::json;
use hcl::{AttributeSchema, BlockHeaderSchema, BodySchema, EvalContext, Pos};

// Ported from TestParse_nonObject:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L15
#[test]
#[ignore = "not yet implemented"]
fn parse_non_object() {
    let src = "true";
    let (file, diags) = json::parse(src.as_bytes(), "");
    assert_eq!(diags.len(), 1, "got {} diagnostics; want 1", diags.len());
    // NOTE(port): upstream checks `file == nil` and `file.Body == nil`; the
    // Rust `parse` returns `File` (with a `BodyRef`) by value, so there are
    // no nil cases.
    // NOTE(port): upstream downcasts to the unexported `*body` type and
    // checks its `val` field is a non-nil placeholder object; the Rust json
    // body type is not exported, so the closest observable assertion is that
    // the returned body really is a JSON body.
    assert!(
        json::is_json_body(&*file.body),
        "got non-JSON Body; want placeholder JSON body object"
    );
}

// Ported from TestParseTemplate:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L32
#[test]
#[ignore = "not yet implemented"]
fn parse_template() {
    let src = r#"{"greeting": "hello ${\"world\"}"}"#;
    let (file, diags) = json::parse(src.as_bytes(), "");
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on parse; want 0\n{diags}",
        diags.len(),
    );
    // NOTE(port): upstream checks `file == nil` and `file.Body == nil`; the
    // Rust `parse` returns `File` by value, so there are no nil cases.
    let (attrs, diags) = file.body.just_attributes();
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );

    let ctx = EvalContext::new();
    let (val, diags) = attrs["greeting"].expr.value(Some(&ctx));
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on eval; want 0\n{diags}",
        diags.len(),
    );

    let want = Value::string("hello world");
    assert!(
        val.raw_equals(&want),
        "wrong result {}; want {}",
        val.go_string(),
        want.go_string(),
    );
}

// Ported from TestParseTemplateUnwrap:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L68
#[test]
#[ignore = "not yet implemented"]
fn parse_template_unwrap() {
    let src = r#"{"greeting": "${true}"}"#;
    let (file, diags) = json::parse(src.as_bytes(), "");
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on parse; want 0\n{diags}",
        diags.len(),
    );
    // NOTE(port): upstream checks `file == nil` and `file.Body == nil`; the
    // Rust `parse` returns `File` by value, so there are no nil cases.
    let (attrs, diags) = file.body.just_attributes();
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );

    let ctx = EvalContext::new();
    let (val, diags) = attrs["greeting"].expr.value(Some(&ctx));
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on eval; want 0\n{diags}",
        diags.len(),
    );

    let want = Value::bool(true);
    assert!(
        val.raw_equals(&want),
        "wrong result {}; want {}",
        val.go_string(),
        want.go_string(),
    );
}

// Ported from TestParse_malformed:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L104
#[test]
#[ignore = "not yet implemented"]
fn parse_malformed() {
    let src = "{\n  \"http_proxy_url: \"http://xxxxxx\",\n}";
    let (_file, diags) = json::parse(src.as_bytes(), "");
    assert_eq!(diags.len(), 2, "got {} diagnostics; want 2", diags.len());
    let err = format!("{diags}");
    let want = "Missing property value colon";
    assert!(
        err.contains(want),
        "diags are {err:?}, but should contain {want:?}",
    );
    // NOTE(port): upstream checks `file == nil`; the Rust `parse` returns
    // `File` by value, so there is no nil case.
}

// Ported from TestParseWithStartPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L120
#[test]
#[ignore = "not yet implemented"]
fn parse_with_start_pos() {
    let src = "{\n  \"foo\": {\n    \"bar\": \"baz\"\n  }\n}";
    let part = "{\n    \"bar\": \"baz\"\n  }";

    let (file, diags) = json::parse(src.as_bytes(), "");
    let (part_file, part_diags) = json::parse_with_start_pos(
        part.as_bytes(),
        "",
        Pos {
            byte: 0,
            line: 2,
            column: 10,
        },
    );
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on parse src; want 0\n{diags}",
        diags.len(),
    );
    assert_eq!(
        part_diags.len(),
        0,
        "got {} diagnostics on parse part src; want 0\n{part_diags}",
        part_diags.len(),
    );

    // NOTE(port): upstream checks `file`/`file.Body`/`partFile`/
    // `partFile.Body` against nil; the Rust `parse` functions return `File`
    // by value, so there are no nil cases.

    let (content, diags) = file.body.content(&BodySchema {
        blocks: vec![BlockHeaderSchema {
            block_type: "foo".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    });
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );
    let (attrs, diags) = content.blocks[0].body.just_attributes();
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );
    let src_range = attrs["bar"].expr.range();

    let (part_attrs, diags) = part_file.body.just_attributes();
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );
    let part_range = part_attrs["bar"].expr.range();

    assert_eq!(
        format!("{src_range}"),
        format!("{part_range}"),
        "The two ranges did not match: src={src_range}, part={part_range}",
    );
}

// Ported from TestParseExpression:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L190
#[test]
#[ignore = "not yet implemented"]
fn parse_expression() {
    struct Case {
        input: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            input: r#""hello""#,
            want: r#"cty.StringVal("hello")"#,
        },
        Case {
            input: r#""hello ${noun}""#,
            want: r#"cty.StringVal("hello world")"#,
        },
        Case {
            input: "true",
            want: "cty.True",
        },
        Case {
            input: "false",
            want: "cty.False",
        },
        Case {
            input: "1",
            want: "cty.NumberIntVal(1)",
        },
        Case {
            input: "{}",
            want: "cty.EmptyObjectVal",
        },
        Case {
            input: r#"{"foo":"bar","baz":1}"#,
            want: r#"cty.ObjectVal(map[string]cty.Value{"baz":cty.NumberIntVal(1), "foo":cty.StringVal("bar")})"#,
        },
        Case {
            input: "[]",
            want: "cty.EmptyTupleVal",
        },
        Case {
            input: r#"["1",2,3]"#,
            want: r#"cty.TupleVal([]cty.Value{cty.StringVal("1"), cty.NumberIntVal(2), cty.NumberIntVal(3)})"#,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, diags) = json::parse_expression(test.input.as_bytes(), "");
        assert!(
            !diags.has_errors(),
            "case {i} ({}): got {} diagnostics; want 0\n{diags}",
            test.input,
            diags.len(),
        );

        let mut ctx = EvalContext::new();
        ctx.variables
            .insert("noun".to_string(), Value::string("world"));
        let (value, diags) = expr.value(Some(&ctx));
        assert!(
            !diags.has_errors(),
            "case {i} ({}): got {} diagnostics on decode value; want 0\n{diags}",
            test.input,
            diags.len(),
        );
        let got = value.go_string();

        assert_eq!(
            got, test.want,
            "case {i} ({}): got {got}, but want {}",
            test.input, test.want,
        );
    }
}

// Ported from TestParseExpression_malformed:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L263
#[test]
#[ignore = "not yet implemented"]
fn parse_expression_malformed() {
    let src = "invalid";
    let (_expr, diags) = json::parse_expression(src.as_bytes(), "");
    assert_eq!(diags.len(), 1, "got {} diagnostics; want 1", diags.len());
    let err = format!("{diags}");
    let want = "Invalid JSON keyword";
    assert!(
        err.contains(want),
        "diags are {err:?}, but should contain {want:?}",
    );
    // NOTE(port): upstream checks `expr == nil`; the Rust `parse_expression`
    // returns an `ExprRef` by value, so there is no nil case.
}

// Ported from TestParseExpressionWithStartPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/public_test.go#L277
#[test]
#[ignore = "not yet implemented"]
fn parse_expression_with_start_pos() {
    let src = "{\n  \"foo\": \"bar\"\n}";
    let part = "\"bar\"";

    let (file, diags) = json::parse(src.as_bytes(), "");
    let (part_expr, part_diags) = json::parse_expression_with_start_pos(
        part.as_bytes(),
        "",
        Pos {
            byte: 0,
            line: 2,
            column: 10,
        },
    );
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on parse src; want 0\n{diags}",
        diags.len(),
    );
    assert_eq!(
        part_diags.len(),
        0,
        "got {} diagnostics on parse part src; want 0\n{part_diags}",
        part_diags.len(),
    );

    // NOTE(port): upstream checks `file`/`file.Body`/`partExpr` against nil;
    // the Rust `parse`/`parse_expression_with_start_pos` return `File` and
    // `ExprRef` by value, so there are no nil cases.

    let (content, diags) = file.body.content(&BodySchema {
        attributes: vec![AttributeSchema {
            name: "foo".to_string(),
            ..Default::default()
        }],
        ..Default::default()
    });
    assert_eq!(
        diags.len(),
        0,
        "got {} diagnostics on decode; want 0\n{diags}",
        diags.len(),
    );
    let expr = &content.attributes["foo"].expr;

    assert_eq!(
        format!("{}", expr.range()),
        format!("{}", part_expr.range()),
        "The two ranges did not match: src={}, part={}",
        expr.range(),
        part_expr.range(),
    );
}

// Ported from TestNavigationContextString:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/navigation_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn navigation_context_string() {
    // The upstream source literal mixes space and tab indentation; the tabs
    // are significant to the byte offsets below, so it is written escaped.
    let src = "\n{\n  \"version\": 1,\n  \"resource\": {\n    \"null_resource\": {\n      \"baz\": {\n        \"id\": \"foo\"\n\t\t\t},\n\t\t\t\"boz\": [\n\t\t\t\t{\n\t\t\t\t\t\"ov\": {   }\n\t\t\t\t}\n\t\t\t]\n    }\n  }\n}\n";
    let (file, diags) = json::parse(src.as_bytes(), "test.json");
    assert_eq!(diags.len(), 0, "Unexpected diagnostics: {diags}");
    // NOTE(port): upstream also checks `file == nil`; the Rust `parse`
    // returns `File` by value, so there is no nil case. The Go type
    // assertion `file.Nav.(navigation)` becomes unwrapping the
    // `Option<Arc<dyn FileNav>>`.
    let nav = file.nav.as_ref().unwrap();

    struct Case {
        offset: usize,
        want: &'static str,
    }

    let tests = [
        Case {
            offset: 0,
            want: "",
        },
        Case {
            offset: 8,
            want: "",
        },
        Case {
            offset: 36,
            want: "resource",
        },
        Case {
            offset: 60,
            want: "resource.null_resource",
        },
        Case {
            offset: 89,
            want: "resource.null_resource.baz",
        },
        Case {
            offset: 141,
            want: "resource.null_resource.boz",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = nav.context_string(test.offset);
        assert_eq!(
            got, test.want,
            "case {i} (offset {}): wrong result",
            test.offset,
        );
    }
}

// Ported from TestKeywordSuggestion:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/didyoumean_test.go#L8
#[test]
#[ignore = "not yet implemented"]
fn keyword_suggestion() {
    struct Case {
        input: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            input: "true",
            want: "true",
        },
        Case {
            input: "false",
            want: "false",
        },
        Case {
            input: "null",
            want: "null",
        },
        Case {
            input: "bananas",
            want: "",
        },
        Case {
            input: "NaN",
            want: "",
        },
        Case {
            input: "Inf",
            want: "",
        },
        Case {
            input: "Infinity",
            want: "",
        },
        Case {
            input: "void",
            want: "",
        },
        Case {
            input: "undefined",
            want: "",
        },
        Case {
            input: "ture",
            want: "true",
        },
        Case {
            input: "tru",
            want: "true",
        },
        Case {
            input: "tre",
            want: "true",
        },
        Case {
            input: "treu",
            want: "true",
        },
        Case {
            input: "rtue",
            want: "true",
        },
        Case {
            input: "flase",
            want: "false",
        },
        Case {
            input: "fales",
            want: "false",
        },
        Case {
            input: "flse",
            want: "false",
        },
        Case {
            input: "fasle",
            want: "false",
        },
        Case {
            input: "fasel",
            want: "false",
        },
        Case {
            input: "flue",
            want: "false",
        },
        Case {
            input: "nil",
            want: "null",
        },
        Case {
            input: "nul",
            want: "null",
        },
        Case {
            input: "unll",
            want: "null",
        },
        Case {
            input: "nll",
            want: "null",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = json::keyword_suggestion(test.input);
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ninput: {:?}\ngot:   {got:?}\nwant:  {:?}",
            test.input, test.want,
        );
    }
}
