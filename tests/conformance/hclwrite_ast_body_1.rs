//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/ast_body_test.go (part 1: TestBodyGetAttribute through
//!   TestBodySetAttributeValue_ReturnsTheAttribute; the remaining test
//!   funcs live in hclwrite_ast_body_2.rs)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::hclsyntax::{self, TokenType};
use hcl::hclwrite::{self, Token, Tokens, tokens_for_value};
use hcl::{Pos, Range, Traversal, Traverser};

/// The parse start position used throughout upstream:
/// `hcl.Pos{Line: 1, Column: 1}` (with `Byte` left as its zero value).
fn start_pos() -> Pos {
    Pos {
        line: 1,
        column: 1,
        byte: 0,
    }
}

/// A write-token literal (Go: the `hclwrite.Token{Type: ..., Bytes: ...,
/// SpacesBefore: ...}` struct literals in the upstream tables).
fn tok(ty: TokenType, bytes: &[u8], spaces_before: usize) -> Token {
    Token {
        ty,
        bytes: bytes.to_vec(),
        spaces_before,
    }
}

/// Parses the source and asserts there are no diagnostics, like the
/// upstream `ParseConfig` + `len(diags) != 0` preamble (and the `testFn`
/// helper used by the `*_ReturnsTheAttribute` tests).
fn parse_no_diags(src: &str) -> hclwrite::File {
    let (f, diags) = hclwrite::parse_config(src.as_bytes(), "", start_pos());
    assert!(
        diags.is_empty(),
        "unexpected diagnostics parsing {src:?}: {diags:?}"
    );
    f
}

