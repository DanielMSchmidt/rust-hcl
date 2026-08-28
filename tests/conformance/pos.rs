//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   pos_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

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

/// A visual representation of a range's byte offsets, stacked in assertion
/// messages to show how ranges relate (Go: the `visRangeOffsets` test
/// helper).
fn vis_range_offsets(rng: &Range) -> String {
    let mut buf = String::new();
    if rng.end.byte < rng.start.byte {
        // Should never happen, but we'll visualize it anyway so we can
        // more easily debug failing tests.
        for _ in 0..rng.end.byte {
            buf.push(' ');
        }
        for _ in rng.end.byte..rng.start.byte {
            buf.push('!');
        }
        return buf;
    }

    for _ in 0..rng.start.byte {
        buf.push(' ');
    }
    for _ in rng.start.byte..rng.end.byte {
        buf.push('#');
    }
    buf
}

// Ported from TestRangeOver:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/pos_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn range_over() {
    struct Case {
        a: Range,
        b: Range,
        want: Range,
    }

    let tests = [
        Case {
            //   ##
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            // ####
            a: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            // #####
            want: rng(
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
        },
        Case {
            //   ####
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  #####
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
        },
        Case {
            //  ####
            a: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ##
            b: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //  ###
            a: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //   ###
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //  ####
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            // ##
            a: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            //     ##
            b: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            // ######
            want: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
        },
        Case {
            //     ##
            a: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            // ##
            b: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            // ######
            want: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = Range::over(test.a.clone(), test.b.clone());
        assert_eq!(
            got,
            test.want,
            "case {i} ({}<=>{}): wrong result\nA   : {:<10} {}\nB   : {:<10} {}\ngot : {:<10} {}\nwant: {:<10} {}",
            test.a,
            test.b,
            vis_range_offsets(&test.a),
            test.a,
            vis_range_offsets(&test.b),
            test.b,
            vis_range_offsets(&got),
            got,
            vis_range_offsets(&test.want),
            test.want,
        );
    }
}

// Ported from TestPosOverlap:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/pos_test.go#L163
#[test]
#[ignore = "not yet implemented"]
fn pos_overlap() {
    struct Case {
        a: Range,
        b: Range,
        want: Range,
    }

    let tests = [
        Case {
            //   ##
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ##
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            // ####
            a: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ###
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            //   ####
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ###
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //  ####
            a: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ##
            b: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //   ##
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            //  ###
            a: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ###
            want: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            //   ###
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ###
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //  ####
            a: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            b: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            want: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            // ##
            a: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            //     ##
            b: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            // (no overlap)
            want: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
            ),
        },
        Case {
            //     ##
            a: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 6,
                    line: 1,
                    column: 7,
                },
            ),
            // ##
            b: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            // (no overlap)
            want: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = test.a.overlap(&test.b);
        assert_eq!(
            got,
            test.want,
            "case {i} ({}<=>{}): wrong result\nA   : {:<10} {}\nB   : {:<10} {}\ngot : {:<10} {}\nwant: {:<10} {}",
            test.a,
            test.b,
            vis_range_offsets(&test.a),
            test.a,
            vis_range_offsets(&test.b),
            test.b,
            vis_range_offsets(&got),
            got,
            vis_range_offsets(&test.want),
            test.want,
        );
    }
}

// Ported from TestRangePartitionAround:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/pos_test.go#L313
#[test]
#[ignore = "not yet implemented"]
fn range_partition_around() {
    struct Case {
        outer: Range,
        inner: Range,
        want_before: Range,
        want_overlap: Range,
        want_after: Range,
    }

    let tests = [
        Case {
            //   ##
            outer: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            inner: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            // (empty)
            want_before: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            //   ##
            want_overlap: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            // (empty)
            want_after: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            // ####
            outer: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  ####
            inner: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            // #
            want_before: rng(
                Pos {
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
            ),
            //  ###
            want_overlap: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            // (empty)
            want_after: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
        },
        Case {
            //   ####
            outer: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  ####
            inner: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //  (empty)
            want_before: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            //   ###
            want_overlap: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //      #
            want_after: rng(
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
        Case {
            //  ####
            outer: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
            //   ##
            inner: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //  #
            want_before: rng(
                Pos {
                    byte: 1,
                    line: 1,
                    column: 2,
                },
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
            ),
            //   ##
            want_overlap: rng(
                Pos {
                    byte: 2,
                    line: 1,
                    column: 3,
                },
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
            ),
            //     #
            want_after: rng(
                Pos {
                    byte: 4,
                    line: 1,
                    column: 5,
                },
                Pos {
                    byte: 5,
                    line: 1,
                    column: 6,
                },
            ),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got_before, got_overlap, got_after) = test.outer.partition_around(&test.inner);
        assert_eq!(
            got_before, test.want_before,
            "case {i} ({} around {}): wrong before",
            test.outer, test.inner,
        );
        assert_eq!(
            got_overlap, test.want_overlap,
            "case {i} ({} around {}): wrong overlap",
            test.outer, test.inner,
        );
        assert_eq!(
            got_after, test.want_after,
            "case {i} ({} around {}): wrong after",
            test.outer, test.inner,
        );
    }
}
