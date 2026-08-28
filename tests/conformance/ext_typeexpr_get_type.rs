//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ext/typeexpr/get_type_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::ext::typeexpr::{self, Defaults};
use hcl::gohcl::{self, FromBody};
use hcl::{DiagnosticSeverity, ExprRef, Pos, hclsyntax, json};

// NOTE(port): upstream drives the unexported `getType(expr, constraint,
// withDefaults)` directly. The public wrappers are one-to-one:
// `getType(expr, false, false)` is `typeexpr.Type` (`typeexpr::ty`),
// `getType(expr, true, false)` is `typeexpr.TypeConstraint`
// (`typeexpr::type_constraint`), and `getType(expr, true, true)` is
// `typeexpr.TypeConstraintWithDefaults`
// (`typeexpr::type_constraint_with_defaults`), so the tests here go through
// the public API.

/// Dispatches to `typeexpr::ty` or `typeexpr::type_constraint` per the
/// case's constraint flag (Go: `getType(expr, test.Constraint, false)`).
fn get_type_for(expr: &dyn hcl::Expression, constraint: bool) -> (Type, hcl::Diagnostics) {
    if constraint {
        typeexpr::type_constraint(expr)
    } else {
        typeexpr::ty(expr)
    }
}

// Ported from TestGetType:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/get_type_test.go#L24
#[test]
#[ignore = "not yet implemented"]
fn get_type() {
    struct Case {
        source: &'static str,
        constraint: bool,
        want: Type,
        want_error: &'static str,
    }

    let tests = vec![
        // keywords
        Case {
            source: r#"bool"#,
            constraint: false,
            want: Type::bool(),
            want_error: "",
        },
        Case {
            source: r#"number"#,
            constraint: false,
            want: Type::number(),
            want_error: "",
        },
        Case {
            source: r#"string"#,
            constraint: false,
            want: Type::string(),
            want_error: "",
        },
        Case {
            source: r#"any"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"The keyword "any" cannot be used in this type specification: an exact type is required."#,
        },
        Case {
            source: r#"any"#,
            constraint: true,
            want: Type::dynamic(),
            want_error: "",
        },
        Case {
            source: r#"list"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The list type constructor requires one argument specifying the element type.",
        },
        Case {
            source: r#"map"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The map type constructor requires one argument specifying the element type.",
        },
        Case {
            source: r#"set"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The set type constructor requires one argument specifying the element type.",
        },
        Case {
            source: r#"object"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The object type constructor requires one argument specifying the attribute types and values as a map.",
        },
        Case {
            source: r#"tuple"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The tuple type constructor requires one argument specifying the element types as a list.",
        },
        // constructors
        Case {
            source: r#"bool()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Primitive type keyword "bool" does not expect arguments."#,
        },
        Case {
            source: r#"number()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Primitive type keyword "number" does not expect arguments."#,
        },
        Case {
            source: r#"string()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Primitive type keyword "string" does not expect arguments."#,
        },
        Case {
            source: r#"any()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Type constraint keyword "any" does not expect arguments."#,
        },
        Case {
            source: r#"any()"#,
            constraint: true,
            want: Type::dynamic(),
            want_error: r#"Type constraint keyword "any" does not expect arguments."#,
        },
        Case {
            source: r#"list(string)"#,
            constraint: false,
            want: Type::list(Type::string()),
            want_error: "",
        },
        Case {
            source: r#"set(string)"#,
            constraint: false,
            want: Type::set(Type::string()),
            want_error: "",
        },
        Case {
            source: r#"map(string)"#,
            constraint: false,
            want: Type::map(Type::string()),
            want_error: "",
        },
        Case {
            source: r#"list()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The list type constructor requires one argument specifying the element type.",
        },
        Case {
            source: r#"list(string, string)"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The list type constructor requires one argument specifying the element type.",
        },
        Case {
            source: r#"list(any)"#,
            constraint: false,
            want: Type::list(Type::dynamic()),
            want_error: r#"The keyword "any" cannot be used in this type specification: an exact type is required."#,
        },
        Case {
            source: r#"list(any)"#,
            constraint: true,
            want: Type::list(Type::dynamic()),
            want_error: "",
        },
        Case {
            source: r#"object({})"#,
            constraint: false,
            want: Type::empty_object(),
            want_error: "",
        },
        Case {
            source: r#"object({name=string})"#,
            constraint: false,
            want: Type::object([("name", Type::string())]),
            want_error: "",
        },
        Case {
            source: r#"object({"name"=string})"#,
            constraint: false,
            want: Type::empty_object(),
            want_error: "Object constructor map keys must be attribute names.",
        },
        Case {
            source: r#"object({name=nope})"#,
            constraint: false,
            want: Type::object([("name", Type::dynamic())]),
            want_error: r#"The keyword "nope" is not a valid type specification."#,
        },
        Case {
            source: r#"object()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The object type constructor requires one argument specifying the attribute types and values as a map.",
        },
        Case {
            source: r#"object(string)"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "Object type constructor requires a map whose keys are attribute names and whose values are the corresponding attribute types.",
        },
        Case {
            source: r#"tuple([])"#,
            constraint: false,
            want: Type::empty_tuple(),
            want_error: "",
        },
        Case {
            source: r#"tuple([string, bool])"#,
            constraint: false,
            want: Type::tuple([Type::string(), Type::bool()]),
            want_error: "",
        },
        Case {
            source: r#"tuple([nope])"#,
            constraint: false,
            want: Type::tuple([Type::dynamic()]),
            want_error: r#"The keyword "nope" is not a valid type specification."#,
        },
        Case {
            source: r#"tuple()"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The tuple type constructor requires one argument specifying the element types as a list.",
        },
        Case {
            source: r#"tuple(string)"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "Tuple type constructor requires a list of element types.",
        },
        Case {
            source: r#"shwoop(string)"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Keyword "shwoop" is not a valid type constructor."#,
        },
        Case {
            source: r#"list("string")"#,
            constraint: false,
            want: Type::list(Type::dynamic()),
            want_error: "A type specification is either a primitive type keyword (bool, number, string) or a complex type constructor call, like list(string).",
        },
        // More interesting combinations
        Case {
            source: r#"list(object({}))"#,
            constraint: false,
            want: Type::list(Type::empty_object()),
            want_error: "",
        },
        Case {
            source: r#"list(map(tuple([])))"#,
            constraint: false,
            want: Type::list(Type::map(Type::empty_tuple())),
            want_error: "",
        },
        // Optional modifier
        Case {
            source: r#"object({name=string,age=optional(number)})"#,
            constraint: true,
            want: Type::object_with_optional_attrs(
                [("name", Type::string()), ("age", Type::number())],
                &["age"],
            ),
            want_error: "",
        },
        Case {
            source: r#"object({name=string,meta=optional(any)})"#,
            constraint: true,
            want: Type::object_with_optional_attrs(
                [("name", Type::string()), ("meta", Type::dynamic())],
                &["meta"],
            ),
            want_error: "",
        },
        Case {
            source: r#"object({name=string,age=optional(number)})"#,
            constraint: false,
            want: Type::object([("name", Type::string()), ("age", Type::number())]),
            want_error: "Optional attribute modifier is only for type constraints, not for exact types.",
        },
        Case {
            source: r#"object({name=string,meta=optional(any)})"#,
            constraint: false,
            want: Type::object([("name", Type::string()), ("meta", Type::dynamic())]),
            want_error: "Optional attribute modifier is only for type constraints, not for exact types.",
        },
        Case {
            source: r#"object({name=string,meta=optional()})"#,
            constraint: true,
            want: Type::object([("name", Type::string())]),
            want_error: "Optional attribute modifier requires the attribute type as its argument.",
        },
        Case {
            source: r#"object({name=string,meta=optional(string, "hello")})"#,
            constraint: true,
            want: Type::object([("name", Type::string()), ("meta", Type::string())]),
            want_error: "Optional attribute modifier expects only one argument: the attribute type.",
        },
        Case {
            source: r#"optional(string)"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"Keyword "optional" is valid only as a modifier for object type attributes."#,
        },
        Case {
            source: r#"optional"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: r#"The keyword "optional" is not a valid type specification."#,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.source.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            !parse_diags.has_errors(),
            "case {i} ({:?}, constraint={}): failed to parse: {:?}",
            test.source,
            test.constraint,
            *parse_diags,
        );

        let (got, diags) = get_type_for(&expr, test.constraint);
        if test.want_error.is_empty() {
            assert!(
                diags.is_empty(),
                "case {i} ({:?}, constraint={}): unexpected diagnostics: {:?}",
                test.source,
                test.constraint,
                *diags,
            );
        } else {
            let found = diags.iter().any(|diag| {
                diag.severity == DiagnosticSeverity::Error && diag.detail == test.want_error
            });
            assert!(
                found,
                "case {i} ({:?}, constraint={}): missing expected error detail message: {}\ngot diagnostics: {:?}",
                test.source, test.constraint, test.want_error, *diags,
            );
        }

        assert_eq!(
            got, test.want,
            "case {i} ({:?}, constraint={}): wrong result",
            test.source, test.constraint,
        );
    }
}

