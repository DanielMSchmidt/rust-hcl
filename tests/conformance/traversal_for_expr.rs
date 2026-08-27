//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   traversal_for_expr_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::any::Any;

use cty::Value;
use hcl::{Diagnostics, EvalContext, ExprRef, Expression, Range, Traversal, Traverser};

/// The [`Expression`] methods every helper type below shares, mirroring the
/// zero-value `staticExpr` the Go helpers embed (Go: `staticExpr` in
/// `static_expr.go`; its zero `val` is `cty.NilVal`, which has no rust-cty
/// analogue, but none of these tests ever call `value()`).
macro_rules! static_expr_base {
    () => {
        fn value(&self, _ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
            unreachable!("not called by these tests")
        }

        fn variables(&self) -> Vec<Traversal> {
            Vec::new()
        }

        fn range(&self) -> Range {
            Range::default()
        }

        fn start_range(&self) -> Range {
            Range::default()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    };
}

/// Go: `asTraversalSupported` — supports `AsTraversal`, yielding a single
/// root step.
#[derive(Debug)]
struct AsTraversalSupported {
    root_name: &'static str,
}

impl Expression for AsTraversalSupported {
    static_expr_base!();

    fn as_traversal(&self) -> Option<Traversal> {
        Some(Traversal(vec![Traverser::Root {
            name: self.root_name.to_string(),
            src_range: Range::default(),
        }]))
    }
}

/// Go: `asTraversalSupportedAttr` — supports `AsTraversal`, yielding a root
/// step followed by an attribute step.
#[derive(Debug)]
struct AsTraversalSupportedAttr {
    root_name: &'static str,
    attr_name: &'static str,
}

impl Expression for AsTraversalSupportedAttr {
    static_expr_base!();

    fn as_traversal(&self) -> Option<Traversal> {
        Some(Traversal(vec![
            Traverser::Root {
                name: self.root_name.to_string(),
                src_range: Range::default(),
            },
            Traverser::Attr {
                name: self.attr_name.to_string(),
                src_range: Range::default(),
            },
        ]))
    }
}

/// Go: `asTraversalNotSupported` — does not implement `AsTraversal` at all
/// (relies on the trait default here).
#[derive(Debug)]
struct AsTraversalNotSupported;

impl Expression for AsTraversalNotSupported {
    static_expr_base!();
}

/// Go: `asTraversalDeclined` — implements `AsTraversal` but declines by
/// returning nil.
#[derive(Debug)]
struct AsTraversalDeclined;

impl Expression for AsTraversalDeclined {
    static_expr_base!();

    fn as_traversal(&self) -> Option<Traversal> {
        None
    }
}

/// Go: `asTraversalWrappedDelegated` — delegates via `UnwrapExpression` to a
/// wrapped expression.
#[derive(Debug)]
struct AsTraversalWrappedDelegated {
    original: ExprRef,
}

impl Expression for AsTraversalWrappedDelegated {
    static_expr_base!();

    fn unwrap_expression(&self) -> Option<ExprRef> {
        Some(self.original.clone())
    }
}

// Ported from TestAbsTraversalForExpr:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/traversal_for_expr_test.go#L61
#[test]
#[ignore = "not yet implemented"]
fn abs_traversal_for_expr() {
    struct Case {
        expr: ExprRef,
        want_root_name: &'static str,
    }

    let tests = [
        Case {
            expr: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
            want_root_name: "foo",
        },
        Case {
            expr: ExprRef::new(AsTraversalNotSupported),
            want_root_name: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalDeclined),
            want_root_name: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalWrappedDelegated {
                original: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
            }),
            want_root_name: "foo",
        },
        Case {
            expr: ExprRef::new(AsTraversalWrappedDelegated {
                original: ExprRef::new(AsTraversalWrappedDelegated {
                    original: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
                }),
            }),
            want_root_name: "foo",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got, diags) = hcl::abs_traversal_for_expr(&*test.expr);
        // NOTE(port): Go switches on `got != nil`; the Rust signature has no
        // nil traversal, so the failure case is the empty traversal.
        if !got.is_empty() {
            assert!(
                !test.want_root_name.is_empty(),
                "case {i}: traversal was returned; want error"
            );
            assert_eq!(got.len(), 1, "case {i}: wrong traversal length; want 1");
            match &got[0] {
                Traverser::Root { name, .. } => {
                    assert_eq!(
                        name, test.want_root_name,
                        "case {i}: wrong root name {name:?}; want {:?}",
                        test.want_root_name,
                    );
                }
                other => {
                    panic!("case {i}: first traversal step is {other:?}; want Traverser::Root")
                }
            }
        } else {
            assert!(
                diags.has_errors(),
                "case {i}: returned empty traversal without error diagnostics"
            );
            assert!(
                test.want_root_name.is_empty(),
                "case {i}: traversal was not returned; want Traverser::Root({:?})",
                test.want_root_name,
            );
        }
    }
}

// Ported from TestRelTraversalForExpr:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/traversal_for_expr_test.go#L124
#[test]
#[ignore = "not yet implemented"]
fn rel_traversal_for_expr() {
    struct Case {
        expr: ExprRef,
        want_first_name: &'static str,
    }

    let tests = [
        Case {
            expr: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
            want_first_name: "foo",
        },
        Case {
            expr: ExprRef::new(AsTraversalNotSupported),
            want_first_name: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalDeclined),
            want_first_name: "",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (got, diags) = hcl::rel_traversal_for_expr(&*test.expr);
        // NOTE(port): Go switches on `got != nil`; the Rust signature has no
        // nil traversal, so the failure case is the empty traversal.
        if !got.is_empty() {
            assert!(
                !test.want_first_name.is_empty(),
                "case {i}: traversal was returned; want error"
            );
            assert_eq!(got.len(), 1, "case {i}: wrong traversal length; want 1");
            match &got[0] {
                Traverser::Attr { name, .. } => {
                    assert_eq!(
                        name, test.want_first_name,
                        "case {i}: wrong root name {name:?}; want {:?}",
                        test.want_first_name,
                    );
                }
                other => {
                    panic!("case {i}: first traversal step is {other:?}; want Traverser::Attr")
                }
            }
        } else {
            assert!(
                diags.has_errors(),
                "case {i}: returned empty traversal without error diagnostics"
            );
            assert!(
                test.want_first_name.is_empty(),
                "case {i}: traversal was not returned; want Traverser::Attr({:?})",
                test.want_first_name,
            );
        }
    }
}

// Ported from TestExprAsKeyword:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/traversal_for_expr_test.go#L173
#[test]
#[ignore = "not yet implemented"]
fn expr_as_keyword() {
    struct Case {
        expr: ExprRef,
        want: &'static str,
    }

    let tests = [
        Case {
            expr: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
            want: "foo",
        },
        Case {
            expr: ExprRef::new(AsTraversalSupportedAttr {
                root_name: "foo",
                attr_name: "bar",
            }),
            want: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalNotSupported),
            want: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalDeclined),
            want: "",
        },
        Case {
            expr: ExprRef::new(AsTraversalWrappedDelegated {
                original: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
            }),
            want: "foo",
        },
        Case {
            expr: ExprRef::new(AsTraversalWrappedDelegated {
                original: ExprRef::new(AsTraversalWrappedDelegated {
                    original: ExprRef::new(AsTraversalSupported { root_name: "foo" }),
                }),
            }),
            want: "foo",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hcl::expr_as_keyword(&*test.expr);
        assert_eq!(
            got, test.want,
            "case {i}: wrong result {got:?}; want {:?}\ninput: {:?}",
            test.want, test.expr,
        );
    }
}
