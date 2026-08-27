//! Conformance harness for the HCL language spec suite
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   specsuite/spec_test.go
//!   specsuite/tests/** (copied byte-for-byte under tests/testdata/specsuite/)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Upstream drives these fixtures through the `hclspecsuite` harness
//! (`cmd/hclspecsuite`), which shells out to an `hcldec`-compatible
//! executable: for each `<name>.t` spec file it decodes the sibling
//! `<name>.hcl` (and, when present, `<name>.hcl.json`) with the sibling
//! `<name>.hcldec` spec, then checks the result value, expected
//! diagnostics, and expected variable traversals declared in the `.t`
//! file. `run_spec_test` below is the in-process Rust analogue of that
//! runner; like all of `src/`, its machinery is unimplemented until the
//! underlying parser/hcldec behavior exists, so every fixture test is
//! ignored.
//!
//! NOTE(port): TestMain
//! (https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L18)
//! is Go test-binary plumbing (building hcldec and the suite runner via
//! `go build` before running); it has no analogue in an in-process
//! harness and is intentionally not ported. TestSpec (#L47) is ported as
//! the per-fixture tests below.

use std::path::Path;

/// Runs one spec-suite fixture: decodes `<name>.hcl` (and `<name>.hcl.json`
/// when present) with `<name>.hcldec`, then checks everything `<name>.t`
/// declares (result value, diagnostics, traversals). The in-process
/// analogue of upstream's `hclspecsuite` runner
/// (cmd/hclspecsuite/runner.go).
fn run_spec_test(name: &str) {
    let base = Path::new("tests/testdata/specsuite/tests").join(name);
    let t_file = base.with_extension("t");
    assert!(
        t_file.exists(),
        "spec fixture missing: {}",
        t_file.display()
    );
    todo!("spec-suite runner: requires hclsyntax/json/hcldec implementations")
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/empty.t
#[test]
#[ignore = "not yet implemented"]
fn spec_empty() {
    run_spec_test("empty");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/comments/hash_comment.t
#[test]
#[ignore = "not yet implemented"]
fn spec_comments_hash_comment() {
    run_spec_test("comments/hash_comment");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/comments/slash_comment.t
#[test]
#[ignore = "not yet implemented"]
fn spec_comments_slash_comment() {
    run_spec_test("comments/slash_comment");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/comments/multiline_comment.t
#[test]
#[ignore = "not yet implemented"]
fn spec_comments_multiline_comment() {
    run_spec_test("comments/multiline_comment");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/expressions/heredoc.t
#[test]
#[ignore = "not yet implemented"]
fn spec_expressions_heredoc() {
    run_spec_test("expressions/heredoc");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/expressions/operators.t
#[test]
#[ignore = "not yet implemented"]
fn spec_expressions_operators() {
    run_spec_test("expressions/operators");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/expressions/primitive_literals.t
#[test]
#[ignore = "not yet implemented"]
fn spec_expressions_primitive_literals() {
    run_spec_test("expressions/primitive_literals");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/attributes/expected.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_attributes_expected() {
    run_spec_test("structure/attributes/expected");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/attributes/singleline_bad.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_attributes_singleline_bad() {
    run_spec_test("structure/attributes/singleline_bad");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/attributes/unexpected.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_attributes_unexpected() {
    run_spec_test("structure/attributes/unexpected");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/blocks/single_empty_oneline.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_blocks_single_empty_oneline() {
    run_spec_test("structure/blocks/single_empty_oneline");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/blocks/single_expected.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_blocks_single_expected() {
    run_spec_test("structure/blocks/single_expected");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/blocks/single_oneline.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_blocks_single_oneline() {
    run_spec_test("structure/blocks/single_oneline");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/blocks/single_oneline_invalid.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_blocks_single_oneline_invalid() {
    run_spec_test("structure/blocks/single_oneline_invalid");
}

// Ported from TestSpec (one test per spec-suite fixture):
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/spec_test.go#L47
// Fixture:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/specsuite/tests/structure/blocks/single_unclosed.t
#[test]
#[ignore = "not yet implemented"]
fn spec_structure_blocks_single_unclosed() {
    run_spec_test("structure/blocks/single_unclosed");
}
