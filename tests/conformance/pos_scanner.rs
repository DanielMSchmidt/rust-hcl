//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   pos_scanner_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::{Pos, Range, RangeScanner, scan_lines};

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

// Ported from TestPosScanner:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/pos_scanner_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn pos_scanner() {
    struct Case {
        name: &'static str,
        input: &'static str,
        want: Vec<Range>,
        want_toks: Vec<&'static [u8]>,
    }

    let tests = [
        Case {
            name: "empty",
            input: "",
            want: vec![],
            want_toks: vec![],
        },
        Case {
            name: "single line",
            input: "hello",
            want: vec![rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            )],
            want_toks: vec![b"hello"],
        },
        Case {
            name: "single line with trailing UNIX newline",
            input: "hello\n",
            want: vec![rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            )],
            want_toks: vec![b"hello"],
        },
        Case {
            name: "single line with trailing Windows newline",
            input: "hello\r\n",
            want: vec![rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            )],
            want_toks: vec![b"hello"],
        },
        Case {
            name: "two lines with UNIX newline",
            input: "hello\nworld",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 5,
                        line: 1,
                        column: 6,
                    },
                ),
                rng(
                    Pos {
                        byte: 6,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 11,
                        line: 2,
                        column: 6,
                    },
                ),
            ],
            want_toks: vec![b"hello", b"world"],
        },
        Case {
            name: "two lines with Windows newline",
            input: "hello\r\nworld",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 5,
                        line: 1,
                        column: 6,
                    },
                ),
                rng(
                    Pos {
                        byte: 7,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 12,
                        line: 2,
                        column: 6,
                    },
                ),
            ],
            want_toks: vec![b"hello", b"world"],
        },
        Case {
            name: "blank line with UNIX newlines",
            input: "hello\n\nworld",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 5,
                        line: 1,
                        column: 6,
                    },
                ),
                rng(
                    Pos {
                        byte: 6,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 6,
                        line: 2,
                        column: 1,
                    },
                ),
                rng(
                    Pos {
                        byte: 7,
                        line: 3,
                        column: 1,
                    },
                    Pos {
                        byte: 12,
                        line: 3,
                        column: 6,
                    },
                ),
            ],
            want_toks: vec![b"hello", b"", b"world"],
        },
        Case {
            name: "blank line with Windows newlines",
            input: "hello\r\n\r\nworld",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 5,
                        line: 1,
                        column: 6,
                    },
                ),
                rng(
                    Pos {
                        byte: 7,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 7,
                        line: 2,
                        column: 1,
                    },
                ),
                rng(
                    Pos {
                        byte: 9,
                        line: 3,
                        column: 1,
                    },
                    Pos {
                        byte: 14,
                        line: 3,
                        column: 6,
                    },
                ),
            ],
            want_toks: vec![b"hello", b"", b"world"],
        },
        Case {
            name: "two lines with combiner and UNIX newline",
            input: "foo \u{1f469}\u{1f3ff} bar\nbaz",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 16,
                        line: 1,
                        column: 10,
                    },
                ),
                rng(
                    Pos {
                        byte: 17,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 20,
                        line: 2,
                        column: 4,
                    },
                ),
            ],
            want_toks: vec!["foo \u{1f469}\u{1f3ff} bar".as_bytes(), b"baz"],
        },
        Case {
            name: "two lines with combiner and Windows newline",
            input: "foo \u{1f469}\u{1f3ff} bar\r\nbaz",
            want: vec![
                rng(
                    Pos {
                        byte: 0,
                        line: 1,
                        column: 1,
                    },
                    Pos {
                        byte: 16,
                        line: 1,
                        column: 10,
                    },
                ),
                rng(
                    Pos {
                        byte: 18,
                        line: 2,
                        column: 1,
                    },
                    Pos {
                        byte: 21,
                        line: 2,
                        column: 4,
                    },
                ),
            ],
            want_toks: vec!["foo \u{1f469}\u{1f3ff} bar".as_bytes(), b"baz"],
        },
    ];

    for test in tests.iter() {
        let name = test.name;
        let src = test.input.as_bytes();
        let mut sc = RangeScanner::new(src, "", scan_lines);
        let mut got: Vec<Range> = Vec::new();
        let mut got_toks: Vec<Vec<u8>> = Vec::new();
        while sc.scan() {
            got.push(sc.range());
            got_toks.push(sc.bytes().to_vec());
        }
        // NOTE(port): upstream checks `sc.Err() != nil` here; the Rust
        // `SplitFunc` cannot fail (bufio.SplitFunc's error return is dropped
        // in the Rust API), so there is no `err()` to assert.
        assert_eq!(got, test.want, "case {name:?}: incorrect ranges");
        let want_toks: Vec<Vec<u8>> = test.want_toks.iter().map(|t| t.to_vec()).collect();
        assert_eq!(got_toks, want_toks, "case {name:?}: incorrect tokens");
    }
}
