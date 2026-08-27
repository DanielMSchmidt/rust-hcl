//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   gohcl/decode_test.go
//!   gohcl/schema_test.go
//!   gohcl/encode_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence (especially the
//! `#[hcl(...)]` attribute grammar mirroring Go's `hcl:"..."` struct tags).

use std::collections::HashMap;

use cty::{Type, Value};
use hcl::gohcl::{EncodeBody, FromBody};
use hcl::{
    AttributeSchema, Attributes, BlockHeaderSchema, BodyRef, BodySchema, Diagnostics, EvalContext,
    ExprRef, Expression, Pos, Range, Traversal, gohcl, hclwrite, json,
};

// ---------------------------------------------------------------------------
// Target struct shapes for TestDecodeBody. Go declares most of these as
// anonymous structs inline in the case table; each distinct shape becomes one
// named private struct here, with the Go literal in a comment.
// ---------------------------------------------------------------------------

// Go: struct{}{} (also the nested `struct{}` block shape).
#[derive(FromBody, Debug, PartialEq)]
struct Empty {}

// Go: struct { Name string `hcl:"name"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithRequiredName {
    #[hcl(attr = "name")]
    name: String,
}

// Go: struct { Name *string `hcl:"name"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNamePtr {
    #[hcl(attr = "name")]
    name: Option<String>,
}

// Go: struct { Name string `hcl:"name,optional"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithOptionalName {
    #[hcl(attr = "name", optional)]
    name: String,
}

// Go: type withNameExpression struct { Name hcl.Expression `hcl:"name"` }
#[derive(FromBody, Debug)]
struct WithNameExpression {
    #[hcl(attr = "name")]
    name: ExprRef,
}

// Go: struct { Name string `hcl:"name"`; Attrs hcl.Attributes `hcl:",remain"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNameAndRemainAttrs {
    #[hcl(attr = "name")]
    name: String,
    #[hcl(remain)]
    attrs: Attributes,
}

// Go: struct { Name string `hcl:"name"`; Remain hcl.Body `hcl:",remain"` }
#[derive(FromBody, Debug)]
struct WithNameAndRemainBody {
    #[hcl(attr = "name")]
    name: String,
    #[hcl(remain)]
    remain: BodyRef,
}

// Go: struct { Name string `hcl:"name"`; Remain map[string]cty.Value `hcl:",remain"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNameAndRemainMap {
    #[hcl(attr = "name")]
    name: String,
    #[hcl(remain)]
    remain: HashMap<String, Value>,
}

// Go: struct { Name string `hcl:"name"`; Body hcl.Body `hcl:",body"`; Remain hcl.Body `hcl:",remain"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNameBodyAndRemain {
    #[hcl(attr = "name")]
    name: String,
    #[hcl(body)]
    body: BodyRef,
    #[hcl(remain)]
    remain: BodyRef,
}

// Go: struct { Noodle struct{} `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNoodleBlock {
    #[hcl(block = "noodle")]
    noodle: Empty,
}

// Go: struct { Noodle *struct{} `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNoodlePtrBlock {
    #[hcl(block = "noodle")]
    noodle: Option<Empty>,
}

// Go: struct { Noodle []struct{} `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithNoodleSliceBlock {
    #[hcl(block = "noodle")]
    noodle: Vec<Empty>,
}

// Go: struct { Name string `hcl:"name,label"` } (the labeled `noodle` block shape)
#[derive(FromBody, Debug, PartialEq)]
struct NoodleWithNameLabel {
    #[hcl(label = "name")]
    name: String,
}

// Go: struct { Noodle struct { Name string `hcl:"name,label"` } `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithLabeledNoodle {
    #[hcl(block = "noodle")]
    noodle: NoodleWithNameLabel,
}

// Go: struct { Noodles []struct { Name string `hcl:"name,label"` } `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithLabeledNoodles {
    #[hcl(block = "noodle")]
    noodles: Vec<NoodleWithNameLabel>,
}

// Go: struct { Name string `hcl:"name,label"`; Type string `hcl:"type"` }
#[derive(FromBody, Debug, PartialEq)]
struct NoodleWithNameLabelAndType {
    #[hcl(label = "name")]
    name: String,
    #[hcl(attr = "type")]
    r#type: String,
}

// Go: struct { Noodle struct { Name string `hcl:"name,label"`; Type string `hcl:"type"` } `hcl:"noodle,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithTypedNoodle {
    #[hcl(block = "noodle")]
    noodle: NoodleWithNameLabelAndType,
}

