//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   diagnostic_text_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use cty::{Type, Value};
use hcl::{
    Diagnostic, DiagnosticSeverity, DiagnosticTextWriter, DiagnosticWriter, Diagnostics,
    EvalContext, ExprRef, Expression, File, FileNav, Pos, Range, Traversal, Traverser,
};

/// Go: the `testDiagnosticTextWriterSource` constant.
const TEST_DIAGNOSTIC_TEXT_WRITER_SOURCE: &str = r#"foo = 1
bar = 2
baz = 3
block "party" {
  pizza = "cheese"
}
"#;

/// Go: `diagnosticTestNav` — a `File.Nav` whose `ContextString` is
/// hard-coded.
#[derive(Debug)]
struct DiagnosticTestNav;

impl FileNav for DiagnosticTestNav {
    fn context_string(&self, _offset: usize) -> String {
        "hardcoded-context".to_string()
    }
}

/// Go: `diagnosticTestExpr` — an expression reporting a fixed set of
/// variables; the other methods mirror the zero-value `staticExpr` the Go
/// helper embeds (its zero `val` is `cty.NilVal`, which has no rust-cty
/// analogue, but this test never calls `value()`).
#[derive(Debug)]
struct DiagnosticTestExpr {
    vars: Vec<Traversal>,
}

impl Expression for DiagnosticTestExpr {
    fn value(&self, _ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        unreachable!("not called by this test")
    }

    fn variables(&self) -> Vec<Traversal> {
        self.vars.clone()
    }

    fn range(&self) -> Range {
        Range::default()
    }

