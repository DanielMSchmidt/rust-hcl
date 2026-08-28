//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hcldec/spec_test.go
//!   hcldec/variables_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;

use cty::function::{self, Function, Parameter};
use cty::{RefinementBuilder, Type, Value};
use hcl::hcldec::{
    self, AttrSpec, BlockAttrsSpec, BlockListSpec, BlockSpec, DefaultSpec, ObjectSpec,
    RefineValueSpec, SpecRef, ValidateSpec,
};
use hcl::{
    Diagnostic, DiagnosticSeverity, Diagnostics, EvalContext, Pos, Range, Traversal, Traverser,
    hclsyntax,
};

// NOTE(port): hcldec/spec_test.go opens (lines 20–52) with compile-time
// assertions that each spec type implements the `Spec`, `attrSpec`,
// `blockSpec`, and `specNeedingVariables` interfaces
// (`var _ Spec = ObjectSpec(nil)` and so on). The public `Spec` half is
// enforced structurally in Rust — each spec type has an `impl Spec` in
// `src/hcldec.rs` — and the unexported `attrSpec`/`blockSpec`/
// `specNeedingVariables` interfaces are implementation details with no
// public Rust analogue, so those assertions have no runtime port here.

/// A range within an unnamed file (Go: `hcl.Range{...}` with `Filename`
/// left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