// Go: the anonymous `Foo` block struct exercising the label/def/type/attr
// range tags:
//
//   struct {
//       Type           string    `hcl:"type,label"`
//       TypeLabelRange hcl.Range `hcl:"type,label_range"`
//       Name           string    `hcl:"name,label"`
//       NameLabelRange hcl.Range `hcl:"name,label_range"`
//       DefRange       hcl.Range `hcl:",def_range"`
//       TypeRange      hcl.Range `hcl:",type_range"`
//       Attribute           string    `hcl:"value,attr"`
//       AttributeRange      hcl.Range `hcl:"value,attr_range"`
//       AttributeNameRange  hcl.Range `hcl:"value,attr_name_range"`
//       AttributeValueRange hcl.Range `hcl:"value,attr_value_range"`
//   }
#[derive(FromBody, Debug, PartialEq)]
struct FooWithRanges {
    #[hcl(label = "type")]
    r#type: String,
    #[hcl(label_range = "type")]
    type_label_range: Range,
    #[hcl(label = "name")]
    name: String,
    #[hcl(label_range = "name")]
    name_label_range: Range,
    #[hcl(def_range)]
    def_range: Range,
    #[hcl(type_range)]
    type_range: Range,
    #[hcl(attr = "value")]
    attribute: String,
    #[hcl(attr_range = "value")]
    attribute_range: Range,
    #[hcl(attr_name_range = "value")]
    attribute_name_range: Range,
    #[hcl(attr_value_range = "value")]
    attribute_value_range: Range,
}

// Go: struct { Foo struct { ... } `hcl:"foo,block"` } (wrapping FooWithRanges)
#[derive(FromBody, Debug, PartialEq)]
struct WithFooBlock {
    #[hcl(block = "foo")]
    foo: FooWithRanges,
}

/// Parses one JSON body for a `TestDecodeBody` case, asserting no parse
/// diagnostics (Go: the `json.Marshal` + `hclJSON.Parse` preamble; the JSON
/// literals below have their keys pre-sorted, matching Go's `json.Marshal`
/// output for maps).
fn parse_json_body(src: &str) -> BodyRef {
    let (file, diags) = json::parse(src.as_bytes(), "test.json");
    assert_eq!(diags.len(), 0, "diagnostics while parsing {src}: {diags:?}");
    file.body
}

/// Go: the `makeRange` helper in decode_test.go.
fn make_range(filename: &str, line: usize, start: usize, end: usize) -> Range {
    Range {
        filename: filename.to_string(),
        start: Pos {
            line,
            column: start,
            byte: start - 1,
        },
        end: Pos {
            line,
            column: end,
            byte: end - 1,
        },
    }
}

