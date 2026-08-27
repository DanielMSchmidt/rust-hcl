//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/format_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use hcl::hclsyntax::TokenType;
use hcl::hclwrite::{self, FormatLine, Token, Tokens};

// Ported from TestFormat:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/format_test.go#L16
//
// Upstream runs the unexported pipeline `lexConfig` → `format` →
// `Tokens.Bytes`; `hclwrite::format` is that same pipeline (hclwrite:
// `hclwrite.Format`), so the byte-for-byte comparison is unchanged.
#[test]
#[ignore = "not yet implemented"]
fn format() {
    let tests: &[(&str, &str)] = &[
        ("", ""),
        ("a=1", "a = 1"),
        ("a=b.c", "a = b.c"),
        ("a=b[c]", "a = b[c]"),
        ("a=b()[c]", "a = b()[c]"),
        (r#"a=["hello"][0]"#, r#"a = ["hello"][0]"#),
        ("( a+2 )", "(a + 2)"),
        ("( a*2 )", "(a * 2)"),
        ("( a+-2 )", "(a + -2)"),
        ("( a*-2 )", "(a * -2)"),
        ("(-2+1)", "(-2 + 1)"),
        ("foo(1, -2,a*b, b,c)", "foo(1, -2, a * b, b, c)"),
        ("foo(a,b...)", "foo(a, b...)"),
        ("! true", "!true"),
        (r#"a="hello ${ name }""#, r#"a = "hello ${name}""#),
        (r#"a="hello ${~ name ~}""#, r#"a = "hello ${~name~}""#),
        (r#"a="${b}${c}${ d } ${e}""#, r#"a = "${b}${c}${d} ${e}""#),
        (
            r#""%{if true}${var.foo}%{endif}""#,
            r#""%{if true}${var.foo}%{endif}""#,
        ),
        ("b{}", "b {}"),
        (
            r#"
"${
hello
}"
"#,
            r#"
"${
  hello
}"
"#,
        ),
        (
            "
foo(
1,
- 2,
a*b,
b,
c,
)
",
            "
foo(
  1,
  -2,
  a * b,
  b,
  c,
)
",
        ),
        ("a?b:c", "a ? b : c"),
        ("[ [ ] ]", "[[]]"),
        ("[for x in y : x]", "[for x in y : x]"),
        ("[for x in [y] : x]", "[for x in [y] : x]"),
        (
            "
[
[
a
]
]
",
            "
[
  [
    a
  ]
]
",
        ),
        (
            "
[[
a
]]
",
            "
[[
  a
]]
",
        ),
        (
            "
[[
[
a
]
]]
",
            "
[[
  [
    a
  ]
]]
",
        ),
        // degenerate case with asymmetrical brackets
        (
            "
[[
[
a
]]
]
",
            "
[[
  [
    a
  ]]
]
",
        ),
        (
            "
b {
a = 1
}
",
            "
b {
  a = 1
}
",
        ),
        (
            "
b {a = 1}
",
            "
b { a = 1 }
",
        ),
        (
            "
a = 1
bungle = 2
",
            "
a      = 1
bungle = 2
",
        ),
        (
            "
a = 1

bungle = 2
",
            "
a = 1

bungle = 2
",
        ),
        (
            "
a = 1 # foo
bungle = 2
",
            "
a      = 1 # foo
bungle = 2
",
        ),
        (
            r#"
a = 1 # foo
bungle = "bonce" # baz
"#,
            r#"
a      = 1       # foo
bungle = "bonce" # baz
"#,
        ),
        (
            r#"
# here we go
a = 1 # foo
bungle = "bonce" # baz
"#,
            r#"
# here we go
a      = 1       # foo
bungle = "bonce" # baz
"#,
        ),
        (
            r#"
foo {} # here we go
a = 1 # foo
bungle = "bonce" # baz
"#,
            r#"
foo {}           # here we go
a      = 1       # foo
bungle = "bonce" # baz
"#,
        ),
        (
            r#"
a = 1 # foo
bungle = "bonce" # baz
zebra = "striped" # baz
"#,
            r#"
a      = 1         # foo
bungle = "bonce"   # baz
zebra  = "striped" # baz
"#,
        ),
        (
            r#"
a = 1 # foo
bungle = (
    "bonce"
) # baz
zebra = "striped" # baz
"#,
            r#"
a = 1 # foo
bungle = (
  "bonce"
)                 # baz
zebra = "striped" # baz
"#,
        ),
        (
            r##"
a="apple"# foo
bungle=(# woo parens
"bonce"
)# baz
zebra="striped"# baz
"##,
            r#"
a = "apple" # foo
bungle = (  # woo parens
  "bonce"
)                 # baz
zebra = "striped" # baz
"#,
        ),
        (
            r#"
𝒜 = 1 # foo
bungle = "🇬🇧" # baz
zebra = "striped" # baz
"#,
            r#"
𝒜      = 1         # foo
bungle = "🇬🇧"       # baz
zebra  = "striped" # baz
"#,
        ),
        (
            "
foo {
# ...
}
",
            "
foo {
  # ...
}
",
        ),
        (
            "
foo = {
# ...
}
",
            "
foo = {
  # ...
}
",
        ),
        (
            "
foo = [
# ...
]
",
            "
foo = [
  # ...
]
",
        ),
        (
            "
foo = [{
# ...
}]
",
            "
foo = [{
  # ...
}]
",
        ),
        (
            "
foo {
bar {
# ...
}
}
",
            "
foo {
  bar {
    # ...
  }
}
",
        ),
        (
            "
foo {
bar = {
# ...
}
}
",
            "
foo {
  bar = {
    # ...
  }
}
",
        ),
        (
            "
foo {
bar = [
# ...
]
}
",
            "
foo {
  bar = [
    # ...
  ]
}
",
        ),
        (
            "
foo {
bar = <<EOT
Foo bar baz
EOT
}
",
            "
foo {
  bar = <<EOT
Foo bar baz
EOT
}
",
        ),
        (
            "
foo {
bar = <<-EOT
Foo bar baz
EOT
}
",
            "
foo {
  bar = <<-EOT
Foo bar baz
EOT
}
",
        ),
        (
            "
foo {
bar = <<-EOT
  Foo bar baz
EOT
}
",
            "
foo {
  bar = <<-EOT
  Foo bar baz
EOT
}
",
        ),
        (
            "
foo {
bar = <<-EOT
  blahblahblah = x
EOT
}
",
            "
foo {
  bar = <<-EOT
  blahblahblah = x
EOT
}
",
        ),
        (
            "
foo {
bar = <<-EOT
  ${{ blahblahblah = x }}
EOT
}
",
            "
foo {
  bar = <<-EOT
  ${ { blahblahblah = x } }
EOT
}
",
        ),
        (
            "
foo {
  bar = <<-EOT
  ${a}${b}${ c } ${d}
EOT
}
",
            "
foo {
  bar = <<-EOT
  ${a}${b}${c} ${d}
EOT
}
",
        ),
        (
            r#"
foo {
bar = <<EOT
Foo bar baz
EOT
}

baz {
default="string"
}
"#,
            r#"
foo {
  bar = <<EOT
Foo bar baz
EOT
}

baz {
  default = "string"
}
"#,
        ),
        (
            r#"
foo {
bar = <<EOT
Foo bar baz
EOT
baz = <<EOT
Foo bar baz
EOT
}

bar {
foo = "bar"
}
"#,
            r#"
foo {
  bar = <<EOT
Foo bar baz
EOT
  baz = <<EOT
Foo bar baz
EOT
}

bar {
  foo = "bar"
}
"#,
        ),
        (
            r#"
module "foo" {
foo = <<EOF
5
EOF
}

module "x" {
a = "b"
abcde = "456"
}"#,
            r#"
module "foo" {
  foo = <<EOF
5
EOF
}

module "x" {
  a     = "b"
  abcde = "456"
}"#,
        ),
        (
            "attr = provider::framework::example()",
            "attr = provider::framework::example()",
        ),
        (
            "attr = provider :: framework :: example()",
            "attr = provider::framework::example()",
        ),
        (
            "attr = provider ::framework:: example()",
            "attr = provider::framework::example()",
        ),
        // This is invalid syntax so formatting it with spaces
        // does not have any meaning other than to make the fact more visible
        (
            "attr = provider::+example()",
            "attr = provider:: + example()",
        ),
    ];

    for (i, (input, want)) in tests.iter().enumerate() {
        let got = hclwrite::format(input.as_bytes());
        let got = String::from_utf8(got).expect("formatted output is not valid UTF-8");
        assert_eq!(
            got, *want,
            "case {i}: wrong result\ninput:\n{input}\ngot:\n{got}\nwant:\n{want}",
        );
    }
}

/// A token with the given type and no bytes (Go: `&Token{Type: ...}` with
/// its other fields left as zero values).
fn tok(ty: TokenType) -> Token {
    Token {
        ty,
        ..Token::default()
    }
}

/// A token with the given type and bytes
/// (Go: `&Token{Type: ..., Bytes: []byte(...)}`).
fn tok_bytes(ty: TokenType, bytes: &[u8]) -> Token {
    Token {
        ty,
        bytes: bytes.to_vec(),
        spaces_before: 0,
    }
}

// Ported from TestLinesForFormat:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/format_test.go#L653
//
// NOTE(port): Go's `reflect.DeepEqual` distinguishes the nil cells that
// upstream leaves unset from the explicitly-empty `lead: Tokens{}`; the
// Rust `FormatLine` has only empty `Tokens` for both, so the cells that
// upstream leaves nil are spelled via `FormatLine::default()` here.
#[test]
#[ignore = "not yet implemented"]
fn lines_for_format() {
    struct Case {
        tokens: Tokens,
        want: Vec<FormatLine>,
    }

    let tests = [
        Case {
            tokens: Tokens(vec![tok(TokenType::EOF)]),
            want: vec![FormatLine {
                lead: Tokens(vec![]),
                ..FormatLine::default()
            }],
        },
        Case {
            tokens: Tokens(vec![tok(TokenType::Ident), tok(TokenType::EOF)]),
            want: vec![FormatLine {
                lead: Tokens(vec![tok(TokenType::Ident)]),
                ..FormatLine::default()
            }],
        },
        Case {
            tokens: Tokens(vec![
                tok(TokenType::Ident),
                tok(TokenType::Newline),
                tok(TokenType::NumberLit),
                tok(TokenType::EOF),
            ]),
            want: vec![
                FormatLine {
                    lead: Tokens(vec![tok(TokenType::Ident), tok(TokenType::Newline)]),
                    ..FormatLine::default()
                },
                FormatLine {
                    lead: Tokens(vec![tok(TokenType::NumberLit)]),
                    ..FormatLine::default()
                },
            ],
        },
        Case {
            tokens: Tokens(vec![
                tok(TokenType::Ident),
                tok_bytes(TokenType::Comment, b"#foo\n"),
                tok(TokenType::NumberLit),
                tok(TokenType::EOF),
            ]),
            want: vec![
                FormatLine {
                    lead: Tokens(vec![tok(TokenType::Ident)]),
                    comment: Tokens(vec![tok_bytes(TokenType::Comment, b"#foo\n")]),
                    ..FormatLine::default()
                },
                FormatLine {
                    lead: Tokens(vec![tok(TokenType::NumberLit)]),
                    ..FormatLine::default()
                },
            ],
        },
        Case {
            tokens: Tokens(vec![
                tok(TokenType::Ident),
                tok(TokenType::Equal),
                tok(TokenType::NumberLit),
                tok(TokenType::EOF),
            ]),
            want: vec![FormatLine {
                lead: Tokens(vec![tok(TokenType::Ident)]),
                assign: Tokens(vec![tok(TokenType::Equal), tok(TokenType::NumberLit)]),
                ..FormatLine::default()
            }],
        },
        Case {
            tokens: Tokens(vec![
                tok(TokenType::Ident),
                tok(TokenType::Equal),
                tok(TokenType::NumberLit),
                tok_bytes(TokenType::Comment, b"#foo\n"),
                tok(TokenType::EOF),
            ]),
            want: vec![
                FormatLine {
                    lead: Tokens(vec![tok(TokenType::Ident)]),
                    assign: Tokens(vec![tok(TokenType::Equal), tok(TokenType::NumberLit)]),
                    comment: Tokens(vec![tok_bytes(TokenType::Comment, b"#foo\n")]),
                },
                FormatLine {
                    lead: Tokens(vec![]),
                    ..FormatLine::default()
                },
            ],
        },
        Case {
            tokens: Tokens(vec![
                // A comment goes into a comment cell only if it is after
                // some non-comment tokens, since whole-line comments must
                // stay flush with the indent level.
                tok_bytes(TokenType::Comment, b"#foo\n"),
                tok(TokenType::EOF),
            ]),
            want: vec![
                FormatLine {
                    lead: Tokens(vec![tok_bytes(TokenType::Comment, b"#foo\n")]),
                    ..FormatLine::default()
                },
                FormatLine {
                    lead: Tokens(vec![]),
                    ..FormatLine::default()
                },
            ],
        },
    ];

    for (i, test) in tests.into_iter().enumerate() {
        let got = hclwrite::lines_for_format(test.tokens);

        assert_eq!(got, test.want, "case {i}: wrong result");
    }
}
