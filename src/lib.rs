//! A Rust implementation of [HCL v2](https://github.com/hashicorp/hcl).
//!
//! **v2 only.** The v1 API (upstream `hcl/` subdirectory) is out of scope.
//!
//! **Status: API-shaped stubs.** Every public signature exists so the
//! conformance suite under `tests/conformance/` compiles; every body is
//! `todo!()`. The Go→Rust API correspondence is documented in
//! `docs/api-mapping.md`, and the implementation is written by hand,
//! deliberately — see `README.md`.
//!
//! Upstream package → module map:
//!
//! | hcl v2 package | here |
//! |---|---|
//! | `hcl` (root: `Body`, `Schema`, `Diagnostics`, `Range`, `Traversal`) | crate root |
//! | `hclsyntax` | [`hclsyntax`] |
//! | `json` | [`json`] |
//! | `hclparse` | [`hclparse`] |
//! | `hcldec` | [`hcldec`] |
//! | `hclwrite` | [`hclwrite`] |
//! | `gohcl` | [`gohcl`] (plus the `hcl-derive` proc-macro crate) |
//! | `hcled` | [`hcled`] |
//! | `hcltest` | [`hcltest`] |
//! | `hclsimple` | [`hclsimple`] |
//! | `ext/dynblock`, `ext/typeexpr`, `ext/tryfunc`, `ext/customdecode`, `ext/transform`, `ext/userfunc` | [`ext`] |
//!
//! Diagnostics *accumulate* rather than short-circuit: where Go returns
//! `(T, hcl.Diagnostics)`, Rust returns `(T, Diagnostics)` — not `Result` —
//! because callers routinely need the partial result alongside the
//! diagnostics. Terraform's entire error UX is these ranges and messages.

pub mod diagnostic;
pub mod diagnostic_text;
pub mod eval_context;
pub mod expr_helpers;
pub mod ext;
pub mod gohcl;
pub mod hcldec;
pub mod hcled;
pub mod hclparse;
pub mod hclsimple;
pub mod hclsyntax;
pub mod hcltest;
pub mod hclwrite;
pub mod json;
pub mod merged;
pub mod ops;
pub mod pos;
pub mod pos_scanner;
pub mod schema;
pub mod structure;
pub mod traversal;

pub use diagnostic::{
    Diagnostic, DiagnosticExtra, DiagnosticSeverity, DiagnosticWriter, Diagnostics,
    diagnostic_extra,
};
pub use diagnostic_text::DiagnosticTextWriter;
pub use eval_context::EvalContext;
pub use expr_helpers::{
    KeyValuePair, StaticCall, abs_traversal_for_expr, expr_as_keyword, expr_call, expr_list,
    expr_map, rel_traversal_for_expr, static_expr, unwrap_expression, unwrap_expression_until,
};
pub use merged::{empty_body, merge_bodies, merge_files};
pub use ops::{apply_path, get_attr, index};
pub use pos::{Pos, Range};
pub use pos_scanner::{RangeScanner, SplitResult, scan_lines};
pub use schema::{AttributeSchema, BlockHeaderSchema, BodySchema};
pub use structure::{
    Attribute, Attributes, Block, Blocks, Body, BodyContent, BodyRef, ExprRef, Expression, File,
    FileNav,
};
pub use traversal::{Traversal, TraversalSplit, Traverser};