// Ported from TestDecodeBody:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/gohcl/decode_test.go#L19
//
// The Go case table is heterogeneous in its target type, so the loop becomes
// a sequence of blocks, one per case, in upstream order.
#[test]
#[ignore = "not yet implemented"]
fn decode_body() {
    // case 0: {} into struct{}{}
    {
        let body = parse_json_body("{}");
        let (got, diags) = gohcl::decode_body::<Empty>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 0: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, Empty {}, "case 0: wrong result");
    }

    // case 1: {} into struct { Name string `hcl:"name"` } (name is required)
    {
        let body = parse_json_body("{}");
        let (got, diags) = gohcl::decode_body::<WithRequiredName>(&*body, None);
        assert_eq!(
            diags.len(),
            1,
            "case 1: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithRequiredName {
                name: String::new()
            },
            "case 1: wrong result"
        );
    }

    // case 2: {} into struct { Name *string `hcl:"name"` } (name nil)
    {
        let body = parse_json_body("{}");
        let (got, diags) = gohcl::decode_body::<WithNamePtr>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 2: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, WithNamePtr { name: None }, "case 2: wrong result");
    }

    // case 3: {} into struct { Name string `hcl:"name,optional"` } (name optional)
    {
        let body = parse_json_body("{}");
        let (got, diags) = gohcl::decode_body::<WithOptionalName>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 3: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithOptionalName {
                name: String::new()
            },
            "case 3: wrong result"
        );
    }

    // case 4: {} into withNameExpression (absent attr yields an expression
    // evaluating to a null value)
    {
        let body = parse_json_body("{}");
        let (got, diags) = gohcl::decode_body::<WithNameExpression>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 4: wrong number of diagnostics: {diags:?}"
        );
        let (name_val, _) = got.name.value(None);
        assert!(name_val.is_null(), "case 4: wrong result: {got:?}");
    }

    // case 5: {"name":"Ermintrude"} into withNameExpression
    {
        let body = parse_json_body(r#"{"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithNameExpression>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 5: wrong number of diagnostics: {diags:?}"
        );
        let (name_val, _) = got.name.value(None);
        assert_eq!(
            name_val,
            Value::string("Ermintrude"),
            "case 5: wrong result"
        );
    }

    // case 6: {"name":"Ermintrude"} into struct { Name string `hcl:"name"` }
    {
        let body = parse_json_body(r#"{"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithRequiredName>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 6: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithRequiredName {
                name: "Ermintrude".to_string()
            },
            "case 6: wrong result"
        );
    }

    // case 7: {"age":23,"name":"Ermintrude"} into struct { Name string `hcl:"name"` }
    // (extraneous "age" property)
    {
        let body = parse_json_body(r#"{"age":23,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithRequiredName>(&*body, None);
        assert_eq!(
            diags.len(),
            1,
            "case 7: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithRequiredName {
                name: "Ermintrude".to_string()
            },
            "case 7: wrong result"
        );
    }

    // case 8: {"age":50,"name":"Ermintrude"} into
    // struct { Name string `hcl:"name"`; Attrs hcl.Attributes `hcl:",remain"` }
    {
        let body = parse_json_body(r#"{"age":50,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithNameAndRemainAttrs>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 8: wrong number of diagnostics: {diags:?}"
        );
        assert!(
            got.name == "Ermintrude" && got.attrs.len() == 1 && got.attrs.contains_key("age"),
            "case 8: wrong result: {got:?}"
        );
    }

    // case 9: {"age":50,"name":"Ermintrude"} into
    // struct { Name string `hcl:"name"`; Remain hcl.Body `hcl:",remain"` }
    {
        let body = parse_json_body(r#"{"age":50,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithNameAndRemainBody>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 9: wrong number of diagnostics: {diags:?}"
        );
        let (attrs, _) = got.remain.just_attributes();
        assert!(
            got.name == "Ermintrude" && attrs.len() == 1 && attrs.contains_key("age"),
            "case 9: wrong result: {got:?}"
        );
    }

    // case 10: {"living":true,"name":"Ermintrude"} into
    // struct { Name string `hcl:"name"`; Remain map[string]cty.Value `hcl:",remain"` }
    {
        let body = parse_json_body(r#"{"living":true,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithNameAndRemainMap>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 10: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithNameAndRemainMap {
                name: "Ermintrude".to_string(),
                remain: HashMap::from([("living".to_string(), Value::bool(true))]),
            },
            "case 10: wrong result"
        );
    }

    // case 11: {"age":50,"name":"Ermintrude"} into
    // struct { Name string `hcl:"name"`; Body hcl.Body `hcl:",body"`; Remain hcl.Body `hcl:",remain"` }
    {
        let body = parse_json_body(r#"{"age":50,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<WithNameBodyAndRemain>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 11: wrong number of diagnostics: {diags:?}"
        );
        let (attrs, _) = got.body.just_attributes();
        assert!(
            got.name == "Ermintrude"
                && attrs.len() == 2
                && attrs.contains_key("name")
                && attrs.contains_key("age"),
            "case 11: wrong result: {got:?}"
        );
    }

    // case 12: {"noodle":{}} into struct { Noodle struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":{}}"#);
        let (_, diags) = gohcl::decode_body::<WithNoodleBlock>(&*body, None);
        // Generating no diagnostics is good enough for this one.
        assert_eq!(
            diags.len(),
            0,
            "case 12: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 13: {"noodle":[{}]} into struct { Noodle struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{}]}"#);
        let (_, diags) = gohcl::decode_body::<WithNoodleBlock>(&*body, None);
        // Generating no diagnostics is good enough for this one.
        assert_eq!(
            diags.len(),
            0,
            "case 13: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 14: {"noodle":[{},{}]} into struct { Noodle struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{},{}]}"#);
        let (_, diags) = gohcl::decode_body::<WithNoodleBlock>(&*body, None);
        // Generating one diagnostic is good enough for this one.
        assert_eq!(
            diags.len(),
            1,
            "case 14: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 15: {} into struct { Noodle struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body("{}");
        let (_, diags) = gohcl::decode_body::<WithNoodleBlock>(&*body, None);
        // Generating one diagnostic is good enough for this one.
        assert_eq!(
            diags.len(),
            1,
            "case 15: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 16: {"noodle":[]} into struct { Noodle struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[]}"#);
        let (_, diags) = gohcl::decode_body::<WithNoodleBlock>(&*body, None);
        // Generating one diagnostic is good enough for this one.
        assert_eq!(
            diags.len(),
            1,
            "case 16: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 17: {"noodle":{}} into struct { Noodle *struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":{}}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodlePtrBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 17: wrong number of diagnostics: {diags:?}"
        );
        assert!(got.noodle.is_some(), "case 17: wrong result: {got:?}");
    }

    // case 18: {"noodle":[{}]} into struct { Noodle *struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{}]}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodlePtrBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 18: wrong number of diagnostics: {diags:?}"
        );
        assert!(got.noodle.is_some(), "case 18: wrong result: {got:?}");
    }

    // case 19: {"noodle":[]} into struct { Noodle *struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[]}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodlePtrBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 19: wrong number of diagnostics: {diags:?}"
        );
        assert!(got.noodle.is_none(), "case 19: wrong result: {got:?}");
    }

    // case 20: {"noodle":[{},{}]} into struct { Noodle *struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{},{}]}"#);
        let (_, diags) = gohcl::decode_body::<WithNoodlePtrBlock>(&*body, None);
        // Generating one diagnostic is good enough for this one.
        assert_eq!(
            diags.len(),
            1,
            "case 20: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 21: {"noodle":[]} into struct { Noodle []struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[]}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodleSliceBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 21: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got.noodle.len(), 0, "case 21: wrong result: {got:?}");
    }

    // case 22: {"noodle":[{}]} into struct { Noodle []struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{}]}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodleSliceBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 22: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got.noodle.len(), 1, "case 22: wrong result: {got:?}");
    }

    // case 23: {"noodle":[{},{}]} into struct { Noodle []struct{} `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":[{},{}]}"#);
        let (got, diags) = gohcl::decode_body::<WithNoodleSliceBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 23: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got.noodle.len(), 2, "case 23: wrong result: {got:?}");
    }

    // case 24: {"noodle":{}} into a labeled-noodle struct (one diagnostic for
    // the missing noodle block and one for the JSON serialization detecting
    // the missing level of hierarchy for the label)
    {
        let body = parse_json_body(r#"{"noodle":{}}"#);
        let (_, diags) = gohcl::decode_body::<WithLabeledNoodle>(&*body, None);
        assert_eq!(
            diags.len(),
            2,
            "case 24: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 25: {"noodle":{"foo_foo":{}}} into the labeled-noodle struct
    {
        let body = parse_json_body(r#"{"noodle":{"foo_foo":{}}}"#);
        let (got, diags) = gohcl::decode_body::<WithLabeledNoodle>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 25: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got.noodle.name, "foo_foo", "case 25: wrong result: {got:?}");
    }

    // case 26: {"noodle":{"bar_baz":{},"foo_foo":{}}} into the labeled-noodle
    // struct (duplicate block; one diagnostic is enough for this one)
    {
        let body = parse_json_body(r#"{"noodle":{"bar_baz":{},"foo_foo":{}}}"#);
        let (_, diags) = gohcl::decode_body::<WithLabeledNoodle>(&*body, None);
        assert_eq!(
            diags.len(),
            1,
            "case 26: wrong number of diagnostics: {diags:?}"
        );
    }

    // case 27: {"noodle":{"bar_baz":{},"foo_foo":{}}} into
    // struct { Noodles []struct { Name string `hcl:"name,label"` } `hcl:"noodle,block"` }
    {
        let body = parse_json_body(r#"{"noodle":{"bar_baz":{},"foo_foo":{}}}"#);
        let (got, diags) = gohcl::decode_body::<WithLabeledNoodles>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 27: wrong number of diagnostics: {diags:?}"
        );
        let noodles = &got.noodles;
        assert!(
            noodles.len() == 2
                && (noodles[0].name == "foo_foo" || noodles[0].name == "bar_baz")
                && (noodles[1].name == "foo_foo" || noodles[1].name == "bar_baz")
                && noodles[0].name != noodles[1].name,
            "case 27: wrong result: {got:?}"
        );
    }

    // case 28: {"noodle":{"foo_foo":{"type":"rice"}}} into the labeled noodle
    // struct with a "type" attribute
    {
        let body = parse_json_body(r#"{"noodle":{"foo_foo":{"type":"rice"}}}"#);
        let (got, diags) = gohcl::decode_body::<WithTypedNoodle>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 28: wrong number of diagnostics: {diags:?}"
        );
        assert!(
            got.noodle.name == "foo_foo" && got.noodle.r#type == "rice",
            "case 28: wrong result: {got:?}"
        );
    }

    // case 29: {"age":34,"name":"Ermintrude"} into map[string]string
    {
        let body = parse_json_body(r#"{"age":34,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<HashMap<String, String>>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 29: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            HashMap::from([
                ("name".to_string(), "Ermintrude".to_string()),
                ("age".to_string(), "34".to_string()),
            ]),
            "case 29: wrong result"
        );
    }

    // case 30: {"age":89,"name":"Ermintrude"} into map[string]*hcl.Attribute
    {
        let body = parse_json_body(r#"{"age":89,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<Attributes>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 30: wrong number of diagnostics: {diags:?}"
        );
        assert!(
            got.len() == 2 && got.contains_key("name") && got.contains_key("age"),
            "case 30: wrong result: {got:?}"
        );
    }

    // case 31: {"age":13,"name":"Ermintrude"} into map[string]hcl.Expression
    {
        let body = parse_json_body(r#"{"age":13,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<HashMap<String, ExprRef>>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 31: wrong number of diagnostics: {diags:?}"
        );
        assert!(
            got.len() == 2 && got.contains_key("name") && got.contains_key("age"),
            "case 31: wrong result: {got:?}"
        );
    }

    // case 32: {"living":true,"name":"Ermintrude"} into map[string]cty.Value
    {
        let body = parse_json_body(r#"{"living":true,"name":"Ermintrude"}"#);
        let (got, diags) = gohcl::decode_body::<HashMap<String, Value>>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 32: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            HashMap::from([
                ("name".to_string(), Value::string("Ermintrude")),
                ("living".to_string(), Value::bool(true)),
            ]),
            "case 32: wrong result"
        );
    }

    // NOTE(port): cases 33-37 decode into a *pre-populated* target value and
    // assert that gohcl.DecodeBody retains (or trims) the existing field
    // values — an in-place decode through a Go pointer. The Rust API
    // (`gohcl::decode_body::<T>(body, ctx) -> (T, Diagnostics)`) always
    // constructs a fresh `T`, so the retention semantics are inexpressible.
    // The Go targets use these named types:
    //
    //   type withTwoAttributes struct {
    //       A string `hcl:"a,optional"`
    //       B string `hcl:"b,optional"`
    //   }
    //   type withNestedBlock struct {
    //       Plain  string             `hcl:"plain,optional"`
    //       Nested *withTwoAttributes `hcl:"nested,block"`
    //   }
    //   type withListofNestedBlocks struct {
    //       Nested []*withTwoAttributes `hcl:"nested,block"`
    //   }
    //   type withListofNestedBlocksNoPointers struct {
    //       Nested []withTwoAttributes `hcl:"nested,block"`
    //   }
    //
    // - case 33: {"plain":"foo"} into &withNestedBlock{Plain: "bar",
    //   Nested: &withTwoAttributes{A: "bar"}}; expects Plain == "foo",
    //   Nested retained with A == "bar"; 0 diagnostics.
    //   ("Retain \"nested\" block while decoding")
    // - case 34: {"nested":{"a":"foo"}} into &withNestedBlock{Nested:
    //   &withTwoAttributes{B: "bar"}}; expects Nested.A == "foo" and
    //   Nested.B == "bar"; 0 diagnostics.
    //   ("Retain values in \"nested\" block while decoding")
    // - case 35: {"nested":[{"a":"foo"}]} into &withListofNestedBlocks{
    //   Nested: []*withTwoAttributes{{B: "bar"}}}; expects Nested[0].A ==
    //   "foo" and Nested[0].B == "bar"; 0 diagnostics.
    //   ("Retain values in \"nested\" block list while decoding")
    // - case 36: {"nested":[{"a":"foo"}]} into &withListofNestedBlocks{
    //   Nested: []*withTwoAttributes{{B: "bar"}, {B: "bar"}}}; expects
    //   len(Nested) == 1; 0 diagnostics.
    //   ("Remove additional elements from the list while decoding nested blocks")
    // - case 37: {"nested":[{"b":"bar"},{"b":"baz"}]} into
    //   &withListofNestedBlocksNoPointers{Nested: []withTwoAttributes{{B:
    //   "foo"}}}; expects Nested[0].B == "bar" and len(Nested) == 2;
    //   0 diagnostics.
    //   ("Make sure decoding value slices works the same as pointer slices.")

    // case 38: {"foo":{"foo_type":{"foo_name":{"value":"foo"}}}} into the
    // range-tagged foo block struct
    {
        let body = parse_json_body(r#"{"foo":{"foo_type":{"foo_name":{"value":"foo"}}}}"#);
        let (got, diags) = gohcl::decode_body::<WithFooBlock>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "case 38: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(
            got,
            WithFooBlock {
                foo: FooWithRanges {
                    r#type: "foo_type".to_string(),
                    type_label_range: make_range("test.json", 1, 9, 19),
                    name: "foo_name".to_string(),
                    name_label_range: make_range("test.json", 1, 21, 31),
                    def_range: make_range("test.json", 1, 32, 33),
                    type_range: make_range("test.json", 1, 2, 7),
                    attribute: "foo".to_string(),
                    attribute_range: make_range("test.json", 1, 33, 46),
                    attribute_name_range: make_range("test.json", 1, 33, 40),
                    attribute_value_range: make_range("test.json", 1, 41, 46),
                },
            },
            "case 38: wrong result"
        );
    }
}

/// Go: the `fixedExpression` test helper type in decode_test.go — an
/// expression that always yields a fixed value, with zero-value ranges and
/// no variables.
#[derive(Debug)]
struct FixedExpression {
    val: Value,
}

impl Expression for FixedExpression {
    fn value(&self, _ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        (self.val.clone(), Diagnostics::default())
    }

    fn variables(&self) -> Vec<Traversal> {
        Vec::new()
    }

    fn range(&self) -> Range {
        Range::default()
    }

    fn start_range(&self) -> Range {
        Range::default()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Ported from TestDecodeExpression:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/gohcl/decode_test.go#L782
//
// The Go case table is heterogeneous in its target type (string, cty.Value,
// bool), so the loop becomes a sequence of blocks, one per case, in upstream
// order.
#[test]
#[ignore = "not yet implemented"]
fn decode_expression() {
    // case 0: cty.StringVal("hello") into string
    {
        let expr = FixedExpression {
            val: Value::string("hello"),
        };
        let (got, diags) = gohcl::decode_expression::<String>(&expr, None);
        assert_eq!(
            diags.len(),
            0,
            "case 0: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, "hello", "case 0: wrong result");
    }

    // case 1: cty.StringVal("hello") into cty.Value
    {
        let expr = FixedExpression {
            val: Value::string("hello"),
        };
        let (got, diags) = gohcl::decode_expression::<Value>(&expr, None);
        assert_eq!(
            diags.len(),
            0,
            "case 1: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, Value::string("hello"), "case 1: wrong result");
    }

    // case 2: cty.NumberIntVal(2) into string
    {
        let expr = FixedExpression {
            val: Value::number_int(2),
        };
        let (got, diags) = gohcl::decode_expression::<String>(&expr, None);
        assert_eq!(
            diags.len(),
            0,
            "case 2: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, "2", "case 2: wrong result");
    }

    // case 3: cty.StringVal("true") into bool
    {
        let expr = FixedExpression {
            val: Value::string("true"),
        };
        let (got, diags) = gohcl::decode_expression::<bool>(&expr, None);
        assert_eq!(
            diags.len(),
            0,
            "case 3: wrong number of diagnostics: {diags:?}"
        );
        assert!(got, "case 3: wrong result");
    }

    // case 4: cty.NullVal(cty.String) into string (null value is not allowed)
    {
        let expr = FixedExpression {
            val: Value::null(Type::string()),
        };
        let (got, diags) = gohcl::decode_expression::<String>(&expr, None);
        assert_eq!(
            diags.len(),
            1,
            "case 4: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, "", "case 4: wrong result");
    }

    // case 5: cty.UnknownVal(cty.String) into string (value must be known)
    {
        let expr = FixedExpression {
            val: Value::unknown(Type::string()),
        };
        let (got, diags) = gohcl::decode_expression::<String>(&expr, None);
        assert_eq!(
            diags.len(),
            1,
            "case 5: wrong number of diagnostics: {diags:?}"
        );
        assert_eq!(got, "", "case 5: wrong result");
    }

    // case 6: cty.ListVal([]cty.Value{cty.True}) into bool (bool required)
    {
        let expr = FixedExpression {
            val: Value::list([Value::bool(true)]),
        };
        let (got, diags) = gohcl::decode_expression::<bool>(&expr, None);
        assert_eq!(
            diags.len(),
            1,
            "case 6: wrong number of diagnostics: {diags:?}"
        );
        assert!(!got, "case 6: wrong result");
    }
}

// ---------------------------------------------------------------------------
// Target struct shapes for TestImpliedBodySchema (anonymous structs in Go).
// `Empty` above is reused for the `struct{}{}` cases.
// ---------------------------------------------------------------------------

// Go: struct { Ignored bool } (untagged field, ignored)
#[derive(FromBody, Debug, PartialEq)]
struct WithIgnored {
    ignored: bool,
}

// Go: struct { Attr1 bool `hcl:"attr1"`; Attr2 bool `hcl:"attr2"` }
#[derive(FromBody, Debug, PartialEq)]
struct TwoBoolAttrs {
    #[hcl(attr = "attr1")]
    attr1: bool,
    #[hcl(attr = "attr2")]
    attr2: bool,
}

// Go: struct { Attr *bool `hcl:"attr,attr"` }
#[derive(FromBody, Debug, PartialEq)]
struct PtrBoolAttr {
    #[hcl(attr = "attr")]
    attr: Option<bool>,
}

// Go: struct { Thing struct{} `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithThingBlock {
    #[hcl(block = "thing")]
    thing: Empty,
}

// Go: struct { Type string `hcl:"type,label"`; Name string `hcl:"name,label"` }
// (the two-label `thing` block shape)
#[derive(FromBody, Debug, PartialEq)]
struct ThingWithTypeAndNameLabels {
    #[hcl(label = "type")]
    r#type: String,
    #[hcl(label = "name")]
    name: String,
}

// Go: struct { Thing struct { Type string `hcl:"type,label"`; Name string `hcl:"name,label"` } `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithLabeledThingBlock {
    #[hcl(block = "thing")]
    thing: ThingWithTypeAndNameLabels,
}

// Go: struct { Thing []struct { Type string `hcl:"type,label"`; Name string `hcl:"name,label"` } `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithLabeledThingSlice {
    #[hcl(block = "thing")]
    thing: Vec<ThingWithTypeAndNameLabels>,
}

// Go: struct { Thing *struct { Type string `hcl:"type,label"`; Name string `hcl:"name,label"` } `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithLabeledThingPtr {
    #[hcl(block = "thing")]
    thing: Option<ThingWithTypeAndNameLabels>,
}

// Go: struct { Name string `hcl:"name,label"`; Something string `hcl:"something"` }
#[derive(FromBody, Debug, PartialEq)]
struct ThingWithNameAndSomething {
    #[hcl(label = "name")]
    name: String,
    #[hcl(attr = "something")]
    something: String,
}

// Go: struct { Thing struct { Name string `hcl:"name,label"`; Something string `hcl:"something"` } `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithThingNameSomething {
    #[hcl(block = "thing")]
    thing: ThingWithNameAndSomething,
}

// Go: struct { Name string `hcl:"name,label"` } (the one-label `thing` block shape)
#[derive(FromBody, Debug, PartialEq)]
struct ThingWithNameLabel {
    #[hcl(label = "name")]
    name: String,
}

// Go: struct { Doodad string `hcl:"doodad"`; Thing struct { Name string `hcl:"name,label"` } `hcl:"thing,block"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithDoodadAndThing {
    #[hcl(attr = "doodad")]
    doodad: String,
    #[hcl(block = "thing")]
    thing: ThingWithNameLabel,
}

// Go: struct { Doodad string `hcl:"doodad"`; Config string `hcl:",remain"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithDoodadAndRemain {
    #[hcl(attr = "doodad")]
    doodad: String,
    #[hcl(remain)]
    config: String,
}

