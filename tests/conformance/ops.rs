//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ops_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Path, Type, Value};
use hcl::{apply_path, index};

// Ported from TestApplyPath:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ops_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn apply_path_cases() {
    struct Case {
        start: Value,
        path: Path,
        // NOTE(port): Go's `cty.NilVal` (the Value zero value, used when
        // only an error is expected) becomes `None`; it is never compared
        // against the result because those cases return early on the
        // error assertion, exactly as upstream does.
        want: Option<Value>,
        want_err: &'static str,
    }

    let tests = [
        Case {
            start: Value::string("hello"),
            // NOTE(port): a nil `cty.Path` behaves as an empty path;
            // `Path::new()` is the Rust equivalent.
            path: Path::new(),
            want: Some(Value::string("hello")),
            want_err: "",
        },
        Case {
            start: Value::string("hello"),
            path: Path::new().index(Value::string("boop")),
            want: None,
            want_err: "Invalid index: This value does not have any indices.",
        },
        Case {
            start: Value::string("hello"),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: "Invalid index: This value does not have any indices.",
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello")),
            want_err: "",
        },
        Case {
            start: Value::list([Value::string("hello")]).mark("x"),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello").mark("x")),
            want_err: "",
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("hello")),
            want_err: "",
        },
        Case {
            start: Value::map([
                ("a", Value::string("foo").mark("x")),
                ("b", Value::string("bar").mark("x")),
            ])
            .mark("x"),
            path: Path::new().attr("a"),
            want: Some(Value::string("foo").mark("x")),
            want_err: "",
        },
        Case {
            start: Value::list_empty(Type::string()),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: the collection has no elements.",
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: the given index is greater than or equal to the length of the collection.",
        },
        Case {
            // prevents us from making statements about the length of the list
            start: Value::list([Value::string("hello")]).mark("boop"),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value.",
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(-1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: a negative number is not a valid index for a sequence.",
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_float(0.5)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: indexing a sequence requires a whole number, but the given index has a fractional part.",
        },
        Case {
            start: Value::list([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: "Unsupported attribute: Can't access attributes on a primitive-typed value (string).",
        },
        Case {
            start: Value::list([Value::empty_object()]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: "Unsupported attribute: This object does not have an attribute named \"foo\".",
        },
        Case {
            start: Value::list([Value::empty_object()]),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Unsupported attribute: Can't access attributes on a list of objects. Did you mean to access an attribute for a specific element of the list, or across all elements of the list?",
        },
        Case {
            start: Value::list([Value::object([("foo", Value::bool(true))])]),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Unsupported attribute: Can't access attributes on a list of objects. Did you mean to access attribute \"foo\" for a specific element of the list, or across all elements of the list?",
        },
        Case {
            start: Value::empty_tuple(),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: the collection has no elements.",
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: the given index is greater than or equal to the length of the collection.",
        },
        Case {
            start: Value::tuple([Value::string("hello")]).mark("boop"),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value.",
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_int(-1)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: a negative number is not a valid index for a sequence.",
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_float(0.5)),
            want: None,
            want_err: "Invalid index: The given key does not identify an element in this collection value: indexing a sequence requires a whole number, but the given index has a fractional part.",
        },
        Case {
            start: Value::tuple([Value::string("hello")]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: "Unsupported attribute: Can't access attributes on a primitive-typed value (string).",
        },
        Case {
            start: Value::tuple([Value::empty_object()]),
            path: Path::new().index(Value::number_int(0)).attr("foo"),
            want: None,
            want_err: "Unsupported attribute: This object does not have an attribute named \"foo\".",
        },
        Case {
            start: Value::tuple([Value::empty_object()]),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Unsupported attribute: This value does not have any attributes.",
        },
        Case {
            start: Value::tuple([Value::object([("foo", Value::bool(true))])]),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Unsupported attribute: This value does not have any attributes.",
        },
        Case {
            start: Value::set([Value::string("hello")]),
            path: Path::new().index(Value::number_int(1)),
            want: None,
            want_err: "Invalid index: Elements of a set are identified only by their value and don't have any separate index or key to select with, so it's only possible to perform operations across all elements of the set.",
        },
        Case {
            start: Value::set([Value::empty_object()]),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Unsupported attribute: Can't access attributes on a set of objects. Did you mean to access an attribute across all elements of the set?",
        },
        Case {
            start: Value::null(Type::list(Type::string())),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: "Attempt to index null value: This value is null, so it does not have any indices.",
        },
        Case {
            start: Value::null(Type::map(Type::string())),
            path: Path::new().index(Value::number_int(0)),
            want: None,
            want_err: "Attempt to index null value: This value is null, so it does not have any indices.",
        },
        Case {
            start: Value::null(Type::empty_object()),
            path: Path::new().attr("foo"),
            want: None,
            want_err: "Attempt to get attribute from null value: This value is null, so it does not have any attributes.",
        },
        // Marks should be retained during index and getattr ops, even when
        // types and values are unknown. This reflects the same behavior when
        // using cty to directly call GetAttr and Index methods.
        Case {
            start: Value::dynamic().mark("marked"),
            path: Path::new().attr("foo"),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::object([("foo", Value::string("should be marked"))]).mark("marked"),
            path: Path::new().attr("foo"),
            want: Some(Value::string("should be marked").mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::unknown(Type::object([("foo", Type::dynamic())])).mark("marked"),
            path: Path::new().attr("foo"),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::dynamic().mark("marked"),
            path: Path::new().index(Value::string("foo")),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::object([("foo", Value::string("should be marked"))]).mark("marked"),
            path: Path::new().index(Value::string("foo")),
            want: Some(Value::string("should be marked").mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::unknown(Type::object([("foo", Type::dynamic())])).mark("marked"),
            path: Path::new().index(Value::string("foo")),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::dynamic().mark("marked"),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::list([Value::string("should be marked")]).mark("marked"),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::string("should be marked").mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::unknown(Type::list(Type::string())).mark("marked"),
            path: Path::new().index(Value::number_int(0)),
            want: Some(Value::unknown(Type::string()).mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::dynamic().mark("marked"),
            path: Path::new().index(Value::unknown(Type::string())),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::object([("foo", Value::string("should be marked"))]).mark("marked"),
            path: Path::new().index(Value::unknown(Type::string())),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::unknown(Type::object([("foo", Type::dynamic())])).mark("marked"),
            path: Path::new().index(Value::unknown(Type::string())),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::dynamic().mark("marked"),
            path: Path::new().index(Value::unknown(Type::number())),
            want: Some(Value::dynamic().mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::list([Value::string("should be marked")]).mark("marked"),
            path: Path::new().index(Value::unknown(Type::number())),
            want: Some(Value::unknown(Type::string()).mark("marked")),
            want_err: "",
        },
        Case {
            start: Value::unknown(Type::list(Type::string())).mark("marked"),
            path: Path::new().index(Value::unknown(Type::number())),
            want: Some(Value::unknown(Type::string()).mark("marked")),
            want_err: "",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got, diags) = apply_path(&test.start, &test.path, None);

        if !test.want_err.is_empty() {
            assert!(
                diags.has_errors(),
                "case {i} ({} {}): succeeded, but want error\nwant error: {}",
                test.start.go_string(),
                test.path.go_string(),
                test.want_err,
            );
            assert_eq!(
                diags.len(),
                1,
                "case {i} ({} {}): wrong number of diagnostics {}; want 1",
                test.start.go_string(),
                test.path.go_string(),
                diags.len(),
            );

            let got_err_str = format!("{}: {}", diags[0].summary, diags[0].detail);
            assert_eq!(
                got_err_str,
                test.want_err,
                "case {i} ({} {}): wrong error\ngot error:  {got_err_str}\nwant error: {}",
                test.start.go_string(),
                test.path.go_string(),
                test.want_err,
            );
            continue;
        }

        assert!(
            !diags.has_errors(),
            "case {i} ({} {}): failed, but want success\ngot diagnostics:\n{diags}",
            test.start.go_string(),
            test.path.go_string(),
        );
        let want = test
            .want
            .as_ref()
            .expect("success case must have a want value");
        assert_eq!(
            &got,
            want,
            "case {i} ({} {}): wrong result\ngot:  {}\nwant: {}",
            test.start.go_string(),
            test.path.go_string(),
            got.go_string(),
            want.go_string(),
        );
    }
}

// Ported from TestIndex:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ops_test.go#L394
#[test]
#[ignore = "not yet implemented"]
fn index_cases() {
    struct Case {
        name: &'static str,
        coll: Value,
        key: Value,
        want: Value,
        err: &'static str,
    }

    let tests = [
        Case {
            name: "marked key to maked value",
            coll: Value::list([Value::string("a")]),
            key: Value::number_int(0).mark("marked"),
            want: Value::string("a").mark("marked"),
            err: "",
        },
        Case {
            name: "missing list key",
            coll: Value::list([Value::string("a")]),
            key: Value::number_int(1).mark("marked"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "null marked key",
            coll: Value::list([Value::string("a")]),
            key: Value::null(Type::number()).mark("marked"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "dynamic key",
            coll: Value::list([Value::string("a")]),
            key: Value::dynamic(),
            want: Value::dynamic(),
            err: "",
        },
        Case {
            name: "invalid marked key type",
            coll: Value::list([Value::string("a")]),
            key: Value::string("foo").mark("marked"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "marked map key",
            coll: Value::map([("foo", Value::string("a"))]),
            key: Value::string("foo").mark("marked"),
            want: Value::string("a").mark("marked"),
            err: "",
        },
        Case {
            name: "missing marked map key",
            coll: Value::map([("foo", Value::string("a"))]),
            key: Value::string("bar").mark("mark"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "marked object key",
            coll: Value::object([("foo", Value::string("a"))]),
            key: Value::string("foo").mark("marked"),
            // an object attribute is fetched by string index, and the marks
            // are not maintained
            want: Value::string("a"),
            err: "",
        },
        Case {
            name: "invalid marked object key type",
            coll: Value::object([("foo", Value::string("a"))]),
            key: Value::list([Value::null(Type::string())]).mark("marked"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "invalid marked object key",
            coll: Value::object([("foo", Value::string("a"))]),
            key: Value::number_int(0).mark("marked"),
            want: Value::dynamic(),
            err: "Invalid index",
        },
        Case {
            name: "unknown object",
            coll: Value::unknown(Type::object([("foo", Type::string())])),
            key: Value::string("foo"),
            want: Value::unknown(Type::string()),
            err: "",
        },
        Case {
            name: "unknown object, invalid index",
            coll: Value::unknown(Type::object([("foo", Type::string())])),
            key: Value::number_int(0),
            want: Value::dynamic(),
            err: "Invalid index",
        },
    ];

    for tc in tests.iter() {
        let (got, diags) = index(&tc.coll, &tc.key, None);

        if !tc.err.is_empty() {
            assert!(
                diags.has_errors(),
                "{}: succeeded, but want error\nwant error: {}",
                tc.name,
                tc.err,
            );
            assert_eq!(
                diags.len(),
                1,
                "{}: wrong number of diagnostics {}; want 1",
                tc.name,
                diags.len(),
            );

            let got_err_str = &diags[0].summary;
            assert_eq!(
                got_err_str, tc.err,
                "{}: wrong error\ngot error:  {got_err_str}\nwant error: {}",
                tc.name, tc.err,
            );
            continue;
        }

        assert!(
            !diags.has_errors(),
            "{}: failed, but want success\ngot diagnostics:\n{diags}",
            tc.name,
        );
        assert_eq!(
            got,
            tc.want,
            "{}: wrong result\ngot:  {}\nwant: {}",
            tc.name,
            got.go_string(),
            tc.want.go_string(),
        );
    }
}
