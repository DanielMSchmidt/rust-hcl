//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/ast_attribute_test.go
//!   hclwrite/ast_block_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::Pos;
use hcl::hclsyntax::TokenType;
use hcl::hclwrite::{self, Token, Tokens};

/// Parses the source as native syntax, panicking on any diagnostics
/// (Go: the `mustParseConfig` test helper in `ast_block_test.go`).
fn must_parse_config(src: &str) -> hclwrite::File {
    let (f, diags) = hclwrite::parse_config(
        src.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    if !diags.is_empty() {
        panic!("{diags}");
    }
    f
}

// Ported from TestAttributeLeadComments:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_attribute_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn attribute_lead_comments() {
    struct Case {
        name: &'static str,
        src: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            name: "basic comment",
            src: "\n# comment\ntest_attribute = foo\n",
            want: "# comment\n",
        },
        Case {
            name: "basic multiline comment",
            src: "\n# multi-line\n# comment (singe comment formatting)\ntest_attribute = foo\n",
            want: "# multi-line\n# comment (singe comment formatting)\n",
        },
        Case {
            name: "go formatted comment",
            src: "\n// comment\ntest_attribute = foo\n",
            want: "// comment\n",
        },
        Case {
            name: "go formatted multi-line comment",
            src: "\n/* \n\tgo-style multi-line \n\tcomment \n*/\ntest_attribute = foo\n",
            want: "", // unsupported
        },
    ];

    for test in &tests {
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
            "case {:?}: unexpected diagnostics: {diags}",
            test.name,
        );
        let attr = &f.body().attributes()["test_attribute"];
        let got = String::from_utf8(attr.lead_comments().bytes()).unwrap();
        assert_eq!(
            got, test.want,
            "case {:?}: wrong result\ngot:  {got}\nwant: {}",
            test.name, test.want,
        );
    }
}

// Ported from TestAttributeLineComments:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_attribute_test.go#L70
#[test]
#[ignore = "not yet implemented"]
fn attribute_line_comments() {
    struct Case {
        src: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            src: "\ntest_attribute = foo # comment\n",
            want: " # comment\n",
        },
        Case {
            src: "\ntest_attribute = foo // comment\n",
            want: " // comment\n",
        },
        Case {
            src: "\ntest_attribute = foo # multi-line\n\t\t\t\t\t # comment (invalid)\n",
            // known limitation: any extra comment lines (when using
            // single-comment syntax) are dropped
            want: " # multi-line\n",
        },
        Case {
            src: "\ntest_attribute = foo /* multi-line\n                        comment in a weird place\n                     */\n",
            // note that all the whitespaces are returned
            want: " /* multi-line\n                        comment in a weird place\n                     */",
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
            "case {i}: unexpected diagnostics: {diags}"
        );
        let attr = &f.body().attributes()["test_attribute"];
        let got = String::from_utf8(attr.line_comments().bytes()).unwrap();
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ngot:  {got}\nwant: {}",
            test.want,
        );
    }
}

// Ported from TestBlockType:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L18
#[test]
#[ignore = "not yet implemented"]
fn block_type() {
    struct Case {
        src: &'static str,
        want: &'static str,
    }

    let tests = [Case {
        src: "\nservice {\n  attr0 = \"val0\"\n}\n",
        want: "service",
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
            "case {i}: unexpected diagnostics: {diags}"
        );

        let block = &f.body().blocks()[0];
        let got = block.block_type();
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ngot:  {got}\nwant: {}",
            test.want,
        );
    }
}

// Ported from TestBlockLabels:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L52
#[test]
#[ignore = "not yet implemented"]
fn block_labels() {
    struct Case {
        src: &'static str,
        want: Vec<&'static str>,
    }

    let tests = [
        Case {
            src: "\nnolabel {\n}\n",
            want: vec![],
        },
        Case {
            src: "\nquoted \"label1\" {\n}\n",
            want: vec!["label1"],
        },
        Case {
            src: "\nquoted \"label1\" \"label2\" {\n}\n",
            want: vec!["label1", "label2"],
        },
        Case {
            src: "\nquoted \"label1\" /* foo */ \"label2\" {\n}\n",
            want: vec!["label1", "label2"],
        },
        Case {
            src: "\nunquoted label1 {\n}\n",
            want: vec!["label1"],
        },
        Case {
            src: "\nunquoted label1 /* foo */ label2 {\n}\n",
            want: vec!["label1", "label2"],
        },
        Case {
            src: "\nmixed label1 \"label2\" {\n}\n",
            want: vec!["label1", "label2"],
        },
        Case {
            src: "\nescape \"\\u0041\" {\n}\n",
            want: vec!["\u{0041}"],
        },
        Case {
            src: "\nblank \"\" {\n}\n",
            want: vec![""],
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
            "case {i}: unexpected diagnostics: {diags}"
        );

        let block = &f.body().blocks()[0];
        let got = block.labels();
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ngot:  {got:#?}\nwant: {:#?}",
            test.want,
        );
    }
}

