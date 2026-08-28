//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/parser_test.go (TestParseConfig_incompleteFunctionCall,
//!   TestParseConfigDiagnostics)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax::{
    self, Attribute, Attributes, Block, Body, FunctionCallExpr, LiteralValueExpr, ObjectConsExpr,
    ObjectConsItem, ObjectConsKeyExpr, ScopeTraversalExpr, TemplateExpr,
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

/// A range within the file `test.hcl`
/// (Go: `hcl.Range{Filename: "test.hcl", Start: ..., End: ...}`).
fn trng(start: Pos, end: Pos) -> Range {
    Range {
        filename: "test.hcl".to_string(),
        start,
        end,
    }
}

// Ported from TestParseConfig_incompleteFunctionCall:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parser_test.go#L2792
//
// NOTE(port): upstream compares with go-cmp options that ignore unexported
// fields on FunctionCallExpr, Body, cty.Value, and hcl.TraverseRoot; the
// corresponding Rust types have no such hidden fields, so plain `PartialEq`
// implements the same comparison. Upstream ignores the returned diagnostics
// (`file, _ := ParseConfig(...)`), so no diagnostic count is asserted here.
#[test]
#[ignore = "not yet implemented"]
fn parse_config_incomplete_function_call() {
    struct Case {
        input: &'static str,
        want: Body,
    }

    let tests = [
        Case {
            input: "attr = object({ foo = })\nattr2 = \"foo\"\n",
            want: Body {
                attributes: Attributes::from([
                    (
                        "attr".to_string(),
                        Attribute {
                            name: "attr".to_string(),
                            expr: FunctionCallExpr {
                                name: "object".to_string(),
                                args: vec![
                                    ObjectConsExpr {
                                        items: vec![ObjectConsItem {
                                            key_expr: ObjectConsKeyExpr {
                                                wrapped: ScopeTraversalExpr {
                                                    traversal: Traversal(vec![Traverser::Root {
                                                        name: "foo".to_string(),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 1,
                                                                column: 17,
                                                                byte: 16,
                                                            },
                                                            Pos {
                                                                line: 1,
                                                                column: 20,
                                                                byte: 19,
                                                            },
                                                        ),
                                                    }]),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 1,
                                                            column: 17,
                                                            byte: 16,
                                                        },
                                                        Pos {
                                                            line: 1,
                                                            column: 20,
                                                            byte: 19,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                                force_non_literal: false,
                                            }
                                            .into(),
                                            value_expr: LiteralValueExpr {
                                                val: Value::dynamic(),
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
                                        }],
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 15,
                                                byte: 14,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 24,
                                                byte: 23,
                                            },
                                        ),
                                        open_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 15,
                                                byte: 14,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 16,
                                                byte: 15,
                                            },
                                        ),
                                    }
                                    .into(),
                                ],
                                expand_final: false,
                                name_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 8,
                                        byte: 7,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 14,
                                        byte: 13,
                                    },
                                ),
                                open_paren_range: rng(
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
                                ),
                                close_paren_range: rng(
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
                                    column: 25,
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
                                    column: 5,
                                    byte: 4,
                                },
                            ),
                            equals_range: rng(
                                Pos {
                                    line: 1,
                                    column: 6,
                                    byte: 5,
                                },
                                Pos {
                                    line: 1,
                                    column: 7,
                                    byte: 6,
                                },
                            ),
                        },
                    ),
                    (
                        "attr2".to_string(),
                        Attribute {
                            name: "attr2".to_string(),
                            expr: TemplateExpr {
                                parts: vec![
                                    LiteralValueExpr {
                                        val: Value::string("foo"),
                                        src_range: rng(
                                            Pos {
                                                line: 2,
                                                column: 10,
                                                byte: 34,
                                            },
                                            Pos {
                                                line: 2,
                                                column: 13,
                                                byte: 37,
                                            },
                                        ),
                                    }
                                    .into(),
                                ],
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 33,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 14,
                                        byte: 38,
                                    },
                                ),
                            }
                            .into(),
                            src_range: rng(
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 25,
                                },
                                Pos {
                                    line: 2,
                                    column: 14,
                                    byte: 38,
                                },
                            ),
                            name_range: rng(
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 25,
                                },
                                Pos {
                                    line: 2,
                                    column: 6,
                                    byte: 30,
                                },
                            ),
                            equals_range: rng(
                                Pos {
                                    line: 2,
                                    column: 7,
                                    byte: 31,
                                },
                                Pos {
                                    line: 2,
                                    column: 8,
                                    byte: 32,
                                },
                            ),
                        },
                    ),
                ]),
                blocks: vec![],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 39,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 39,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 39,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object(\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        LiteralValueExpr {
                                            val: Value::dynamic(),
                                            src_range: rng(
                                                Pos {
                                                    line: 3,
                                                    column: 1,
                                                    byte: 33,
                                                },
                                                Pos {
                                                    line: 3,
                                                    column: 1,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 1,
                                        byte: 33,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 33,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 33,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 33,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 33,
                        },
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 33,
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
                        line: 3,
                        column: 1,
                        byte: 33,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 33,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 33,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n\t\t  attr = object({\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 19,
                                                    byte: 34,
                                                },
                                                Pos {
                                                    line: 3,
                                                    column: 1,
                                                    byte: 36,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 19,
                                                    byte: 34,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 20,
                                                    byte: 35,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 12,
                                            byte: 27,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 18,
                                            byte: 33,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 18,
                                            byte: 33,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 19,
                                            byte: 34,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 5,
                                        byte: 20,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 1,
                                        byte: 36,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 5,
                                        byte: 20,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 10,
                                        byte: 25,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 11,
                                        byte: 26,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 36,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 36,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 36,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 36,
                        },
                        Pos {
                            line: 3,
                            column: 1,
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
                        line: 3,
                        column: 1,
                        byte: 36,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 36,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 36,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({ foo\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 3,
                                                    column: 1,
                                                    byte: 38,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 1,
                                        byte: 38,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 38,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 38,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 38,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 38,
                        },
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 38,
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
                        line: 3,
                        column: 1,
                        byte: 38,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 38,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 38,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({ foo =\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 19,
                                                                        byte: 34,
                                                                    },
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 22,
                                                                        byte: 37,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 2,
                                                                column: 19,
                                                                byte: 34,
                                                            },
                                                            Pos {
                                                                line: 2,
                                                                column: 22,
                                                                byte: 37,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 2,
                                                            column: 24,
                                                            byte: 39,
                                                        },
                                                        Pos {
                                                            line: 3,
                                                            column: 1,
                                                            byte: 40,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 3,
                                                    column: 1,
                                                    byte: 40,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 1,
                                        byte: 40,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 40,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 40,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 40,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 40,
                        },
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 40,
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
                        line: 3,
                        column: 1,
                        byte: 40,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 40,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 40,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({ foo = }\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 19,
                                                                        byte: 34,
                                                                    },
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 22,
                                                                        byte: 37,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 2,
                                                                column: 19,
                                                                byte: 34,
                                                            },
                                                            Pos {
                                                                line: 2,
                                                                column: 22,
                                                                byte: 37,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 2,
                                                            column: 25,
                                                            byte: 40,
                                                        },
                                                        Pos {
                                                            line: 2,
                                                            column: 26,
                                                            byte: 41,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 26,
                                                    byte: 41,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 1,
                                        byte: 42,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 42,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 42,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 42,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 42,
                        },
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 42,
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
                        line: 3,
                        column: 1,
                        byte: 42,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 42,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 42,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({ foo = })\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 19,
                                                                        byte: 34,
                                                                    },
                                                                    Pos {
                                                                        line: 2,
                                                                        column: 22,
                                                                        byte: 37,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 2,
                                                                column: 19,
                                                                byte: 34,
                                                            },
                                                            Pos {
                                                                line: 2,
                                                                column: 22,
                                                                byte: 37,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 2,
                                                            column: 25,
                                                            byte: 40,
                                                        },
                                                        Pos {
                                                            line: 2,
                                                            column: 26,
                                                            byte: 41,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 26,
                                                    byte: 41,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 26,
                                            byte: 41,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 27,
                                            byte: 42,
                                        },
                                    ),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 27,
                                        byte: 42,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 43,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 43,
                            },
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 43,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 43,
                        },
                        Pos {
                            line: 3,
                            column: 1,
                            byte: 43,
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
                        line: 3,
                        column: 1,
                        byte: 43,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 43,
                    },
                    Pos {
                        line: 3,
                        column: 1,
                        byte: 43,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({\n    foo =\n\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 5,
                                                                        byte: 38,
                                                                    },
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 8,
                                                                        byte: 41,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 3,
                                                                column: 5,
                                                                byte: 38,
                                                            },
                                                            Pos {
                                                                line: 3,
                                                                column: 8,
                                                                byte: 41,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 3,
                                                            column: 10,
                                                            byte: 43,
                                                        },
                                                        Pos {
                                                            line: 4,
                                                            column: 1,
                                                            byte: 44,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 5,
                                                    column: 1,
                                                    byte: 45,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 5,
                                        column: 1,
                                        byte: 45,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 45,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 45,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 45,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 45,
                        },
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 45,
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
                        line: 5,
                        column: 1,
                        byte: 45,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 45,
                    },
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 45,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({\n    foo =\n\nanother_block {\n\n}\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 5,
                                                                        byte: 38,
                                                                    },
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 8,
                                                                        byte: 41,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 3,
                                                                column: 5,
                                                                byte: 38,
                                                            },
                                                            Pos {
                                                                line: 3,
                                                                column: 8,
                                                                byte: 41,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 3,
                                                            column: 10,
                                                            byte: 43,
                                                        },
                                                        Pos {
                                                            line: 4,
                                                            column: 1,
                                                            byte: 44,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 8,
                                                    column: 1,
                                                    byte: 64,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 8,
                                        column: 1,
                                        byte: 64,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 8,
                                column: 1,
                                byte: 64,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 8,
                                column: 1,
                                byte: 64,
                            },
                            Pos {
                                line: 8,
                                column: 1,
                                byte: 64,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 8,
                            column: 1,
                            byte: 64,
                        },
                        Pos {
                            line: 8,
                            column: 1,
                            byte: 64,
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
                        line: 8,
                        column: 1,
                        byte: 64,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 8,
                        column: 1,
                        byte: 64,
                    },
                    Pos {
                        line: 8,
                        column: 1,
                        byte: 64,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({\n    foo =\n  }\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 5,
                                                                        byte: 38,
                                                                    },
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 8,
                                                                        byte: 41,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 3,
                                                                column: 5,
                                                                byte: 38,
                                                            },
                                                            Pos {
                                                                line: 3,
                                                                column: 8,
                                                                byte: 41,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 3,
                                                            column: 10,
                                                            byte: 43,
                                                        },
                                                        Pos {
                                                            line: 4,
                                                            column: 1,
                                                            byte: 44,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 4,
                                                    column: 4,
                                                    byte: 47,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: Range::default(),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 5,
                                        column: 1,
                                        byte: 48,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 48,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 48,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 48,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 48,
                        },
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 48,
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
                        line: 5,
                        column: 1,
                        byte: 48,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 48,
                    },
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 48,
                    },
                ),
            },
        },
        Case {
            input: "block \"label\" {\n  attr = object({\n    foo =\n  })\n",
            want: Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["label".to_string()],
                    body: Body {
                        attributes: Attributes::from([(
                            "attr".to_string(),
                            Attribute {
                                name: "attr".to_string(),
                                expr: FunctionCallExpr {
                                    name: "object".to_string(),
                                    args: vec![
                                        ObjectConsExpr {
                                            items: vec![ObjectConsItem {
                                                key_expr: ObjectConsKeyExpr {
                                                    wrapped: ScopeTraversalExpr {
                                                        traversal: Traversal(vec![
                                                            Traverser::Root {
                                                                name: "foo".to_string(),
                                                                src_range: rng(
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 5,
                                                                        byte: 38,
                                                                    },
                                                                    Pos {
                                                                        line: 3,
                                                                        column: 8,
                                                                        byte: 41,
                                                                    },
                                                                ),
                                                            },
                                                        ]),
                                                        src_range: rng(
                                                            Pos {
                                                                line: 3,
                                                                column: 5,
                                                                byte: 38,
                                                            },
                                                            Pos {
                                                                line: 3,
                                                                column: 8,
                                                                byte: 41,
                                                            },
                                                        ),
                                                    }
                                                    .into(),
                                                    force_non_literal: false,
                                                }
                                                .into(),
                                                value_expr: LiteralValueExpr {
                                                    val: Value::dynamic(),
                                                    src_range: rng(
                                                        Pos {
                                                            line: 3,
                                                            column: 10,
                                                            byte: 43,
                                                        },
                                                        Pos {
                                                            line: 4,
                                                            column: 1,
                                                            byte: 44,
                                                        },
                                                    ),
                                                }
                                                .into(),
                                            }],
                                            src_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 4,
                                                    column: 4,
                                                    byte: 47,
                                                },
                                            ),
                                            open_range: rng(
                                                Pos {
                                                    line: 2,
                                                    column: 17,
                                                    byte: 32,
                                                },
                                                Pos {
                                                    line: 2,
                                                    column: 18,
                                                    byte: 33,
                                                },
                                            ),
                                        }
                                        .into(),
                                    ],
                                    expand_final: false,
                                    name_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 10,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                    ),
                                    open_paren_range: rng(
                                        Pos {
                                            line: 2,
                                            column: 16,
                                            byte: 31,
                                        },
                                        Pos {
                                            line: 2,
                                            column: 17,
                                            byte: 32,
                                        },
                                    ),
                                    close_paren_range: rng(
                                        Pos {
                                            line: 4,
                                            column: 4,
                                            byte: 47,
                                        },
                                        Pos {
                                            line: 4,
                                            column: 5,
                                            byte: 48,
                                        },
                                    ),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 4,
                                        column: 5,
                                        byte: 48,
                                    },
                                ),
                                name_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 3,
                                        byte: 18,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 7,
                                        byte: 22,
                                    },
                                ),
                                equals_range: rng(
                                    Pos {
                                        line: 2,
                                        column: 8,
                                        byte: 23,
                                    },
                                    Pos {
                                        line: 2,
                                        column: 9,
                                        byte: 24,
                                    },
                                ),
                            },
                        )]),
                        blocks: vec![],
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 15,
                                byte: 14,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 49,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 49,
                            },
                            Pos {
                                line: 5,
                                column: 1,
                                byte: 49,
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
                    label_ranges: vec![rng(
                        Pos {
                            line: 1,
                            column: 7,
                            byte: 6,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 15,
                            byte: 14,
                        },
                        Pos {
                            line: 1,
                            column: 16,
                            byte: 15,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 49,
                        },
                        Pos {
                            line: 5,
                            column: 1,
                            byte: 49,
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
                        line: 5,
                        column: 1,
                        byte: 49,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 49,
                    },
                    Pos {
                        line: 5,
                        column: 1,
                        byte: 49,
                    },
                ),
            },
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (file, _diags) = hclsyntax::parse_config(test.input.as_bytes(), "", Pos::initial());

        let got = file
            .body
            .as_any()
            .downcast_ref::<hclsyntax::Body>()
            .expect("file body is not hclsyntax::Body");
        assert_eq!(got, &test.want, "case {i} ({:?}): wrong result", test.input);
    }
}

