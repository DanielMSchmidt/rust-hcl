//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   json/parser_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

// NOTE(port): upstream's `func init()` sets `deep.MaxDepth = 999` for the
// go-test/deep comparer; the `deep.Equal(got, test.Want)` per-field diff
// becomes a plain `assert_eq!` on the `json::Node` tree here (`PartialEq`
// implements the same deep, exported-field equality).

use cty::Value;
use hcl::json::{self, Node, ObjectAttr};
use hcl::{Pos, Range};

/// `hcl.Pos{Byte: byte, Line: line, Column: column}`.
fn pos(byte: usize, line: usize, column: usize) -> Pos {
    Pos { byte, line, column }
}

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

/// Go: the `mustBigFloat` test helper (`(&big.Float{}).Parse(s, 10)`,
/// panicking on error).
fn must_big_float(s: &str) -> Value {
    Value::parse_number(s).unwrap()
}

// Ported from TestParse:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/parser_test.go#L18
#[test]
#[ignore = "not yet implemented"]
fn parse() {
    struct Case {
        input: &'static str,
        want: Node,
        diag_count: usize,
    }

    let tests = [
        // Simple, single-token constructs
        Case {
            input: r#"true"#,
            want: Node::Boolean {
                value: true,
                src_range: rng(pos(0, 1, 1), pos(4, 1, 5)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"false"#,
            want: Node::Boolean {
                value: false,
                src_range: rng(pos(0, 1, 1), pos(5, 1, 6)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"null"#,
            want: Node::Null {
                src_range: rng(pos(0, 1, 1), pos(4, 1, 5)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"undefined"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(9, 1, 10)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"flase"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(5, 1, 6)),
            },
            diag_count: 1,
        },
        Case {
            input: r#""hello""#,
            want: Node::String {
                value: "hello".to_string(),
                src_range: rng(pos(0, 1, 1), pos(7, 1, 8)),
            },
            diag_count: 0,
        },
        Case {
            input: r#""hello\nworld""#,
            want: Node::String {
                value: "hello\nworld".to_string(),
                src_range: rng(pos(0, 1, 1), pos(14, 1, 15)),
            },
            diag_count: 0,
        },
        Case {
            input: r#""hello \"world\"""#,
            want: Node::String {
                value: r#"hello "world""#.to_string(),
                src_range: rng(pos(0, 1, 1), pos(17, 1, 18)),
            },
            diag_count: 0,
        },
        Case {
            input: r#""hello \\""#,
            want: Node::String {
                value: "hello \\".to_string(),
                src_range: rng(pos(0, 1, 1), pos(10, 1, 11)),
            },
            diag_count: 0,
        },
        Case {
            input: r#""hello"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(6, 1, 7)),
            },
            diag_count: 1,
        },
        Case {
            input: r#""he\llo""#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(8, 1, 9)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"1"#,
            want: Node::Number {
                value: must_big_float("1"),
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"1.2"#,
            want: Node::Number {
                value: must_big_float("1.2"),
                src_range: rng(pos(0, 1, 1), pos(3, 1, 4)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"-1"#,
            want: Node::Number {
                value: must_big_float("-1"),
                src_range: rng(pos(0, 1, 1), pos(2, 1, 3)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"1.2e5"#,
            want: Node::Number {
                value: must_big_float("120000"),
                src_range: rng(pos(0, 1, 1), pos(5, 1, 6)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"1.2e+5"#,
            want: Node::Number {
                value: must_big_float("120000"),
                src_range: rng(pos(0, 1, 1), pos(6, 1, 7)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"1.2e-5"#,
            want: Node::Number {
                value: must_big_float("1.2e-5"),
                src_range: rng(pos(0, 1, 1), pos(6, 1, 7)),
            },
            diag_count: 0,
        },
        Case {
            input: r#".1"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(2, 1, 3)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"+2"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(2, 1, 3)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"1 2"#,
            want: Node::Number {
                value: must_big_float("1"),
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        // Objects
        Case {
            input: r#"{"hello": true}"#,
            want: Node::Object {
                attrs: vec![ObjectAttr {
                    name: "hello".to_string(),
                    value: Node::Boolean {
                        value: true,
                        src_range: rng(pos(10, 1, 11), pos(14, 1, 15)),
                    },
                    name_range: rng(pos(1, 1, 2), pos(8, 1, 9)),
                }],
                src_range: rng(pos(0, 1, 1), pos(15, 1, 16)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                close_range: rng(pos(14, 1, 15), pos(15, 1, 16)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"{"hello": true, "bye": false}"#,
            want: Node::Object {
                attrs: vec![
                    ObjectAttr {
                        name: "hello".to_string(),
                        value: Node::Boolean {
                            value: true,
                            src_range: rng(pos(10, 1, 11), pos(14, 1, 15)),
                        },
                        name_range: rng(pos(1, 1, 2), pos(8, 1, 9)),
                    },
                    ObjectAttr {
                        name: "bye".to_string(),
                        value: Node::Boolean {
                            value: false,
                            src_range: rng(pos(23, 1, 24), pos(28, 1, 29)),
                        },
                        name_range: rng(pos(16, 1, 17), pos(21, 1, 22)),
                    },
                ],
                src_range: rng(pos(0, 1, 1), pos(29, 1, 30)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                close_range: rng(pos(28, 1, 29), pos(29, 1, 30)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"{}"#,
            want: Node::Object {
                attrs: vec![],
                src_range: rng(pos(0, 1, 1), pos(2, 1, 3)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                close_range: rng(pos(1, 1, 2), pos(2, 1, 3)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"{"hello":true"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"hello":true]"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"hello":true,}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{true:false}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"hello": true, "hello": true}"#,
            want: Node::Object {
                attrs: vec![
                    ObjectAttr {
                        name: "hello".to_string(),
                        value: Node::Boolean {
                            value: true,
                            src_range: rng(pos(10, 1, 11), pos(14, 1, 15)),
                        },
                        name_range: rng(pos(1, 1, 2), pos(8, 1, 9)),
                    },
                    ObjectAttr {
                        name: "hello".to_string(),
                        value: Node::Boolean {
                            value: true,
                            src_range: rng(pos(25, 1, 26), pos(29, 1, 30)),
                        },
                        name_range: rng(pos(16, 1, 17), pos(23, 1, 24)),
                    },
                ],
                src_range: rng(pos(0, 1, 1), pos(30, 1, 31)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
                close_range: rng(pos(29, 1, 30), pos(30, 1, 31)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"{"hello": true, "hello": true, "hello", true}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1, // comma used where colon is expected
        },
        Case {
            input: r#"{"hello", "world"}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"[]"#,
            want: Node::Array {
                values: vec![],
                src_range: rng(pos(0, 1, 1), pos(2, 1, 3)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"[true]"#,
            want: Node::Array {
                values: vec![Node::Boolean {
                    value: true,
                    src_range: rng(pos(1, 1, 2), pos(5, 1, 6)),
                }],
                src_range: rng(pos(0, 1, 1), pos(6, 1, 7)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"[true, false]"#,
            want: Node::Array {
                values: vec![
                    Node::Boolean {
                        value: true,
                        src_range: rng(pos(1, 1, 2), pos(5, 1, 6)),
                    },
                    Node::Boolean {
                        value: false,
                        src_range: rng(pos(7, 1, 8), pos(12, 1, 13)),
                    },
                ],
                src_range: rng(pos(0, 1, 1), pos(13, 1, 14)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"[[]]"#,
            want: Node::Array {
                values: vec![Node::Array {
                    values: vec![],
                    src_range: rng(pos(1, 1, 2), pos(3, 1, 4)),
                    open_range: rng(pos(1, 1, 2), pos(2, 1, 3)),
                }],
                src_range: rng(pos(0, 1, 1), pos(4, 1, 5)),
                open_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 0,
        },
        Case {
            input: r#"["#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 2,
        },
        Case {
            input: r#"[true"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"]"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"[true,]"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"[[],]"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"["hello":true]"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"[true}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"wrong"=true}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"wrong" = true}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
        Case {
            input: r#"{"wrong" true}"#,
            want: Node::Invalid {
                src_range: rng(pos(0, 1, 1), pos(1, 1, 2)),
            },
            diag_count: 1,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got, diag) = json::parse_file_content(
            test.input.as_bytes(),
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
        );

        assert_eq!(
            diag.len(),
            test.diag_count,
            "case {i} ({}): got {} diagnostics; want {}\n{diag}",
            test.input,
            diag.len(),
            test.diag_count,
        );

        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.input);
    }
}

// Ported from TestParseWithPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/json/parser_test.go#L622
#[test]
#[ignore = "not yet implemented"]
fn parse_with_pos() {
    struct Case {
        input: &'static str,
        start_pos: Pos,
        want: Node,
        diag_count: usize,
    }

    let tests = [
        // Simple, single-token constructs
        Case {
            input: r#"true"#,
            start_pos: Pos {
                byte: 0,
                line: 3,
                column: 10,
            },
            want: Node::Boolean {
                value: true,
                src_range: rng(pos(0, 3, 10), pos(4, 3, 14)),
            },
            diag_count: 0,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got, diag) = json::parse_file_content(test.input.as_bytes(), "", test.start_pos);

        assert_eq!(
            diag.len(),
            test.diag_count,
            "case {i} ({}): got {} diagnostics; want {}\n{diag}",
            test.input,
            diag.len(),
            test.diag_count,
        );

        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.input);
    }
}
