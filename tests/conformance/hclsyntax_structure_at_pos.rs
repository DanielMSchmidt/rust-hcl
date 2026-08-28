//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/structure_at_pos_test.go
//!   hclsyntax/navigation_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax;
use hcl::{Pos, Range};

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

/// The Go zero value `hcl.Range{}`.
fn zero_range() -> Range {
    rng(
        Pos {
            line: 0,
            column: 0,
            byte: 0,
        },
        Pos {
            line: 0,
            column: 0,
            byte: 0,
        },
    )
}

/// A position with only the byte offset set (Go: `hcl.Pos{Byte: n}` with
/// `Line` and `Column` left as their zero values).
fn byte_pos(byte: usize) -> Pos {
    Pos {
        line: 0,
        column: 0,
        byte,
    }
}

// Ported from TestBlocksAtPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_at_pos_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn blocks_at_pos() {
    struct Case {
        name: &'static str,
        src: &'static str,
        pos: Pos,
        want_types: &'static [&'static str],
    }

    // NOTE(port): upstream is a map keyed by case name; cases are kept in
    // source order. The multi-line sources are Go raw string literals whose
    // leading tabs (from the test file's indentation) are part of the source,
    // so the tab characters below are literal content, not formatting.
    let tests = [
        Case {
            name: "empty",
            src: "",
            pos: byte_pos(0),
            want_types: &[],
        },
        Case {
            name: "spaces",
            src: "    ",
            pos: byte_pos(1),
            want_types: &[],
        },
        Case {
            name: "single in header",
            src: "foo {}",
            pos: byte_pos(1),
            want_types: &["foo"],
        },
        Case {
            name: "single in body",
            src: "foo {    }",
            pos: byte_pos(7),
            want_types: &["foo"],
        },
        Case {
            name: "single in body with unselected nested",
            src: "\n\t\t\tfoo {\n\n\t\t\t\tbar {\n\n\t\t\t\t}\n\t\t\t}\n\t\t\t",
            pos: byte_pos(10),
            want_types: &["foo"],
        },
        Case {
            name: "single in body with unselected sibling",
            src: "\n\t\t\tfoo {  }\n\t\t\tbar {  }\n\t\t\t",
            pos: byte_pos(10),
            want_types: &["foo"],
        },
        Case {
            name: "selected nested two levels",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar {\n\n\t\t\t\t}\n\t\t\t}\n\t\t\t",
            pos: byte_pos(20),
            want_types: &["foo", "bar"],
        },
        Case {
            name: "selected nested three levels",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar {\n\t\t\t\t\tbaz {\n\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\t",
            pos: byte_pos(31),
            want_types: &["foo", "bar", "baz"],
        },
        Case {
            name: "selected nested three levels with unselected sibling after",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar {\n\t\t\t\t\tbaz {\n\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\tnot_wanted {}\n\t\t\t}\n\t\t\t",
            pos: byte_pos(31),
            want_types: &["foo", "bar", "baz"],
        },
        Case {
            name: "selected nested three levels with unselected sibling before",
            src: "\n\t\t\tfoo {\n\t\t\t\tnot_wanted {}\n\t\t\t\tbar {\n\t\t\t\t\tbaz {\n\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\t",
            pos: byte_pos(49),
            want_types: &["foo", "bar", "baz"],
        },
        Case {
            name: "unterminated",
            src: "foo {    ",
            pos: byte_pos(7),
            want_types: &["foo"],
        },
        Case {
            name: "unterminated nested",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar {\n\t\t\t}\n\t\t\t",
            pos: byte_pos(16),
            want_types: &["foo", "bar"],
        },
        Case {
            // customblock "fi"\n
            // Position inside the label "fi" should be inside the block range.
            name: "no braces with label newline",
            src: "customblock \"fi\"\n",
            pos: byte_pos(13), // inside "fi"
            want_types: &["customblock"],
        },
        Case {
            // resource "aws_instance" "foo"\n
            // Position inside the second label should be inside the block range.
            name: "no braces two labels newline",
            src: "resource \"aws_instance\" \"foo\"\n",
            pos: byte_pos(25), // inside "foo"
            want_types: &["resource"],
        },
        Case {
            // resource "aws_instance" "foo" (no newline, EOF)
            // Position inside the second label should be inside the block range.
            name: "no braces two labels eof",
            src: "resource \"aws_instance\" \"foo\"",
            pos: byte_pos(25), // inside "foo"
            want_types: &["resource"],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        // Diagnostics are intentionally ignored: we should be able to work
        // with the incomplete configuration that results when the parser does
        // its recovery behavior.
        let (f, _diags) = hclsyntax::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );

        let blocks = f.blocks_at_pos(test.pos);
        let outermost = f.outermost_block_at_pos(test.pos);
        let innermost = f.innermost_block_at_pos(test.pos);

        let got_types: Vec<&str> = blocks.iter().map(|b| b.block_type.as_str()).collect();

        if test.want_types.is_empty() {
            assert!(
                got_types.is_empty(),
                "case {i} ({}): wrong block types\ngot:  {got_types:?}\nwant: (none)",
                test.name,
            );
            assert!(
                outermost.is_none(),
                "case {i} ({}): wrong outermost type\ngot:  {:?}\nwant: (none)",
                test.name,
                outermost.as_ref().map(|b| &b.block_type),
            );
            assert!(
                innermost.is_none(),
                "case {i} ({}): wrong innermost type\ngot:  {:?}\nwant: (none)",
                test.name,
                innermost.as_ref().map(|b| &b.block_type),
            );
            continue;
        }

        assert_eq!(
            got_types, test.want_types,
            "case {i} ({}): wrong block types",
            test.name,
        );
        assert_eq!(
            outermost.unwrap().block_type,
            test.want_types[0],
            "case {i} ({}): wrong outermost type",
            test.name,
        );
        assert_eq!(
            innermost.unwrap().block_type,
            *test.want_types.last().unwrap(),
            "case {i} ({}): wrong innermost type",
            test.name,
        );
    }
}

// Ported from TestAttributeAtPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_at_pos_test.go#L197
#[test]
#[ignore = "not yet implemented"]
fn attribute_at_pos() {
    struct Case {
        name: &'static str,
        src: &'static str,
        pos: Pos,
        want_name: &'static str,
    }

    // NOTE(port): as in `blocks_at_pos`, the leading tabs in the multi-line
    // sources are literal content copied from the Go raw string literals.
    let tests = [
        Case {
            name: "empty",
            src: "",
            pos: byte_pos(0),
            want_name: "",
        },
        Case {
            name: "top-level",
            src: "foo = 1",
            pos: byte_pos(0),
            want_name: "foo",
        },
        Case {
            name: "top-level with ignored sibling after",
            src: "\n\t\t\tfoo = 1\n\t\t\tbar = 2\n\t\t\t",
            pos: byte_pos(6),
            want_name: "foo",
        },
        Case {
            name: "top-level ignored sibling before",
            src: "\n\t\t\tfoo = 1\n\t\t\tbar = 2\n\t\t\t",
            pos: byte_pos(17),
            want_name: "bar",
        },
        Case {
            name: "nested",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar = 2\n\t\t\t}\n\t\t\t",
            pos: byte_pos(17),
            want_name: "bar",
        },
        Case {
            name: "nested in unterminated block",
            src: "\n\t\t\tfoo {\n\t\t\t\tbar = 2\n\t\t\t",
            pos: byte_pos(17),
            want_name: "bar",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        // Diagnostics are intentionally ignored: we should be able to work
        // with the incomplete configuration that results when the parser does
        // its recovery behavior.
        let (f, _diags) = hclsyntax::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );

        let got = f.attribute_at_pos(test.pos);

        if test.want_name.is_empty() {
            assert!(
                got.is_none(),
                "case {i} ({}): wrong attribute name\ngot:  {:?}\nwant: (none)",
                test.name,
                got.as_ref().map(|a| &a.name),
            );
            continue;
        }

        let got = got.unwrap_or_else(|| {
            panic!(
                "case {i} ({}): wrong attribute name\ngot:  (none)\nwant: {:?}",
                test.name, test.want_name,
            )
        });

        assert_eq!(
            got.name, test.want_name,
            "case {i} ({}): wrong attribute name",
            test.name,
        );
    }
}

