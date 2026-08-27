//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/parse_traversal_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax;
use hcl::{Pos, Range, Traversal, Traverser};

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

// Ported from TestParseTraversalAbs:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parse_traversal_test.go#L16
#[test]
#[ignore = "not yet implemented"]
fn parse_traversal_abs() {
    struct Case {
        src: &'static str,
        want: Traversal,
        diag_count: usize,
    }

    let tests = vec![
        Case {
            src: "",
            // NOTE(port): upstream's `want` is a nil hcl.Traversal; the
            // empty Traversal is its Rust analogue.
            want: Traversal(vec![]),
            diag_count: 1, // variable name required
        },
        Case {
            src: "foo",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 0,
        },
        Case {
            src: "foo.bar.baz",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                Traverser::Attr {
                    name: "bar".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                    ),
                },
                Traverser::Attr {
                    name: "baz".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                        Pos {
                            line: 1,
                            column: 12,
                            byte: 11,
                        },
                    ),
                },
            ]),
            diag_count: 0,
        },
        Case {
            src: "foo[1]",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                Traverser::Index {
                    key: Value::number_int(1),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                    ),
                },
            ]),
            diag_count: 0,
        },
        Case {
            src: "foo[1][2]",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                Traverser::Index {
                    key: Value::number_int(1),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                    ),
                },
                Traverser::Index {
                    key: Value::number_int(2),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 10,
                            byte: 9,
                        },
                    ),
                },
            ]),
            diag_count: 0,
        },
        Case {
            src: "foo[1].bar",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                Traverser::Index {
                    key: Value::number_int(1),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                    ),
                },
                Traverser::Attr {
                    name: "bar".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 11,
                            byte: 10,
                        },
                    ),
                },
            ]),
            diag_count: 0,
        },
        Case {
            src: "foo.",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1, // attribute name required
        },
        Case {
            src: "foo[",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1, // index required
        },
        Case {
            src: "foo[index]",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1, // index must be literal
        },
        Case {
            src: "foo[0",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                Traverser::Index {
                    key: Value::number_int(0),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 6,
                            byte: 5,
                        },
                    ),
                },
            ]),
            diag_count: 1, // missing close bracket
        },
        Case {
            src: "foo 0",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1, // extra junk after traversal
        },
        Case {
            src: "foo[*]",
            want: Traversal(vec![
                Traverser::Root {
                    name: "foo".to_string(),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                    ),
                },
                // NOTE(port): upstream leaves `TraverseSplat.Each` as its
                // nil zero value; the empty Traversal is its Rust analogue.
                Traverser::Splat {
                    each: Traversal(vec![]),
                    src_range: rng(
                        Pos {
                            line: 1,
                            column: 4,
                            byte: 3,
                        },
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                    ),
                },
            ]),
            diag_count: 0,
        },
        Case {
            // Still not supporting this.
            src: "foo.*",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1,
        },
        Case {
            // Run this through the unsupported function.
            src: "foo[*].bar",
            want: Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 4,
                        byte: 3,
                    },
                ),
            }]),
            diag_count: 1,
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        // Go subtest t.Run(test.src, ...): ParseTraversalAbs.
        if test.src == "foo[*]" {
            // The foo[*] test will fail because the function we test in
            // this branch does not support the splat syntax. So we will
            // skip this test case here.
            // (Go: t.Skip("skipping test for unsupported splat syntax"))
        } else {
            let (got, diags) = hclsyntax::parse_traversal_abs(
                test.src.as_bytes(),
                "",
                Pos {
                    line: 1,
                    column: 1,
                    byte: 0,
                },
            );
            assert_eq!(
                diags.len(),
                test.diag_count,
                "case {i} ({:?}): wrong number of diagnostics {}; want {}\ndiags: {:?}",
                test.src,
                diags.len(),
                test.diag_count,
                *diags,
            );

            assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.src);
        }

        // Go subtest t.Run(fmt.Sprintf("partial_%s", test.src), ...):
        // ParseTraversalPartial.
        if test.src == "foo[*].bar" {
            // The foo[*].bar test will fail because the function we test in
            // this branch does support the splat syntax and this test is
            // designed to make sure that the other branch still fails with
            // the splat syntax. So we will skip this test case here.
            // (Go: t.Skip("skipping test that fails for splat syntax"))
        } else {
            let (got, diags) = hclsyntax::parse_traversal_partial(
                test.src.as_bytes(),
                "",
                Pos {
                    line: 1,
                    column: 1,
                    byte: 0,
                },
            );
            assert_eq!(
                diags.len(),
                test.diag_count,
                "case partial_{i} ({:?}): wrong number of diagnostics {}; want {}\ndiags: {:?}",
                test.src,
                diags.len(),
                test.diag_count,
                *diags,
            );

            assert_eq!(
                got, test.want,
                "case partial_{i} ({:?}): wrong result",
                test.src,
            );
        }
    }
}
