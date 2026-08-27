//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_test.go (TestExpressionParseAndValue, part 2)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::function::stdlib;
use cty::{Type, Value, ValueMarks};
use hcl::hclsyntax;
use hcl::{EvalContext, Pos};

/// An eval context with only the given variables in scope
/// (Go: `&hcl.EvalContext{Variables: map[string]cty.Value{...}}`).
fn ctx_vars(vars: impl IntoIterator<Item = (&'static str, Value)>) -> Option<EvalContext> {
    let mut ctx = EvalContext::new();
    ctx.variables = vars.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Some(ctx)
}

/// An eval context with only the given functions in scope
/// (Go: `&hcl.EvalContext{Functions: map[string]function.Function{...}}`).
fn ctx_funcs(
    funcs: impl IntoIterator<Item = (&'static str, cty::function::Function)>,
) -> Option<EvalContext> {
    let mut ctx = EvalContext::new();
    ctx.functions = funcs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Some(ctx)
}

struct Case {
    input: &'static str,
    ctx: Option<EvalContext>,
    want: Value,
    diag_count: usize,
}

// Ported from TestExpressionParseAndValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_test.go#L17
// (part 2: cases with opening brace in upstream lines 848-1667)
#[test]
#[ignore = "not yet implemented"]
fn expression_parse_and_value_part2() {
    // This is a combo test that exercises both the parser and the Value
    // method, with the focus on the latter but indirectly testing the former.
    let tests = vec![
        Case {
            input: r#"[for k, v in {hello: "world"}: k => v]"#,
            ctx: None,
            want: Value::object([("hello", Value::string("world"))]),
            diag_count: 1, // can't have a key expr when producing a tuple
        },
        Case {
            input: r#"{for v in {hello: "world"}: v}"#,
            ctx: None,
            want: Value::tuple([Value::string("world")]),
            diag_count: 1, // must have a key expr when producing a map
        },
        Case {
            input: r#"{for i, v in ["a", "b", "c", "b", "d"]: v => i...}"#,
            ctx: None,
            want: Value::object([
                ("a", Value::tuple([Value::number_int(0)])),
                (
                    "b",
                    Value::tuple([Value::number_int(1), Value::number_int(3)]),
                ),
                ("c", Value::tuple([Value::number_int(2)])),
                ("d", Value::tuple([Value::number_int(4)])),
            ]),
            diag_count: 0,
        },
        Case {
            input: r#"{for i, v in ["a", "b", "c", "b", "d"]: v => i... if i <= 2}"#,
            ctx: None,
            want: Value::object([
                ("a", Value::tuple([Value::number_int(0)])),
                ("b", Value::tuple([Value::number_int(1)])),
                ("c", Value::tuple([Value::number_int(2)])),
            ]),
            diag_count: 0,
        },
        Case {
            input: r#"{for i, v in ["a", "b", "c", "b", "d"]: v => i}"#,
            ctx: None,
            want: Value::object([
                ("a", Value::number_int(0)),
                ("b", Value::number_int(1)),
                ("c", Value::number_int(2)),
                ("d", Value::number_int(4)),
            ]),
            diag_count: 1, // duplicate key "b"
        },
        Case {
            input: r#"[for v in {hello: "world"}: v...]"#,
            ctx: None,
            want: Value::tuple([Value::string("world")]),
            diag_count: 1, // can't use grouping when producing a tuple
        },
        Case {
            input: r#"[for v in "hello": v]"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // can't iterate over a string
        },
        Case {
            input: r#"[for v in null: v]"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // can't iterate over a null value
        },
        Case {
            input: r#"[for v in unk: v]"#,
            ctx: ctx_vars([("unk", Value::unknown(Type::list(Type::string())))]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"[for v in unk: v]"#,
            ctx: ctx_vars([("unk", Value::dynamic())]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"[for v in unk: v]"#,
            ctx: ctx_vars([("unk", Value::unknown(Type::string()))]),
            want: Value::dynamic(),
            diag_count: 1, // can't iterate over a string (even if it's unknown)
        },
        Case {
            input: r#"[for v in ["a", "b"]: v if unkbool]"#,
            ctx: ctx_vars([("unkbool", Value::unknown(Type::bool()))]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"[for v in ["a", "b"]: v if nullbool]"#,
            ctx: ctx_vars([("nullbool", Value::null(Type::bool()))]),
            want: Value::dynamic(),
            diag_count: 1, // value of if clause must not be null
        },
        Case {
            input: r#"[for v in ["a", "b"]: v if dyn]"#,
            ctx: ctx_vars([("dyn", Value::dynamic())]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"[for v in ["a", "b"]: v if unknum]"#,
            ctx: ctx_vars([("unknum", Value::unknown(Type::list(Type::number())))]),
            want: Value::dynamic(),
            diag_count: 1, // if expression must be bool
        },
        Case {
            input: r#"[for i, v in ["a", "b"]: v if i + i]"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // if expression must be bool
        },
        Case {
            input: r#"[for v in ["a", "b"]: unkstr]"#,
            ctx: ctx_vars([("unkstr", Value::unknown(Type::string()))]),
            want: Value::tuple([
                Value::unknown(Type::string()),
                Value::unknown(Type::string()),
            ]),
            diag_count: 0,
        },
        // Marked sequence results in a marked tuple
        Case {
            input: r#"[for x in things: x if x != ""]"#,
            ctx: ctx_vars([(
                "things",
                Value::list([
                    Value::string("a"),
                    Value::string("b"),
                    Value::string(""),
                    Value::string("c"),
                ])
                .mark("sensitive"),
            )]),
            want: Value::tuple([Value::string("a"), Value::string("b"), Value::string("c")])
                .mark("sensitive"),
            diag_count: 0,
        },
        // Marked map results in a marked object
        Case {
            input: r#"{for k, v in things: k => !v}"#,
            ctx: ctx_vars([(
                "things",
                Value::map([("a", Value::bool(true)), ("b", Value::bool(false))]).mark("sensitive"),
            )]),
            want: Value::object([("a", Value::bool(false)), ("b", Value::bool(true))])
                .mark("sensitive"),
            diag_count: 0,
        },
        // Marked map member carries marks through
        Case {
            input: r#"{for k, v in things: k => !v}"#,
            ctx: ctx_vars([(
                "things",
                Value::map([
                    ("a", Value::bool(true).mark("sensitive")),
                    ("b", Value::bool(false)),
                ]),
            )]),
            want: Value::object([
                ("a", Value::bool(false).mark("sensitive")),
                ("b", Value::bool(true)),
            ]),
            diag_count: 0,
        },
        // Mark object if keys include marked values, members retain
        // their original marks in their values
        Case {
            input: r#"{for v in things: v => "${v}-friend"}"#,
            ctx: ctx_vars([(
                "things",
                Value::map([
                    ("a", Value::string("rosie").mark("marked")),
                    ("b", Value::string("robin")),
                    // Check for double-marking when a key val has a duplicate mark
                    ("c", Value::string("rowan").mark("marked")),
                    ("d", Value::string("ruben").mark("also-marked")),
                ]),
            )]),
            want: Value::object([
                ("rosie", Value::string("rosie-friend").mark("marked")),
                ("robin", Value::string("robin-friend")),
                ("rowan", Value::string("rowan-friend").mark("marked")),
                ("ruben", Value::string("ruben-friend").mark("also-marked")),
            ])
            .with_marks([ValueMarks::from_marks(["marked", "also-marked"])]),
            diag_count: 0,
        },
        // object itself is marked, contains marked value
        Case {
            input: r#"{for v in things: v => "${v}-friend"}"#,
            ctx: ctx_vars([(
                "things",
                Value::map([
                    ("a", Value::string("rosie").mark("marked")),
                    ("b", Value::string("robin")),
                ])
                .mark("marks"),
            )]),
            want: Value::object([
                ("rosie", Value::string("rosie-friend").mark("marked")),
                ("robin", Value::string("robin-friend")),
            ])
            .with_marks([ValueMarks::from_marks(["marked", "marks"])]),
            diag_count: 0,
        },
        // Sequence for loop with marked conditional expression
        Case {
            input: r#"[for x in things: x if x != secret]"#,
            ctx: ctx_vars([
                (
                    "things",
                    Value::list([Value::string("a"), Value::string("b"), Value::string("c")]),
                ),
                ("secret", Value::string("b").mark("sensitive")),
            ]),
            want: Value::tuple([Value::string("a"), Value::string("c")]).mark("sensitive"),
            diag_count: 0,
        },
        // Map for loop with marked conditional expression
        Case {
            input: r#"{ for k, v in things: k => v if k != secret }"#,
            ctx: ctx_vars([
                (
                    "things",
                    Value::map([
                        ("a", Value::bool(true)),
                        ("b", Value::bool(false)),
                        ("c", Value::bool(false)),
                    ]),
                ),
                ("secret", Value::string("b").mark("sensitive")),
            ]),
            want: Value::object([("a", Value::bool(true)), ("c", Value::bool(false))])
                .mark("sensitive"),
            diag_count: 0,
        },
        Case {
            input: r#"{ for k, v in things: k => v if k != unknown_secret }"#,
            ctx: ctx_vars([
                (
                    "things",
                    Value::map([
                        ("a", Value::bool(true)),
                        ("b", Value::bool(false)),
                        ("c", Value::bool(false)),
                    ]),
                ),
                (
                    "unknown_secret",
                    Value::unknown(Type::string()).mark("sensitive"),
                ),
            ]),
            want: Value::dynamic().mark("sensitive"),
            diag_count: 0,
        },
        Case {
            input: r#"[ for v in things: v if v != unknown_secret ]"#,
            ctx: ctx_vars([
                (
                    "things",
                    Value::tuple([Value::string("a"), Value::string("b")]),
                ),
                (
                    "unknown_secret",
                    Value::unknown(Type::string()).mark("sensitive"),
                ),
            ]),
            want: Value::dynamic().mark("sensitive"),
            diag_count: 0,
        },
        Case {
            input: r#"[ for v in things: v if v != secret ]"#,
            ctx: ctx_vars([
                (
                    "things",
                    Value::tuple([
                        Value::unknown(Type::string()).mark("mark"),
                        Value::string("b"),
                    ]),
                ),
                ("secret", Value::string("b").mark("sensitive")),
            ]),
            want: Value::dynamic().with_marks([ValueMarks::from_marks(["mark", "sensitive"])]),
            diag_count: 0,
        },
        Case {
            input: r#"[{name: "Steve"}, {name: "Ermintrude"}].*.name"#,
            ctx: None,
            want: Value::tuple([Value::string("Steve"), Value::string("Ermintrude")]),
            diag_count: 0,
        },
        Case {
            input: r#"{name: "Steve"}.*.name"#,
            ctx: None,
            want: Value::tuple([Value::string("Steve")]),
            diag_count: 0,
        },
        Case {
            input: r#"null[*]"#,
            ctx: None,
            want: Value::empty_tuple(),
            diag_count: 0,
        },
        Case {
            input: r#"{name: "Steve"}[*].name"#,
            ctx: None,
            want: Value::tuple([Value::string("Steve")]),
            diag_count: 0,
        },
        Case {
            input: r#"set.*.name"#,
            ctx: ctx_vars([(
                "set",
                Value::set([Value::object([("name", Value::string("Steve"))])]),
            )]),
            want: Value::list([Value::string("Steve")]),
            diag_count: 0,
        },
        Case {
            input: r#"unkstr[*]"#,
            ctx: ctx_vars([("unkstr", Value::unknown(Type::string()))]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"unkstr[*]"#,
            ctx: ctx_vars([("unkstr", Value::unknown(Type::string()).refine_not_null())]),
            // If the unknown string is definitely not null then we already
            // know that the result will be a single-element tuple.
            want: Value::tuple([Value::unknown(Type::string()).refine_not_null()]),
            diag_count: 0,
        },
        Case {
            input: r#"unkstr.*.name"#,
            ctx: ctx_vars([("unkstr", Value::unknown(Type::string()))]),
            want: Value::dynamic(),
            diag_count: 1, // a string has no attribute "name"
        },
        Case {
            input: r#"dyn.*.name"#,
            ctx: ctx_vars([("dyn", Value::dynamic())]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"unkobj.*.name"#,
            ctx: ctx_vars([(
                "unkobj",
                Value::unknown(Type::object([("name", Type::string())])),
            )]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"unkobj.*.name"#,
            ctx: ctx_vars([(
                "unkobj",
                Value::unknown(Type::object([("name", Type::string())])).refine_not_null(),
            )]),
            want: Value::tuple([Value::unknown(Type::string())]),
            diag_count: 0,
        },
        Case {
            input: r#"unkobj.*.names"#,
            ctx: ctx_vars([(
                "unkobj",
                Value::unknown(Type::object([("names", Type::list(Type::string()))])),
            )]),
            want: Value::dynamic(),
            diag_count: 0,
        },
        Case {
            input: r#"unklistobj.*.name"#,
            ctx: ctx_vars([(
                "unklistobj",
                Value::unknown(Type::list(Type::object([("name", Type::string())]))),
            )]),
            want: Value::unknown(Type::list(Type::string())).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"unklistobj.*.name"#,
            ctx: ctx_vars([(
                "unklistobj",
                Value::unknown(Type::list(Type::object([("name", Type::string())])))
                    .refine()
                    .collection_length_upper_bound(5)
                    .new_value(),
            )]),
            want: Value::unknown(Type::list(Type::string()))
                .refine()
                .not_null()
                .collection_length_upper_bound(5)
                .new_value(),
            diag_count: 0,
        },
        Case {
            input: r#"unktupleobj.*.name"#,
            ctx: ctx_vars([(
                "unktupleobj",
                Value::unknown(Type::tuple([
                    Type::object([("name", Type::string())]),
                    Type::object([("name", Type::bool())]),
                ])),
            )]),
            want: Value::unknown(Type::tuple([Type::string(), Type::bool()])).refine_not_null(),
            diag_count: 0,
        },
        Case {
            input: r#"nullobj.*.name"#,
            ctx: ctx_vars([(
                "nullobj",
                Value::null(Type::object([("name", Type::string())])),
            )]),
            want: Value::tuple(Vec::<Value>::new()),
            diag_count: 0,
        },
        Case {
            input: r#"nulllist.*.name"#,
            ctx: ctx_vars([(
                "nulllist",
                Value::null(Type::list(Type::object([("name", Type::string())]))),
            )]),
            want: Value::dynamic(),
            diag_count: 1, // splat cannot be applied to null sequence
        },
        Case {
            input: r#"listofobj[*].scalar[*]"#,
            ctx: ctx_vars([(
                "listofobj",
                Value::list([
                    Value::object([("scalar", Value::string("foo"))]),
                    Value::object([("scalar", Value::string("bar"))]),
                ]),
            )]),
            want: Value::list([
                // The second-level splat promotes the scalars to single-element tuples.
                Value::tuple([Value::string("foo")]),
                Value::tuple([Value::string("bar")]),
            ]),
            diag_count: 0,
        },
        // This is a particularly tricky case where two splat rules interact in
        // a sub-optimal way:
        // 1. The top-level splat is applied to a list and so it wants to return a list.
        // 2. The nested splat is applied to a scalar, and so it wants to return different tuple types depending on the nullness.
        // Rule 2 breaks rule 1, because we can't make a list with elements of different types.
        // For now we're treating this as an error because we didn't learn of this bad
        // interaction until long after both of these rules were in separate wide use,
        // and so it isn't clear how to make this work without potentially breaking other
        // behavior. Perhaps this can become valid in future if we find a viable way to
        // do it.
        Case {
            input: r#"listofobj[*].scalar[*]"#,
            ctx: ctx_vars([(
                "listofobj",
                Value::list([
                    Value::object([("scalar", Value::null(Type::string()))]),
                    Value::object([("scalar", Value::string("bar"))]),
                ]),
            )]),
            want: Value::dynamic(),
            diag_count: 1, // nested splat produces non-homogenously-typed results in this case, so cannot produce a valid list
        },
        Case {
            input: r#"["hello", "goodbye"].*"#,
            ctx: None,
            want: Value::tuple([Value::string("hello"), Value::string("goodbye")]),
            diag_count: 0,
        },
        Case {
            input: r#""hello".*"#,
            ctx: None,
            want: Value::tuple([Value::string("hello")]),
            diag_count: 0,
        },
        Case {
            input: r#"[["hello"], ["world", "unused"]].*.0"#,
            ctx: None,
            want: Value::tuple([Value::string("hello"), Value::string("world")]),
            diag_count: 0,
        },
        Case {
            input: r#"[[{name:"foo"}], [{name:"bar"}, {name:"baz"}]].*.0.name"#,
            ctx: None,
            want: Value::tuple([Value::string("foo"), Value::string("bar")]),
            diag_count: 0,
        },
        Case {
            input: r#"[[[{name:"foo"}]], [[{name:"bar"}], [{name:"baz"}]]].*.0.0.name"#,
            ctx: None,
            want: Value::tuple([Value::dynamic(), Value::dynamic()]),
            diag_count: 1, // can't chain legacy index syntax together, like .0.0 (because 0.0 parses as a single number)
        },
        // For an "attribute-only" splat, an index operator applies to
        // the splat result as a whole, rather than being incorporated
        // into the splat traversal itself.
        Case {
            input: r#"[{name: "Steve"}, {name: "Ermintrude"}].*.name[0]"#,
            ctx: None,
            want: Value::string("Steve"),
            diag_count: 0,
        },
        // For a "full" splat, an index operator is consumed as part
        // of the splat's traversal.
        Case {
            input: r#"[{names: ["Steve"]}, {names: ["Ermintrude"]}][*].names[0]"#,
            ctx: None,
            want: Value::tuple([Value::string("Steve"), Value::string("Ermintrude")]),
            diag_count: 0,
        },
        // Another "full" splat, this time with the index first.
        Case {
            input: r#"[[{name: "Steve"}], [{name: "Ermintrude"}]][*][0].name"#,
            ctx: None,
            want: Value::tuple([Value::string("Steve"), Value::string("Ermintrude")]),
            diag_count: 0,
        },
        // Full splats can nest, which produces nested tuples.
        Case {
            input: r#"[[{name: "Steve"}], [{name: "Ermintrude"}]][*][*].name"#,
            ctx: None,
            want: Value::tuple([
                Value::tuple([Value::string("Steve")]),
                Value::tuple([Value::string("Ermintrude")]),
            ]),
            diag_count: 0,
        },
        Case {
            input: r#"[["hello"], ["goodbye"]].*.*"#,
            ctx: None,
            want: Value::tuple([
                Value::tuple([Value::string("hello")]),
                Value::tuple([Value::string("goodbye")]),
            ]),
            diag_count: 1,
        },
        // splat with sensitive collection
        Case {
            input: r#"maps.*.enabled"#,
            ctx: ctx_vars([(
                "maps",
                Value::list([
                    Value::map([("enabled", Value::bool(true))]),
                    Value::map([("enabled", Value::bool(false))]),
                ])
                .mark("sensitive"),
            )]),
            want: Value::list([Value::bool(true), Value::bool(false)]).mark("sensitive"),
            diag_count: 0,
        },
        // splat with sensitive collection that's unknown
        Case {
            input: r#"maps.*.enabled"#,
            ctx: ctx_vars([(
                "maps",
                Value::unknown(Type::list(Type::map(Type::bool()))).mark("sensitive"),
            )]),
            want: Value::unknown(Type::list(Type::bool()))
                .refine_not_null()
                .mark("sensitive"),
            diag_count: 0,
        },
        // splat with sensitive non-collection that's unknown
        Case {
            input: r#"not_a_list.*"#,
            ctx: ctx_vars([(
                "not_a_list",
                Value::unknown(Type::empty_object())
                    .refine_not_null()
                    .mark("sensitive"),
            )]),
            want: Value::tuple([Value::unknown(Type::empty_object())
                .refine_not_null()
                .mark("sensitive")])
            .mark("sensitive"),
            diag_count: 0,
        },
        // splat with sensitive collection that's unknown and not null
        Case {
            input: r#"maps.*.enabled"#,
            ctx: ctx_vars([(
                "maps",
                Value::unknown(Type::list(Type::map(Type::bool())))
                    .refine_not_null()
                    .mark("sensitive"),
            )]),
            want: Value::unknown(Type::list(Type::bool()))
                .refine_not_null()
                .mark("sensitive"),
            diag_count: 0,
        },
        // splat with collection with sensitive elements
        Case {
            input: r#"maps.*.x"#,
            ctx: ctx_vars([(
                "maps",
                Value::list([
                    Value::map([("x", Value::string("foo").mark("sensitive"))]),
                    Value::map([("x", Value::string("bar"))]),
                ]),
            )]),
            want: Value::list([Value::string("foo").mark("sensitive"), Value::string("bar")]),
            diag_count: 0,
        },
        Case {
            input: r#"["hello"][0]"#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"["hello"].0"#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"[["hello"]].0.0"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // can't chain legacy index syntax together (because 0.0 parses as 0)
        },
        Case {
            input: r#"[{greeting = "hello"}].0.greeting"#,
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"[][0]"#,
            ctx: None,
            want: Value::dynamic(),
            diag_count: 1, // invalid index
        },
        Case {
            input: r#"["hello"][negate(0)]"#,
            ctx: ctx_funcs([("negate", stdlib::negate_func())]),
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"[][negate(0)]"#,
            ctx: ctx_funcs([("negate", stdlib::negate_func())]),
            want: Value::dynamic(),
            diag_count: 1, // invalid index
        },
        Case {
            input: r#"["hello"]["0"]"#, // key gets converted to number
            ctx: None,
            want: Value::string("hello"),
            diag_count: 0,
        },
        Case {
            input: r#"["boop"].foo[index]"#, // index is a variable to force IndexExpr instead of traversal
            ctx: ctx_vars([("index", Value::number_int(0))]),
            want: Value::dynamic(),
            diag_count: 1, // expression ["boop"] does not have attributes
        },
        Case {
            input: r#"foo"#,
            ctx: ctx_vars([("foo", Value::string("hello"))]),
            want: Value::string("hello"),
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, parse_diags) = hclsyntax::parse_expression(
            test.input.as_bytes(),
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
        );
        let (got, val_diags) = expr.value(test.ctx.as_ref());

        let diag_count = parse_diags.len() + val_diags.len();
        assert_eq!(
            diag_count, test.diag_count,
            "case {i} ({}): wrong number of diagnostics\nparse: {:?}\nvalue: {:?}",
            test.input, parse_diags, val_diags,
        );

        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.input,);
    }
}
