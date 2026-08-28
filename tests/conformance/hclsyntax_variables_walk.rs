//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/variables_test.go
//!   hclsyntax/walk_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax::{
    self, BinaryOpExpr, ConditionalExpr, Expression, ForExpr, FunctionCallExpr, LiteralValueExpr,
    Node, Operation, RelativeTraversalExpr, ScopeTraversalExpr, UnaryOpExpr, Walker,
};
use hcl::{Diagnostics, Pos, Range, Traversal, Traverser};

// Ported from TestVariables:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/variables_test.go#L16
#[test]
#[ignore = "not yet implemented"]
fn variables() {
    struct Case {
        expr: Expression,
        want: Vec<Traversal>,
    }

    // NOTE(port): upstream leaves every SrcRange field at its Go zero value;
    // those port as `Range::default()`.
    let tests = [
        Case {
            expr: LiteralValueExpr {
                val: Value::bool(true),
                src_range: Range::default(),
            }
            .into(),
            // NOTE(port): upstream `Want` is nil (a nil slice); the nearest
            // Rust analogue of a nil `[]hcl.Traversal` is an empty vec.
            want: vec![],
        },
        Case {
            expr: ScopeTraversalExpr {
                traversal: Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: Range::default(),
                }]),
                src_range: Range::default(),
            }
            .into(),
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: Range::default(),
            }])],
        },
        Case {
            expr: BinaryOpExpr {
                lhs: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "foo".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                op: Operation::Add,
                rhs: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "bar".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                src_range: Range::default(),
            }
            .into(),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: Range::default(),
                }]),
            ],
        },
        Case {
            expr: UnaryOpExpr {
                val: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "foo".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                op: Operation::Negate,
                src_range: Range::default(),
                symbol_range: Range::default(),
            }
            .into(),
            want: vec![Traversal(vec![Traverser::Root {
                name: "foo".to_string(),
                src_range: Range::default(),
            }])],
        },
        Case {
            expr: ConditionalExpr {
                condition: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "foo".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                true_result: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "bar".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                false_result: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "baz".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                src_range: Range::default(),
            }
            .into(),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "baz".to_string(),
                    src_range: Range::default(),
                }]),
            ],
        },
        Case {
            expr: ForExpr {
                key_var: "k".to_string(),
                val_var: "v".to_string(),
                coll_expr: ScopeTraversalExpr {
                    traversal: Traversal(vec![Traverser::Root {
                        name: "foo".to_string(),
                        src_range: Range::default(),
                    }]),
                    src_range: Range::default(),
                }
                .into(),
                key_expr: Some(
                    BinaryOpExpr {
                        lhs: ScopeTraversalExpr {
                            traversal: Traversal(vec![Traverser::Root {
                                name: "k".to_string(),
                                src_range: Range::default(),
                            }]),
                            src_range: Range::default(),
                        }
                        .into(),
                        op: Operation::Add,
                        rhs: ScopeTraversalExpr {
                            traversal: Traversal(vec![Traverser::Root {
                                name: "bar".to_string(),
                                src_range: Range::default(),
                            }]),
                            src_range: Range::default(),
                        }
                        .into(),
                        src_range: Range::default(),
                    }
                    .into(),
                ),
                val_expr: BinaryOpExpr {
                    lhs: ScopeTraversalExpr {
                        traversal: Traversal(vec![Traverser::Root {
                            name: "v".to_string(),
                            src_range: Range::default(),
                        }]),
                        src_range: Range::default(),
                    }
                    .into(),
                    op: Operation::Add,
                    rhs: ScopeTraversalExpr {
                        traversal: Traversal(vec![Traverser::Root {
                            name: "baz".to_string(),
                            src_range: Range::default(),
                        }]),
                        src_range: Range::default(),
                    }
                    .into(),
                    src_range: Range::default(),
                }
                .into(),
                cond_expr: Some(
                    BinaryOpExpr {
                        lhs: ScopeTraversalExpr {
                            traversal: Traversal(vec![Traverser::Root {
                                name: "k".to_string(),
                                src_range: Range::default(),
                            }]),
                            src_range: Range::default(),
                        }
                        .into(),
                        op: Operation::LessThan,
                        rhs: ScopeTraversalExpr {
                            traversal: Traversal(vec![Traverser::Root {
                                name: "limit".to_string(),
                                src_range: Range::default(),
                            }]),
                            src_range: Range::default(),
                        }
                        .into(),
                        src_range: Range::default(),
                    }
                    .into(),
                ),
                group: false,
                src_range: Range::default(),
                open_range: Range::default(),
                close_range: Range::default(),
            }
            .into(),
            want: vec![
                Traversal(vec![Traverser::Root {
                    name: "foo".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "bar".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "baz".to_string(),
                    src_range: Range::default(),
                }]),
                Traversal(vec![Traverser::Root {
                    name: "limit".to_string(),
                    src_range: Range::default(),
                }]),
            ],
        },
        Case {
            expr: ScopeTraversalExpr {
                traversal: Traversal(vec![
                    Traverser::Root {
                        name: "data".to_string(),
                        src_range: Range::default(),
                    },
                    Traverser::Attr {
                        name: "null_data_source".to_string(),
                        src_range: Range::default(),
                    },
                    Traverser::Attr {
                        name: "multi".to_string(),
                        src_range: Range::default(),
                    },
                    Traverser::Index {
                        key: Value::number_float(0.0),
                        src_range: Range::default(),
                    },
                ]),
                src_range: Range::default(),
            }
            .into(),
            want: vec![Traversal(vec![
                Traverser::Root {
                    name: "data".to_string(),
                    src_range: Range::default(),
                },
                Traverser::Attr {
                    name: "null_data_source".to_string(),
                    src_range: Range::default(),
                },
                Traverser::Attr {
                    name: "multi".to_string(),
                    src_range: Range::default(),
                },
                Traverser::Index {
                    key: Value::number_float(0.0),
                    src_range: Range::default(),
                },
            ])],
        },
        Case {
            expr: RelativeTraversalExpr {
                source: FunctionCallExpr {
                    name: "sort".to_string(),
                    args: vec![
                        ScopeTraversalExpr {
                            traversal: Traversal(vec![
                                Traverser::Root {
                                    name: "data".to_string(),
                                    src_range: Range::default(),
                                },
                                Traverser::Attr {
                                    name: "null_data_source".to_string(),
                                    src_range: Range::default(),
                                },
                                Traverser::Attr {
                                    name: "multi".to_string(),
                                    src_range: Range::default(),
                                },
                            ]),
                            src_range: Range::default(),
                        }
                        .into(),
                    ],
                    expand_final: false,
                    name_range: Range::default(),
                    open_paren_range: Range::default(),
                    close_paren_range: Range::default(),
                }
                .into(),
                traversal: Traversal(vec![Traverser::Index {
                    key: Value::number_float(0.0),
                    src_range: Range::default(),
                }]),
                src_range: Range::default(),
            }
            .into(),
            want: vec![Traversal(vec![
                Traverser::Root {
                    name: "data".to_string(),
                    src_range: Range::default(),
                },
                Traverser::Attr {
                    name: "null_data_source".to_string(),
                    src_range: Range::default(),
                },
                Traverser::Attr {
                    name: "multi".to_string(),
                    src_range: Range::default(),
                },
            ])],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclsyntax::variables(&test.expr);
        assert_eq!(got, test.want, "case {i} ({:?}): wrong result", test.expr,);
    }
}