// Ported from TestBodyGetAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L20
#[test]
#[ignore = "not yet implemented"]
fn body_get_attribute() {
    struct Case {
        src: &'static str,
        name: &'static str,
        // Go `nil` Tokens (attribute expected absent) ⇒ `None`.
        want: Option<Tokens>,
    }

    let tests = [
        Case {
            src: "",
            name: "a",
            want: None,
        },
        Case {
            src: "a = 1\n",
            name: "a",
            want: Some(Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"1", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
        Case {
            src: "a = 1\nb = 1\nc = 1\n",
            name: "a",
            want: Some(Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"1", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
        Case {
            src: "a = 1\nb = 2\nc = 3\n",
            name: "b",
            want: Some(Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"2", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
        Case {
            src: "a = 1\nb = 2\nc = 3\n",
            name: "c",
            want: Some(Tokens(vec![
                tok(TokenType::Ident, b"c", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"3", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
        Case {
            src: "a = 1\n# b is a b\nb = 2\nc = 3\n",
            name: "b",
            want: Some(Tokens(vec![
                // Recognized as a lead comment and so attached to the attribute
                tok(TokenType::Comment, b"# b is a b\n", 0),
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"2", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
        Case {
            src: "a = 1\n# not attached to a or b\n\nb = 2\nc = 3\n",
            name: "b",
            want: Some(Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"2", 1),
                tok(TokenType::Newline, b"\n", 0),
            ])),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = parse_no_diags(test.src);

        let attr = f.body().get_attribute(test.name);
        match attr {
            None => {
                assert!(
                    test.want.is_none(),
                    "case {i} ({} in {:?}): attribute not found, but want it to exist",
                    test.name,
                    test.src,
                );
            }
            Some(attr) => {
                let want = test.want.as_ref().unwrap_or_else(|| {
                    panic!(
                        "case {i} ({} in {:?}): attribute found, but expecting not found",
                        test.name, test.src,
                    )
                });

                let got = attr.build_tokens();
                assert_eq!(
                    &got, want,
                    "case {i} ({} in {:?}): wrong result",
                    test.name, test.src,
                );
            }
        }
    }
}

// Ported from TestBodyFirstMatchingBlock:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L224
#[test]
#[ignore = "not yet implemented"]
fn body_first_matching_block() {
    let src = r#"a = "b"
service {
  attr0 = "val0"
}
service "label1" {
  attr1 = "val1"
}
service "label1" "label2" {
  attr2 = "val2"
}
parent {
  attr3 = "val3"
  child {
    attr4 = "val4"
  }
}
"#;

    struct Case {
        src: &'static str,
        type_name: &'static str,
        labels: &'static [&'static str],
        // Go `""` means the block is expected absent (`nil`).
        want: &'static str,
    }

    let tests = [
        Case {
            src,
            type_name: "service",
            labels: &[],
            want: "service {\n  attr0 = \"val0\"\n}\n",
        },
        Case {
            src,
            type_name: "service",
            labels: &["label1"],
            want: "service \"label1\" {\n  attr1 = \"val1\"\n}\n",
        },
        Case {
            src,
            type_name: "service",
            labels: &["label1", "label2"],
            want: "service \"label1\" \"label2\" {\n  attr2 = \"val2\"\n}\n",
        },
        Case {
            src,
            type_name: "parent",
            labels: &[],
            want: "parent {\n  attr3 = \"val3\"\n  child {\n    attr4 = \"val4\"\n  }\n}\n",
        },
        Case {
            src,
            type_name: "hoge",
            labels: &[],
            want: "",
        },
        Case {
            src,
            type_name: "hoge",
            labels: &["label1"],
            want: "",
        },
        Case {
            src,
            type_name: "service",
            labels: &["label2"],
            want: "",
        },
        Case {
            src,
            type_name: "service",
            labels: &["label2", "label1"],
            want: "",
        },
        Case {
            src,
            type_name: "child",
            labels: &[],
            want: "",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = parse_no_diags(test.src);

        let block = f.body().first_matching_block(test.type_name, test.labels);
        match block {
            None => {
                assert!(
                    test.want.is_empty(),
                    "case {i} ({} {}): block not found, but want it to exist",
                    test.type_name,
                    test.labels.join(" "),
                );
            }
            Some(block) => {
                assert!(
                    !test.want.is_empty(),
                    "case {i} ({} {}): block found, but expecting not found",
                    test.type_name,
                    test.labels.join(" "),
                );

                let got = String::from_utf8(block.build_tokens().bytes()).unwrap();
                assert_eq!(
                    got,
                    test.want,
                    "case {i} ({} {}): wrong result",
                    test.type_name,
                    test.labels.join(" "),
                );
            }
        }
    }
}

// Ported from TestBodySetAttributeValue:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L349
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_value() {
    struct Case {
        src: &'static str,
        name: &'static str,
        val: Value,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            name: "a",
            val: Value::bool(true),
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "b = false\n",
            name: "a",
            val: Value::bool(true),
            want: Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = false\n",
            name: "a",
            val: Value::bool(true),
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = 1\nb = false\n",
            name: "a",
            val: Value::bool(true),
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = parse_no_diags(test.src);

        f.body().set_attribute_value(test.name, test.val.clone());
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got, test.want,
            "case {i} ({} = {:?} in {:?}): wrong result",
            test.name, test.val, test.src,
        );
    }
}

// Ported from TestBodySetAttributeTraversal:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L547
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_traversal() {
    struct Case {
        src: &'static str,
        name: &'static str,
        trav: &'static str,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            name: "a",
            trav: "b",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"b", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "",
            name: "a",
            trav: "b.c.d",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"b", 1),
                tok(TokenType::Dot, b".", 0),
                tok(TokenType::Ident, b"c", 0),
                tok(TokenType::Dot, b".", 0),
                tok(TokenType::Ident, b"d", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "",
            name: "a",
            trav: "b[0]",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"b", 1),
                tok(TokenType::OBrack, b"[", 0),
                tok(TokenType::NumberLit, b"0", 0),
                tok(TokenType::CBrack, b"]", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "",
            name: "a",
            trav: "b[0].c",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"b", 1),
                tok(TokenType::OBrack, b"[", 0),
                tok(TokenType::NumberLit, b"0", 0),
                tok(TokenType::CBrack, b"]", 0),
                tok(TokenType::Dot, b".", 0),
                tok(TokenType::Ident, b"c", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = parse_no_diags(test.src);

        let (traversal, diags) =
            hclsyntax::parse_traversal_abs(test.trav.as_bytes(), "", start_pos());
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics from traversal {:?}: {diags:?}",
            test.trav,
        );

        f.body().set_attribute_traversal(test.name, traversal);
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got, test.want,
            "case {i} ({} = {} in {:?}): wrong result",
            test.name, test.trav, test.src,
        );
    }
}

// Ported from TestBodySetAttributeTraversal_ReturnsTheAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L773
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_traversal_returns_the_attribute() {
    struct Case {
        name: &'static str,
        config: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            name: "attribute `one` is already set to a value",
            config: "one = 1",
            want: "one =the.loneliest.number",
        },
        Case {
            name: "attribute `one` is not set to a value",
            config: "two = 2",
            want: "one=the.loneliest.number\n",
        },
    ];

    for test in &tests {
        let f = parse_no_diags(test.config);

        // Go: hcl.Traversal{TraverseRoot{Name: "the"}, TraverseAttr{...}}
        // with SrcRange left as its zero value.
        let traversal = Traversal(vec![
            Traverser::Root {
                name: "the".to_string(),
                src_range: Range::default(),
            },
            Traverser::Attr {
                name: "loneliest".to_string(),
                src_range: Range::default(),
            },
            Traverser::Attr {
                name: "number".to_string(),
                src_range: Range::default(),
            },
        ]);

        // NOTE(port): upstream checks `attr == nil`; the Rust signature
        // returns the Attribute directly, so a nil result is
        // unrepresentable.
        let attr = f.body().set_attribute_traversal("one", traversal);

        let got = attr.build_tokens().bytes();
        assert_eq!(
            String::from_utf8_lossy(&got),
            test.want,
            "case {:?}: wrong result",
            test.name,
        );
    }
}

// Ported from TestBodySetAttributeRaw:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L811
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_raw() {
    struct Case {
        src: &'static str,
        name: &'static str,
        tokens: Tokens,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            name: "a",
            tokens: Tokens(vec![tok(TokenType::Ident, b"true", 0)]),
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = 23\n",
            name: "a",
            tokens: Tokens(vec![tok(TokenType::Ident, b"true", 0)]),
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "b = 23\n",
            name: "a",
            tokens: Tokens(vec![tok(TokenType::Ident, b"true", 0)]),
            want: Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::NumberLit, b"23", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"true", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = parse_no_diags(test.src);

        f.body().set_attribute_raw(test.name, test.tokens.clone());
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got,
            test.want,
            "case {i} ({} = {} in {:?}): wrong result",
            test.name,
            String::from_utf8_lossy(&test.tokens.bytes()),
            test.src,
        );
    }
}

// Ported from TestBodySetAttributeRaw_ReturnsTheAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L975
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_raw_returns_the_attribute() {
    struct Case {
        name: &'static str,
        config: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            name: "attribute `one` is already set to a value",
            config: "one = 1",
            want: "one =\"the loneliest number\"",
        },
        Case {
            name: "attribute `one` is not set to a value",
            config: "two = 2",
            want: "one=\"the loneliest number\"\n",
        },
    ];

    for test in &tests {
        let f = parse_no_diags(test.config);

        // NOTE(port): upstream checks `attr == nil`; the Rust signature
        // returns the Attribute directly, so a nil result is
        // unrepresentable.
        let attr = f.body().set_attribute_raw(
            "one",
            tokens_for_value(&Value::string("the loneliest number")),
        );

        let got = attr.build_tokens().bytes();
        assert_eq!(
            String::from_utf8_lossy(&got),
            test.want,
            "case {:?}: wrong result",
            test.name,
        );
    }
}

// Ported from TestBodySetAttributeValue_ReturnsTheAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1006
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_value_returns_the_attribute() {
    struct Case {
        name: &'static str,
        config: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            name: "attribute `one` is already set to a value",
            config: "one = 1",
            want: "one =\"the loneliest number\"",
        },
        Case {
            name: "attribute `one` is not set to a value",
            config: "two = 2",
            want: "one=\"the loneliest number\"\n",
        },
    ];

    for test in &tests {
        let f = parse_no_diags(test.config);

        // NOTE(port): upstream checks `attr == nil`; the Rust signature
        // returns the Attribute directly, so a nil result is
        // unrepresentable.
        let attr = f
            .body()
            .set_attribute_value("one", Value::string("the loneliest number"));

        let got = attr.build_tokens().bytes();
        assert_eq!(
            String::from_utf8_lossy(&got),
            test.want,
            "case {:?}: wrong result",
            test.name,
        );
    }
}
