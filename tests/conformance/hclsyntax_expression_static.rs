//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/expression_static_test.go
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

// Ported from TestTraversalStatic:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_static_test.go#L14
#[test]
#[ignore = "not yet implemented"]
fn traversal_static() {
    let (expr, mut diags) = hclsyntax::parse_expression(
        b"a.b.c",
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    let (got, more_diags) = hcl::abs_traversal_for_expr(&expr);
    diags.extend(more_diags);

    assert_eq!(
        diags.len(),
        0,
        "wrong number of diags {}; want 0\ndiags: {:?}",
        diags.len(),
        *diags,
    );

    let want = Traversal(vec![
        Traverser::Root {
            name: "a".to_string(),
            src_range: rng(
                Pos {
                    line: 1,
                    column: 1,
                    byte: 0,
                },
                Pos {
                    line: 1,
                    column: 2,
                    byte: 1,
                },
            ),
        },
        Traverser::Attr {
            name: "b".to_string(),
            src_range: rng(
                Pos {
                    line: 1,
                    column: 2,
                    byte: 1,
                },
                Pos {
                    line: 1,
                    column: 4,
                    byte: 3,
                },
            ),
        },
        Traverser::Attr {
            name: "c".to_string(),
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
    ]);

    assert_eq!(got, want, "wrong traversal");
}

// Ported from TestTupleStatic:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_static_test.go#L56
#[test]
#[ignore = "not yet implemented"]
fn tuple_static() {
    let (expr, mut diags) = hclsyntax::parse_expression(
        b"[true, false]",
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    let (exprs, more_diags) = hcl::expr_list(&expr);
    diags.extend(more_diags);
    assert_eq!(
        diags.len(),
        0,
        "wrong number of diags {}; want 0\ndiags: {:?}",
        diags.len(),
        *diags,
    );

    assert_eq!(exprs.len(), 2, "wrong length {}; want 2", exprs.len());

    let want = vec![Value::bool(true), Value::bool(false)];
    let mut got = Vec::with_capacity(exprs.len());
    for item_expr in &exprs {
        let (val, val_diags) = item_expr.value(None);
        assert_eq!(
            val_diags.len(),
            0,
            "wrong number of diags {}; want 0\ndiags: {:?}",
            val_diags.len(),
            *val_diags,
        );
        got.push(val);
    }

    assert_eq!(got, want, "wrong element values");
}

// Ported from TestMapStatic:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/expression_static_test.go#L94
#[test]
#[ignore = "not yet implemented"]
fn map_static() {
    let (expr, mut diags) = hclsyntax::parse_expression(
        br#"{"foo":true,"bar":false}"#,
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    let (items, more_diags) = hcl::expr_map(&expr);
    diags.extend(more_diags);
    assert_eq!(
        diags.len(),
        0,
        "wrong number of diags {}; want 0\ndiags: {:?}",
        diags.len(),
        *diags,
    );

    assert_eq!(items.len(), 2, "wrong length {}; want 2", items.len());

    // NOTE(port): upstream collects into a `map[cty.Value]cty.Value` and
    // compares order-insensitively with deep.Equal. `cty::Value` is not
    // hashable in Rust, so we compare the evaluated pairs in source order,
    // which is the order `expr_map` yields them (and the order of the Go
    // `items` slice).
    let want = vec![
        (Value::string("foo"), Value::bool(true)),
        (Value::string("bar"), Value::bool(false)),
    ];
    let mut got = Vec::with_capacity(items.len());
    for item in &items {
        let mut item_diags = hcl::Diagnostics::default();
        let (key, key_diags) = item.key.value(None);
        item_diags.extend(key_diags);
        let (val, val_diags) = item.value.value(None);
        item_diags.extend(val_diags);
        assert_eq!(
            item_diags.len(),
            0,
            "wrong number of diags {}; want 0\ndiags: {:?}",
            item_diags.len(),
            *item_diags,
        );
        got.push((key, val));
    }

    assert_eq!(got, want, "wrong pair values");
}