/// Which walker method was called (Go: `testWalkMethod` with the
/// `testWalkEnter`/`testWalkExit` constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestWalkMethod {
    Enter,
    Exit,
}

/// One recorded walker call (Go: `testWalkCall`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestWalkCall {
    method: TestWalkMethod,
    node_type: String,
}

/// The Go type name of the dynamic type behind a `Node`, as Go's `%T` verb
/// would print it. Upstream records each visited node with
/// `fmt.Sprintf("%T", node)` and asserts the resulting strings; the Rust
/// `Node`/`Expression` enums stand in for Go's interface, so this helper
/// maps each variant back to the exact upstream type-name string.
fn go_node_type_name(node: Node<'_>) -> String {
    let name = match node {
        Node::Body(_) => "*hclsyntax.Body",
        Node::Attributes(_) => "hclsyntax.Attributes",
        Node::Attribute(_) => "*hclsyntax.Attribute",
        Node::Blocks(_) => "hclsyntax.Blocks",
        Node::Block(_) => "*hclsyntax.Block",
        Node::ChildScope(_) => "hclsyntax.ChildScope",
        Node::Expr(expr) => match expr {
            Expression::LiteralValue(_) => "*hclsyntax.LiteralValueExpr",
            Expression::ScopeTraversal(_) => "*hclsyntax.ScopeTraversalExpr",
            Expression::RelativeTraversal(_) => "*hclsyntax.RelativeTraversalExpr",
            Expression::FunctionCall(_) => "*hclsyntax.FunctionCallExpr",
            Expression::Conditional(_) => "*hclsyntax.ConditionalExpr",
            Expression::Index(_) => "*hclsyntax.IndexExpr",
            Expression::TupleCons(_) => "*hclsyntax.TupleConsExpr",
            Expression::ObjectCons(_) => "*hclsyntax.ObjectConsExpr",
            Expression::ObjectConsKey(_) => "*hclsyntax.ObjectConsKeyExpr",
            Expression::For(_) => "*hclsyntax.ForExpr",
            Expression::Splat(_) => "*hclsyntax.SplatExpr",
            Expression::AnonSymbol(_) => "*hclsyntax.AnonSymbolExpr",
            Expression::BinaryOp(_) => "*hclsyntax.BinaryOpExpr",
            Expression::UnaryOp(_) => "*hclsyntax.UnaryOpExpr",
            Expression::Template(_) => "*hclsyntax.TemplateExpr",
            Expression::TemplateJoin(_) => "*hclsyntax.TemplateJoinExpr",
            Expression::TemplateWrap(_) => "*hclsyntax.TemplateWrapExpr",
            Expression::Parentheses(_) => "*hclsyntax.ParenthesesExpr",
            Expression::SyntaxError(_) => "*hclsyntax.ExprSyntaxError",
        },
    };
    name.to_string()
}

