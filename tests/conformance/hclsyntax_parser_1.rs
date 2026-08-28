//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/parser_test.go (TestParseConfig, part 1)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax::{
    self, Attribute, Attributes, Block, Blocks, Expression, LiteralValueExpr, TemplateExpr,
};
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

struct Case {
    input: &'static str,
    diag_count: usize,
    want: hclsyntax::Body,
}

// Ported from TestParseConfig:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/parser_test.go#L18
// (part 1: cases with opening brace before upstream line 984)
#[test]
#[ignore = "not yet implemented"]
fn parse_config_part1() {
    let tests = [
        Case {
            input: "",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: Blocks::new(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
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
                        column: 1,
                        byte: 0,
                    },
                ),
            },
        },
        Case {
            input: "block {}\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec![],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
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
                        ),
                        end_range: rng(
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
                    label_ranges: vec![],
                    open_brace_range: rng(
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
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                        Pos {
                            line: 1,
                            column: 9,
                            byte: 8,
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
                        line: 2,
                        column: 1,
                        byte: 9,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 9,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 9,
                    },
                ),
            },
        },
        Case {
            input: "block {}",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec![],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
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
                        ),
                        end_range: rng(
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
                    label_ranges: vec![],
                    open_brace_range: rng(
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
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                        Pos {
                            line: 1,
                            column: 9,
                            byte: 8,
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
                        column: 9,
                        byte: 8,
                    },
                ),
                end_range: rng(
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
                ),
            },
        },
        Case {
            input: "block {}block {}\n",
            diag_count: 1, // missing newline after block definition
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec![],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
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
                        ),
                        end_range: rng(
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
                    label_ranges: vec![],
                    open_brace_range: rng(
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
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 8,
                            byte: 7,
                        },
                        Pos {
                            line: 1,
                            column: 9,
                            byte: 8,
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
            input: "block { block {} }\n",
            diag_count: 1, // can't nest another block in the single-line block syntax
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec![],
                    // NOTE(port): upstream leaves this inner body's
                    // Attributes/Blocks as nil; Rust has no nil map/slice,
                    // so they are the empty map/vec here.
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 7,
                                byte: 6,
                            },
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 19,
                            },
                        ),
                        // Parser recovery behavior leaves us after this
                        // whole construct, on the next line
                        end_range: rng(
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 19,
                            },
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 19,
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
                    label_ranges: vec![],
                    open_brace_range: rng(
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
                    // Parser recovery behavior leaves us after this whole
                    // construct, on the next line
                    close_brace_range: rng(
                        Pos {
                            line: 2,
                            column: 1,
                            byte: 19,
                        },
                        Pos {
                            line: 2,
                            column: 1,
                            byte: 19,
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
                        line: 2,
                        column: 1,
                        byte: 19,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 19,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 19,
                    },
                ),
            },
        },
        Case {
            input: "block \"foo\" {}\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["foo".to_string()],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 13,
                                byte: 12,
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
                            column: 12,
                            byte: 11,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 13,
                            byte: 12,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    ),
                    close_brace_range: rng(
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
                }],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
            },
        },
        Case {
            input: "block foo {}\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["foo".to_string()],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
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
                        end_range: rng(
                            Pos {
                                line: 1,
                                column: 13,
                                byte: 12,
                            },
                            Pos {
                                line: 1,
                                column: 13,
                                byte: 12,
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
                            column: 10,
                            byte: 9,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 11,
                            byte: 10,
                        },
                        Pos {
                            line: 1,
                            column: 12,
                            byte: 11,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 12,
                            byte: 11,
                        },
                        Pos {
                            line: 1,
                            column: 13,
                            byte: 12,
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
            input: "block \"invalid ${not_allowed_here} foo\" {}\n",
            // Invalid string literal; Template sequences are not allowed
            // in this string.
            diag_count: 1,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    // invalid interpolation gets replaced with a
                    // placeholder here
                    labels: vec!["invalid ${ ... } foo".to_string()],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 41,
                                byte: 40,
                            },
                            Pos {
                                line: 1,
                                column: 43,
                                byte: 42,
                            },
                        ),
                        end_range: rng(
                            Pos {
                                line: 1,
                                column: 43,
                                byte: 42,
                            },
                            Pos {
                                line: 1,
                                column: 43,
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
                            column: 40,
                            byte: 39,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 41,
                            byte: 40,
                        },
                        Pos {
                            line: 1,
                            column: 42,
                            byte: 41,
                        },
                    ),
                    close_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 42,
                            byte: 41,
                        },
                        Pos {
                            line: 1,
                            column: 43,
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
            input: "\nblock \"invalid\" 1.2 {}\nblock \"valid\" {}\n",
            diag_count: 1,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![
                    Block {
                        block_type: "block".to_string(),
                        labels: vec!["invalid".to_string()],
                        // NOTE(port): upstream leaves this inner body's
                        // Attributes/Blocks as nil; Rust has no nil
                        // map/slice, so they are the empty map/vec here.
                        body: hclsyntax::Body {
                            attributes: Attributes::new(),
                            blocks: Blocks::new(),
                            src_range: rng(
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 1,
                                },
                                Pos {
                                    line: 2,
                                    column: 6,
                                    byte: 6,
                                },
                            ),
                            end_range: rng(
                                Pos {
                                    line: 2,
                                    column: 1,
                                    byte: 1,
                                },
                                Pos {
                                    line: 2,
                                    column: 6,
                                    byte: 6,
                                },
                            ),
                        },
                        type_range: rng(
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 1,
                            },
                            Pos {
                                line: 2,
                                column: 6,
                                byte: 6,
                            },
                        ),
                        label_ranges: vec![rng(
                            Pos {
                                line: 2,
                                column: 7,
                                byte: 7,
                            },
                            Pos {
                                line: 2,
                                column: 16,
                                byte: 16,
                            },
                        )],
                        // Since we failed parsing before we got to the
                        // braces, the type range is used as a placeholder
                        // for these.
                        open_brace_range: rng(
                            Pos {
                                line: 2,
                                column: 1,
                                byte: 1,
                            },
                            Pos {
                                line: 2,
                                column: 6,
                                byte: 6,
                            },
                        ),
                        close_brace_range: rng(
                            Pos {
                                line: 2,
                                column: 7,
                                byte: 7,
                            },
                            Pos {
                                line: 2,
                                column: 16,
                                byte: 16,
                            },
                        ),
                    },
                    // Recovery behavior should allow us to still see this
                    // second block, even though the first was invalid.
                    Block {
                        block_type: "block".to_string(),
                        labels: vec!["valid".to_string()],
                        body: hclsyntax::Body {
                            attributes: Attributes::new(),
                            blocks: Blocks::new(),
                            src_range: rng(
                                Pos {
                                    line: 3,
                                    column: 15,
                                    byte: 38,
                                },
                                Pos {
                                    line: 3,
                                    column: 17,
                                    byte: 40,
                                },
                            ),
                            end_range: rng(
                                Pos {
                                    line: 3,
                                    column: 17,
                                    byte: 40,
                                },
                                Pos {
                                    line: 3,
                                    column: 17,
                                    byte: 40,
                                },
                            ),
                        },
                        type_range: rng(
                            Pos {
                                line: 3,
                                column: 1,
                                byte: 24,
                            },
                            Pos {
                                line: 3,
                                column: 6,
                                byte: 29,
                            },
                        ),
                        label_ranges: vec![rng(
                            Pos {
                                line: 3,
                                column: 7,
                                byte: 30,
                            },
                            Pos {
                                line: 3,
                                column: 14,
                                byte: 37,
                            },
                        )],
                        open_brace_range: rng(
                            Pos {
                                line: 3,
                                column: 15,
                                byte: 38,
                            },
                            Pos {
                                line: 3,
                                column: 16,
                                byte: 39,
                            },
                        ),
                        close_brace_range: rng(
                            Pos {
                                line: 3,
                                column: 16,
                                byte: 39,
                            },
                            Pos {
                                line: 3,
                                column: 17,
                                byte: 40,
                            },
                        ),
                    },
                ],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 4,
                        column: 1,
                        byte: 41,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 4,
                        column: 1,
                        byte: 41,
                    },
                    Pos {
                        line: 4,
                        column: 1,
                        byte: 41,
                    },
                ),
            },
        },
        Case {
            input: "block \"f\\o\" {}\n",
            diag_count: 1, // "\o" is not a valid escape sequence
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["fo".to_string()],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 13,
                                byte: 12,
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
                            column: 12,
                            byte: 11,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 13,
                            byte: 12,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    ),
                    close_brace_range: rng(
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
                }],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
            },
        },
        Case {
            input: "block \"f\\n\" {}\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: Attributes::new(),
                blocks: vec![Block {
                    block_type: "block".to_string(),
                    labels: vec!["f\n".to_string()],
                    body: hclsyntax::Body {
                        attributes: Attributes::new(),
                        blocks: Blocks::new(),
                        src_range: rng(
                            Pos {
                                line: 1,
                                column: 13,
                                byte: 12,
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
                            column: 12,
                            byte: 11,
                        },
                    )],
                    open_brace_range: rng(
                        Pos {
                            line: 1,
                            column: 13,
                            byte: 12,
                        },
                        Pos {
                            line: 1,
                            column: 14,
                            byte: 13,
                        },
                    ),
                    close_brace_range: rng(
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
                }],
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
            },
        },
        Case {
            input: "a = 1\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 6,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 6,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 6,
                    },
                ),
            },
        },
        Case {
            input: "a = 1",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
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
                        column: 6,
                        byte: 5,
                    },
                    Pos {
                        line: 1,
                        column: 6,
                        byte: 5,
                    },
                ),
            },
        },
        Case {
            input: "a = \"hello ${true}\"\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![
                                Expression::LiteralValue(LiteralValueExpr {
                                    val: Value::string("hello "),
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
                                }),
                                Expression::LiteralValue(LiteralValueExpr {
                                    val: Value::bool(true),
                                    src_range: rng(
                                        Pos {
                                            line: 1,
                                            column: 14,
                                            byte: 13,
                                        },
                                        Pos {
                                            line: 1,
                                            column: 18,
                                            byte: 17,
                                        },
                                    ),
                                }),
                            ],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 20,
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
                                line: 1,
                                column: 20,
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 20,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 20,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 20,
                    },
                ),
            },
        },
        Case {
            input: "a = \"hello $${true}\"\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![Expression::LiteralValue(LiteralValueExpr {
                                val: Value::string("hello ${true}"),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 6,
                                        byte: 5,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 20,
                                        byte: 19,
                                    },
                                ),
                            })],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 21,
                                    byte: 20,
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
                                column: 21,
                                byte: 20,
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
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
            input: "a = \"hello %%{true}\"\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![Expression::LiteralValue(LiteralValueExpr {
                                val: Value::string("hello %{true}"),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 6,
                                        byte: 5,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 20,
                                        byte: 19,
                                    },
                                ),
                            })],
                            src_range: rng(
                                Pos {
                                    line: 1,
                                    column: 5,
                                    byte: 4,
                                },
                                Pos {
                                    line: 1,
                                    column: 21,
                                    byte: 20,
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
                                column: 21,
                                byte: 20,
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
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
            input: "a = \"hello $$\"\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![Expression::LiteralValue(LiteralValueExpr {
                                val: Value::string("hello $$"),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 6,
                                        byte: 5,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 14,
                                        byte: 13,
                                    },
                                ),
                            })],
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
            },
        },
        Case {
            input: "a = \"hello $\"\n",
            diag_count: 0, // unterminated template interpolation sequence
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![Expression::LiteralValue(LiteralValueExpr {
                                val: Value::string("hello $"),
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
                            })],
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
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
            input: "a = \"hello %%\"\n",
            diag_count: 0,
            want: hclsyntax::Body {
                attributes: [(
                    "a".to_string(),
                    Attribute {
                        name: "a".to_string(),
                        expr: TemplateExpr {
                            parts: vec![Expression::LiteralValue(LiteralValueExpr {
                                val: Value::string("hello %%"),
                                src_range: rng(
                                    Pos {
                                        line: 1,
                                        column: 6,
                                        byte: 5,
                                    },
                                    Pos {
                                        line: 1,
                                        column: 14,
                                        byte: 13,
                                    },
                                ),
                            })],
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
                )]
                .into_iter()
                .collect(),
                blocks: Blocks::new(),
                src_range: rng(
                    Pos {
                        line: 1,
                        column: 1,
                        byte: 0,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                ),
                end_range: rng(
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
                    },
                    Pos {
                        line: 2,
                        column: 1,
                        byte: 15,
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
            diags,
        );

        let got = file
            .body
            .as_any()
            .downcast_ref::<hclsyntax::Body>()
            .expect("file body is not hclsyntax::Body");
        assert_eq!(got, &test.want, "case {i} ({:?}): wrong result", test.input);
    }
}
