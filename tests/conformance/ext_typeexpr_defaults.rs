//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ext/typeexpr/defaults_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;

use cty::{Type, Value};
use hcl::ext::typeexpr::Defaults;

// NOTE(port): upstream's `valueComparer` (a go-cmp comparer over
// `cty.Value.RawEquals`) needs no analogue: rust-cty's `Value: PartialEq`
// is `RawEquals`, so plain `assert_eq!` matches the upstream comparison.

// Ported from TestDefaults_Apply:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/defaults_test.go#L17
#[test]
#[ignore = "not yet implemented"]
fn defaults_apply() {
    let simple_object =
        Type::object_with_optional_attrs([("a", Type::string()), ("b", Type::bool())], &["b"]);
    let nested_object = Type::object_with_optional_attrs(
        [("c", simple_object.clone()), ("d", Type::number())],
        &["c"],
    );

    struct Case {
        name: &'static str,
        defaults: Defaults,
        value: Value,
        want: Value,
    }

    // NOTE(port): upstream keys the cases by name in a Go map (unordered
    // iteration); here they are an array in upstream source order, with the
    // upstream name carried in each case.
    let test_cases = [
        // Nothing happens when there are no default values and no children.
        Case {
            name: "no defaults",
            defaults: Defaults {
                ty: Type::map(Type::string()),
                default_values: HashMap::new(),
                children: HashMap::new(),
            },
            value: Value::map([("a", Value::string("foo")), ("b", Value::string("bar"))]),
            want: Value::map([("a", Value::string("foo")), ("b", Value::string("bar"))]),
        },
        // Passing a map which does not include one of the attributes with a
        // default results in the default being applied to the output. Output
        // is always an object.
        Case {
            name: "simple object with defaults applied",
            defaults: Defaults {
                ty: simple_object.clone(),
                default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                children: HashMap::new(),
            },
            value: Value::map([("a", Value::string("foo"))]),
            want: Value::map([("a", Value::string("foo")), ("b", Value::string("true"))]),
        },
        // Unknown values may be assigned to root modules during validation,
        // and we cannot apply defaults at that time.
        Case {
            name: "simple object with defaults but unknown value",
            defaults: Defaults {
                ty: simple_object.clone(),
                default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                children: HashMap::new(),
            },
            value: Value::unknown(Type::map(Type::string())),
            want: Value::unknown(Type::map(Type::string())),
        },
        // Defaults do not override attributes which are present in the given
        // value.
        Case {
            name: "simple object with optional attributes specified",
            defaults: Defaults {
                ty: simple_object.clone(),
                default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                children: HashMap::new(),
            },
            value: Value::map([("a", Value::string("foo")), ("b", Value::string("false"))]),
            want: Value::map([("a", Value::string("foo")), ("b", Value::string("false"))]),
        },
        // Defaults will replace explicit nulls.
        Case {
            name: "object with explicit null for attribute with default",
            defaults: Defaults {
                ty: simple_object.clone(),
                default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                children: HashMap::new(),
            },
            value: Value::map([
                ("a", Value::string("foo")),
                ("b", Value::null(Type::string())),
            ]),
            want: Value::map([("a", Value::string("foo")), ("b", Value::string("true"))]),
        },
        // Defaults can be specified at any level of depth and will be applied
        // so long as there is a parent value to populate.
        Case {
            name: "nested object with defaults applied",
            defaults: Defaults {
                ty: nested_object.clone(),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "c".to_string(),
                    Defaults {
                        ty: simple_object.clone(),
                        default_values: HashMap::from([("b".to_string(), Value::bool(false))]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::object([
                ("c", Value::object([("a", Value::string("foo"))])),
                ("d", Value::number_int(5)),
            ]),
            want: Value::object([
                (
                    "c",
                    Value::object([("a", Value::string("foo")), ("b", Value::bool(false))]),
                ),
                ("d", Value::number_int(5)),
            ]),
        },
        // Testing traversal of collections.
        Case {
            name: "map of objects with defaults applied",
            defaults: Defaults {
                ty: Type::map(simple_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: simple_object.clone(),
                        default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::map([
                ("f", Value::object([("a", Value::string("foo"))])),
                ("b", Value::object([("a", Value::string("bar"))])),
            ]),
            want: Value::map([
                (
                    "f",
                    Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                ),
                (
                    "b",
                    Value::object([("a", Value::string("bar")), ("b", Value::bool(true))]),
                ),
            ]),
        },
        // A map variable value specified in a tfvars file will be an object,
        // in which case we must still traverse the defaults structure
        // correctly.
        Case {
            name: "map of objects with defaults applied, given object instead of map",
            defaults: Defaults {
                ty: Type::map(simple_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: simple_object.clone(),
                        default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::object([
                ("f", Value::object([("a", Value::string("foo"))])),
                ("b", Value::object([("a", Value::string("bar"))])),
            ]),
            want: Value::object([
                (
                    "f",
                    Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                ),
                (
                    "b",
                    Value::object([("a", Value::string("bar")), ("b", Value::bool(true))]),
                ),
            ]),
        },
        // Another example of a collection type, this time exercising the code
        // processing a tuple input.
        Case {
            name: "list of objects with defaults applied",
            defaults: Defaults {
                ty: Type::list(simple_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: simple_object.clone(),
                        default_values: HashMap::from([("b".to_string(), Value::bool(true))]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([("a", Value::string("foo"))]),
                Value::object([("a", Value::string("bar"))]),
            ]),
            want: Value::tuple([
                Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                Value::object([("a", Value::string("bar")), ("b", Value::bool(true))]),
            ]),
        },
        // Unlike collections, tuple variable types can have defaults for
        // multiple element types.
        Case {
            name: "tuple of objects with defaults applied",
            defaults: Defaults {
                ty: Type::tuple([simple_object.clone(), nested_object.clone()]),
                default_values: HashMap::new(),
                children: HashMap::from([
                    (
                        "0".to_string(),
                        Defaults {
                            ty: simple_object.clone(),
                            default_values: HashMap::from([("b".to_string(), Value::bool(false))]),
                            children: HashMap::new(),
                        },
                    ),
                    (
                        "1".to_string(),
                        Defaults {
                            ty: nested_object.clone(),
                            default_values: HashMap::from([(
                                "c".to_string(),
                                Value::object([
                                    ("a", Value::string("default")),
                                    ("b", Value::bool(true)),
                                ]),
                            )]),
                            children: HashMap::new(),
                        },
                    ),
                ]),
            },
            value: Value::tuple([
                Value::object([("a", Value::string("foo"))]),
                Value::object([("d", Value::number_int(5))]),
            ]),
            want: Value::tuple([
                Value::object([("a", Value::string("foo")), ("b", Value::bool(false))]),
                Value::object([
                    (
                        "c",
                        Value::object([("a", Value::string("default")), ("b", Value::bool(true))]),
                    ),
                    ("d", Value::number_int(5)),
                ]),
            ]),
        },
        // More complex cases with deeply nested defaults, testing the "default
        // within a default" edges.
        Case {
            name: "set of nested objects, no default sub-object",
            defaults: Defaults {
                ty: Type::set(nested_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: nested_object.clone(),
                        default_values: HashMap::new(),
                        children: HashMap::from([(
                            "c".to_string(),
                            Defaults {
                                ty: simple_object.clone(),
                                default_values: HashMap::from([(
                                    "b".to_string(),
                                    Value::bool(true),
                                )]),
                                children: HashMap::new(),
                            },
                        )]),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([
                    ("c", Value::object([("a", Value::string("foo"))])),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([("d", Value::number_int(7))]),
            ]),
            want: Value::tuple([
                Value::object([
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([
                    // No default value for "c" specified, so none applied. The
                    // convert stage will fill in a null.
                    ("d", Value::number_int(7)),
                ]),
            ]),
        },
        Case {
            name: "set of nested objects, empty default sub-object",
            defaults: Defaults {
                ty: Type::set(nested_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: nested_object.clone(),
                        default_values: HashMap::from([(
                            // This is a convenient shorthand which causes a
                            // missing sub-object to be filled with an object
                            // with all of the default values specified in the
                            // sub-object's type.
                            "c".to_string(),
                            Value::empty_object(),
                        )]),
                        children: HashMap::from([(
                            "c".to_string(),
                            Defaults {
                                ty: simple_object.clone(),
                                default_values: HashMap::from([(
                                    "b".to_string(),
                                    Value::bool(true),
                                )]),
                                children: HashMap::new(),
                            },
                        )]),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([
                    ("c", Value::object([("a", Value::string("foo"))])),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([("d", Value::number_int(7))]),
            ]),
            want: Value::tuple([
                Value::object([
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([
                    (
                        "c",
                        Value::object([
                            // Default value for "b" is applied to the empty object
                            // specified as the default for "c"
                            ("b", Value::bool(true)),
                        ]),
                    ),
                    ("d", Value::number_int(7)),
                ]),
            ]),
        },
        Case {
            name: "set of nested objects, overriding default sub-object",
            defaults: Defaults {
                ty: Type::set(nested_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: nested_object.clone(),
                        default_values: HashMap::from([(
                            // If no value is given for "c", we use this object
                            // of non-default values instead. These take
                            // precedence over the default values specified in
                            // the child type.
                            "c".to_string(),
                            Value::object([
                                ("a", Value::string("fallback")),
                                ("b", Value::bool(false)),
                            ]),
                        )]),
                        children: HashMap::from([(
                            "c".to_string(),
                            Defaults {
                                ty: simple_object.clone(),
                                default_values: HashMap::from([(
                                    "b".to_string(),
                                    Value::bool(true),
                                )]),
                                children: HashMap::new(),
                            },
                        )]),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([
                    ("c", Value::object([("a", Value::string("foo"))])),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([("d", Value::number_int(7))]),
            ]),
            want: Value::tuple([
                Value::object([
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([
                    (
                        "c",
                        Value::object([
                            // The default value for "b" is not applied, as the
                            // default value for "c" includes a non-default value
                            // already.
                            ("a", Value::string("fallback")),
                            ("b", Value::bool(false)),
                        ]),
                    ),
                    ("d", Value::number_int(7)),
                ]),
            ]),
        },
        Case {
            name: "set of nested objects, nulls in default sub-object overridden",
            defaults: Defaults {
                ty: Type::set(nested_object.clone()),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: nested_object.clone(),
                        default_values: HashMap::from([(
                            // The default value for "c" is used to prepopulate
                            // the nested object's value if not specified, but
                            // the null default for its "b" attribute will be
                            // overridden by the default specified in the child
                            // type.
                            "c".to_string(),
                            Value::object([
                                ("a", Value::string("fallback")),
                                ("b", Value::null(Type::bool())),
                            ]),
                        )]),
                        children: HashMap::from([(
                            "c".to_string(),
                            Defaults {
                                ty: simple_object.clone(),
                                default_values: HashMap::from([(
                                    "b".to_string(),
                                    Value::bool(true),
                                )]),
                                children: HashMap::new(),
                            },
                        )]),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([
                    ("c", Value::object([("a", Value::string("foo"))])),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([("d", Value::number_int(7))]),
            ]),
            want: Value::tuple([
                Value::object([
                    (
                        "c",
                        Value::object([("a", Value::string("foo")), ("b", Value::bool(true))]),
                    ),
                    ("d", Value::number_int(5)),
                ]),
                Value::object([
                    (
                        "c",
                        Value::object([
                            // The default value for "b" overrides the explicit
                            // null in the default value for "c".
                            ("a", Value::string("fallback")),
                            ("b", Value::bool(true)),
                        ]),
                    ),
                    ("d", Value::number_int(7)),
                ]),
            ]),
        },
        Case {
            name: "null objects do not get default values inserted",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs(
                    [("required", Type::string()), ("optional", Type::string())],
                    &["optional"],
                ),
                default_values: HashMap::from([(
                    "optional".to_string(),
                    Value::string("optional"),
                )]),
                children: HashMap::new(),
            },
            value: Value::null(Type::object([
                ("required", Type::string()),
                ("optional", Type::string()),
            ])),
            want: Value::null(Type::object([
                ("required", Type::string()),
                ("optional", Type::string()),
            ])),
        },
        Case {
            name: "defaults with unset defaults are still applied (null)",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs(
                    [
                        ("required", Type::string()),
                        (
                            "optional_object",
                            Type::object_with_optional_attrs(
                                [
                                    ("nested_required", Type::string()),
                                    ("nested_optional", Type::string()),
                                ],
                                &["nested_optional"],
                            ),
                        ),
                    ],
                    &["optional_object"],
                ),
                default_values: HashMap::from([(
                    "optional_object".to_string(),
                    Value::object([
                        ("nested_required", Value::string("required")),
                        ("nested_optional", Value::null(Type::string())),
                    ]),
                )]),
                children: HashMap::from([(
                    "optional_object".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("nested_required", Type::string()),
                                ("nested_optional", Type::string()),
                            ],
                            &["nested_optional"],
                        ),
                        default_values: HashMap::from([(
                            "nested_optional".to_string(),
                            Value::string("optional"),
                        )]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::object([
                ("required", Value::string("required")),
                // optional_object is explicitly set to null for this test case.
                (
                    "optional_object",
                    Value::null(Type::object([
                        ("nested_required", Type::string()),
                        ("nested_optional", Type::string()),
                    ])),
                ),
            ]),
            want: Value::object([
                ("required", Value::string("required")),
                (
                    "optional_object",
                    Value::object([
                        ("nested_required", Value::string("required")),
                        ("nested_optional", Value::string("optional")),
                    ]),
                ),
            ]),
        },
        Case {
            name: "defaults with unset defaults are still applied (missing)",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs(
                    [
                        ("required", Type::string()),
                        (
                            "optional_object",
                            Type::object_with_optional_attrs(
                                [
                                    ("nested_required", Type::string()),
                                    ("nested_optional", Type::string()),
                                ],
                                &["nested_optional"],
                            ),
                        ),
                    ],
                    &["optional_object"],
                ),
                default_values: HashMap::from([(
                    "optional_object".to_string(),
                    Value::object([
                        ("nested_required", Value::string("required")),
                        ("nested_optional", Value::null(Type::string())),
                    ]),
                )]),
                children: HashMap::from([(
                    "optional_object".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("nested_required", Type::string()),
                                ("nested_optional", Type::string()),
                            ],
                            &["nested_optional"],
                        ),
                        default_values: HashMap::from([(
                            "nested_optional".to_string(),
                            Value::string("optional"),
                        )]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::object([
                ("required", Value::string("required")),
                // optional_object is missing but not null for this test case.
            ]),
            want: Value::object([
                ("required", Value::string("required")),
                (
                    "optional_object",
                    Value::object([
                        ("nested_required", Value::string("required")),
                        ("nested_optional", Value::string("optional")),
                    ]),
                ),
            ]),
        },
        // https://discuss.hashicorp.com/t/request-for-feedback-optional-object-type-attributes-with-defaults-in-v1-3-alpha/40550/6?u=alisdair
        Case {
            name: "all child and nested values are optional with defaults",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs(
                    [(
                        "settings",
                        Type::object_with_optional_attrs(
                            [
                                ("setting_one", Type::string()),
                                ("setting_two", Type::number()),
                            ],
                            &["setting_one", "setting_two"],
                        ),
                    )],
                    &["settings"],
                ),
                default_values: HashMap::from([("settings".to_string(), Value::empty_object())]),
                children: HashMap::from([(
                    "settings".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("setting_one", Type::string()),
                                ("setting_two", Type::string()),
                            ],
                            &["setting_one", "setting_two"],
                        ),
                        default_values: HashMap::from([
                            ("setting_one".to_string(), Value::string("")),
                            ("setting_two".to_string(), Value::number_int(0)),
                        ]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::empty_object(),
            want: Value::object([(
                "settings",
                Value::object([
                    ("setting_one", Value::string("")),
                    ("setting_two", Value::number_int(0)),
                ]),
            )]),
        },
        Case {
            name: "all nested values are optional with defaults, but direct child has no default",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs(
                    [(
                        "settings",
                        Type::object_with_optional_attrs(
                            [
                                ("setting_one", Type::string()),
                                ("setting_two", Type::number()),
                            ],
                            &["setting_one", "setting_two"],
                        ),
                    )],
                    &["settings"],
                ),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "settings".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("setting_one", Type::string()),
                                ("setting_two", Type::string()),
                            ],
                            &["setting_one", "setting_two"],
                        ),
                        default_values: HashMap::from([
                            ("setting_one".to_string(), Value::string("")),
                            ("setting_two".to_string(), Value::number_int(0)),
                        ]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::empty_object(),
            want: Value::empty_object(),
        },
        Case {
            name: "tuples retain custom values and dynamic types",
            defaults: Defaults {
                ty: Type::list(Type::object_with_optional_attrs(
                    [
                        ("name", Type::string()),
                        ("taints", Type::list(Type::map(Type::dynamic()))),
                    ],
                    &["name", "taints"],
                )),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("name", Type::string()),
                                ("taints", Type::list(Type::map(Type::dynamic()))),
                            ],
                            &["name", "taints"],
                        ),
                        default_values: HashMap::from([
                            ("name".to_string(), Value::string("default")),
                            (
                                "taints".to_string(),
                                Value::list_empty(Type::map(Type::dynamic())),
                            ),
                        ]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::tuple([
                Value::object([("name", Value::string("node-pool-32"))]),
                Value::object([
                    ("name", Value::string("node-envoy-32")),
                    (
                        "taints",
                        Value::list([Value::map([
                            ("key", Value::string("etsy.com/nodepool")),
                            ("value", Value::string("envoy")),
                        ])]),
                    ),
                ]),
            ]),
            want: Value::tuple([
                Value::object([
                    ("name", Value::string("node-pool-32")),
                    ("taints", Value::list_empty(Type::map(Type::dynamic()))),
                ]),
                Value::object([
                    ("name", Value::string("node-envoy-32")),
                    (
                        "taints",
                        Value::list([Value::map([
                            ("key", Value::string("etsy.com/nodepool")),
                            ("value", Value::string("envoy")),
                        ])]),
                    ),
                ]),
            ]),
        },
        Case {
            name: "lists merge dynamic types with concrete types",
            defaults: Defaults {
                ty: Type::list(Type::object_with_optional_attrs(
                    [
                        ("name", Type::string()),
                        ("taints", Type::list(Type::map(Type::dynamic()))),
                    ],
                    &["name", "taints"],
                )),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("name", Type::string()),
                                ("taints", Type::list(Type::map(Type::dynamic()))),
                            ],
                            &["name", "taints"],
                        ),
                        default_values: HashMap::from([
                            ("name".to_string(), Value::string("default")),
                            (
                                "taints".to_string(),
                                Value::list_empty(Type::map(Type::dynamic())),
                            ),
                        ]),
                        children: HashMap::new(),
                    },
                )]),
            },
            value: Value::list([
                Value::object([
                    ("name", Value::string("node-pool-32")),
                    ("taints", Value::null(Type::list(Type::map(Type::string())))),
                ]),
                Value::object([
                    ("name", Value::string("node-envoy-32")),
                    (
                        "taints",
                        Value::list([Value::map([
                            ("key", Value::string("etsy.com/nodepool")),
                            ("value", Value::string("envoy")),
                        ])]),
                    ),
                ]),
            ]),
            want: Value::list([
                Value::object([
                    ("name", Value::string("node-pool-32")),
                    ("taints", Value::list_empty(Type::map(Type::string()))),
                ]),
                Value::object([
                    ("name", Value::string("node-envoy-32")),
                    (
                        "taints",
                        Value::list([Value::map([
                            ("key", Value::string("etsy.com/nodepool")),
                            ("value", Value::string("envoy")),
                        ])]),
                    ),
                ]),
            ]),
        },
        Case {
            name: "applies default safely where possible when types mismatch",
            defaults: Defaults {
                ty: Type::map(Type::object_with_optional_attrs(
                    [
                        ("description", Type::string()),
                        (
                            "rules",
                            Type::map(Type::object_with_optional_attrs(
                                [
                                    ("description", Type::string()),
                                    ("destination_ports", Type::list(Type::string())),
                                    ("destination_addresses", Type::list(Type::string())),
                                    ("translated_address", Type::string()),
                                    ("translated_port", Type::string()),
                                ],
                                &["destination_addresses"],
                            )),
                        ),
                    ],
                    &["description"],
                )),
                default_values: HashMap::new(),
                children: HashMap::from([(
                    "".to_string(),
                    Defaults {
                        ty: Type::object_with_optional_attrs(
                            [
                                ("description", Type::string()),
                                (
                                    "rules",
                                    Type::map(Type::object_with_optional_attrs(
                                        [
                                            ("description", Type::string()),
                                            ("destination_ports", Type::list(Type::string())),
                                            ("destination_addresses", Type::list(Type::string())),
                                            ("translated_address", Type::string()),
                                            ("translated_port", Type::string()),
                                        ],
                                        &["destination_addresses"],
                                    )),
                                ),
                            ],
                            &["description"],
                        ),
                        default_values: HashMap::from([(
                            "description".to_string(),
                            Value::string("unknown"),
                        )]),
                        children: HashMap::from([(
                            "rules".to_string(),
                            Defaults {
                                ty: Type::map(Type::object_with_optional_attrs(
                                    [
                                        ("description", Type::string()),
                                        ("destination_ports", Type::list(Type::string())),
                                        ("destination_addresses", Type::list(Type::string())),
                                        ("translated_address", Type::string()),
                                        ("translated_port", Type::string()),
                                    ],
                                    &["destination_addresses"],
                                )),
                                default_values: HashMap::new(),
                                children: HashMap::from([(
                                    "".to_string(),
                                    Defaults {
                                        ty: Type::object_with_optional_attrs(
                                            [
                                                ("description", Type::string()),
                                                ("destination_ports", Type::list(Type::string())),
                                                (
                                                    "destination_addresses",
                                                    Type::list(Type::string()),
                                                ),
                                                ("translated_address", Type::string()),
                                                ("translated_port", Type::string()),
                                            ],
                                            &["destination_addresses"],
                                        ),
                                        default_values: HashMap::from([(
                                            "destination_addresses".to_string(),
                                            Value::list_empty(Type::string()),
                                        )]),
                                        children: HashMap::new(),
                                    },
                                )]),
                            },
                        )]),
                    },
                )]),
            },
            value: Value::map([(
                "mysql",
                Value::object([(
                    "rules",
                    Value::object([
                        ("description", Value::string("Port forward")),
                        ("destination_ports", Value::list([Value::string("3306")])),
                        (
                            "destination_addresses",
                            Value::list([Value::string("192.168.0.1")]),
                        ),
                        ("translated_address", Value::string("192.168.0.1")),
                        ("translated_port", Value::string("3306")),
                    ]),
                )]),
            )]),
            want: Value::map([(
                "mysql",
                Value::object([
                    ("description", Value::string("unknown")),
                    (
                        "rules",
                        Value::object([
                            ("description", Value::string("Port forward")),
                            ("destination_ports", Value::list([Value::string("3306")])),
                            (
                                "destination_addresses",
                                Value::list([Value::string("192.168.0.1")]),
                            ),
                            ("translated_address", Value::string("192.168.0.1")),
                            ("translated_port", Value::string("3306")),
                        ]),
                    ),
                ]),
            )]),
        },
        Case {
            name: "optional attribute with a default can never be null",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs([("foo", Type::string())], &["foo"]),
                default_values: HashMap::from([(
                    "foo".to_string(),
                    Value::string("bar"), // Important: default is non-null
                )]),
                children: HashMap::new(),
            },
            value: Value::object([
                // could potentially be null once known
                ("foo", Value::unknown(Type::string())),
            ]),
            want: Value::object([
                // Because the default isn't null we can guarantee that the
                // result cannot be null even if the given value turns out to be.
                ("foo", Value::unknown(Type::string()).refine_not_null()),
            ]),
        },
        Case {
            name: "optional attribute with a null default could be null",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs([("foo", Type::string())], &["foo"]),
                default_values: HashMap::from([(
                    "foo".to_string(),
                    Value::null(Type::string()), // Important: default is null
                )]),
                children: HashMap::new(),
            },
            value: Value::object([
                // could potentially be null once known
                ("foo", Value::unknown(Type::string())),
            ]),
            want: Value::object([
                // The default value is itself null, so this result is nullable.
                ("foo", Value::unknown(Type::string())),
            ]),
        },
        Case {
            name: "optional attribute with no default could be null",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs([("foo", Type::string())], &["foo"]),
                default_values: HashMap::new(),
                children: HashMap::new(),
            },
            value: Value::object([
                // could potentially be null once known
                ("foo", Value::unknown(Type::string())),
            ]),
            want: Value::object([
                // The default value is itself null, so this result is nullable.
                ("foo", Value::unknown(Type::string())),
            ]),
        },
        Case {
            name: "optional attribute with non-null unknown value cannot be null",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs([("foo", Type::string())], &["foo"]),
                default_values: HashMap::from([(
                    "foo".to_string(),
                    Value::null(Type::string()), // Important: default is null
                )]),
                children: HashMap::new(),
            },
            value: Value::object([("foo", Value::unknown(Type::string()).refine_not_null())]),
            want: Value::object([
                // If the input is guaranteed not null then the default
                // value can't possibly be selected, and so the result can
                // also not be null.
                ("foo", Value::unknown(Type::string()).refine_not_null()),
            ]),
        },
        Case {
            name: "optional attribute with dynamic value can be null",
            defaults: Defaults {
                ty: Type::object_with_optional_attrs([("foo", Type::string())], &["foo"]),
                default_values: HashMap::from([(
                    "foo".to_string(),
                    Value::string("bar"), // Important: default is non-null
                )]),
                children: HashMap::new(),
            },
            value: Value::object([("foo", Value::dynamic())]),
            want: Value::object([
                // The default value is itself non-null, but dynamic value cannot be refined.
                ("foo", Value::dynamic()),
            ]),
        },
    ];

    for tc in &test_cases {
        let got = tc.defaults.apply(&tc.value);
        assert_eq!(got, tc.want, "{}: wrong result", tc.name);
    }
}
