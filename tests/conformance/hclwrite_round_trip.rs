//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   hclwrite/round_trip_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test file above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;

use cty::Value;
use cty::function::stdlib;
use hcl::hclwrite;
use hcl::{EvalContext, Pos};

// Ported from TestRoundTripVerbatim:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/round_trip_test.go#L19
//
// NOTE(port): upstream calls the unexported `parse(src, "", hcl.Pos{Line: 1,
// Column: 1})`; `hclwrite::parse_config` is the exported wrapper around that
// same parse and is used here. The Go `hcl.Pos` composite literal leaves
// `Byte` as its zero value, spelled `byte: 0` below.
#[test]
#[ignore = "not yet implemented"]
fn round_trip_verbatim() {
    let tests: &[&str] = &[
        "",
        "foo = 1\n",
        "
foobar = 1
baz    = 1
",
        r#"
# this file is awesome

# tossed salads and scrambled eggs
foobar = 1
baz    = 1

block {
  a = "a"
  b = "b"
  c = "c"
  d = "d"

  subblock {
  }

  subblock {
    e = "e"
  }
}

# and they all lived happily ever after
"#,
    ];

    for (i, test) in tests.iter().enumerate() {
        let src = test.as_bytes();
        let (file, diags) = hclwrite::parse_config(
            src,
            "",
            Pos {
                line: 1,
                column: 1,
                byte: 0,
            },
        );
        assert_eq!(
            diags.len(),
            0,
            "case {i}: unexpected diagnostics\ninput:\n{test}\n{diags}",
        );

        let mut wr: Vec<u8> = Vec::new();
        let n = file
            .write_to(&mut wr)
            .unwrap_or_else(|e| panic!("case {i}: error from write_to: {e}"));
        assert_eq!(
            n,
            test.len() as u64,
            "case {i}: wrong number of bytes {n}; want {}",
            test.len(),
        );

        let result = String::from_utf8(wr).expect("case {i}: output is not valid UTF-8");
        assert_eq!(
            result, *test,
            "case {i}: wrong result\ninput:\n{test}\ngot:\n{result}",
        );
    }
}

// Ported from TestRoundTripFormat:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/round_trip_test.go#L81
//
// The goal of this test is to verify that the formatter doesn't change
// the semantics of any expressions when it adds and removes whitespace.
// String templates are the primary area of concern here, but we also
// test some other things for completeness sake.
//
// The tests here must define zero or more attributes, which will be
// extracted with `just_attributes` (hcl: `JustAttributes`) and evaluated
// both before and after formatting.
#[test]
#[ignore = "not yet implemented"]
fn round_trip_format() {
    let tests: &[&str] = &[
        "",
        "\n\n\n",
        "a=1\n",
        "a=\"hello\"\n",
        "a=\"${hello} world\"\n",
        "a=upper(\"hello\")\n",
        "a=upper(hello)\n",
        "a=[1,2,3,4,five]\n",
        "a={greeting=hello}\n",
        "a={\ngreeting=hello\n}\n",
        "a={\ngreeting=hello}\n",
        "a={greeting=hello\n}\n",
        "a={greeting=hello,number=five,sarcastic=\"${upper(hello)}\"\n}\n",
        "a={\ngreeting=hello\nnumber=five\nsarcastic=\"${upper(hello)}\"\n}\n",
        "a=<<EOT\nhello\nEOT\n\n",
        "a=[<<EOT\nhello\nEOT\n]\n",
        "a=[\n<<EOT\nhello\nEOT\n]\n",
        "a=[\n]\n",
        "a=1\nb=2\nc=3\n",
        "a=\"${\n5\n}\"\n",
    ];

    let mut ctx = EvalContext::new();
    ctx.variables = HashMap::from([
        ("hello".to_string(), Value::string("hello")),
        ("five".to_string(), Value::number_int(5)),
    ]);
    ctx.functions = HashMap::from([("upper".to_string(), stdlib::upper_func())]);
    let ctx = &ctx;

    for (i, test) in tests.iter().enumerate() {
        let attrs_as_obj = |src: &[u8], phase: &str| -> Value {
            let (f, diags) = hcl::hclsyntax::parse_config(
                src,
                "",
                Pos {
                    line: 1,
                    column: 1,
                    byte: 0,
                },
            );
            assert_eq!(
                diags.len(),
                0,
                "case {i}: unexpected diagnostics in parse {phase}\ninput:\n{test}\n{diags}",
            );

            let (attrs, diags) = f.body.just_attributes();
            assert_eq!(
                diags.len(),
                0,
                "case {i}: unexpected diagnostics in just_attributes {phase}\ninput:\n{test}\n{diags}",
            );

            let mut vals: HashMap<String, Value> = HashMap::new();
            for (k, attr) in &attrs {
                let (val, diags) = attr.expr.value(Some(ctx));
                assert_eq!(
                    diags.len(),
                    0,
                    "case {i}: unexpected diagnostics evaluating {phase}\ninput:\n{test}\n{diags}",
                );
                vals.insert(k.clone(), val);
            }
            Value::object(vals)
        };

        let src = test.as_bytes();
        let before = attrs_as_obj(src, "before");

        let formatted = hclwrite::format(src);
        let after = attrs_as_obj(&formatted, "after");

        assert!(
            after.raw_equals(&before),
            "case {i}: mismatching after format\ninput:\n{test}\nbefore: {}\nafter:  {}",
            before.go_string(),
            after.go_string(),
        );
    }
}

// Ported from TestRoundTripSafeConcurrent:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/round_trip_test.go#L173
//
// Upstream concurrently generates a new file, so that a data race between
// the two goroutines is reported when the test runs under Go's race
// detector.
//
// NOTE(port): each Go `go func() {...}()` becomes `std::thread::spawn`,
// preserving the shape of one independent parse/generate workload per
// thread. Upstream spawns its goroutines without any `sync.WaitGroup` (the
// race detector observes them regardless of test completion); the Rust
// analogue of a WaitGroup is collecting the `JoinHandle`s and joining each
// one, which we do so the threads are guaranteed to finish (and any panic
// inside a thread fails the test).
#[test]
#[ignore = "not yet implemented"]
fn round_trip_safe_concurrent() {
    let mut handles = Vec::new();
    for _i in 0..2 {
        handles.push(std::thread::spawn(|| {
            let f = hclwrite::File::new();
            let b = f.body();
            b.set_attribute_value("foo", Value::string("bar"));
        }));
    }
    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
}

// NOTE(port): hclwrite/examples_test.go contains only Go `Example` functions
// (Go documentation examples), not `func Test*` tests, so it is outside the
// conformance universe and is not ported:
//   - Example_generateFromScratch:
//     https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/examples_test.go#L14
//   - ExampleExpression_RenameVariablePrefix:
//     https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/hclwrite/examples_test.go#L77
