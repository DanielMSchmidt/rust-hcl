//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   ext/typeexpr/type_string_test.go
//!   ext/typeexpr/type_type_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::{Type, Value};
use hcl::ext::typeexpr;

// Ported from TestTypeString:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/type_string_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn type_string() {
    struct Case {
        ty: Type,
        want: &'static str,
    }

    let tests = [
        Case {
            ty: Type::dynamic(),
            want: "any",
        },
        Case {
            ty: Type::string(),
            want: "string",
        },
        Case {
            ty: Type::number(),
            want: "number",
        },
        Case {
            ty: Type::bool(),
            want: "bool",
        },
        Case {
            ty: Type::list(Type::number()),
            want: "list(number)",
        },
        Case {
            ty: Type::set(Type::bool()),
            want: "set(bool)",
        },
        Case {
            ty: Type::map(Type::string()),
            want: "map(string)",
        },
        Case {
            ty: Type::empty_object(),
            want: "object({})",
        },
        Case {
            ty: Type::object([("foo", Type::bool())]),
            want: "object({foo=bool})",
        },
        Case {
            ty: Type::object([("foo", Type::bool()), ("bar", Type::string())]),
            want: "object({bar=string,foo=bool})",
        },
        Case {
            ty: Type::empty_tuple(),
            want: "tuple([])",
        },
        Case {
            ty: Type::tuple([Type::bool()]),
            want: "tuple([bool])",
        },
        Case {
            ty: Type::tuple([Type::bool(), Type::string()]),
            want: "tuple([bool,string])",
        },
        Case {
            ty: Type::list(Type::dynamic()),
            want: "list(any)",
        },
        Case {
            ty: Type::tuple([Type::dynamic()]),
            want: "tuple([any])",
        },
        Case {
            ty: Type::object([("foo", Type::dynamic())]),
            want: "object({foo=any})",
        },
        Case {
            // We don't expect to find attributes that aren't valid identifiers
            // because we only promise to support types that this package
            // would've created, but we allow this situation during rendering
            // just because it's convenient for applications trying to produce
            // error messages about mismatched types. Note that the quoted
            // attribute name is not actually accepted by our Type and
            // TypeConstraint functions, so this is one situation where the
            // TypeString result cannot be re-parsed by those functions.
            ty: Type::object([("foo bar baz", Type::string())]),
            want: r#"object({"foo bar baz"=string})"#,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = typeexpr::type_string(&test.ty);
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ntype: {:?}\ngot:  {got}\nwant: {}",
            test.ty, test.want,
        );
    }
}

// Ported from TestTypeConstraintType:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/type_type_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn type_constraint_type() {
    let ty_val1 = typeexpr::type_constraint_val(Type::string());
    let ty_val2 = typeexpr::type_constraint_val(Type::string());
    let ty_val3 = typeexpr::type_constraint_val(Type::number());

    assert!(
        ty_val1.raw_equals(&ty_val2),
        "tyVal1 not equal to tyVal2\ntyVal1: {ty_val1:?}\ntyVal2: {ty_val2:?}",
    );
    assert!(
        !ty_val1.raw_equals(&ty_val3),
        "tyVal1 equal to tyVal2, but should not be\ntyVal1: {ty_val1:?}\ntyVal3: {ty_val3:?}",
    );

    let (got, want) = (typeexpr::type_constraint_from_val(&ty_val1), Type::string());
    assert!(
        got.equals(&want),
        "wrong type extracted from tyVal1\ngot:  {got:?}\nwant: {want:?}",
    );
    let (got, want) = (typeexpr::type_constraint_from_val(&ty_val3), Type::number());
    assert!(
        got.equals(&want),
        "wrong type extracted from tyVal3\ngot:  {got:?}\nwant: {want:?}",
    );
}

// Ported from TestConvertFunc:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/ext/typeexpr/type_type_test.go#L33
#[test]
#[ignore = "not yet implemented"]
fn convert_func() {
    // This is testing the convert function directly, skipping over the HCL
    // parsing and evaluation steps that would normally lead there. There is
    // another test in the "integrationtest" package called TestTypeConvertFunc
    // that exercises the full path to this function via the hclsyntax parser.

    struct Case {
        val: Value,
        ty: Value,
        // NOTE(port): Go's `cty.NilVal` (the Value zero value, used when
        // only an error is expected) becomes `None`; it is never compared
        // against the result because those cases return early on the error
        // assertion, exactly as upstream does.
        want: Option<Value>,
        want_err: &'static str,
    }

    let tests = [
        // The goal here is not an exhaustive set of conversions, since that's
        // already covered in cty/convert, but rather exercising different
        // permutations of success and failure to make sure the function
        // handles all of the results in a reasonable way.
        Case {
            val: Value::string("hello"),
            ty: typeexpr::type_constraint_val(Type::string()),
            want: Some(Value::string("hello")),
            want_err: "",
        },
        Case {
            val: Value::bool(true),
            ty: typeexpr::type_constraint_val(Type::string()),
            want: Some(Value::string("true")),
            want_err: "",
        },
        Case {
            val: Value::string("hello"),
            ty: typeexpr::type_constraint_val(Type::bool()),
            want: None,
            want_err: "a bool is required",
        },
        Case {
            val: Value::unknown(Type::bool()),
            ty: typeexpr::type_constraint_val(Type::bool()),
            want: Some(Value::unknown(Type::bool())),
            want_err: "",
        },
        Case {
            val: Value::dynamic(),
            ty: typeexpr::type_constraint_val(Type::bool()),
            want: Some(Value::unknown(Type::bool())),
            want_err: "",
        },
        Case {
            val: Value::null(Type::bool()),
            ty: typeexpr::type_constraint_val(Type::bool()),
            want: Some(Value::null(Type::bool())),
            want_err: "",
        },
        Case {
            val: Value::null(Type::dynamic()),
            ty: typeexpr::type_constraint_val(Type::bool()),
            want: Some(Value::null(Type::bool())),
            want_err: "",
        },
        Case {
            // NOTE(port): Go's `.Mark(1)` marks with the int `1`; rust-cty
            // marks take `impl Into<Mark>`, with `From<i64>` covering Go's
            // untyped int literal.
            val: Value::string("hello").mark(1i64),
            ty: typeexpr::type_constraint_val(Type::string()),
            want: Some(Value::string("hello").mark(1i64)),
            want_err: "",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        match typeexpr::convert_func().call(&[test.val.clone(), test.ty.clone()]) {
            Err(err) => {
                if !test.want_err.is_empty() {
                    assert_eq!(
                        format!("{err}"),
                        test.want_err,
                        "case {i} ({:?} to {:?}): wrong error",
                        test.val,
                        test.ty,
                    );
                } else {
                    panic!(
                        "case {i} ({:?} to {:?}): unexpected error\ngot:  {err}\nwant: <nil>",
                        test.val, test.ty,
                    );
                }
            }
            Ok(got) => {
                assert!(
                    test.want_err.is_empty(),
                    "case {i} ({:?} to {:?}): wrong error\ngot:  <nil>\nwant: {}",
                    test.val,
                    test.ty,
                    test.want_err,
                );
                let want = test
                    .want
                    .as_ref()
                    .expect("want is NilVal only in error cases");
                assert!(
                    want.raw_equals(&got),
                    "case {i} ({:?} to {:?}): wrong result\ngot:  {got:?}\nwant: {want:?}",
                    test.val,
                    test.ty,
                );
            }
        }
    }
}
