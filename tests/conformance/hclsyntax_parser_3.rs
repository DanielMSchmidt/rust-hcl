//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/parser_test.go (TestParseConfig, part 3)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax::{
    self, Attribute, Attributes, Block, Body, ExprSyntaxError, ForExpr, FunctionCallExpr,
    IndexExpr, LiteralValueExpr, ObjectConsExpr, ObjectConsItem, ObjectConsKeyExpr,
    ScopeTraversalExpr,
};
use hcl::{Diagnostic, DiagnosticSeverity, Diagnostics, Pos, Range, Traversal, Traverser};

/// A range within an unnamed file (Go: `hcl.Range{Start: ..., End: ...}`
/// with `Filename` left as its zero value).
fn rng(start: Pos, end: Pos) -> Range {
    Range {
        filename: String::new(),
        start,
        end,
    }
}

// Ported from TestParseConfig:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parser_test.go#L18
// (part 3: cases with opening brace at upstream line 1881 or later)
#[test]
#[ignore = "not yet implemented"]
fn parse_config_part3() {
    struct Case {
        input: &'static str,
        diag_count: usize,
        want: Body,
    }

    let tests = [
        Case {
            input: "a = 1 # line comment\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: LiteralValueExpr {
                            val: Value::number_int(1),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 6,
                                byte: 5,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 21,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 21,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 21,
                    },
                ),
            },
        },
        Case {
            input: "a = [for k, v in foo: v if true]\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ForExpr {
                            key_var: "k".to_string(),
                            val_var: "v".to_string(),
                            coll_expr: ScopeTraversalExpr {
                                traversal: Traversal(vec![Traverser::Root {
                                    name: "foo".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 18,
                                            byte: 17,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 21,
                                            byte: 20,
                                        },
                                    ),
                                }]),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 18,
                                        byte: 17,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 21,
                                        byte: 20,
                                    },
                                ),
                            }
                            .into(),
                            key_expr: None,
                            val_expr: ScopeTraversalExpr {
                                traversal: Traversal(vec![Traverser::Root {
                                    name: "v".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 23,
                                            byte: 22,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 24,
                                            byte: 23,
                                        },
                                    ),
                                }]),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 23,
                                        byte: 22,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 24,
                                        byte: 23,
                                    },
                                ),
                            }
                            .into(),
                            cond_expr: Some(
                                LiteralValueExpr {
                                    val: Value::bool(true),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 28,
                                            byte: 27,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 32,
                                            byte: 31,
                                        },
                                    ),
                                }
                                .into(),
                            ),
                            group: false,
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 33,
                                    byte: 32,
                                },
                            ),
                            open_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                            close_range: rng(
                                Pos {
                                    line: 1,
                                    column: 32,
                                    byte: 31,
                                },
                                Pos {
                                    line: 1,
                                    column: 33,
                                    byte: 32,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 33,
                                byte: 32,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 33,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 33,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 33,
                    },
                ),
            },
        },
        Case {
            input: "a = [for k, v in foo: k => v... if true]\n",
            diag_count: 2, // can't use => or ... in a tuple for
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ForExpr {
                            key_var: "k".to_string(),
                            val_var: "v".to_string(),
                            coll_expr: ScopeTraversalExpr {
                                traversal: Traversal(vec![Traverser::Root {
                                    name: "foo".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 18,
                                            byte: 17,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 21,
                                            byte: 20,
                                        },
                                    ),
                                }]),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 18,
                                        byte: 17,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 21,
                                        byte: 20,
                                    },
                                ),
                            }
                            .into(),
                            key_expr: Some(
                                ScopeTraversalExpr {
                                    traversal: Traversal(vec![Traverser::Root {
                                        name: "k".to_string(),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 23,
                                                byte: 22,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 24,
                                                byte: 23,
                                            },
                                        ),
                                    }]),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 23,
                                            byte: 22,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 24,
                                            byte: 23,
                                        },
                                    ),
                                }
                                .into(),
                            ),
                            val_expr: ScopeTraversalExpr {
                                traversal: Traversal(vec![Traverser::Root {
                                    name: "v".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 28,
                                            byte: 27,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 29,
                                            byte: 28,
                                        },
                                    ),
                                }]),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 28,
                                        byte: 27,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 29,
                                        byte: 28,
                                    },
                                ),
                            }
                            .into(),
                            cond_expr: Some(
                                LiteralValueExpr {
                                    val: Value::bool(true),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 36,
                                            byte: 35,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 40,
                                            byte: 39,
                                        },
                                    ),
                                }
                                .into(),
                            ),
                            group: true,
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 41,
                                    byte: 40,
                                },
                            ),
                            open_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                            close_range: rng(
                                Pos {
                                    line: 1,
                                    column: 40,
                                    byte: 39,
                                },
                                Pos {
                                    line: 1,
                                    column: 41,
                                    byte: 40,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 41,
                                byte: 40,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 41,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 41,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 41,
                    },
                ),
            },
        },
        Case {
            input: "\t",
            diag_count: 0, // the tab character is treated as a single whitespace character
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                ),
            },
        },
        Case {
            input: r"\x81",
            diag_count: 2, // invalid UTF-8, and body item is required here
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![],
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
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                    Pos {
                        line: 1,
                        column: 2,
                        byte: 1,
                    },
                ),
            },
        },
        Case {
            input: "a = 1,",
            diag_count: 1,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: LiteralValueExpr {
                            val: Value::number_int(1),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 6,
                                byte: 5,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                ),
            },
        },
        Case {
            input: "a = `str`",
            diag_count: 2, // Invalid character and expression
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: LiteralValueExpr {
                            val: Value::dynamic(),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                        }
                        .into(),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
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
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                ),
            },
        },
        Case {
            input: "a = 'str'",
            diag_count: 2, // Invalid character and expression
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: LiteralValueExpr {
                            val: Value::dynamic(),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                        }
                        .into(),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
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
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                ),
            },
        },
        Case {
            input: "a = sort(data.first.ref.attr)[count.index]\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: IndexExpr {
                            collection: FunctionCallExpr {
                                name: "sort".to_string(),
                                args: vec![
                                    ScopeTraversalExpr {
                                        traversal: Traversal(vec![
                                            Traverser::Root {
                                                name: "data".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 10,
                                                        byte: 9,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 14,
                                                        byte: 13,
                                                    },
                                                ),
                                            },
                                            Traverser::Attr {
                                                name: "first".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 14,
                                                        byte: 13,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 20,
                                                        byte: 19,
                                                    },
                                                ),
                                            },
                                            Traverser::Attr {
                                                name: "ref".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 20,
                                                        byte: 19,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 24,
                                                        byte: 23,
                                                    },
                                                ),
                                            },
                                            Traverser::Attr {
                                                name: "attr".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 24,
                                                        byte: 23,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 29,
                                                        byte: 28,
                                                    },
                                                ),
                                            },
                                        ]),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 10,
                                                byte: 9,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 29,
                                                byte: 28,
                                            },
                                        ),
                                    }
                                    .into(),
                                ],
                                expand_final: false,
                                name_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 5,
                                        byte: 4,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 9,
                                        byte: 8,
                                    },
                                ),
                                open_paren_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 9,
                                        byte: 8,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 10,
                                        byte: 9,
                                    },
                                ),
                                close_paren_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 29,
                                        byte: 28,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 30,
                                        byte: 29,
                                    },
                                ),
                            }
                            .into(),
                            key: ScopeTraversalExpr {
                                traversal: Traversal(vec![
                                    Traverser::Root {
                                        name: "count".to_string(),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 31,
                                                byte: 30,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 36,
                                                byte: 35,
                                            },
                                        ),
                                    },
                                    Traverser::Attr {
                                        name: "index".to_string(),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 36,
                                                byte: 35,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 42,
                                                byte: 41,
                                            },
                                        ),
                                    },
                                ]),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 31,
                                        byte: 30,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 42,
                                        byte: 41,
                                    },
                                ),
                            }
                            .into(),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 43,
                                    byte: 42,
                                },
                            ),
                            open_range: rng(
                                Pos {
                                    line: 1,
                                    column: 30,
                                    byte: 29,
                                },
                                Pos {
                                    line: 1,
                                    column: 31,
                                    byte: 30,
                                },
                            ),
                            bracket_range: rng(
                                Pos {
                                    line: 1,
                                    column: 30,
                                    byte: 29,
                                },
                                Pos {
                                    line: 1,
                                    column: 43,
                                    byte: 42,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 43,
                                byte: 42,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 43,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 43,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 43,
                    },
                ),
            },
        },
        Case {
            input: r#"block "unterminated_string "name" {}"#,
            diag_count: 2, // "Invalid string literal" and "Invalid block definition"
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec![
                        "unterminated_string ".to_string(),
                        "name".to_string(),
                        " {}".to_string(),
                    ],
                    // NOTE(port): upstream leaves this inner body's
                    // Attributes/Blocks as nil; Rust has no nil map/slice,
                    // so they are the empty map/vec here.
                    body: Body {
                        attributes: Attributes::new(),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 6,
                                byte: 5,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 6,
                                byte: 5,
                            },
                        ),
                    },
                    type_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 6,
                            byte: 5,
                        },
                    ),
                    label_ranges: vec![
                        rng(
                            Pos {
                                line: 1,
                                column: 7,
                                byte: 6,
                            },
                            Pos {
                                line: 1,
                                column: 29,
                                byte: 28,
                            },
                        ),
                        rng(
                            Pos {
                                line: 1,
                                column: 29,
                                byte: 28,
                            },
                            Pos {
                                line: 1,
                                column: 33,
                                byte: 32,
                            },
                        ),
                        rng(
                            Pos {
                                line: 1,
                                column: 33,
                                byte: 32,
                            },
                            Pos {
                                line: 1,
                                column: 37,
                                byte: 36,
                            },
                        ),
                    ],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 1,
                            byte: 0,
                        },
                        Pos {
                            line: 1,
                            column: 6,
                            byte: 5,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 33,
                            byte: 32,
                        },
                        Pos {
                            line: 1,
                            column: 37,
                            byte: 36,
                        },
                    ),
                }],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 37,
                        byte: 36,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 37,
                        byte: 36,
                    },
                    Pos {
                        line: 1,
                        column: 37,
                        byte: 36,
                    },
                ),
            },
        },
        Case {
            input: "a = a::namespaced::func(data.first.ref.attr)\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: FunctionCallExpr {
                            name: "a::namespaced::func".to_string(),
                            args: vec![
                                ScopeTraversalExpr {
                                    traversal: Traversal(vec![
                                        Traverser::Root {
                                            name: "data".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 25,
                                                    byte: 24,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 29,
                                                    byte: 28,
                                                },
                                            ),
                                        },
                                        Traverser::Attr {
                                            name: "first".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 29,
                                                    byte: 28,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 35,
                                                    byte: 34,
                                                },
                                            ),
                                        },
                                        Traverser::Attr {
                                            name: "ref".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 35,
                                                    byte: 34,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 39,
                                                    byte: 38,
                                                },
                                            ),
                                        },
                                        Traverser::Attr {
                                            name: "attr".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 39,
                                                    byte: 38,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 44,
                                                    byte: 43,
                                                },
                                            ),
                                        },
                                    ]),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 25,
                                            byte: 24,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 44,
                                            byte: 43,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            expand_final: false,
                            name_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 24,
                                    byte: 23,
                                },
                            ),
                            open_paren_range: rng(
                                Pos {
                                    line: 1,
                                    column: 24,
                                    byte: 23,
                                },
                                Pos {
                                    line: 1,
                                    column: 25,
                                    byte: 24,
                                },
                            ),
                            close_paren_range: rng(
                                Pos {
                                    line: 1,
                                    column: 44,
                                    byte: 43,
                                },
                                Pos {
                                    line: 1,
                                    column: 45,
                                    byte: 44,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 45,
                                byte: 44,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 45,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 45,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 45,
                    },
                ),
            },
        },
        Case {
            input: "a = partial::namespaced\n",
            diag_count: 1,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ExprSyntaxError {
                            placeholder: Value::dynamic(),
                            parse_diags: Diagnostics(vec![Diagnostic {
                                severity: DiagnosticSeverity::Error,
                                summary: "Missing open parenthesis".to_string(),
                                detail: "Function selector must be followed by an open \
                                         parenthesis to begin the function call."
                                    .to_string(),
                                subject: Some(rng(
                                    Pos {
                                        line: 1,
                                        column: 24,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 1,
                                        byte: 24,
                                    },
                                )),
                                context: Some(rng(
                                    Pos {
                                        line: 1,
                                        column: 5,
                                        byte: 4,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 1,
                                        byte: 24,
                                    },
                                )),
                                ..Default::default()
                            }]),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 24,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 24,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 24,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 24,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 24,
                    },
                ),
            },
        },
        Case {
            input: "a = partial::\n",
            diag_count: 1,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ExprSyntaxError {
                            placeholder: Value::dynamic(),
                            parse_diags: Diagnostics(vec![Diagnostic {
                                severity: DiagnosticSeverity::Error,
                                summary: "Missing function name".to_string(),
                                detail: "Function scope resolution symbol :: must be followed \
                                         by a function name in this scope."
                                    .to_string(),
                                subject: Some(rng(
                                    Pos {
                                        line: 1,
                                        column: 14,
                                        byte: 13,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 1,
                                        byte: 14,
                                    },
                                )),
                                context: Some(rng(
                                    Pos {
                                        line: 1,
                                        column: 5,
                                        byte: 4,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 1,
                                        byte: 14,
                                    },
                                )),
                                ..Default::default()
                            }]),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 14,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 14,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 14,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 14,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 14,
                    },
                ),
            },
        },
        Case {
            input: "a = { b = c. }",
            diag_count: 1,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ObjectConsExpr {
                            items: vec![ObjectConsItem {
                                key_expr: ObjectConsKeyExpr {
                                    wrapped: ScopeTraversalExpr {
                                        traversal: Traversal(vec![Traverser::Root {
                                            name: "b".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 7,
                                                    byte: 6,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 8,
                                                    byte: 7,
                                                },
                                            ),
                                        }]),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 7,
                                                byte: 6,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 8,
                                                byte: 7,
                                            },
                                        ),
                                    }
                                    .into(),
                                    force_non_literal: false,
                                }
                                .into(),
                                value_expr: ExprSyntaxError {
                                    placeholder: Value::dynamic(),
                                    parse_diags: Diagnostics(vec![Diagnostic {
                                        severity: DiagnosticSeverity::Error,
                                        summary: "Invalid attribute name".to_string(),
                                        detail: "An attribute name is required after a dot."
                                            .to_string(),
                                        subject: Some(rng(
                                            Pos {
                                                line: 1,
                                                column: 14,
                                                byte: 13,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 15,
                                                byte: 14,
                                            },
                                        )),
                                        ..Default::default()
                                    }]),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 11,
                                            byte: 10,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 13,
                                            byte: 12,
                                        },
                                    ),
                                }
                                .into(),
                            }],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 15,
                                    byte: 14,
                                },
                            ),
                            open_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                            ),
                        }
                        .into(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 1,
                                byte: 0,
                            },
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                        ),
                        name_range: rng(
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
                        equals_range: rng(
                            Pos {
                                line: 1,
                                column: 3,
                                byte: 2,
                            },
                            Pos {
                                line: 1,
                                column: 4,
                                byte: 3,
                            },
                        ),
                    },
                )]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 15,
                        byte: 14,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 1,
                        column: 15,
                        byte: 14,
                    },
                    Pos {
                        line: 1,
                        column: 15,
                        byte: 14,
                    },
                ),
            },
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, diags) = hclsyntax::parse_config(
            test.input.as_bytes(),
            "",
            Pos {
                byte: 0,
                line: 1,
                column: 1,
            },
        );
        assert_eq!(
            diags.len(),
            test.diag_count,
            "case {i} ({:?}): wrong number of diagnostics: {:#?}",
            test.input,
            *diags,
        );

        let got = file
            .body
            .as_any()
            .downcast_ref::<hclsyntax::Body>()
            .expect("file body is not hclsyntax::Body");
        assert_eq!(got, &test.want, "case {i} ({:?}): wrong result", test.input);
    }
}
