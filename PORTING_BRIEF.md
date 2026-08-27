# Porting brief: transcribing hcl v2 tests into `tests/conformance/`

You are transcribing Go test files from the pinned upstream checkout of
`github.com/hashicorp/hcl` @ `6abbb088cdb82416d1b3d9fcbaab29534133567a`
into Rust conformance tests. **You port tests; you implement nothing.**
Read `docs/api-mapping.md` first — it is the law for every Go→Rust API
question. This file is the law for process.

## Non-negotiables

1. **Port every test function and every case, in upstream order.** A
   table-driven Go test becomes one `#[test]` fn looping over a case array.
   Do not sample, summarize, dedupe, or reorder cases.
2. **Expected values are literals**, copied from upstream — never computed,
   never "corrected", even when they look wrong. The Go tests are the spec.
3. **Every test starts ignored and links upstream.** Immediately above each
   `#[test]`:

   ```rust
   // Ported from TestXxx:
   // https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/<path>#L<line>
   #[test]
   #[ignore = "not yet implemented"]
   fn test_xxx() { ... }
   ```

   `<line>` is the line of the `func TestXxx(` declaration in the pinned
   checkout (you have it locally — verify with `grep -n`). One Go test
   func = one Rust test fn (same name, snake_case, keep the `Test` → bare
   mapping used by the file's existing tests). When a single Go func is
   split across several Rust targets by case ranges, EVERY part carries
   the same permalink and a `(part n: cases X–Y)` note in the comment.
4. **File header.** Every conformance file starts with the header comment
   style of `tests/conformance/pos.rs` (the exemplar): MPL-2.0 notice,
   upstream file path(s), pinned SHA.
5. **Untranslatable Go-only cases stay visible.** Keep them in place as
   `// NOTE(port): ...` comments explaining why (nil zero-value semantics,
   Go pointer identity, reflection-only shapes, etc.). Same for whole test
   funcs you cannot port — the NOTE plus permalink goes at the spot where
   the test would live.
6. **Assertion style.** `assert_eq!(got, want, "case {i}: ...")` with the
   case index and (when short) the input in the message. Where upstream
   expects a panic, use `std::panic::catch_unwind`. Where upstream checks
   `len(diags) != 0` or `diags.HasErrors()`, do the same — do not tighten
   or loosen.
7. **Quality gate.** Before reporting done:
   - `cargo clippy --test <your target> -- -D warnings` passes
     (allow(clippy::…) at file top only if the exemplar does it).
   - `cargo fmt --all` (run it; do not hand-format).
   - **Never run plain `cargo test`** — everything is `#[ignore]`d anyway
     and the workspace is large.
8. **Missing API.** If a signature you need genuinely does not exist in
   `src/`, add a minimal documented `todo!()` stub following the existing
   stub style (one-line doc with the `(hcl: ...)` upstream reference).
   Other agents edit `src/` concurrently: **re-read the src file
   immediately before editing it**, keep the edit minimal, and list every
   src addition in your final report. Do not restructure existing code.
9. **Fixtures.** If a test reads a testdata file, copy it into
   `tests/testdata/<package>/...` with a provenance header comment naming
   the upstream path and SHA (skip the header only where the format cannot
   carry comments — note that in your report). Load it with a path
   relative to the crate root (`tests/testdata/...`).
10. **Go test helpers.** Unexported helpers used only by tests (custom
    comparers, fixture builders) get transcribed as private fns inside the
    test file, not into `src/`.

## Report format (your final message)

- Ported: `<TestName>` → `<rust fn>` (N cases) — per function.
- Omitted (with reasons + permalink): ...
- `src/` additions: exact items added, or "none".
- Fixtures copied: list, or "none".
