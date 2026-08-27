# hcl

A Rust implementation of [HCL v2](https://github.com/hashicorp/hcl) — HashiCorp
Configuration Language. **v2 only**; the v1 API is out of scope.

**Status: early. `src/` is API-shaped stubs by design** — every public signature
exists (so the conformance suite compiles), every body is `todo!()`. The Go→Rust
API correspondence is documented in `docs/api-mapping.md`.

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

The upstream Go test suite is the specification. Tests under `tests/conformance/`
are transcribed from it, with expected values taken as literals from upstream so
they specify behavior rather than mirror this implementation. Each file carries
the upstream path(s) and pinned commit SHA
(`6abbb088cdb82416d1b3d9fcbaab29534133567a`), and each test a permalink to the
`func Test*` it was ported from. Go-only cases (nil zero values, pointer
identity, reflection shapes) are kept in place as `NOTE(port)` comments rather
than silently dropped.

Every test starts marked `#[ignore = "not yet implemented"]`; that is the backlog.
As behavior lands, run the backlog with `cargo test -- --ignored`, and delete the
`#[ignore]` from tests that now pass — from then on they gate CI like any other
test. `cargo test -- --include-ignored` runs everything.

HCL also ships an [implementation-agnostic spec suite](https://github.com/hashicorp/hcl/tree/main/specsuite)
that drives an implementation through an `hcldec`-compatible executable rather than the Go
API; it maps case-by-case onto [the HCL spec](https://github.com/hashicorp/hcl/blob/main/spec.md).
Its fixtures are copied under `tests/testdata/specsuite/` and driven by the
`conformance_specsuite` test target (ignored like everything else, one test per
fixture). Once this crate ships an `hcldec`-compatible executable, the upstream
harness also runs against it unmodified — its pass rate is the compliance number
that counts.

The `gohcl` package's struct-tag reflection is ported via the `hcl-derive`
workspace crate: `#[derive(FromBody)]` / `#[derive(EncodeBody)]` with
`#[hcl(...)]` field attributes mirroring the Go `hcl:"..."` tags (grammar in
`docs/api-mapping.md`). The derives accept the grammar so annotated structs
compile; their emitted bodies are `todo!()` like everything else.

## Licensing

Original work in this repository — everything under `src/`, `hcl-derive/`,
`docs/`, and the build/CI scaffolding — is MIT licensed (see `LICENSE`).

[hashicorp/hcl](https://github.com/hashicorp/hcl) is MPL-2.0, and the
transcribed test files are derivative works of its test code, as are the
copied fixtures. Therefore **every file under `tests/conformance/` and
`tests/testdata/` is licensed MPL-2.0** (see `LICENSE-MPL-2.0`) and carries an
MPL-2.0 notice plus a provenance header naming the upstream file and pinned
commit. If you redistribute this repository, both licenses apply, each to its
part.

## Development

```sh
cargo test
```

Toolchain is pinned in `rust-toolchain.toml`.

To build against a local `cty` working copy instead of the pinned git revision,
with both repos checked out as siblings:

```sh
cp .cargo/config.toml.example .cargo/config.toml
```