/// A walker recording every call made to it (Go: `testWalker`).
#[derive(Default)]
struct TestWalker {
    calls: Vec<TestWalkCall>,
}

impl Walker for TestWalker {
    fn enter(&mut self, node: Node<'_>) -> Diagnostics {
        self.calls.push(TestWalkCall {
            method: TestWalkMethod::Enter,
            node_type: go_node_type_name(node),
        });
        Diagnostics::new()
    }

    fn exit(&mut self, node: Node<'_>) -> Diagnostics {
        self.calls.push(TestWalkCall {
            method: TestWalkMethod::Exit,
            node_type: go_node_type_name(node),
        });
        Diagnostics::new()
    }
}

// Ported from TestWalk:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/walk_test.go#L17
#[test]
#[ignore = "not yet implemented"]
fn walk() {
    /// A `(method, Go type name)` pair for the expected call lists.
    fn call(method: TestWalkMethod, node_type: &str) -> TestWalkCall {
        TestWalkCall {
            method,
            node_type: node_type.to_string(),
        }
    }
    use TestWalkMethod::{Enter, Exit};

    struct Case {
        src: &'static str,
        want: Vec<TestWalkCall>,
    }

    let tests = [
        Case {
            src: r#"1"#,
            want: vec![
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
            ],
        },
        Case {
            src: r#"foo"#,
            want: vec![
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
            ],
        },
        Case {
            src: r#"1 + 1"#,
            want: vec![
                call(Enter, "*hclsyntax.BinaryOpExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.BinaryOpExpr"),
            ],
        },
        Case {
            src: r#"(1 + 1)"#,
            want: vec![
                call(Enter, "*hclsyntax.ParenthesesExpr"),
                call(Enter, "*hclsyntax.BinaryOpExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.BinaryOpExpr"),
                call(Exit, "*hclsyntax.ParenthesesExpr"),
            ],
        },
        Case {
            src: r#"a[0]"#,
            want: vec![
                // because the index is constant here, the index is absorbed
                // into the traversal
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
            ],
        },
        Case {
            // semantically incorrect, but should still parse and be walkable
            src: r#"0[foo]"#,
            want: vec![
                call(Enter, "*hclsyntax.IndexExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.IndexExpr"),
            ],
        },
        Case {
            src: r#"bar()"#,
            want: vec![
                call(Enter, "*hclsyntax.FunctionCallExpr"),
                call(Exit, "*hclsyntax.FunctionCallExpr"),
            ],
        },
        Case {
            src: r#"bar(1, a)"#,
            want: vec![
                call(Enter, "*hclsyntax.FunctionCallExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.FunctionCallExpr"),
            ],
        },
        Case {
            src: r#"bar(1, a)[0]"#,
            want: vec![
                call(Enter, "*hclsyntax.RelativeTraversalExpr"),
                call(Enter, "*hclsyntax.FunctionCallExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.FunctionCallExpr"),
                call(Exit, "*hclsyntax.RelativeTraversalExpr"),
            ],
        },
        Case {
            src: r#"[for x in foo: x + 1 if x < 10]"#,
            want: vec![
                call(Enter, "*hclsyntax.ForExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Enter, "hclsyntax.ChildScope"),
                call(Enter, "*hclsyntax.BinaryOpExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.BinaryOpExpr"),
                call(Exit, "hclsyntax.ChildScope"),
                call(Enter, "hclsyntax.ChildScope"),
                call(Enter, "*hclsyntax.BinaryOpExpr"),
                call(Enter, "*hclsyntax.ScopeTraversalExpr"),
                call(Exit, "*hclsyntax.ScopeTraversalExpr"),
                call(Enter, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.LiteralValueExpr"),
                call(Exit, "*hclsyntax.BinaryOpExpr"),
                call(Exit, "hclsyntax.ChildScope"),
                call(Exit, "*hclsyntax.ForExpr"),
            ],
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (expr, diags) = hclsyntax::parse_expression(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            !diags.has_errors(),
            "case {i} ({:?}): failed to parse expression: {:?}",
            test.src,
            *diags,
        );

        let mut w = TestWalker::default();
        // NOTE(port): Go passes `expr` directly because every expression
        // implements the `Node` interface; the Rust `Node` enum of borrows
        // requires the explicit `Node::Expr` wrapping.
        let diags = hclsyntax::walk(Node::Expr(&expr), &mut w);
        assert!(
            !diags.has_errors(),
            "case {i} ({:?}): failed to walk: {:?}",
            test.src,
            *diags,
        );

        let got = &w.calls;
        assert_eq!(got, &test.want, "case {i} ({:?}): wrong calls", test.src);
    }
}