// Ported from TestDefaultSpec:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/spec_test.go#L54
#[test]
#[ignore = "not yet implemented"]
fn default_spec() {
    let config = "\nfoo = fooval\nbar = barval\n";
    let (f, diags) = hclsyntax::parse_config(
        config.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    assert!(!diags.has_errors(), "{diags}");

    // Subtest "primary set" (Go: t.Run("primary set", ...)).
    {
        let spec = DefaultSpec {
            primary: SpecRef::new(AttrSpec {
                name: "foo".to_string(),
                ty: Type::string(),
                required: false,
            }),
            default: SpecRef::new(AttrSpec {
                name: "bar".to_string(),
                ty: Type::string(),
                required: false,
            }),
        };

        let got_vars = hcldec::variables(&*f.body, &spec);
        let want_vars = vec![
            Traversal(vec![Traverser::Root {
                name: "fooval".to_string(),
                src_range: rng(
                    Pos {
                        line: 2,
                        column: 7,
                        byte: 7,
                    },
                    Pos {
                        line: 2,
                        column: 13,
                        byte: 13,
                    },
                ),
            }]),
            Traversal(vec![Traverser::Root {
                name: "barval".to_string(),
                src_range: rng(
                    Pos {
                        line: 3,
                        column: 7,
                        byte: 20,
                    },
                    Pos {
                        line: 3,
                        column: 13,
                        byte: 26,
                    },
                ),
            }]),
        ];
        assert_eq!(got_vars, want_vars, "primary set: wrong Variables result");

        let mut ctx = EvalContext::new();
        ctx.variables = HashMap::from([
            ("fooval".to_string(), Value::string("foo value")),
            ("barval".to_string(), Value::string("bar value")),
        ]);

        let (got, diags) = hcldec::decode(&*f.body, &spec, Some(&ctx));
        // NOTE(port): Go's `if err != nil` on the returned `hcl.Diagnostics`
        // is a nil-slice check, so any diagnostic at all is fatal.
        assert!(diags.is_empty(), "{diags}");
        let want = Value::string("foo value");
        assert_eq!(got, want, "primary set: wrong Decode result");
    }

    // Subtest "primary not set" (Go: t.Run("primary not set", ...)).
    {
        let spec = DefaultSpec {
            primary: SpecRef::new(AttrSpec {
                name: "foo".to_string(),
                ty: Type::string(),
                required: false,
            }),
            default: SpecRef::new(AttrSpec {
                name: "bar".to_string(),
                ty: Type::string(),
                required: false,
            }),
        };

        let mut ctx = EvalContext::new();
        ctx.variables = HashMap::from([
            ("fooval".to_string(), Value::null(Type::string())),
            ("barval".to_string(), Value::string("bar value")),
        ]);

        let (got, diags) = hcldec::decode(&*f.body, &spec, Some(&ctx));
        assert!(diags.is_empty(), "{diags}");
        let want = Value::string("bar value");
        assert_eq!(got, want, "primary not set: wrong Decode result");
    }
}

// Ported from TestValidateFuncSpec:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/spec_test.go#L159
#[test]
#[ignore = "not yet implemented"]
fn validate_func_spec() {
    let config = "\nfoo = \"invalid\"\n";
    let (f, diags) = hclsyntax::parse_config(
        config.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    assert!(!diags.has_errors(), "{diags}");

    // NOTE(port): upstream iterates a `map[string]*hcl.Range` in Go's
    // unspecified map order; the two subtests port as an array in the map
    // literal's order.
    let expect_range: [(&str, Option<Range>); 2] = [
        ("without_range", None),
        (
            "with_range",
            Some(Range {
                filename: "foobar".to_string(),
                start: Pos {
                    line: 99,
                    column: 99,
                    byte: 0,
                },
                end: Pos {
                    line: 999,
                    column: 999,
                    byte: 0,
                },
            }),
        ),
    ];

    for (name, expect) in &expect_range {
        let spec = ValidateSpec {
            wrapped: SpecRef::new(AttrSpec {
                name: "foo".to_string(),
                ty: Type::string(),
                required: false,
            }),
            func: {
                let expect = expect.clone();
                Box::new(move |value: &Value| {
                    if value.as_string() != "invalid" {
                        return Diagnostics(vec![Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            summary: "incorrect value".to_string(),
                            detail: format!("invalid value passed in: {}", value.go_string()),
                            ..Default::default()
                        }]);
                    }

                    Diagnostics(vec![Diagnostic {
                        severity: DiagnosticSeverity::Warning,
                        summary: "OK".to_string(),
                        detail: "validation called correctly".to_string(),
                        subject: expect.clone(),
                        ..Default::default()
                    }])
                })
            },
        };

        let (_, diags) = hcldec::decode(&*f.body, &spec, None);
        assert!(
            diags.len() == 1
                && diags[0].severity == DiagnosticSeverity::Warning
                && diags[0].summary == "OK"
                && diags[0].detail == "validation called correctly",
            "subtest {name}: unexpected diagnostics: {diags}"
        );

        assert!(
            !(expect.is_none() && diags[0].subject.is_none()),
            "subtest {name}: returned diagnostic subject missing"
        );

        if expect.is_some() {
            assert_eq!(
                expect.as_ref(),
                diags[0].subject.as_ref(),
                "subtest {name}: wrong returned diagnostic subject"
            );
        }
    }
}

// Ported from TestRefineValueSpec:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/spec_test.go#L225
#[test]
#[ignore = "not yet implemented"]
fn refine_value_spec() {
    let config = "\nfoo = \"hello\"\nbar = unk\ndyn = dyn\nmarked = mark(unk)\n";

    let (f, diags) = hclsyntax::parse_config(config.as_bytes(), "", Pos::initial());
    assert!(!diags.has_errors(), "{diags}");

    // Go: the `attrSpec` closure inside TestRefineValueSpec.
    fn attr_spec(name: &str) -> SpecRef {
        SpecRef::new(RefineValueSpec {
            // RefineValueSpec should typically have a ValidateSpec wrapped
            // inside it to catch any values that are outside of the required
            // range and return a helpful error message about it. In this
            // case our refinement is .NotNull so the validation function
            // must reject null values.
            wrapped: SpecRef::new(ValidateSpec {
                wrapped: SpecRef::new(AttrSpec {
                    name: name.to_string(),
                    required: true,
                    ty: Type::string(),
                }),
                func: Box::new(|value: &Value| {
                    let mut diags = Diagnostics::new();
                    if value.is_null() {
                        diags.push(Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            summary: "Cannot be null".to_string(),
                            detail: "Argument is required.".to_string(),
                            ..Default::default()
                        });
                    }
                    diags
                }),
            }),
            refine: Box::new(|rb: RefinementBuilder| rb.not_null()),
        })
    }
    let spec: ObjectSpec = [
        ("foo", attr_spec("foo")),
        ("bar", attr_spec("bar")),
        ("dyn", attr_spec("dyn")),
        ("marked", attr_spec("marked")),
    ]
    .into_iter()
    .collect();

    let mut ctx = EvalContext::new();
    ctx.variables = HashMap::from([
        ("unk".to_string(), Value::unknown(Type::string())),
        ("dyn".to_string(), Value::dynamic()),
    ]);
    ctx.functions = HashMap::from([(
        "mark".to_string(),
        Function::new(function::Spec {
            description: String::new(),
            params: vec![Parameter {
                name: "v".to_string(),
                ty: Some(Type::dynamic()),
                allow_marked: true,
                allow_null: true,
                allow_unknown: true,
                allow_dynamic_type: true,
                ..Default::default()
            }],
            var_param: None,
            type_fn: Box::new(|args| Ok(args[0].ty())),
            refine_result: None,
            impl_fn: Box::new(|args, _ret_type| Ok(args[0].mark("boop"))),
        }),
    )]);

    let (got, diags) = hcldec::decode(&*f.body, &spec, Some(&ctx));
    assert!(!diags.has_errors(), "{diags}");

    let want = Value::object([
        // This argument had a known value, so it's unchanged but the
        // RefineValueSpec still checks that it isn't null to catch
        // bugs in the application's validation function.
        ("foo", Value::string("hello")),
        // The final value of bar is unknown but refined as non-null.
        ("bar", Value::unknown(Type::string()).refine_not_null()),
        // The final value of dyn is unknown but refined as non-null.
        // Correct behavior here requires that we convert the DynamicVal
        // to an unknown string first and then refine it.
        ("dyn", Value::unknown(Type::string()).refine_not_null()),
        // This argument had a mark applied, which should be preserved
        // despite the refinement.
        (
            "marked",
            Value::unknown(Type::string())
                .refine_not_null()
                .mark("boop"),
        ),
    ]);
    assert_eq!(got, want, "wrong result");
}