// Ported from TestGetTypeJSON:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/get_type_test.go#L359
#[test]
#[ignore = "not yet implemented"]
fn get_type_json() {
    // We have fewer test cases here because we're mainly exercising the
    // extra indirection in the JSON syntax package, which ultimately calls
    // into the native syntax parser (which we tested extensively in
    // TestGetType).
    struct Case {
        source: &'static str,
        constraint: bool,
        want: Type,
        want_error: &'static str,
    }

    let tests = [
        Case {
            source: r#"{"expr":"bool"}"#,
            constraint: false,
            want: Type::bool(),
            want_error: "",
        },
        Case {
            source: r#"{"expr":"list(bool)"}"#,
            constraint: false,
            want: Type::list(Type::bool()),
            want_error: "",
        },
        Case {
            source: r#"{"expr":"list"}"#,
            constraint: false,
            want: Type::dynamic(),
            want_error: "The list type constructor requires one argument specifying the element type.",
        },
    ];

    // Go: `type TestContent struct { Expr hcl.Expression `hcl:"expr"` }`.
    #[derive(FromBody)]
    struct TestContent {
        #[hcl(attr = "expr")]
        expr: ExprRef,
    }

    for (i, test) in tests.iter().enumerate() {
        let (file, parse_diags) = json::parse(test.source.as_bytes(), "");
        assert!(
            !parse_diags.has_errors(),
            "case {i} ({:?}): failed to parse: {:?}",
            test.source,
            *parse_diags,
        );

        let (content, decode_diags) = gohcl::decode_body::<TestContent>(&*file.body, None);
        assert!(
            !decode_diags.has_errors(),
            "case {i} ({:?}): failed to decode: {:?}",
            test.source,
            *decode_diags,
        );

        let (got, diags) = get_type_for(&*content.expr, test.constraint);
        if test.want_error.is_empty() {
            assert!(
                diags.is_empty(),
                "case {i} ({:?}): unexpected diagnostics: {:?}",
                test.source,
                *diags,
            );
        } else {
            let found = diags.iter().any(|diag| {
                diag.severity == DiagnosticSeverity::Error && diag.detail == test.want_error
            });
            assert!(
                found,
                "case {i} ({:?}): missing expected error detail message: {}\ngot diagnostics: {:?}",
                test.source, test.want_error, *diags,
            );
        }

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.source,);
    }
}