    fn start_range(&self) -> Range {
        Range::default()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A traversal step with a zero source range (Go: composite literals like
/// `hcl.TraverseRoot{Name: ...}` with `SrcRange` left as its zero value).
fn root(name: &str) -> Traverser {
    Traverser::Root {
        name: name.to_string(),
        src_range: Range::default(),
    }
}

// Ported from TestDiagnosticTextWriter:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/diagnostic_text_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn diagnostic_text_writer() {
    struct Case {
        input: Diagnostic,
        want: &'static str,
    }

    let tests = [
        Case {
            input: Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Splines not reticulated".to_string(),
                detail: "All splines must be pre-reticulated.".to_string(),
                subject: Some(Range {
                    filename: String::new(),
                    start: Pos {
                        byte: 0,
                        column: 1,
                        line: 1,
                    },
                    end: Pos {
                        byte: 3,
                        column: 4,
                        line: 1,
                    },
                }),
                ..Default::default()
            },
            want: r#"Error: Splines not reticulated

  on  line 1, in hardcoded-context:
   1: foo = 1

All splines must be pre-reticulated.

"#,
        },
        Case {
            input: Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unsupported attribute".to_string(),
                detail: r#""baz" is not a supported top-level attribute. Did you mean "bam"?"#
                    .to_string(),
                subject: Some(Range {
                    filename: String::new(),
                    start: Pos {
                        byte: 16,
                        column: 1,
                        line: 3,
                    },
                    end: Pos {
                        byte: 19,
                        column: 4,
                        line: 3,
                    },
                }),
                ..Default::default()
            },
            want: r#"Error: Unsupported attribute

  on  line 3, in hardcoded-context:
   3: baz = 3

"baz" is not a supported top-level
attribute. Did you mean "bam"?

"#,
        },
        Case {
            input: Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unsupported attribute".to_string(),
                detail: r#""pizza" is not a supported attribute. Did you mean "pizzetta"?"#
                    .to_string(),
                subject: Some(Range {
                    filename: String::new(),
                    start: Pos {
                        byte: 42,
                        column: 3,
                        line: 5,
                    },
                    end: Pos {
                        byte: 47,
                        column: 8,
                        line: 5,
                    },
                }),
                // This is actually not a great example of a context, but is
                // here to test whether we're able to show a multi-line
                // context when needed.
                context: Some(Range {
                    filename: String::new(),
                    start: Pos {
                        byte: 24,
                        column: 1,
                        line: 4,
                    },
                    end: Pos {
                        byte: 60,
                        column: 2,
                        line: 6,
                    },
                }),
                ..Default::default()
            },
            want: r#"Error: Unsupported attribute

  on  line 5, in hardcoded-context:
   4: block "party" {
   5:   pizza = "cheese"
   6: }

"pizza" is not a supported attribute.
Did you mean "pizzetta"?

"#,
        },
        Case {
            input: Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Test of including relevant variable values".to_string(),
                detail: "This diagnostic includes an expression and an evalcontext.".to_string(),
                subject: Some(Range {
                    filename: String::new(),
                    start: Pos {
                        byte: 42,
                        column: 3,
                        line: 5,
                    },
                    end: Pos {
                        byte: 47,
                        column: 8,
                        line: 5,
                    },
                }),
                expression: Some(ExprRef::new(DiagnosticTestExpr {
                    vars: vec![
                        Traversal(vec![root("foo")]),
                        Traversal(vec![
                            root("bar"),
                            Traverser::Attr {
                                name: "baz".to_string(),
                                src_range: Range::default(),
                            },
                        ]),
                        Traversal(vec![root("missing")]),
                        Traversal(vec![root("boz")]),
                        Traversal(vec![root("marked")]),
                        Traversal(vec![root("null")]),
                        Traversal(vec![root("unknown")]),
                    ],
                })),
                // Go constructs `&EvalContext{parent: ..., Variables: ...}`
                // directly via the unexported parent field; the child-of-
                // parent construction goes through `new_child` here.
                eval_context: Some(Arc::new({
                    let mut parent = EvalContext::new();
                    parent.variables =
                        HashMap::from([("foo".to_string(), Value::string("foo value"))]);
                    let mut ctx = EvalContext::new_child(&Arc::new(parent));
                    ctx.variables = HashMap::from([
                        (
                            "bar".to_string(),
                            Value::object([("baz", Value::list_empty(Type::string()))]),
                        ),
                        ("boz".to_string(), Value::number_int(5)),
                        ("marked".to_string(), Value::string("marked").mark("x")),
                        ("null".to_string(), Value::null(Type::string())),
                        ("unknown".to_string(), Value::unknown(Type::string())),
                        ("unused".to_string(), Value::bool(true)),
                    ]);
                    ctx
                })),
                ..Default::default()
            },
            want: r#"Error: Test of including relevant variable values

  on  line 5, in hardcoded-context:
   5:   pizza = "cheese"

with bar.baz as empty list of string,
     boz as 5,
     foo as "foo value",
     null set to null.

This diagnostic includes an expression
and an evalcontext.

"#,
        },
    ];

    // NOTE(port): Go's `&File{Bytes: ..., Nav: ...}` leaves `Body` nil,
    // which has no Rust analogue (`File.body` is a `BodyRef`); the empty
    // body stands in — the writer only consults `bytes` and `nav`.
    let files: HashMap<String, File> = HashMap::from([(
        String::new(),
        File {
            body: hcl::empty_body(),
            bytes: TEST_DIAGNOSTIC_TEXT_WRITER_SOURCE.as_bytes().to_vec(),
            nav: Some(Arc::new(DiagnosticTestNav)),
        },
    )]);

    for (i, test) in tests.iter().enumerate() {
        let mut bwr: Vec<u8> = Vec::new();
        {
            let mut dwr = DiagnosticTextWriter::new(&mut bwr, files.clone(), 40, false);
            dwr.write_diagnostic(&test.input)
                .unwrap_or_else(|err| panic!("case {i}: unexpected error: {err}"));
        }
        let got = String::from_utf8(bwr).expect("output is valid UTF-8");
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\n\ngot:\n{got}want:\n{}",
            test.want,
        );
    }
}
