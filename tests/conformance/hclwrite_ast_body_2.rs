//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/ast_body_test.go (test funcs from line 1038 to end of file)
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use cty::Value;
use hcl::Pos;
use hcl::hclsyntax::TokenType;
use hcl::hclwrite::{self, Token, Tokens};

/// `hclwrite.Token{Type: ty, Bytes: bytes, SpacesBefore: spaces_before}`.
fn tok(ty: TokenType, bytes: &[u8], spaces_before: usize) -> Token {
    Token {
        ty,
        bytes: bytes.to_vec(),
        spaces_before,
    }
}

// Ported from TestBodySetAttributeValueInBlock:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1038
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_value_in_block() {
    let src = "service \"label1\" {\n  attr1 = \"val1\"\n}\n";

    struct Case {
        src: &'static str,
        type_name: &'static str,
        labels: &'static [&'static str],
        attr: &'static str,
        val: Value,
        want: &'static str,
    }

    let tests = [Case {
        src,
        type_name: "service",
        labels: &["label1"],
        attr: "attr1",
        val: Value::string("updated1"),
        want: "service \"label1\" {\n  attr1 = \"updated1\"\n}\n",
    }];

    for (i, test) in tests.iter().enumerate() {
        let (f, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics: {diags:?}"
        );

        let b = f
            .body()
            .first_matching_block(test.type_name, test.labels)
            .unwrap();
        b.body().set_attribute_value(test.attr, test.val.clone());
        let mut tokens = f.build_tokens();
        hclwrite::format_tokens(&mut tokens);
        let got = String::from_utf8(tokens.bytes()).unwrap();
        assert_eq!(
            got, test.want,
            "case {i} ({} in {} {:?}): wrong result",
            test.attr, test.type_name, test.labels,
        );
    }
}

// Ported from TestBodySetAttributeValueInNestedBlock:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1086
#[test]
#[ignore = "not yet implemented"]
fn body_set_attribute_value_in_nested_block() {
    let src = "parent {\n  attr1 = \"val1\"\n  child {\n    attr2 = \"val2\"\n  }\n}\n";

    struct Case {
        src: &'static str,
        parent_type_name: &'static str,
        child_type_name: &'static str,
        attr: &'static str,
        val: Value,
        want: &'static str,
    }

    let tests = [Case {
        src,
        parent_type_name: "parent",
        child_type_name: "child",
        attr: "attr2",
        val: Value::string("updated2"),
        want: "parent {\n  attr1 = \"val1\"\n  child {\n    attr2 = \"updated2\"\n  }\n}\n",
    }];

    for (i, test) in tests.iter().enumerate() {
        let (f, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics: {diags:?}"
        );

        let parent = f
            .body()
            .first_matching_block(test.parent_type_name, &[])
            .unwrap();
        let child = parent
            .body()
            .first_matching_block(test.child_type_name, &[])
            .unwrap();
        child
            .body()
            .set_attribute_value(test.attr, test.val.clone());
        let mut tokens = f.build_tokens();
        hclwrite::format_tokens(&mut tokens);
        let got = String::from_utf8(tokens.bytes()).unwrap();
        assert_eq!(
            got, test.want,
            "case {i} ({} in {} in {}): wrong result",
            test.attr, test.child_type_name, test.parent_type_name,
        );
    }
}

// Ported from TestBodyRemoveAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1141
#[test]
#[ignore = "not yet implemented"]
fn body_remove_attribute() {
    struct Case {
        src: &'static str,
        name: &'static str,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            name: "a",
            want: Tokens(vec![tok(TokenType::EOF, b"", 0)]),
        },
        Case {
            src: "b = false\n",
            name: "a",
            want: Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = false\n",
            name: "a",
            want: Tokens(vec![tok(TokenType::EOF, b"", 0)]),
        },
        Case {
            src: "a = 1\nb = false\n",
            name: "a",
            want: Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (f, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics: {diags:?}"
        );

        f.body().remove_attribute(test.name);
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got, test.want,
            "case {i} ({} in {:?}): wrong result",
            test.name, test.src,
        );
    }
}