// Ported from TestVariables:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/variables_test.go#L16
#[test]
#[ignore = "not yet implemented"]
fn variables() {
    struct Case {
        config: &'static str,
        spec: SpecRef,
        want: Vec<Traversal>,
    }

    // NOTE(port): several upstream cases leave `AttrSpec.Type` at its Go
    // zero value (`cty.NilType`), which has no rust-cty analogue; `Variables`
    // never consults the type, so those port with `Type::dynamic()` as an
    // inert stand-in. Upstream `want: nil` (a nil `[]hcl.Traversal`) ports
    // as an empty vec.
    let tests = [
        Case {
            config: "",
            spec: SpecRef::new(ObjectSpec::default()),
            want: vec![],
        },
        Case {
            config: "a = foo\n",
            spec: SpecRef::new(ObjectSpec::default()),
            // "a" is not actually used, so "foo" is not required
            want: vec![],
        },
        Case {
            config: "a = foo\n",
            spec: SpecRef::new(AttrSpec {
                name: "a".to_string(),
                ty: Type::dynamic(),
                required: false,
            }),
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 5,
                        byte: 4,
                    },
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                ),
            }])],
        },
        Case {
            config: "a = foo\nb = bar\n",
            spec: SpecRef::new(DefaultSpec {
                primary: SpecRef::new(AttrSpec {
                    name: "a".to_string(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                default: SpecRef::new(AttrSpec {
                    name: "b".to_string(),
                    ty: Type::dynamic(),
                    required: false,
                }),
            }),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 5,
                            byte: 4,
                        },
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                    ),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: rng(
                        Pos {
                            line: 2,
                            column: 5,
                            byte: 12,
                        },
                        Pos {
                            line: 2,
                            column: 8,
                            byte: 15,
                        },
                    ),
                }]),
            ],
        },
        Case {
            config: "a = foo\n",
            spec: SpecRef::new(ObjectSpec::from_iter([(
                "a",
                SpecRef::new(AttrSpec {
                    name: "a".to_string(),
                    ty: Type::dynamic(),
                    required: false,
                }),
            )])),
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 5,
                        byte: 4,
                    },
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                ),
            }])],
        },
        Case {
            config: "\nb {\n  a = foo\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".to_string(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".to_string(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                required: false,
            }),
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 3,
                        column: 7,
                        byte: 11,
                    },
                    Pos {
                        line: 3,
                        column: 10,
                        byte: 14,
                    },
                ),
            }])],
        },
        Case {
            config: "\nb {\n  a = foo\n  b = bar\n}\n",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".to_string(),
                element_type: Type::string(),
                required: false,
            }),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 3,
                            column: 7,
                            byte: 11,
                        },
                        Pos {
                            line: 3,
                            column: 10,
                            byte: 14,
                        },
                    ),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: rng(
                        Pos {
                            line: 4,
                            column: 7,
                            byte: 21,
                        },
                        Pos {
                            line: 4,
                            column: 10,
                            byte: 24,
                        },
                    ),
                }]),
            ],
        },
        Case {
            config: "\nb {\n  a = foo\n}\nb {\n  a = bar\n}\nc {\n  a = baz\n}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".to_string(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".to_string(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 3,
                            column: 7,
                            byte: 11,
                        },
                        Pos {
                            line: 3,
                            column: 10,
                            byte: 14,
                        },
                    ),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: rng(
                        Pos {
                            line: 6,
                            column: 7,
                            byte: 27,
                        },
                        Pos {
                            line: 6,
                            column: 10,
                            byte: 30,
                        },
                    ),
                }]),
            ],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, diags) = hclsyntax::parse_config(
            test.config.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert_eq!(
            diags.len(),
            0,
            "case {i} ({:?}): wrong number of diagnostics from ParseConfig: {diags}",
            test.config,
        );
        let body = &file.body;

        let got = hcldec::variables(&**body, &*test.spec);

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.config,);
    }
}