// Ported from TestOutermostExprAtPos:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/structure_at_pos_test.go#L280
#[test]
#[ignore = "not yet implemented"]
fn outermost_expr_at_pos() {
    struct Case {
        name: &'static str,
        src: &'static str,
        pos: Pos,
        want_src: &'static str,
    }

    let tests = [
        Case {
            name: "empty",
            src: "",
            pos: byte_pos(0),
            want_src: "",
        },
        Case {
            name: "simple bool",
            src: "a = true",
            pos: byte_pos(6),
            want_src: "true",
        },
        Case {
            name: "simple reference",
            src: "a = blah",
            pos: byte_pos(6),
            want_src: "blah",
        },
        Case {
            name: "attribute reference",
            src: "a = blah.foo",
            pos: byte_pos(6),
            want_src: "blah.foo",
        },
        Case {
            name: "parens",
            src: "a = (1 + 1)",
            pos: byte_pos(6),
            want_src: "(1 + 1)",
        },
        Case {
            name: "tuple cons",
            src: "a = [1, 2, 3]",
            pos: byte_pos(5),
            want_src: "[1, 2, 3]",
        },
        Case {
            name: "function call",
            src: "a = foom(\"a\")",
            pos: byte_pos(10),
            want_src: "foom(\"a\")",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let input_src = test.src.as_bytes();
        // Diagnostics are intentionally ignored: we should be able to work
        // with the incomplete configuration that results when the parser does
        // its recovery behavior.
        let (f, _diags) = hclsyntax::parse_config(
            input_src,
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );

        let got_expr = f.outermost_expr_at_pos(test.pos);
        let got_src = match &got_expr {
            Some(expr) => {
                let rng = expr.range();
                String::from_utf8_lossy(rng.slice_bytes(input_src)).into_owned()
            }
            None => String::new(),
        };

        if test.want_src.is_empty() {
            assert!(
                got_expr.is_none(),
                "case {i} ({}): wrong expression source\ngot:  {got_src}\nwant: (none)",
                test.name,
            );
            continue;
        }

        assert!(
            got_expr.is_some(),
            "case {i} ({}): wrong expression source\ngot:  (none)\nwant: {}",
            test.name,
            test.want_src,
        );

        assert_eq!(
            got_src, test.want_src,
            "case {i} ({}): wrong expression source",
            test.name,
        );
    }
}

/// The configuration shared by both navigation tests (upstream `cfg`).
const NAV_CFG: &str = "\n\n\nresource {\n}\n\nresource \"random_type\" {\n}\n\nresource \"null_resource\" \"baz\" {\n  name = \"foo\"\n  boz = {\n  \tone = \"111\"\n  \ttwo = \"22222\"\n  }\n}\n\ndata \"another\" \"baz\" {\n  name = \"foo\"\n  boz = {\n  \tone = \"111\"\n  \ttwo = \"22222\"\n  }\n}\n";

// Ported from TestNavigationContextString:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/navigation_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn navigation_context_string() {
    let (file, diags) = hclsyntax::parse_config(
        NAV_CFG.as_bytes(),
        "",
        Pos {
            byte: 0,
            line: 1,
            column: 1,
        },
    );
    assert_eq!(diags.len(), 0, "unexpected diagnostics: {diags:?}");
    // NOTE(port): upstream also checks `file == nil`; the Rust `parse_config`
    // returns `File` by value, so there is no nil case. The Go type assertion
    // `file.Nav.(navigation)` becomes unwrapping the `Option<Arc<dyn FileNav>>`.
    let nav = file.nav.as_ref().unwrap();

    struct Case {
        offset: usize,
        want: &'static str,
    }

    let test_cases = [
        Case {
            offset: 0,
            want: "",
        },
        Case {
            offset: 2,
            want: "",
        },
        Case {
            offset: 4,
            want: "resource",
        },
        Case {
            offset: 17,
            want: "resource \"random_type\"",
        },
        Case {
            offset: 25,
            want: "resource \"random_type\"",
        },
        Case {
            offset: 45,
            want: "resource \"null_resource\" \"baz\"",
        },
        Case {
            offset: 142,
            want: "data \"another\" \"baz\"",
        },
        Case {
            offset: 180,
            want: "data \"another\" \"baz\"",
        },
        Case {
            offset: 99999,
            want: "",
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        let got = nav.context_string(tc.offset);
        assert_eq!(
            got, tc.want,
            "case {i} (offset {}): wrong result",
            tc.offset,
        );
    }
}

// Ported from TestNavigationContextDefRange:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/navigation_test.go#L76
#[test]
#[ignore = "not yet implemented"]
fn navigation_context_def_range() {
    let (file, diags) = hclsyntax::parse_config(
        NAV_CFG.as_bytes(),
        "",
        Pos {
            byte: 0,
            line: 1,
            column: 1,
        },
    );
    assert_eq!(diags.len(), 0, "unexpected diagnostics: {diags:?}");
    // NOTE(port): upstream also checks `file == nil`; the Rust `parse_config`
    // returns `File` by value, so there is no nil case. The Go type assertion
    // `file.Nav.(navigation)` becomes unwrapping the `Option<Arc<dyn FileNav>>`.
    let nav = file.nav.as_ref().unwrap();

    struct Case {
        offset: usize,
        want_range: Range,
    }

    let test_cases = [
        Case {
            offset: 0,
            want_range: zero_range(),
        },
        Case {
            offset: 2,
            want_range: zero_range(),
        },
        Case {
            offset: 4,
            want_range: rng(
                Pos {
                    line: 4,
                    column: 1,
                    byte: 3,
                },
                Pos {
                    line: 4,
                    column: 9,
                    byte: 11,
                },
            ),
        },
        Case {
            offset: 17,
            want_range: rng(
                Pos {
                    line: 7,
                    column: 1,
                    byte: 17,
                },
                Pos {
                    line: 7,
                    column: 23,
                    byte: 39,
                },
            ),
        },
        Case {
            offset: 25,
            want_range: rng(
                Pos {
                    line: 7,
                    column: 1,
                    byte: 17,
                },
                Pos {
                    line: 7,
                    column: 23,
                    byte: 39,
                },
            ),
        },
        Case {
            offset: 45,
            want_range: rng(
                Pos {
                    line: 10,
                    column: 1,
                    byte: 45,
                },
                Pos {
                    line: 10,
                    column: 31,
                    byte: 75,
                },
            ),
        },
        Case {
            offset: 142,
            want_range: rng(
                Pos {
                    line: 18,
                    column: 1,
                    byte: 142,
                },
                Pos {
                    line: 18,
                    column: 21,
                    byte: 162,
                },
            ),
        },
        Case {
            offset: 180,
            want_range: rng(
                Pos {
                    line: 18,
                    column: 1,
                    byte: 142,
                },
                Pos {
                    line: 18,
                    column: 21,
                    byte: 162,
                },
            ),
        },
        Case {
            offset: 99999,
            want_range: zero_range(),
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        // NOTE(port): `FileNav::context_def_range` returns `Option<Range>`
        // because the Option models Go's optional `contextDefRanger`
        // interface; hclsyntax's navigation supports it, so `unwrap()` here.
        // Go's "no context" result is the zero value `hcl.Range{}`, ported
        // literally as `zero_range()`.
        let got = nav.context_def_range(tc.offset).unwrap();
        assert_eq!(
            got, tc.want_range,
            "case {i} (offset {}): wrong range",
            tc.offset,
        );
    }
}