// Ported from TestBodyRenameAttribute:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1254
#[test]
#[ignore = "not yet implemented"]
fn body_rename_attribute() {
    struct Case {
        src: &'static str,
        old_name: &'static str,
        new_name: &'static str,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            old_name: "a",
            new_name: "b",
            want: Tokens(vec![tok(TokenType::EOF, b"", 0)]),
        },
        Case {
            src: "a = false\n",
            old_name: "a",
            new_name: "b",
            want: Tokens(vec![
                tok(TokenType::Ident, b"b", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = false\n",
            old_name: "b",
            new_name: "c",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "a = false\nb = false\n",
            old_name: "a",
            new_name: "b",
            want: Tokens(vec![
                tok(TokenType::Ident, b"a", 0),
                tok(TokenType::Equal, b"=", 1),
                tok(TokenType::Ident, b"false", 1),
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
        let (f, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics: {diags:?}"
        );

        let old_exists = f.body().get_attribute(test.old_name).is_some();
        let new_exists = f.body().get_attribute(test.new_name).is_some();
        let should_succeed = old_exists && !new_exists;
        let success = f.body().rename_attribute(test.old_name, test.new_name);

        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got, test.want,
            "case {i} ({}->{} in {:?}): wrong result",
            test.old_name, test.new_name, test.src,
        );
        assert_eq!(
            success, should_succeed,
            "case {i}: RenameAttribute returned {success} when it should have returned {should_succeed}",
        );
    }
}

// Ported from TestBodyAppendBlock:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1418
#[test]
#[ignore = "not yet implemented"]
fn body_append_block() {
    struct Case {
        src: &'static str,
        block_type: &'static str,
        labels: &'static [&'static str],
        want: Tokens,
    }

    let tests = [
        Case {
            src: "",
            block_type: "foo",
            labels: &[],
            want: Tokens(vec![
                tok(TokenType::Ident, b"foo", 0),
                tok(TokenType::OBrace, b"{", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::CBrace, b"}", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "",
            block_type: "foo",
            labels: &["bar"],
            want: Tokens(vec![
                tok(TokenType::Ident, b"foo", 0),
                tok(TokenType::OQuote, b"\"", 1),
                tok(TokenType::QuotedLit, b"bar", 0),
                tok(TokenType::CQuote, b"\"", 0),
                tok(TokenType::OBrace, b"{", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::CBrace, b"}", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "",
            block_type: "foo",
            labels: &["bar", "baz"],
            want: Tokens(vec![
                tok(TokenType::Ident, b"foo", 0),
                tok(TokenType::OQuote, b"\"", 1),
                tok(TokenType::QuotedLit, b"bar", 0),
                tok(TokenType::CQuote, b"\"", 0),
                tok(TokenType::OQuote, b"\"", 1),
                tok(TokenType::QuotedLit, b"baz", 0),
                tok(TokenType::CQuote, b"\"", 0),
                tok(TokenType::OBrace, b"{", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::CBrace, b"}", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
        Case {
            src: "bar {}\n",
            block_type: "foo",
            labels: &[],
            want: Tokens(vec![
                tok(TokenType::Ident, b"bar", 0),
                tok(TokenType::OBrace, b"{", 1),
                tok(TokenType::CBrace, b"}", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::Ident, b"foo", 0),
                tok(TokenType::OBrace, b"{", 1),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::CBrace, b"}", 0),
                tok(TokenType::Newline, b"\n", 0),
                tok(TokenType::EOF, b"", 0),
            ]),
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (f, diags) = hclwrite::parse_config(
            test.src.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert!(
            diags.is_empty(),
            "case {i}: unexpected diagnostics: {diags:?}"
        );

        f.body().append_new_block(test.block_type, test.labels);
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(
            got, test.want,
            "case {i} ({} {:?} in {:?}): wrong result",
            test.block_type, test.labels, test.src,
        );
    }
}

// Ported from TestBodyRemoveBlock:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1661
#[test]
#[ignore = "not yet implemented"]
fn body_remove_block() {
    // Go: strings.TrimSpace of a raw literal — no trailing newline.
    let src = "a = 1\n\n# Foo\nfoo {\n  b = 1\n}\nfoo {\n  b = 2\n}\nbar {}";
    let (f, diags) = hclwrite::parse_config(
        src.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");

    // Removing the first block.
    let body = f.body();
    let block = body
        .first_matching_block("foo", &[])
        .expect("didn't find a 'foo' block");
    let removed = body.remove_block(&block);
    assert!(removed, "didn't remove first block");
    let mut got = f.build_tokens();
    let want = Tokens(vec![
        tok(TokenType::Ident, b"a", 0),
        tok(TokenType::Equal, b"=", 1),
        tok(TokenType::NumberLit, b"1", 1),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Ident, b"foo", 0),
        tok(TokenType::OBrace, b"{", 1),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Ident, b"b", 2),
        tok(TokenType::Equal, b"=", 1),
        tok(TokenType::NumberLit, b"2", 1),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::CBrace, b"}", 0),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Ident, b"bar", 0),
        tok(TokenType::OBrace, b"{", 1),
        tok(TokenType::CBrace, b"}", 0),
        tok(TokenType::EOF, b"", 0),
    ]);
    hclwrite::format_tokens(&mut got);
    assert_eq!(got, want, "wrong result after removing the first block");

    // Removing the second block.
    let block = body
        .first_matching_block("foo", &[])
        .expect("didn't find a 'foo' block");
    let removed = body.remove_block(&block);
    assert!(removed, "didn't remove second block");
    let mut got = f.build_tokens();
    let want = Tokens(vec![
        tok(TokenType::Ident, b"a", 0),
        tok(TokenType::Equal, b"=", 1),
        tok(TokenType::NumberLit, b"1", 1),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Newline, b"\n", 0),
        tok(TokenType::Ident, b"bar", 0),
        tok(TokenType::OBrace, b"{", 1),
        tok(TokenType::CBrace, b"}", 0),
        tok(TokenType::EOF, b"", 0),
    ]);
    hclwrite::format_tokens(&mut got);
    assert_eq!(got, want, "wrong result after removing the second block");
}

// NOTE(port): the upstream file ends with the generic test helper
// `testFn[E any]` (line 1865), which wraps a (value, diags) pair and fails
// on diagnostics. It is only used by tests owned by the sibling porting
// target; no transcription is needed here.
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_body_test.go#L1865
