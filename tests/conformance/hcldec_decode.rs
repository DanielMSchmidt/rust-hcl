//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hcldec/public_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::hcldec::{
    self, AttrSpec, BlockAttrsSpec, BlockLabelSpec, BlockListSpec, BlockMapSpec, BlockObjectSpec,
    BlockSetSpec, BlockSpec, BlockTupleSpec, DefaultSpec, LiteralSpec, ObjectSpec, SpecRef,
    TupleSpec,
};
use hcl::hclsyntax;
use hcl::{EvalContext, Pos, Range};

// Ported from TestDecode:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/public_test.go#L16
#[test]
#[ignore = "not yet implemented"]
fn decode() {
    struct Case {
        config: &'static str,
        spec: SpecRef,
        ctx: Option<EvalContext>,
        want: Value,
        diag_count: usize,
    }

    let tests = [
        Case {
            config: "",
            spec: SpecRef::new(ObjectSpec::default()),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 0,
        },
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(ObjectSpec::default()),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 1, // attribute named "a" is not expected here
        },
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(ObjectSpec::from_iter([(
                "a",
                SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::number(),
                    required: false,
                }),
            )])),
            ctx: None,
            want: Value::object([("a", Value::number_int(1))]),
            diag_count: 0,
        },
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::number(),
                required: false,
            }),
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(DefaultSpec {
                primary: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::number(),
                    required: false,
                }),
                default: SpecRef::new(LiteralSpec {
                    value: Value::number_int(10),
                }),
            }),
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            config: "",
            spec: SpecRef::new(DefaultSpec {
                primary: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::number(),
                    required: false,
                }),
                default: SpecRef::new(LiteralSpec {
                    value: Value::number_int(10),
                }),
            }),
            ctx: None,
            want: Value::number_int(10),
            diag_count: 0,
        },
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(ObjectSpec::from_iter([(
                "foo",
                SpecRef::new(DefaultSpec {
                    primary: SpecRef::new(AttrSpec {
                        name: "a".into(),
                        ty: Type::number(),
                        required: false,
                    }),
                    default: SpecRef::new(LiteralSpec {
                        value: Value::number_int(10),
                    }),
                }),
            )])),
            ctx: None,
            want: Value::object([("foo", Value::number_int(1))]),
            diag_count: 0,
        },
        Case {
            config: "a = \"1\"\n",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::number(),
                required: false,
            }),
            ctx: None,
            want: Value::number_int(1),
            diag_count: 0,
        },
        Case {
            config: "a = true\n",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::number(),
                required: false,
            }),
            ctx: None,
            want: Value::unknown(Type::number()),
            diag_count: 1, // incorrect type - number required.
        },
        Case {
            config: "",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::number(),
                required: true,
            }),
            ctx: None,
            want: Value::null(Type::number()),
            diag_count: 1, // attribute "a" is required
        },
        Case {
            config: "",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::object_with_optional_attrs([("attr", Type::string())], &["attr"]),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::object([("attr", Type::string())])),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                required: false,
            }),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 0,
        },
        Case {
            config: "\nb \"baz\" {\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockLabelSpec {
                    index: 0,
                    name: "name".into(),
                }),
                required: false,
            }),
            ctx: None,
            want: Value::string("baz"),
            diag_count: 0,
        },
        Case {
            config: "\nb \"baz\" {}\nb \"foo\" {}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockLabelSpec {
                    index: 0,
                    name: "name".into(),
                }),
                required: false,
            }),
            ctx: None,
            want: Value::string("baz"),
            diag_count: 1, // duplicate "b" block
        },
        Case {
            config: "\nb {\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockLabelSpec {
                    index: 0,
                    name: "name".into(),
                }),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::string()),
            diag_count: 1, // missing name label
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::empty_object()),
            diag_count: 0,
        },
        Case {
            config: "a {}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::empty_object()),
            diag_count: 1, // blocks of type "a" are not supported
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                required: true,
            }),
            ctx: None,
            want: Value::null(Type::empty_object()),
            diag_count: 1, // a block of type "b" is required
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                required: true,
            }),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 1, // only one "b" block is allowed
        },
        Case {
            config: "\nb {\n}\n",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::map_empty(Type::string()),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n  hello = \"world\"\n}\n",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::map([("hello", Value::string("world"))]),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n  hello = true\n}\n",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::map([("hello", Value::string("true"))]),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n  hello   = true\n  goodbye = 5\n}\n",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::map([
                ("hello", Value::string("true")),
                ("goodbye", Value::string("5")),
            ]),
            diag_count: 0,
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::map(Type::string())),
            diag_count: 0,
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: true,
            }),
            ctx: None,
            want: Value::null(Type::map(Type::string())),
            diag_count: 1, // missing b block
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::object_with_optional_attrs(
                    [("attr", Type::string())],
                    &["attr"],
                ),
                required: false,
            }),
            ctx: None,
            want: Value::null(Type::map(Type::object([("attr", Type::string())]))),
            diag_count: 0,
        },
        Case {
            // NOTE(port): the trailing tabs are part of the Go raw string
            // literal (the closing backtick is indented in the upstream
            // source).
            config: "\nb {\n}\nb {\n}\n\t\t\t",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: false,
            }),
            ctx: None,
            want: Value::map_empty(Type::string()),
            diag_count: 1, // duplicate b block
        },
        Case {
            // NOTE(port): the trailing tabs are part of the Go raw string
            // literal (the closing backtick is indented in the upstream
            // source).
            config: "\nb {\n}\nb {\n}\n\t\t\t",
            spec: SpecRef::new(BlockAttrsSpec {
                type_name: "b".into(),
                element_type: Type::string(),
                required: true,
            }),
            ctx: None,
            want: Value::map_empty(Type::string()),
            diag_count: 1, // duplicate b block
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::list([Value::empty_object(), Value::empty_object()]),
            diag_count: 0,
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::list_empty(Type::empty_object()),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::list([Value::string("foo"), Value::string("bar")]),
            diag_count: 0,
        },
        Case {
            config: "\nb {}\nb {}\nb {}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 2,
            }),
            ctx: None,
            want: Value::list([
                Value::empty_object(),
                Value::empty_object(),
                Value::empty_object(),
            ]),
            diag_count: 1, // too many b blocks
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 10,
                max_items: 0,
            }),
            ctx: None,
            want: Value::list([Value::empty_object(), Value::empty_object()]),
            diag_count: 1, // insufficient b blocks
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = 1\n}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // Unconsistent argument types in b blocks
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = \"not a bool\"\n}\n",
            spec: SpecRef::new(BlockListSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::list([
                Value::string("true"), // type unification generalizes all the values to strings
                Value::string("not a bool"),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockSetSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 2,
            }),
            ctx: None,
            want: Value::set([Value::empty_object(), Value::empty_object()]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"baz\" {}\n",
            spec: SpecRef::new(BlockSetSpec {
                type_name: "b".into(),
                nested: SpecRef::new(TupleSpec(vec![
                    SpecRef::new(BlockLabelSpec {
                        name: "name".into(),
                        index: 1,
                    }),
                    SpecRef::new(BlockLabelSpec {
                        name: "type".into(),
                        index: 0,
                    }),
                ])),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::set([
                Value::tuple([Value::string("bar"), Value::string("foo")]),
                Value::tuple([Value::string("baz"), Value::string("bar")]),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = 1\n}\n",
            spec: SpecRef::new(BlockSetSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // Unconsistent argument types in b blocks
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = \"not a bool\"\n}\n",
            spec: SpecRef::new(BlockSetSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::set([
                Value::string("true"), // type unification generalizes all the values to strings
                Value::string("not a bool"),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([
                ("foo", Value::empty_object()),
                ("bar", Value::empty_object()),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"baz\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([
                ("foo", Value::map([("bar", Value::empty_object())])),
                ("bar", Value::map([("baz", Value::empty_object())])),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"bar\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([
                ("foo", Value::map([("bar", Value::empty_object())])),
                ("bar", Value::map([("bar", Value::empty_object())])),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"foo\" \"baz\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([(
                "foo",
                Value::map([
                    ("bar", Value::empty_object()),
                    ("baz", Value::empty_object()),
                ]),
            )]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map_empty(Type::empty_object()),
            diag_count: 1, // too many labels
        },
        Case {
            config: "\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map_empty(Type::empty_object()),
            diag_count: 1, // not enough labels
        },
        Case {
            config: "\nb \"foo\" {}\nb \"foo\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([("foo", Value::empty_object())]),
            diag_count: 1, // duplicate b block
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"foo\" \"bar\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::map([("foo", Value::map([("bar", Value::empty_object())]))]),
            diag_count: 1, // duplicate b block
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"baz\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["type".into()],
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
            }),
            ctx: None,
            want: Value::map([("foo", Value::string("bar")), ("bar", Value::string("baz"))]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\n",
            spec: SpecRef::new(BlockMapSpec {
                type_name: "b".into(),
                label_names: vec!["type".into()],
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
            }),
            ctx: None,
            want: Value::map_empty(Type::string()),
            diag_count: 1, // missing name
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::tuple([Value::empty_object(), Value::empty_object()]),
            diag_count: 0,
        },
        Case {
            config: "",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::empty_tuple(),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::tuple([Value::string("foo"), Value::string("bar")]),
            diag_count: 0,
        },
        Case {
            config: "\nb {}\nb {}\nb {}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 0,
                max_items: 2,
            }),
            ctx: None,
            want: Value::tuple([
                Value::empty_object(),
                Value::empty_object(),
                Value::empty_object(),
            ]),
            diag_count: 1, // too many b blocks
        },
        Case {
            config: "\nb {}\nb {}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(ObjectSpec::default()),
                min_items: 10,
                max_items: 0,
            }),
            ctx: None,
            want: Value::tuple([Value::empty_object(), Value::empty_object()]),
            diag_count: 1, // insufficient b blocks
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = 1\n}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::tuple([Value::bool(true), Value::number_int(1)]),
            diag_count: 0,
        },
        Case {
            config: "\nb {\n\ta = true\n}\nb {\n\ta = \"not a bool\"\n}\n",
            spec: SpecRef::new(BlockTupleSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                min_items: 0,
                max_items: 0,
            }),
            ctx: None,
            want: Value::tuple([Value::bool(true), Value::string("not a bool")]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([
                ("foo", Value::empty_object()),
                ("bar", Value::empty_object()),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"baz\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([
                ("foo", Value::object([("bar", Value::empty_object())])),
                ("bar", Value::object([("baz", Value::empty_object())])),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"bar\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([
                ("foo", Value::object([("bar", Value::empty_object())])),
                ("bar", Value::object([("bar", Value::empty_object())])),
            ]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"foo\" \"baz\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([(
                "foo",
                Value::object([
                    ("bar", Value::empty_object()),
                    ("baz", Value::empty_object()),
                ]),
            )]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 1, // too many labels
        },
        Case {
            config: "\nb \"bar\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 1, // not enough labels
        },
        Case {
            config: "\nb \"foo\" {}\nb \"foo\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([("foo", Value::empty_object())]),
            diag_count: 1, // duplicate b block
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"foo\" \"bar\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["key1".into(), "key2".into()],
                nested: SpecRef::new(ObjectSpec::default()),
            }),
            ctx: None,
            want: Value::object([("foo", Value::object([("bar", Value::empty_object())]))]),
            diag_count: 1, // duplicate b block
        },
        Case {
            config: "\nb \"foo\" \"bar\" {}\nb \"bar\" \"baz\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["type".into()],
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
            }),
            ctx: None,
            want: Value::object([("foo", Value::string("bar")), ("bar", Value::string("baz"))]),
            diag_count: 0,
        },
        Case {
            config: "\nb \"foo\" {}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["type".into()],
                nested: SpecRef::new(BlockLabelSpec {
                    name: "name".into(),
                    index: 0,
                }),
            }),
            ctx: None,
            want: Value::empty_object(),
            diag_count: 1, // missing name
        },
        Case {
            config: "\nb \"foo\" {\n\targ = true\n}\nb \"bar\" {\n\targ = 1\n}\n",
            spec: SpecRef::new(BlockObjectSpec {
                type_name: "b".into(),
                label_names: vec!["type".into()],
                nested: SpecRef::new(AttrSpec {
                    name: "arg".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
            }),
            ctx: None,
            want: Value::object([("foo", Value::bool(true)), ("bar", Value::number_int(1))]),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, parse_diags) = hclsyntax::parse_config(
            test.config.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        let body = &file.body;
        let (got, val_diags) = hcldec::decode(&**body, &*test.spec, test.ctx.as_ref());

        let mut diags = parse_diags;
        diags.extend(val_diags);

        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i} ({:?}): wrong number of diagnostics {}; want {}\n{diags}",
            test.config,
            diags.len(),
            test.diag_count,
        );

        assert!(
            got.raw_equals(&test.want),
            "case {i} ({:?}): wrong result\ngot:  {got:?}\nwant: {:?}",
            test.config,
            test.want,
        );
    }
}

