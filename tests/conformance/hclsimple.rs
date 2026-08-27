//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsimple/hclsimple_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::gohcl::FromBody;
use hcl::hclsimple;

// NOTE(port): hclsimple/hclsimple_test.go also contains two Go `Example`
// functions (Go documentation examples), not `func Test*` tests, so they are
// outside the conformance universe and are not ported:
//   - Example_nativeSyntax:
//     https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsimple/hclsimple_test.go#L15
//   - Example_jsonSyntax:
//     https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsimple/hclsimple_test.go#L40

// Ported from TestDecodeFile:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsimple/hclsimple_test.go#L67
#[test]
#[ignore = "not yet implemented"]
fn decode_file() {
    // Go: the test-local `Config` struct with `hcl:"foo"` / `hcl:"baz"` tags.
    #[derive(Debug, PartialEq, FromBody)]
    struct Config {
        #[hcl(attr = "foo")]
        foo: String,
        #[hcl(attr = "baz")]
        baz: String,
    }

    // Go reads `testdata/test.hcl` relative to the hclsimple package; the
    // fixture is copied byte-for-byte to `tests/testdata/hclsimple/test.hcl`
    // (see tests/testdata/PROVENANCE.md) and loaded relative to the crate
    // root.
    let got: Config = hclsimple::decode_file("tests/testdata/hclsimple/test.hcl", None)
        .unwrap_or_else(|err| panic!("unexpected error(s): {err}"));
    let want = Config {
        foo: "bar".to_string(),
        baz: "boop".to_string(),
    };
    assert_eq!(got, want, "wrong result");
}