// Go: struct { Expr hcl.Expression `hcl:"expr"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithExprAttr {
    #[hcl(attr = "expr")]
    expr: ExprRef,
}

// Go: struct { Meh string `hcl:"meh,optional"` }
#[derive(FromBody, Debug, PartialEq)]
struct WithOptionalMeh {
    #[hcl(attr = "meh", optional)]
    meh: String,
}

// Ported from TestImpliedBodySchema:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/gohcl/schema_test.go#L15
//
// The Go case table is heterogeneous in its target type, so the loop becomes
// a sequence of blocks, one per case, in upstream order.
#[test]
#[ignore = "not yet implemented"]
fn implied_body_schema() {
    // case 0: struct{}{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<Empty>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![],
            },
            "case 0: wrong schema"
        );
        assert!(!partial, "case 0: wrong partial flag");
    }

    // case 1: struct { Ignored bool }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithIgnored>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![],
            },
            "case 1: wrong schema"
        );
        assert!(!partial, "case 1: wrong partial flag");
    }

    // case 2: struct { Attr1 bool `hcl:"attr1"`; Attr2 bool `hcl:"attr2"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<TwoBoolAttrs>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![
                    AttributeSchema {
                        name: "attr1".to_string(),
                        required: true,
                    },
                    AttributeSchema {
                        name: "attr2".to_string(),
                        required: true,
                    },
                ],
                blocks: vec![],
            },
            "case 2: wrong schema"
        );
        assert!(!partial, "case 2: wrong partial flag");
    }

    // case 3: struct { Attr *bool `hcl:"attr,attr"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<PtrBoolAttr>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![AttributeSchema {
                    name: "attr".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            "case 3: wrong schema"
        );
        assert!(!partial, "case 3: wrong partial flag");
    }

    // case 4: struct { Thing struct{} `hcl:"thing,block"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithThingBlock>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec![],
                }],
            },
            "case 4: wrong schema"
        );
        assert!(!partial, "case 4: wrong partial flag");
    }

    // case 5: struct { Thing struct { Type string `hcl:"type,label"`; Name string `hcl:"name,label"` } `hcl:"thing,block"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithLabeledThingBlock>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec!["type".to_string(), "name".to_string()],
                }],
            },
            "case 5: wrong schema"
        );
        assert!(!partial, "case 5: wrong partial flag");
    }

    // case 6: same as case 5 but with a slice of blocks
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithLabeledThingSlice>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec!["type".to_string(), "name".to_string()],
                }],
            },
            "case 6: wrong schema"
        );
        assert!(!partial, "case 6: wrong partial flag");
    }

    // case 7: same as case 5 but with a pointer to the block struct
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithLabeledThingPtr>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec!["type".to_string(), "name".to_string()],
                }],
            },
            "case 7: wrong schema"
        );
        assert!(!partial, "case 7: wrong partial flag");
    }

    // case 8: struct { Thing struct { Name string `hcl:"name,label"`; Something string `hcl:"something"` } `hcl:"thing,block"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithThingNameSomething>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            "case 8: wrong schema"
        );
        assert!(!partial, "case 8: wrong partial flag");
    }

    // case 9: struct { Doodad string `hcl:"doodad"`; Thing struct { Name string `hcl:"name,label"` } `hcl:"thing,block"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithDoodadAndThing>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![AttributeSchema {
                    name: "doodad".to_string(),
                    required: true,
                }],
                blocks: vec![BlockHeaderSchema {
                    block_type: "thing".to_string(),
                    label_names: vec!["name".to_string()],
                }],
            },
            "case 9: wrong schema"
        );
        assert!(!partial, "case 9: wrong partial flag");
    }

    // case 10: struct { Doodad string `hcl:"doodad"`; Config string `hcl:",remain"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithDoodadAndRemain>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![AttributeSchema {
                    name: "doodad".to_string(),
                    required: true,
                }],
                blocks: vec![],
            },
            "case 10: wrong schema"
        );
        assert!(partial, "case 10: wrong partial flag");
    }

    // case 11: struct { Expr hcl.Expression `hcl:"expr"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithExprAttr>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![AttributeSchema {
                    name: "expr".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            "case 11: wrong schema"
        );
        assert!(!partial, "case 11: wrong partial flag");
    }

    // case 12: struct { Meh string `hcl:"meh,optional"` }{}
    {
        let (schema, partial) = gohcl::implied_body_schema::<WithOptionalMeh>();
        assert_eq!(
            schema,
            BodySchema {
                attributes: vec![AttributeSchema {
                    name: "meh".to_string(),
                    required: false,
                }],
                blocks: vec![],
            },
            "case 12: wrong schema"
        );
        assert!(!partial, "case 12: wrong partial flag");
    }
}

