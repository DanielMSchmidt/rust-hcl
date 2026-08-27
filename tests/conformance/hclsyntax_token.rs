//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclsyntax/token_test.go
//!   hclsyntax/peeker_test.go
//!   hclsyntax/public_test.go
//!   hclsyntax/didyoumean_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::{self, Peeker, Token, TokenType, Tokens};
use hcl::{DiagnosticSeverity, Pos};

// Ported from TestCheckInvalidTokensTest:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/token_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn check_invalid_tokens_test() {
    struct Case {
        input: &'static str,
        want_summary: &'static str,
        want_detail: &'static str,
    }

    let tests = [
        Case {
            input: "block \u{201c}invalid\u{201d} {}",
            want_summary: "Invalid character",
            want_detail: "\"Curly quotes\" are not valid here. These can sometimes be inadvertently introduced when sharing code via documents or discussion forums. It might help to replace the character with a \"straight quote\".",
        },
        Case {
            input: "block 'invalid' {}",
            want_summary: "Invalid character",
            want_detail: "Single quotes are not valid. Use double quotes (\") to enclose strings.",
        },
        Case {
            input: "block `invalid` {}",
            want_summary: "Invalid character",
            want_detail: "The \"`\" character is not valid. To create a multi-line string, use the \"heredoc\" syntax, like \"<<EOT\".",
        },
        Case {
            input: "foo = a & b",
            want_summary: "Unsupported operator",
            want_detail: "Bitwise operators are not supported. Did you mean boolean AND (\"&&\")?",
        },
        Case {
            input: "foo = a | b",
            want_summary: "Unsupported operator",
            want_detail: "Bitwise operators are not supported. Did you mean boolean OR (\"||\")?",
        },
        Case {
            input: "foo = ~a",
            want_summary: "Unsupported operator",
            want_detail: "Bitwise operators are not supported. Did you mean boolean NOT (\"!\")?",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let (_, diags) = hclsyntax::lex_config(
            test.input.as_bytes(),
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        let found = diags.iter().any(|diag| {
            diag.severity == DiagnosticSeverity::Error
                && diag.summary == test.want_summary
                && diag.detail == test.want_detail
        });
        // If we fall out here then we didn't find the diagnostic we were
        // looking for.
        assert!(
            found,
            "case {i} ({}): wrong errors\ngot:  {}\nwant: {}; {}",
            test.input, diags, test.want_summary, test.want_detail,
        );
    }
}

// NOTE(port): peeker_test.go's init() sets the package-level trace variable
// `tracePeekerNewlinesStack = true` so peeker misuse panics with a trace
// during tests; that Go-only test instrumentation has no Rust analogue here.
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/peeker_test.go#L11

// Ported from TestPeeker:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/peeker_test.go#L16
#[test]
#[ignore = "not yet implemented"]
fn peeker() {
    let tokens: Tokens = vec![
        Token {
            ty: TokenType::Ident,
            ..Default::default()
        },
        Token {
            ty: TokenType::Comment,
            ..Default::default()
        },
        Token {
            ty: TokenType::Ident,
            ..Default::default()
        },
        Token {
            ty: TokenType::Comment,
            ..Default::default()
        },
        Token {
            ty: TokenType::Ident,
            ..Default::default()
        },
        Token {
            ty: TokenType::Newline,
            ..Default::default()
        },
        Token {
            ty: TokenType::Ident,
            ..Default::default()
        },
        Token {
            ty: TokenType::Newline,
            ..Default::default()
        },
        Token {
            ty: TokenType::Ident,
            ..Default::default()
        },
        Token {
            ty: TokenType::Newline,
            ..Default::default()
        },
        Token {
            ty: TokenType::EOF,
            ..Default::default()
        },
    ];

    {
        let mut peeker = Peeker::new(tokens.clone(), true);

        let want_types = vec![
            TokenType::Ident,
            TokenType::Comment,
            TokenType::Ident,
            TokenType::Comment,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::EOF,
        ];
        let mut got_types: Vec<TokenType> = Vec::new();

        loop {
            let peeked = peeker.peek();
            let read = peeker.read();
            assert_eq!(
                peeked.ty, read.ty,
                "mismatched Peek {peeked:?} and Read {read:?}",
            );

            got_types.push(read.ty);

            if read.ty == TokenType::EOF {
                break;
            }
        }

        assert_eq!(
            got_types, want_types,
            "include comments: wrong types\ngot:  {got_types:?}\nwant: {want_types:?}",
        );
    }

    {
        let mut peeker = Peeker::new(tokens.clone(), false);

        let want_types = vec![
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::Ident,
            TokenType::Newline,
            TokenType::EOF,
        ];
        let mut got_types: Vec<TokenType> = Vec::new();

        loop {
            let peeked = peeker.peek();
            let read = peeker.read();
            assert_eq!(
                peeked.ty, read.ty,
                "mismatched Peek {peeked:?} and Read {read:?}",
            );

            got_types.push(read.ty);

            if read.ty == TokenType::EOF {
                break;
            }
        }

        assert_eq!(
            got_types, want_types,
            "skip comments: wrong types\ngot:  {got_types:?}\nwant: {want_types:?}",
        );
    }

    {
        let mut peeker = Peeker::new(tokens.clone(), false);

        peeker.push_include_newlines(false);

        let want_types = vec![
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Ident,
            TokenType::Newline, // we'll pop off the PushIncludeNewlines before we get here
            TokenType::EOF,
        ];
        let mut got_types: Vec<TokenType> = Vec::new();

        let mut idx = 0;
        loop {
            let peeked = peeker.peek();
            let read = peeker.read();
            assert_eq!(
                peeked.ty, read.ty,
                "mismatched Peek {peeked:?} and Read {read:?}",
            );

            got_types.push(read.ty);

            if read.ty == TokenType::EOF {
                break;
            }

            if idx == 4 {
                peeker.pop_include_newlines();
            }

            idx += 1;
        }

        assert_eq!(
            got_types, want_types,
            "push/pop include newlines: wrong types\ngot:  {got_types:?}\nwant: {want_types:?}",
        );
    }
}

// Ported from TestValidIdentifier:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/public_test.go#L12
#[test]
#[ignore = "not yet implemented"]
fn valid_identifier() {
    struct Case {
        input: &'static str,
        want: bool,
    }

    let tests = [
        Case {
            input: "",
            want: false,
        },
        Case {
            input: "hello",
            want: true,
        },
        Case {
            input: "hello.world",
            want: false,
        },
        Case {
            input: "hello ",
            want: false,
        },
        Case {
            input: " hello",
            want: false,
        },
        Case {
            input: "hello\n",
            want: false,
        },
        Case {
            input: "hello world",
            want: false,
        },
        Case {
            input: "aws_instance",
            want: true,
        },
        Case {
            input: "aws.instance",
            want: false,
        },
        Case {
            input: "foo-bar",
            want: true,
        },
        Case {
            input: "foo--bar",
            want: true,
        },
        Case {
            input: "foo_",
            want: true,
        },
        Case {
            input: "foo-",
            want: true,
        },
        Case {
            input: "_foobar",
            want: true,
        },
        Case {
            input: "-foobar",
            want: false,
        },
        Case {
            input: "blah1",
            want: true,
        },
        Case {
            input: "blah1blah",
            want: true,
        },
        Case {
            input: "1blah1blah",
            want: false,
        },
        Case {
            input: "h\u{301}llo", // combining acute accent
            want: true,
        },
        Case {
            input: "Χαίρετε",
            want: true,
        },
        Case {
            input: "звать",
            want: true,
        },
        Case {
            input: "今日は",
            want: true,
        },
        // NOTE(port): Go's `{"\x80", false}` (UTF-8 continuation without an
        // introducer) and `{"a\x80", false}` (UTF-8 continuation after a
        // non-introducer) cannot be expressed: a Go string can hold
        // arbitrary bytes, but `valid_identifier` takes a Rust `&str`,
        // which is valid UTF-8 by construction.
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclsyntax::valid_identifier(test.input);
        assert_eq!(
            got, test.want,
            "case {i} ({:?}): wrong result {got:?}; want {:?}",
            test.input, test.want,
        );
    }
}

// NOTE(port): BenchmarkLexConfig is a Go benchmark, not a test; benchmarks
// are not ported.
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/public_test.go#L55

// Ported from TestNameSuggestion:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclsyntax/didyoumean_test.go#L8
#[test]
#[ignore = "not yet implemented"]
fn name_suggestion() {
    let keywords = ["false", "true", "null"];

    struct Case {
        input: &'static str,
        want: &'static str,
    }

    let tests = [
        Case {
            input: "true",
            want: "true",
        },
        Case {
            input: "false",
            want: "false",
        },
        Case {
            input: "null",
            want: "null",
        },
        Case {
            input: "bananas",
            want: "",
        },
        Case {
            input: "NaN",
            want: "",
        },
        Case {
            input: "Inf",
            want: "",
        },
        Case {
            input: "Infinity",
            want: "",
        },
        Case {
            input: "void",
            want: "",
        },
        Case {
            input: "undefined",
            want: "",
        },
        Case {
            input: "ture",
            want: "true",
        },
        Case {
            input: "tru",
            want: "true",
        },
        Case {
            input: "tre",
            want: "true",
        },
        Case {
            input: "treu",
            want: "true",
        },
        Case {
            input: "rtue",
            want: "true",
        },
        Case {
            input: "flase",
            want: "false",
        },
        Case {
            input: "fales",
            want: "false",
        },
        Case {
            input: "flse",
            want: "false",
        },
        Case {
            input: "fasle",
            want: "false",
        },
        Case {
            input: "fasel",
            want: "false",
        },
        Case {
            input: "flue",
            want: "false",
        },
        Case {
            input: "nil",
            want: "null",
        },
        Case {
            input: "nul",
            want: "null",
        },
        Case {
            input: "unll",
            want: "null",
        },
        Case {
            input: "nll",
            want: "null",
        },
    ];

    for (i, test) in tests.iter().enumerate() {
        let got = hclsyntax::name_suggestion(test.input, &keywords);
        assert_eq!(
            got, test.want,
            "case {i}: wrong result\ninput: {:?}\ngot:   {got:?}\nwant:  {:?}",
            test.input, test.want,
        );
    }
}