/// A `Defaults` literal (Go: `&Defaults{Type: ..., DefaultValues: ...,
/// Children: ...}`; Go's nil zero-value maps become empty maps here, per
/// the by-value `children` mapping in docs/api-mapping.md).
fn defaults(
    ty: Type,
    default_values: Vec<(&str, Value)>,
    children: Vec<(&str, Defaults)>,
) -> Defaults {
    Defaults {
        ty,
        default_values: default_values
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        children: children
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
    }
}

// Ported from TestGetTypeDefaults:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/get_type_test.go#L431
#[test]
#[ignore = "not yet implemented"]
fn get_type_defaults() {
    struct Case {
        source: &'static str,
        want: Option<Defaults>,
        want_error: &'static str,
    }

    let tests = vec![
        // primitive types have nil defaults
        Case {
            source: r#"bool"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"number"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"string"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"any"#,
            want: None,
            want_error: "",
        },
        // complex structures with no defaults have nil defaults
        Case {
            source: r#"map(string)"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"set(number)"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"tuple([number, string])"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"object({ a = string, b = number })"#,
            want: None,
            want_error: "",
        },
        Case {
            source: r#"map(list(object({ a = string, b = optional(number) })))"#,
            want: None,
            want_error: "",
        },
        // object optional attribute with defaults
        Case {
            source: r#"object({ a = string, b = optional(number, 5) })"#,
            want: Some(defaults(
                Type::object_with_optional_attrs(
                    [("a", Type::string()), ("b", Type::number())],
                    &["b"],
                ),
                vec![("b", Value::number_int(5))],
                vec![],
            )),
            want_error: "",
        },
        // nested defaults
        Case {
            source: r#"object({ a = optional(object({ b = optional(number, 5) }), {}) })"#,
            want: Some(defaults(
                Type::object_with_optional_attrs(
                    [(
                        "a",
                        Type::object_with_optional_attrs([("b", Type::number())], &["b"]),
                    )],
                    &["a"],
                ),
                vec![("a", Value::object([("b", Value::null(Type::number()))]))],
                vec![(
                    "a",
                    defaults(
                        Type::object_with_optional_attrs([("b", Type::number())], &["b"]),
                        vec![("b", Value::number_int(5))],
                        vec![],
                    ),
                )],
            )),
            want_error: "",
        },
        // collections of objects with defaults
        Case {
            source: r#"map(object({ a = string, b = optional(number, 5) }))"#,
            want: Some(defaults(
                Type::map(Type::object_with_optional_attrs(
                    [("a", Type::string()), ("b", Type::number())],
                    &["b"],
                )),
                vec![],
                vec![(
                    "",
                    defaults(
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::number())],
                            &["b"],
                        ),
                        vec![("b", Value::number_int(5))],
                        vec![],
                    ),
                )],
            )),
            want_error: "",
        },
        Case {
            source: r#"list(object({ a = string, b = optional(number, 5) }))"#,
            want: Some(defaults(
                Type::list(Type::object_with_optional_attrs(
                    [("a", Type::string()), ("b", Type::number())],
                    &["b"],
                )),
                vec![],
                vec![(
                    "",
                    defaults(
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::number())],
                            &["b"],
                        ),
                        vec![("b", Value::number_int(5))],
                        vec![],
                    ),
                )],
            )),
            want_error: "",
        },
        Case {
            source: r#"set(object({ a = string, b = optional(number, 5) }))"#,
            want: Some(defaults(
                Type::set(Type::object_with_optional_attrs(
                    [("a", Type::string()), ("b", Type::number())],
                    &["b"],
                )),
                vec![],
                vec![(
                    "",
                    defaults(
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::number())],
                            &["b"],
                        ),
                        vec![("b", Value::number_int(5))],
                        vec![],
                    ),
                )],
            )),
            want_error: "",
        },
        // tuples containing objects with defaults work differently from
        // collections
        Case {
            source: r#"tuple([string, bool, object({ a = string, b = optional(number, 5) })])"#,
            want: Some(defaults(
                Type::tuple([
                    Type::string(),
                    Type::bool(),
                    Type::object_with_optional_attrs(
                        [("a", Type::string()), ("b", Type::number())],
                        &["b"],
                    ),
                ]),
                vec![],
                vec![(
                    "2",
                    defaults(
                        Type::object_with_optional_attrs(
                            [("a", Type::string()), ("b", Type::number())],
                            &["b"],
                        ),
                        vec![("b", Value::number_int(5))],
                        vec![],
                    ),
                )],
            )),
            want_error: "",
        },
        // Lists should remove optional metadata from the concrete default
        // values.
        Case {
            source: r#"object({ list = optional(list(object({ required = string, optional = optional(string) })), [])})"#,
            want: Some(defaults(
                Type::object_with_optional_attrs(
                    [(
                        "list",
                        Type::list(Type::object_with_optional_attrs(
                            [("required", Type::string()), ("optional", Type::string())],
                            &["optional"],
                        )),
                    )],
                    &["list"],
                ),
                vec![(
                    "list",
                    Value::list_empty(Type::object([
                        ("required", Type::string()),
                        ("optional", Type::string()),
                    ])),
                )],
                vec![],
            )),
            want_error: "",
        },
        // Lists should remove optional metadata from the concrete default
        // values but should still apply recursive defaults.
        Case {
            source: r#"object({ list = optional(list(object({ required = string, optional = optional(string, "optional") })), [{ required = "required" }])})"#,
            want: Some(defaults(
                Type::object_with_optional_attrs(
                    [(
                        "list",
                        Type::list(Type::object_with_optional_attrs(
                            [("required", Type::string()), ("optional", Type::string())],
                            &["optional"],
                        )),
                    )],
                    &["list"],
                ),
                vec![(
                    "list",
                    Value::list([Value::object([
                        ("required", Value::string("required")),
                        ("optional", Value::null(Type::string())),
                    ])]),
                )],
                vec![(
                    "list",
                    defaults(
                        Type::list(Type::object_with_optional_attrs(
                            [("required", Type::string()), ("optional", Type::string())],
                            &["optional"],
                        )),
                        vec![],
                        vec![(
                            "",
                            defaults(
                                Type::object_with_optional_attrs(
                                    [("required", Type::string()), ("optional", Type::string())],
                                    &["optional"],
                                ),
                                vec![("optional", Value::string("optional"))],
                                vec![],
                            ),
                        )],
                    ),
                )],
            )),
            want_error: "",
        },
        // incompatible default value causes an error
        Case {
            source: r#"object({ a = optional(string, "hello"), b = optional(number, true) })"#,
            want: Some(defaults(
                Type::object_with_optional_attrs(
                    [("a", Type::string()), ("b", Type::number())],
                    &["a", "b"],
                ),
                vec![("a", Value::string("hello"))],
                vec![],
            )),
            want_error: "This default value is not compatible with the attribute's type constraint: number required, but have bool.",
        },
        // Too many arguments
        Case {
            source: r#"object({name=string,meta=optional(string, "hello", "world")})"#,
            want: None,
            want_error: "Optional attribute modifier expects at most two arguments: the attribute type, and a default value.",
        },
        // Duplicate arguments.
        Case {
            source: r#"map(object({operations=optional(list(string), []),type=optional(string, "ABC"),type=optional(number)}))"#,
            want: Some(defaults(
                Type::map(Type::object_with_optional_attrs(
                    [
                        ("operations", Type::list(Type::string())),
                        ("type", Type::string()),
                    ],
                    &["operations", "type"],
                )),
                vec![],
                vec![(
                    "",
                    defaults(
                        Type::object_with_optional_attrs(
                            [
                                ("operations", Type::list(Type::string())),
                                ("type", Type::string()),
                            ],
                            &["operations", "type"],
                        ),
                        vec![
                            ("operations", Value::list_empty(Type::string())),
                            ("type", Value::string("ABC")),
                        ],
                        vec![],
                    ),
                )],
            )),
            want_error: "Object constructor map keys must be unique.",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.source.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            !parse_diags.has_errors(),
            "case {i} ({:?}): failed to parse: {:?}",
            test.source,
            *parse_diags,
        );

        let (_, got, diags) = typeexpr::type_constraint_with_defaults(&expr);
        if test.want_error.is_empty() {
            assert!(
                diags.is_empty(),
                "case {i} ({:?}): unexpected diagnostics: {:?}",
                test.source,
                *diags,
            );
        } else {
            let found = diags.iter().any(|diag| {
                diag.severity == DiagnosticSeverity::Error && diag.detail == test.want_error
            });
            assert!(
                found,
                "case {i} ({:?}): missing expected error detail message: {}\ngot diagnostics: {:?}",
                test.source, test.want_error, *diags,
            );
        }

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.source,);
    }
}