// Ported from TestSourceRange:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hcldec/public_test.go#L1077
#[test]
#[ignore = "not yet implemented"]
fn source_range() {
    // NOTE(port): upstream leaves `AttrSpec.Type` as its zero value
    // (`cty.NilType`), which has no Rust analogue; `SourceRange` never
    // consults the type, so `Type::dynamic()` stands in for it here.
    struct Case {
        config: &'static str,
        spec: SpecRef,
        want: Range,
    }

    let tests = [
        Case {
            config: "a = 1\n",
            spec: SpecRef::new(AttrSpec {
                name: "a".into(),
                ty: Type::dynamic(),
                required: false,
            }),
            want: Range {
                filename: String::new(),
                start: Pos {
                    line: 1,
                    column: 5,
                    byte: 4,
                },
                end: Pos {
                    line: 1,
                    column: 6,
                    byte: 5,
                },
            },
        },
        Case {
            config: "\nb {\n  a = 1\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(AttrSpec {
                    name: "a".into(),
                    ty: Type::dynamic(),
                    required: false,
                }),
                required: false,
            }),
            want: Range {
                filename: String::new(),
                start: Pos {
                    line: 3,
                    column: 7,
                    byte: 11,
                },
                end: Pos {
                    line: 3,
                    column: 8,
                    byte: 12,
                },
            },
        },
        Case {
            config: "\nb {\n  c {\n    a = 1\n  }\n}\n",
            spec: SpecRef::new(BlockSpec {
                type_name: "b".into(),
                nested: SpecRef::new(BlockSpec {
                    type_name: "c".into(),
                    nested: SpecRef::new(AttrSpec {
                        name: "a".into(),
                        ty: Type::dynamic(),
                        required: false,
                    }),
                    required: false,
                }),
                required: false,
            }),
            want: Range {
                filename: String::new(),
                start: Pos {
                    line: 4,
                    column: 9,
                    byte: 19,
                },
                end: Pos {
                    line: 4,
                    column: 10,
                    byte: 20,
                },
            },
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
            "case {i} ({:?}): wrong number of diagnostics {}; want 0\n{diags}",
            test.config,
            diags.len(),
        );
        let body = &file.body;

        let got = hcldec::source_range(&**body, &*test.spec);

        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.config,);
    }
}
