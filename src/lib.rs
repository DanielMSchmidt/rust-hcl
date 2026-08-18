//! A Rust implementation of [HCL v2](https://github.com/hashicorp/hcl).
//!
//! **v2 only.** The v1 API (upstream `hcl/` subdirectory) is out of scope.
//!
//! Empty on purpose. Everything under `src/` is Daniel's to write — see `CLAUDE.md`.
//!
//! Upstream layout, as a rough map of what eventually lives here:
//!
//! | hcl v2 package | notes |
//! |---|---|
//! | `hcl` | `Body`, `Schema`, `Diagnostics`, `Range`, `Traversal` |
//! | `hclsyntax` | native-syntax scanner, parser, expression evaluation |
//! | `json` | the JSON syntax variant |
//! | `hcldec` | spec-driven decoding to `cty.Value` |
//! | `hclwrite` | round-trip-preserving AST for rewriting config |
//! | `ext/dynblock` | `dynamic` block expansion |
//! | `ext/typeexpr` | type constraint expressions |
//! | `ext/tryfunc`, `ext/customdecode` | Terraform-facing extensions |
//!
//! Diagnostics are the thing to get right early: they *accumulate* rather than
//! short-circuit, so a `?`-based error design will fight the domain. Terraform's entire
//! error UX is these ranges and messages.