// Ported from TestBlockSetType_buildTokens:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L141
#[test]
#[ignore = "not yet implemented"]
fn block_set_type_build_tokens() {
    struct Case {
        src: &'static str,
        old_type_name: &'static str,
        new_type_name: &'static str,
        labels: Vec<&'static str>,
        want: Tokens,
    }

    let tests = [Case {
        src: "foo {}",
        old_type_name: "foo",
        new_type_name: "bar",
        labels: vec![],
        want: Tokens(vec![
            Token {
                ty: TokenType::Ident,
                bytes: b"bar".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::OBrace,
                bytes: b"{".to_vec(),
                spaces_before: 1,
            },
            Token {
                ty: TokenType::CBrace,
                bytes: b"}".to_vec(),
                spaces_before: 0,
            },
            Token {
                ty: TokenType::EOF,
                bytes: b"".to_vec(),
                spaces_before: 0,
            },
        ]),
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
            "case {i} ({}): unexpected diagnostics: {diags}",
            test.src,
        );

        let b = f
            .body()
            .first_matching_block(test.old_type_name, &test.labels)
            .unwrap();
        b.set_type(test.new_type_name);
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.src,);
    }
}

// Ported from TestBlockSetType:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L201
#[test]
#[ignore = "not yet implemented"]
fn block_set_type() {
    struct Case {
        old_type_name: &'static str,
        new_type_name: &'static str,
    }

    let tests = [Case {
        old_type_name: "foo",
        new_type_name: "bar",
    }];

    for (i, test) in tests.iter().enumerate() {
        let b = hclwrite::Block::new(test.old_type_name, &[]);
        b.set_type(test.new_type_name);

        assert_eq!(
            b.block_type(),
            test.new_type_name,
            "case {i}: wrong result\ngot: {}\nwant: {}",
            b.block_type(),
            test.new_type_name,
        );
    }
}

// Ported from TestBlockSetLabels:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L222
#[test]
#[ignore = "not yet implemented"]
fn block_set_labels() {
    struct Case {
        src: &'static str,
        type_name: &'static str,
        old_labels: Vec<&'static str>,
        new_labels: Vec<&'static str>,
        want: Tokens,
    }

    let tests = [
        Case {
            src: "foo \"hoge\" {}",
            type_name: "foo",
            old_labels: vec!["hoge"],
            new_labels: vec!["fuga"], // update first label
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"fuga".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            src: "foo \"hoge\" \"fuga\" {}",
            type_name: "foo",
            old_labels: vec!["hoge", "fuga"],
            new_labels: vec!["hoge", "piyo"], // update second label
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"hoge".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"piyo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            src: "foo {}",
            type_name: "foo",
            old_labels: vec![],
            new_labels: vec!["fuga"], // insert a new label to empty list
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"fuga".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            src: "foo \"hoge\" {}",
            type_name: "foo",
            old_labels: vec!["hoge"],
            new_labels: vec![], // remove all labels
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            src: "foo \"hoge\" /* fuga */ \"piyo\" {}",
            type_name: "foo",
            old_labels: vec!["hoge", "piyo"],
            new_labels: vec!["fuga"], // force quoted form even if the old one is unquoted.
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"fuga".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
            ]),
        },
        Case {
            src: "foo \"hoge\" /* foo */  \"\" {}",
            type_name: "foo",
            old_labels: vec!["hoge", ""],
            new_labels: vec!["fuga"], // force quoted form even if the old one is unquoted.
            want: Tokens(vec![
                Token {
                    ty: TokenType::Ident,
                    bytes: b"foo".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::QuotedLit,
                    bytes: b"fuga".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::CQuote,
                    bytes: b"\"".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::OBrace,
                    bytes: b"{".to_vec(),
                    spaces_before: 1,
                },
                Token {
                    ty: TokenType::CBrace,
                    bytes: b"}".to_vec(),
                    spaces_before: 0,
                },
                Token {
                    ty: TokenType::EOF,
                    bytes: b"".to_vec(),
                    spaces_before: 0,
                },
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
            "case {i} ({}): unexpected diagnostics: {diags}",
            test.src,
        );

        let b = f
            .body()
            .first_matching_block(test.type_name, &test.old_labels)
            .unwrap();
        b.set_labels(&test.new_labels);
        let mut got = f.build_tokens();
        hclwrite::format_tokens(&mut got);
        assert_eq!(got, test.want, "case {i} ({}): wrong result", test.src,);
    }
}

// Ported from TestBlockLeadComments:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/ast_block_test.go#L512
#[test]
#[ignore = "not yet implemented"]
fn block_lead_comments() {
    struct Case {
        src: &'static str,
        want_comment: &'static str,
    }

    let tests = [
        Case {
            src: "# block comment\nblock \"test\" {}\n",
            want_comment: "# block comment\n",
        },
        Case {
            src: "// block comment\nblock \"test\" {}\n",
            want_comment: "// block comment\n",
        },
        Case {
            src: "// block comment\n// that goes on a bit\n// for fun\nblock \"test\" {}\n",
            want_comment: "// block comment\n// that goes on a bit\n// for fun\n",
        },
        // Terraform accepts multi-line go-style comments, but hclwrite does
        // not consistently support this style comment.
        Case {
            src: "/* multiline\ncomment\n*/\nblock \"test\" {}\n",
            want_comment: "", // unsupported
        },
        Case {
            src: "/* multiline\ncomment\n*/\nblock \"test\" {}\n",
            want_comment: "", // unsupported
        },
        Case {
            src: "/* multiline comment, single line */\nblock \"test\" {}\n",
            want_comment: "", // unsupported
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let f = must_parse_config(test.src);
        let b = f.body().first_matching_block("block", &["test"]).unwrap();
        let got = String::from_utf8(b.lead_comments().bytes()).unwrap();
        assert_eq!(
            got, test.want_comment,
            "case {i}: wrong result\ngot:  {got}\nwant: {}",
            test.want_comment,
        );
    }
}