// Ported from TestParseConfigDiagnostics:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parser_test.go#L4066
//
// This test function is a variant of TestParseConfig which tests for
// specific error messages for certain kinds of invalid input where we
// intend to produce a particular helpful error message.
//
// NOTE(port): upstream keys the cases by name in a Go map (random iteration
// order); here they are an array in upstream source order with the name kept
// in each case.
#[test]
#[ignore = "not yet implemented"]
fn parse_config_diagnostics() {
    struct Case {
        name: &'static str,
        input: &'static str,
        want: Diagnostics,
    }

    let tests = [
        Case {
            name: "unclosed multi-line block (no contents)",
            input: "blah {\n",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed configuration block".to_string(),
                detail: "There is no closing brace for this block before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 6,
                        byte: 5,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed multi-line block (after one argument)",
            input: "blah {\n  a = 1\n",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed configuration block".to_string(),
                detail: "There is no closing brace for this block before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 6,
                        byte: 5,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed single-line block (no contents)",
            input: "blah {",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed configuration block".to_string(),
                detail: "There is no closing brace for this block before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 6,
                        byte: 5,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed single-line block (after its argument)",
            input: "blah { a = 1",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed configuration block".to_string(),
                detail: "There is no closing brace for this block before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 6,
                        byte: 5,
                    },
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                )),
                // In this case we can also report a context because we
                // detect this error in a more convenient place in the parser
                context: Some(trng(
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
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed object constructor (before element separator)",
            input: "foo = { a = 1",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unterminated object constructor expression".to_string(),
                detail: "There is no corresponding closing brace before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
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
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed object constructor (before equals)",
            input: "foo = { a ",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unterminated object constructor expression".to_string(),
                detail: "There is no corresponding closing brace before the end of the file. This may be caused by incorrect brace nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
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
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed tuple constructor (before element separator)",
            input: "foo = [ a",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unterminated tuple constructor expression".to_string(),
                detail: "There is no corresponding closing bracket before the end of the file. This may be caused by incorrect bracket nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
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
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed function call",
            input: "foo = boop(\"a\"",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unterminated function call".to_string(),
                detail: "There is no closing parenthesis for this function call before the end of the file. This may be caused by incorrect parenthesis nesting elsewhere in this file.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                    Pos {
                        line: 1,
                        column: 12,
                        byte: 11,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed grouping parentheses",
            input: "foo = (1",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unbalanced parentheses".to_string(),
                detail: "Expected a closing parenthesis to terminate the expression.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 9,
                        byte: 8,
                    },
                    Pos {
                        line: 1,
                        column: 9,
                        byte: 8,
                    },
                )),
                context: Some(trng(
                    Pos {
                        line: 1,
                        column: 7,
                        byte: 6,
                    },
                    Pos {
                        line: 1,
                        column: 9,
                        byte: 8,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed template interpolation at EOF",
            input: "foo = \"${a",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed template interpolation sequence".to_string(),
                detail: "There is no closing brace for this interpolation sequence before the end of the file. This might be caused by incorrect nesting inside the given expression.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed quoted template interpolation at closing quote",
            input: "foo = \"${a\"",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unclosed template interpolation sequence".to_string(),
                detail: "There is no closing brace for this interpolation sequence before the end of the quoted template. This might be caused by incorrect nesting inside the given expression.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                    Pos {
                        line: 1,
                        column: 10,
                        byte: 9,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed quoted template at literal part",
            input: "foo = \"${a}",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Unterminated template string".to_string(),
                detail: "No closing marker was found for the string.".to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 12,
                        byte: 11,
                    },
                    Pos {
                        line: 1,
                        column: 12,
                        byte: 11,
                    },
                )),
                context: Some(trng(
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
                )),
                ..Default::default()
            }]),
        },
        // Some of our "unclosed" situations happen at a less convenient time
        // when we only know we're waiting for an expression, so those get
        // an error message with much less context.
        Case {
            name: "unclosed object constructor (before any expression)",
            input: "foo = {",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Missing expression".to_string(),
                detail: "Expected the start of an expression, but found the end of the file."
                    .to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed tuple constructor (before any expression)",
            input: "foo = [",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Missing expression".to_string(),
                detail: "Expected the start of an expression, but found the end of the file."
                    .to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                    Pos {
                        line: 1,
                        column: 8,
                        byte: 7,
                    },
                )),
                ..Default::default()
            }]),
        },
        Case {
            name: "unclosed function call (before any argument)",
            input: "foo = foo(",
            want: Diagnostics(vec![Diagnostic {
                severity: DiagnosticSeverity::Error,
                summary: "Missing expression".to_string(),
                detail: "Expected the start of an expression, but found the end of the file."
                    .to_string(),
                subject: Some(trng(
                    Pos {
                        line: 1,
                        column: 11,
                        byte: 10,
                    },
                    Pos {
                        line: 1,
                        column: 11,
                        byte: 10,
                    },
                )),
                ..Default::default()
            }]),
        },
    ];

    for test in &tests {
        let (_file, diags) =
            hclsyntax::parse_config(test.input.as_bytes(), "test.hcl", Pos::initial());

        assert_eq!(
            diags, test.want,
            "case {:?} ({:?}): wrong diagnostics",
            test.name, test.input,
        );
    }
}
