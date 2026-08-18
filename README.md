# hcl

A Rust implementation of [HCL v2](https://github.com/hashicorp/hcl) — HashiCorp
Configuration Language. **v2 only**; the v1 API is out of scope.

**Status: early. `src/` is empty by design** — see below.

Not published to crates.io. Depend on it by git URL:

```toml
hcl = { git = "https://github.com/DanielMSchmidt/rust-hcl", tag = "v0.1.0" }
```

## Why this exists

A learning project: a from-scratch Rust port of HCL, working toward an alternative
Terraform implementation. Every line of the implementation is written by hand, deliberately.
AI assistance here is limited to research, test porting, and debugging help — it does not
write the implementation.

## Conformance

HCL ships an [implementation-agnostic spec suite](https://github.com/hashicorp/hcl/tree/main/specsuite)
that drives an implementation through an `hcldec`-compatible executable rather than the Go
API. Once this crate ships one, that suite runs against it unmodified — its pass rate is the
compliance number that counts, and it maps case-by-case onto
[the HCL spec](https://github.com/hashicorp/hcl/blob/main/spec.md).

Tests needing the pinned upstream checkout read `$REFERENCE_DIR` and skip when it is unset.

## Development

```sh
cargo test
```

Toolchain is pinned in `rust-toolchain.toml`.

To build against a local `cty` working copy instead of the pinned git tag, with both
repos checked out as siblings:

```sh
cp .cargo/config.toml.example .cargo/config.toml
```
