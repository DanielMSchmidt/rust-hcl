//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/parser_test.go (TestParseConfig, part 2)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::sync::Arc;

use cty::Value;
use hcl::hclsyntax::{
    self, AnonSymbolExpr, Attribute, Attributes, Body, IndexExpr, LiteralValueExpr,
    ScopeTraversalExpr, SplatExpr, TemplateExpr, TemplateWrapExpr,
};
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

// Ported from TestParseConfig:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parser_test.go#L18
// (part 2: cases with opening brace in upstream lines 984-1880)
#[test]
#[ignore = "not yet implemented"]
fn parse_config_part2() {
    struct Case {
        input: &'static str,
        diag_count: usize,
        want: Body,
    }

    let tests = [
        Case {
            input: "a = \"hello %\"\n",
            diag_count: 0, // unterminated template control sequence
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("hello %"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 13,
                                            byte: 12,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 14,
                                    byte: 13,
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
                                column: 14,
                                byte: 13,
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
            input: "a = \"hello!\"\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("hello!"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 12,
                                            byte: 11,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 13,
                                    byte: 12,
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
                                column: 13,
                                byte: 12,
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
                        byte: 13,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\u2022\"\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\u{2022}"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 12,
                                            byte: 11,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 13,
                                    byte: 12,
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
                                column: 13,
                                byte: 12,
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
                        byte: 13,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\uu2022\"\n",
            diag_count: 1, // \u must be followed by four hex digits
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\\uu2022"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 13,
                                            byte: 12,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 14,
                                    byte: 13,
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
                                column: 14,
                                byte: 13,
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
            input: "a = \"\\U0001d11e\"\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\u{1d11e}"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
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
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 17,
                                    byte: 16,
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
                                column: 17,
                                byte: 16,
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
                        byte: 17,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\u0001d11e\"\n",
            diag_count: 0, // This is valid, but probably not what the user intended :(
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    // Only the first four digits were used for the
                                    // escape sequence, so the remaining four just
                                    // get echoed out literally.
                                    val: Value::string("\u{0001}d11e"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
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
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 17,
                                    byte: 16,
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
                                column: 17,
                                byte: 16,
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
                        byte: 17,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\U2022\"\n",
            diag_count: 1, // Invalid escape sequence, since we need eight hex digits for \U
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\\U2022"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 12,
                                            byte: 11,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 13,
                                    byte: 12,
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
                                column: 13,
                                byte: 12,
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
                        byte: 13,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\u20m2\"\n",
            diag_count: 1, // Invalid escape sequence
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\\u20m2"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 12,
                                            byte: 11,
                                        },
                                    ),
                                }
                                .into(),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 13,
                                    byte: 12,
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
                                column: 13,
                                byte: 12,
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
                        byte: 13,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 13,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\U00300000\"\n",
            diag_count: 1, // Invalid unicode character (can't encode in UTF-8)
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\\U00300000"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
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
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 17,
                                    byte: 16,
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
                                column: 17,
                                byte: 16,
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
                        byte: 17,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                ),
            },
        },
        Case {
            input: "a = \"\\Ub2705550\"\n",
            diag_count: 1, // Invalid unicode character (can't encode in UTF-8)
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                LiteralValueExpr {
                                    val: Value::string("\\Ub2705550"),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 6,
                                            byte: 5,
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
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 17,
                                    byte: 16,
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
                                column: 17,
                                byte: 16,
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
                        byte: 17,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 17,
                    },
                ),
            },
        },
        Case {
            input: "a = <<EOT\nHello\nEOT\nb = \"Hi\"",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([
                    (
                        "a".to_string(),
                        Attribute {
                            name: "a".to_string(),
                            expr: TemplateExpr {
                                parts: vec![
                                    LiteralValueExpr {
                                        val: Value::string("Hello\n"),
                                        src_range: rng(
                                            Pos {
                                                line: 2,
                                                column: 1,
                                                byte: 10,
                                            },
                                            Pos {
                                                line: 3,
                                                column: 1,
                                                byte: 16,
                                            },
                                        ),
                                    }
                                    .into(),
                                ],
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 5,
                                        byte: 4,
                                    },
                                    Pos {
                                        line: 3,
                                        column: 4,
                                        byte: 19,
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
                                    line: 3,
                                    column: 4,
                                    byte: 19,
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
                    ),
                    (
                        "b".to_string(),
                        Attribute {
                            name: "b".to_string(),
                            expr: TemplateExpr {
                                parts: vec![
                                    LiteralValueExpr {
                                        val: Value::string("Hi"),
                                        src_range: rng(
                                            Pos {
                                                line: 4,
                                                column: 6,
                                                byte: 25,
                                            },
                                            Pos {
                                                line: 4,
                                                column: 8,
                                                byte: 27,
                                            },
                                        ),
                                    }
                                    .into(),
                                ],
                                src_range: rng(
                                    Pos {
                                        line: 4,
                                        column: 5,
                                        byte: 24,
                                    },
                                    Pos {
                                        line: 4,
                                        column: 9,
                                        byte: 28,
                                    },
                                ),
                            }
                            .into(),
                            src_range: rng(
                                Pos {
                                    line: 4,
                                    column: 1,
                                    byte: 20,
                                },
                                Pos {
                                    line: 4,
                                    column: 9,
                                    byte: 28,
                                },
                            ),
                            name_range: rng(
                                Pos {
                                    line: 4,
                                    column: 1,
                                    byte: 20,
                                },
                                Pos {
                                    line: 4,
                                    column: 2,
                                    byte: 21,
                                },
                            ),
                            equals_range: rng(
                                Pos {
                                    line: 4,
                                    column: 3,
                                    byte: 22,
                                },
                                Pos {
                                    line: 4,
                                    column: 4,
                                    byte: 23,
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
                        line: 4,
                        column: 9,
                        byte: 28,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 4,
                        column: 9,
                        byte: 28,
                    },
                    Pos {
                        line: 4,
                        column: 9,
                        byte: 28,
                    },
                ),
            },
        },
        Case {
            input: "a = foo.bar\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ScopeTraversalExpr {
                            traversal: Traversal(vec![
                                Traverser::Root {
                                    name: "foo".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 5,
                                            byte: 4,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 8,
                                            byte: 7,
                                        },
                                    ),
                                },
                                Traverser::Attr {
                                    name: "bar".to_string(),
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
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 12,
                                    byte: 11,
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
                                column: 12,
                                byte: 11,
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
                        byte: 12,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 12,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 12,
                    },
                ),
            },
        },
        Case {
            input: "a = foo.0.1.baz\n",
            diag_count: 1, // Chaining legacy index syntax is not supported
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: ScopeTraversalExpr {
                            traversal: Traversal(vec![
                                Traverser::Root {
                                    name: "foo".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 5,
                                            byte: 4,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 8,
                                            byte: 7,
                                        },
                                    ),
                                },
                                Traverser::Index {
                                    key: Value::dynamic(),
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
                                Traverser::Attr {
                                    name: "baz".to_string(),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 12,
                                            byte: 11,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 16,
                                            byte: 15,
                                        },
                                    ),
                                },
                            ]),
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 16,
                                    byte: 15,
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
                                column: 16,
                                byte: 15,
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
                        byte: 16,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 16,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 16,
                    },
                ),
            },
        },
        Case {
            input: "a = \"${var.public_subnets[count.index]}\"\n",
            diag_count: 0,
            want: Body {
                attributes: Attributes::from([(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateWrapExpr {
                            wrapped: IndexExpr {
                                collection: ScopeTraversalExpr {
                                    traversal: Traversal(vec![
                                        Traverser::Root {
                                            name: "var".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 8,
                                                    byte: 7,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 11,
                                                    byte: 10,
                                                },
                                            ),
                                        },
                                        Traverser::Attr {
                                            name: "public_subnets".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 11,
                                                    byte: 10,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 26,
                                                    byte: 25,
                                                },
                                            ),
                                        },
                                    ]),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 8,
                                            byte: 7,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 26,
                                            byte: 25,
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
                                                    column: 27,
                                                    byte: 26,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 32,
                                                    byte: 31,
                                                },
                                            ),
                                        },
                                        Traverser::Attr {
                                            name: "index".to_string(),
                                            src_range: rng(
                                                Pos {
                                                    line: 1,
                                                    column: 32,
                                                    byte: 31,
                                                },
                                                Pos {
                                                    line: 1,
                                                    column: 38,
                                                    byte: 37,
                                                },
                                            ),
                                        },
                                    ]),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 27,
                                            byte: 26,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 38,
                                            byte: 37,
                                        },
                                    ),
                                }
                                .into(),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 8,
                                        byte: 7,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 39,
                                        byte: 38,
                                    },
                                ),
                                open_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 26,
                                        byte: 25,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 27,
                                        byte: 26,
                                    },
                                ),
                                bracket_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 26,
                                        byte: 25,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 39,
                                        byte: 38,
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
        {
            // Go builds SplatExpr.Each and SplatExpr.Item as two separate
            // AnonSymbolExpr literals with identical ranges (the parser
            // shares one); here one Arc is cloned into both places.
            let anon = Arc::new(AnonSymbolExpr {
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 26,
                        byte: 25,
                    },
                    Pos {
                        line: 1,
                        column: 29,
                        byte: 28,
                    },
                ),
            });
            Case {
                input: "a = \"${var.public_subnets[*]}\"\n",
                diag_count: 0,
                want: Body {
                    attributes: Attributes::from([(
                        "a".to_string(),
                        Attribute {
                            name: "a".to_string(),
                            expr: TemplateWrapExpr {
                                wrapped: SplatExpr {
                                    source: ScopeTraversalExpr {
                                        traversal: Traversal(vec![
                                            Traverser::Root {
                                                name: "var".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 8,
                                                        byte: 7,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 11,
                                                        byte: 10,
                                                    },
                                                ),
                                            },
                                            Traverser::Attr {
                                                name: "public_subnets".to_string(),
                                                src_range: rng(
                                                    Pos {
                                                        line: 1,
                                                        column: 11,
                                                        byte: 10,
                                                    },
                                                    Pos {
                                                        line: 1,
                                                        column: 26,
                                                        byte: 25,
                                                    },
                                                ),
                                            },
                                        ]),
                                        src_range: rng(
                                            Pos {
                                                line: 1,
                                                column: 8,
                                                byte: 7,
                                            },
                                            Pos {
                                                line: 1,
                                                column: 26,
                                                byte: 25,
                                            },
                                        ),
                                    }
                                    .into(),
                                    each: anon.clone().into(),
                                    item: anon,
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 8,
                                            byte: 7,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 29,
                                            byte: 28,
                                        },
                                    ),
                                    marker_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 26,
                                            byte: 25,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 29,
                                            byte: 28,
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
                                        column: 31,
                                        byte: 30,
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
                                    column: 31,
                                    byte: 30,
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
                            byte: 31,
                        },
                    ),
                    end_range: rng(
                        Pos {
                            line: 2,
                            column: 1,
                            byte: 31,
                        },
                        Pos {
                            line: 2,
                            column: 1,
                            byte: 31,
                        },
                    ),
                },
            }
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