// ---------------------------------------------------------------------------
// Struct shapes for ExampleEncodeIntoBody.
// ---------------------------------------------------------------------------

// Go: type Service struct { Name string `hcl:"name,label"`; Exe []string `hcl:"executable"` }
#[derive(EncodeBody, Debug, PartialEq)]
struct Service {
    #[hcl(label = "name")]
    name: String,
    #[hcl(attr = "executable")]
    exe: Vec<String>,
}

// Go: type Constraints struct { OS string `hcl:"os"`; Arch string `hcl:"arch"` }
#[derive(EncodeBody, Debug, PartialEq)]
struct Constraints {
    #[hcl(attr = "os")]
    os: String,
    #[hcl(attr = "arch")]
    arch: String,
}

// Go: type App struct { Name string `hcl:"name"`; Desc string `hcl:"description"`;
//     Constraints *Constraints `hcl:"constraints,block"`; Services []Service `hcl:"service,block"` }
#[derive(EncodeBody, Debug, PartialEq)]
struct App {
    #[hcl(attr = "name")]
    name: String,
    #[hcl(attr = "description")]
    desc: String,
    #[hcl(block = "constraints")]
    constraints: Option<Constraints>,
    #[hcl(block = "service")]
    services: Vec<Service>,
}

// Ported from ExampleEncodeIntoBody:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/gohcl/encode_test.go#L13
#[test]
#[ignore = "not yet implemented"]
fn example_encode_into_body() {
    let app = App {
        name: "awesome-app".to_string(),
        desc: "Such an awesome application".to_string(),
        constraints: Some(Constraints {
            os: "linux".to_string(),
            arch: "amd64".to_string(),
        }),
        services: vec![
            Service {
                name: "web".to_string(),
                exe: vec!["./web".to_string(), "--listen=:8080".to_string()],
            },
            Service {
                name: "worker".to_string(),
                exe: vec!["./worker".to_string()],
            },
        ],
    };

    let f = hclwrite::File::new();
    gohcl::encode_into_body(&app, &f.body());

    // The Go example's `// Output:` block, byte for byte.
    let want = "\
name        = \"awesome-app\"
description = \"Such an awesome application\"

constraints {
  os   = \"linux\"
  arch = \"amd64\"
}

service \"web\" {
  executable = [\"./web\", \"--listen=:8080\"]
}
service \"worker\" {
  executable = [\"./worker\"]
}
";
    assert_eq!(
        String::from_utf8(f.bytes()).unwrap(),
        want,
        "wrong rendered output"
    );
}
