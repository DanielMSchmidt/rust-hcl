//! Conformance tests transcribed from hcl v2
//! (github.com/hashicorp/hcl @ 6abbb088cdb82416d1b3d9fcbaab29534133567a):
//!   integrationtest/convertfunc_test.go
//!   integrationtest/hcldec_into_expr_test.go
//!   integrationtest/terraformlike_test.go
//!
//! SPDX-License-Identifier: MPL-2.0
//! Derivative work of the MPL-2.0 licensed upstream test files above; see
//! LICENSE-MPL-2.0 and the licensing section of README.md.
//!
//! Expected values are literals from the upstream tables; see
//! docs/api-mapping.md for the Go→Rust API correspondence.

use std::collections::HashMap;
use std::sync::Arc;

use cty::function::{Function, Parameter, Spec, static_return_type};
use cty::{Type, Value};
use hcl::ext::typeexpr;
use hcl::ext::{customdecode, dynblock};
use hcl::gohcl::{self, FromBody};
use hcl::hcldec::{self, AttrSpec, BlockListSpec, ObjectSpec, SpecRef};
use hcl::{
    BodyRef, Diagnostics, EvalContext, ExprRef, File, Pos, Range, Traversal, Traverser, hclsyntax,
    json,
};

// TestTypeConvertFunc is an integration test of all of the layers involved
// in making the type conversion function from ext/typeexpr work.
//
// This requires co-operation between the hclsyntax package, the ext/typeexpr
// package, and the underlying cty functionality in order to work correctly.
//
// There are unit tests for the function implementation itself in the
// ext/typeexpr package, so this test is focused on making sure the function
// is given the opportunity to decode the second argument as a type expression
// when the function is called from HCL native syntax.
//
// Ported from TestTypeConvertFunc:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/integrationtest/convertfunc_test.go#L26
#[test]
#[ignore = "not yet implemented"]
fn type_convert_func() {
    // The convert function is special because it takes a type expression
    // rather than a value expression as its second argument. In this case,
    // we're asking it to convert a tuple into a list of strings:
    let expr_src = r#"convert(["hello"], list(string))"#;
    // It achieves this by marking that second argument as being of a custom
    // type (a "capsule type", in cty terminology) that has a special
    // annotation which hclsyntax::FunctionCallExpr understands as allowing
    // the type to handle the analysis of the unevaluated expression, instead
    // of evaluating it as normal.
    //
    // To see more details of how this works, look at the definitions of
    // typeexpr::type_constraint_type and typeexpr::convert_func, and at the
    // implementation of hclsyntax::FunctionCallExpr's evaluation.

    let (expr, diags) = hclsyntax::parse_expression(
        expr_src.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    assert!(!diags.has_errors(), "unexpected problems: {diags}");

    let mut ctx = EvalContext::new();
    ctx.functions = HashMap::from([("convert".to_string(), typeexpr::convert_func())]);
    let (got, diags) = expr.value(Some(&ctx));
    assert!(!diags.has_errors(), "unexpected problems: {diags}");
    let want = Value::list([Value::string("hello")]);
    assert!(
        want.raw_equals(&got),
        "wrong result\ngot:  {got:?}\nwant: {want:?}"
    );
}

// TestHCLDecDecodeToExpr tests both hcldec's support for types with custom
// expression decoding rules and the two expression capsule types implemented
// in ext/customdecode. This mechanism requires cooperation between those
// two components and cty in order to work, so it's helpful to exercise it in
// an integration test.
//
// Ported from TestHCLDecDecodeToExpr:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/integrationtest/hcldec_into_expr_test.go#L23
#[test]
#[ignore = "not yet implemented"]
fn hcldec_decode_to_expr() {
    // Here we're going to capture the structure of two simple expressions
    // without immediately evaluating them.
    let input = "\na = foo\nb = foo\nc = \"hello\"\n";
    // We'll capture "a" directly as an expression, losing its evaluation
    // context but retaining its structure. We'll capture "b" as a
    // customdecode::ExpressionClosure, which gives us both the expression
    // itself and the evaluation context it was originally evaluated in.
    // We also have "c" here just to make sure we can still decode into a
    // "normal" type via standard expression evaluation.

    let (f, diags) = hclsyntax::parse_config(
        input.as_bytes(),
        "",
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        },
    );
    assert!(!diags.has_errors(), "unexpected problems: {diags}");

    let spec: ObjectSpec = [
        (
            "a",
            SpecRef::new(AttrSpec {
                name: "a".to_string(),
                ty: customdecode::expression_type(),
                required: true,
            }),
        ),
        (
            "b",
            SpecRef::new(AttrSpec {
                name: "b".to_string(),
                ty: customdecode::expression_closure_type(),
                required: true,
            }),
        ),
        (
            "c",
            SpecRef::new(AttrSpec {
                name: "c".to_string(),
                ty: Type::string(),
                required: true,
            }),
        ),
    ]
    .into_iter()
    .collect();
    let mut ctx = EvalContext::new();
    ctx.variables = HashMap::from([("foo".to_string(), Value::string("foo value"))]);
    let (obj_val, diags) = hcldec::decode(&*f.body, &spec, Some(&ctx));
    assert!(!diags.has_errors(), "unexpected problems: {diags}");

    let a_val = obj_val.get_attr("a");
    let b_val = obj_val.get_attr("b");
    let c_val = obj_val.get_attr("c");

    let (got, want) = (a_val.ty(), customdecode::expression_type());
    assert!(
        got.equals(&want),
        "wrong type for 'a'\ngot:  {got:?}\nwant: {want:?}"
    );
    let (got, want) = (b_val.ty(), customdecode::expression_closure_type());
    assert!(
        got.equals(&want),
        "wrong type for 'b'\ngot:  {got:?}\nwant: {want:?}"
    );
    let (got, want) = (c_val.ty(), Type::string());
    assert!(
        got.equals(&want),
        "wrong type for 'c'\ngot:  {got:?}\nwant: {want:?}"
    );

    let got_a_expr = customdecode::expression_from_val(&a_val);
    let want_a_expr = hclsyntax::Expression::ScopeTraversal(hclsyntax::ScopeTraversalExpr {
        traversal: Traversal(vec![Traverser::Root {
            name: "foo".to_string(),
            src_range: Range {
                filename: String::new(),
                start: Pos {
                    line: 2,
                    column: 5,
                    byte: 5,
                },
                end: Pos {
                    line: 2,
                    column: 8,
                    byte: 8,
                },
            },
        }]),
        src_range: Range {
            filename: String::new(),
            start: Pos {
                line: 2,
                column: 5,
                byte: 5,
            },
            end: Pos {
                line: 2,
                column: 8,
                byte: 8,
            },
        },
    });
    // Go compares with cmp.Diff (deep equality over exported fields); the
    // captured expression downcasts to the hclsyntax::Expression enum and
    // compares with PartialEq.
    let got_a_downcast = got_a_expr
        .as_any()
        .downcast_ref::<hclsyntax::Expression>()
        .unwrap_or_else(|| {
            panic!("wrong expression type for a {got_a_expr:?}; want hclsyntax::Expression")
        });
    assert_eq!(got_a_downcast, &want_a_expr, "wrong expression for a");

    let b_closure = customdecode::expression_closure_from_val(&b_val);
    let (got_b_val, diags) = b_closure.value();
    let want_b_val = Value::string("foo value");
    assert!(!diags.has_errors(), "unexpected problems: {diags}");
    assert!(
        want_b_val.raw_equals(&got_b_val),
        "wrong 'b' result\ngot:  {got_b_val:?}\nwant: {want_b_val:?}"
    );

    let want_c_val = Value::string("hello");
    assert!(
        want_c_val.raw_equals(&c_val),
        "wrong 'c'\ngot:  {c_val:?}\nwant: {want_c_val:?}"
    );

    // One additional "trick" we can do with the expression closure is to
    // evaluate the expression in a _derived_ EvalContext, rather than the
    // captured one. This could be useful for introducing additional local
    // variables/functions in a particular context, for example.
    let mut derive_ctx = EvalContext::new_child(
        b_closure
            .eval_context
            .as_ref()
            .expect("closure has an EvalContext"),
    );
    derive_ctx.variables =
        HashMap::from([("foo".to_string(), Value::string("overridden foo value"))]);
    let (got_b_val2, diags) = b_closure.expression.value(Some(&derive_ctx));
    let want_b_val2 = Value::string("overridden foo value");
    assert!(!diags.has_errors(), "unexpected problems: {diags}");
    assert!(
        want_b_val2.raw_equals(&got_b_val2),
        "wrong 'b' result with derived EvalContext\ngot:  {got_b_val2:?}\nwant: {want_b_val2:?}"
    );
}

// Go: the test-local struct types of TestTerraformLike, with their
// `hcl:"..."` tags mapped per docs/api-mapping.md (Go `[]*T` block slices
// become `Vec<T>`).
#[derive(Debug, PartialEq, FromBody)]
struct Variable {
    #[hcl(label = "name")]
    name: String,
}

#[derive(Debug, FromBody)]
struct Resource {
    #[hcl(label = "type")]
    r#type: String,
    #[hcl(label = "name")]
    name: String,
    #[hcl(remain)]
    config: BodyRef,
    #[hcl(attr = "depends_on")]
    depends_on: ExprRef,
}

#[derive(Debug, FromBody)]
struct Module {
    #[hcl(label = "name")]
    name: String,
    #[hcl(attr = "providers")]
    providers: ExprRef,
}

#[derive(Debug, FromBody)]
struct Locals {
    #[hcl(remain)]
    config: BodyRef,
}

#[derive(Debug, FromBody)]
struct Root {
    #[hcl(block = "variable")]
    variables: Vec<Variable>,
    #[hcl(block = "resource")]
    resources: Vec<Resource>,
    #[hcl(block = "module")]
    modules: Vec<Module>,
    #[hcl(block = "locals")]
    locals: Vec<Locals>,
}

// TestTerraformLike parses both a native syntax and a JSON representation
// of the same HashiCorp Terraform-like configuration structure and then makes
// assertions against the result of each.
//
// Terraform exercises a lot of different HCL codepaths, so this is not
// exhaustive but tries to cover a variety of different relevant scenarios.
//
// Ported from TestTerraformLike:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/integrationtest/terraformlike_test.go#L28
#[test]
#[ignore = "not yet implemented"]
fn terraform_like() {
    type LoadFunc = fn() -> (File, Diagnostics);
    // Go ranges over a map here; the two entries are ported in written order.
    let tests: [(&str, LoadFunc); 2] = [
        ("native syntax", || {
            hclsyntax::parse_config(
                TERRAFORM_LIKE_NATIVE_SYNTAX.as_bytes(),
                "config.tf",
                Pos {
                    line: 1,
                    column: 1,
                    byte: 0,
                },
            )
        }),
        ("JSON", || {
            json::parse(TERRAFORM_LIKE_JSON.as_bytes(), "config.tf.json")
        }),
    ];

    let instance_decode: ObjectSpec = [
        (
            "image_id",
            SpecRef::new(AttrSpec {
                name: "image_id".to_string(),
                required: true,
                ty: Type::string(),
            }),
        ),
        (
            "instance_type",
            SpecRef::new(AttrSpec {
                name: "instance_type".to_string(),
                required: true,
                ty: Type::string(),
            }),
        ),
        (
            "tags",
            SpecRef::new(AttrSpec {
                name: "tags".to_string(),
                required: false,
                ty: Type::map(Type::string()),
            }),
        ),
    ]
    .into_iter()
    .collect();
    let security_group_decode: ObjectSpec = [(
        "ingress",
        SpecRef::new(BlockListSpec {
            type_name: "ingress".to_string(),
            nested: SpecRef::new(
                [(
                    "cidr_block",
                    SpecRef::new(AttrSpec {
                        name: "cidr_block".to_string(),
                        required: true,
                        ty: Type::string(),
                    }),
                )]
                .into_iter()
                .collect::<ObjectSpec>(),
            ),
            min_items: 0,
            max_items: 0,
        }),
    )]
    .into_iter()
    .collect();

    for (name, load_func) in tests {
        let (file, diags) = load_func();
        assert_eq!(
            diags.len(),
            0,
            "{name}: unexpected diagnostics during parse:\n{diags}"
        );

        let body = file.body;

        let (mut root, diags) = gohcl::decode_body::<Root>(&*body, None);
        assert_eq!(
            diags.len(),
            0,
            "{name}: unexpected diagnostics during root eval:\n{diags}"
        );

        let want_vars = vec![Variable {
            name: "image_id".to_string(),
        }];
        assert_eq!(root.variables, want_vars, "{name}: wrong Variables");

        assert_eq!(root.resources.len(), 3, "{name}: wrong number of Resources");

        root.resources.sort_by(|i, j| i.name.cmp(&j.name));

        // Subtest "resource 0" (Go: t.Run("resource 0", ...)).
        {
            let r = &root.resources[0];
            assert_eq!(
                r.r#type, "happycloud_security_group",
                "{name}: resource 0: wrong type"
            );
            assert_eq!(r.name, "private", "{name}: resource 0: wrong type");

            // For this one we're including support for the dynamic block
            // extension, since Terraform uses this to allow dynamic
            // generation of blocks within resource configuration.
            let mut for_each_ctx = EvalContext::new();
            for_each_ctx.variables = HashMap::from([(
                "var".to_string(),
                Value::object([(
                    "extra_private_cidr_blocks",
                    Value::list([
                        Value::string("172.16.0.0/12"),
                        Value::string("169.254.0.0/16"),
                    ]),
                )]),
            )]);
            let dyn_body = dynblock::expand(r.config.clone(), Some(Arc::new(for_each_ctx)), vec![]);

            let (cfg, diags) = hcldec::decode(&*dyn_body, &security_group_decode, None);
            assert_eq!(
                diags.len(),
                0,
                "{name}: resource 0: unexpected diagnostics decoding Config:\n{diags}"
            );
            let want_cfg = Value::object([(
                "ingress",
                Value::list([
                    Value::object([("cidr_block", Value::string("10.0.0.0/8"))]),
                    Value::object([("cidr_block", Value::string("192.168.0.0/16"))]),
                    Value::object([("cidr_block", Value::string("172.16.0.0/12"))]),
                    Value::object([("cidr_block", Value::string("169.254.0.0/16"))]),
                ]),
            )]);
            assert!(
                cfg.raw_equals(&want_cfg),
                "{name}: resource 0: wrong config\ngot:  {cfg:?}\nwant: {want_cfg:?}"
            );
        }

        // Subtest "resource 1" (Go: t.Run("resource 1", ...)).
        {
            let r = &root.resources[1];
            assert_eq!(
                r.r#type, "happycloud_security_group",
                "{name}: resource 1: wrong type"
            );
            assert_eq!(r.name, "public", "{name}: resource 1: wrong type");

            let (cfg, diags) = hcldec::decode(&*r.config, &security_group_decode, None);
            assert_eq!(
                diags.len(),
                0,
                "{name}: resource 1: unexpected diagnostics decoding Config:\n{diags}"
            );
            let want_cfg = Value::object([(
                "ingress",
                Value::list([Value::object([("cidr_block", Value::string("0.0.0.0/0"))])]),
            )]);
            assert!(
                cfg.raw_equals(&want_cfg),
                "{name}: resource 1: wrong config\ngot:  {cfg:?}\nwant: {want_cfg:?}"
            );
        }

        // Subtest "resource 2" (Go: t.Run("resource 2", ...)).
        {
            let r = &root.resources[2];
            assert_eq!(
                r.r#type, "happycloud_instance",
                "{name}: resource 2: wrong type"
            );
            assert_eq!(r.name, "test", "{name}: resource 2: wrong type");

            let vars = hcldec::variables(
                &*r.config,
                &AttrSpec {
                    name: "image_id".to_string(),
                    ty: Type::string(),
                    required: false,
                },
            );
            assert_eq!(
                vars.len(),
                1,
                "{name}: resource 2: wrong number of variables in image_id"
            );
            assert_eq!(
                vars[0].root_name(),
                "var",
                "{name}: resource 2: wrong image_id variable RootName"
            );

            let mut ctx = EvalContext::new();
            ctx.variables = HashMap::from([(
                "var".to_string(),
                Value::object([("image_id", Value::string("image-1234"))]),
            )]);
            let (cfg, diags) = hcldec::decode(&*r.config, &instance_decode, Some(&ctx));
            assert_eq!(
                diags.len(),
                0,
                "{name}: resource 2: unexpected diagnostics decoding Config:\n{diags}"
            );
            let want_cfg = Value::object([
                ("instance_type", Value::string("z3.weedy")),
                ("image_id", Value::string("image-1234")),
                (
                    "tags",
                    Value::map([
                        ("Name", Value::string("foo")),
                        ("Environment", Value::string("prod")),
                    ]),
                ),
            ]);
            assert!(
                cfg.raw_equals(&want_cfg),
                "{name}: resource 2: wrong config\ngot:  {cfg:?}\nwant: {want_cfg:?}"
            );

            let (exprs, diags) = hcl::expr_list(&*r.depends_on);
            assert_eq!(
                diags.len(),
                0,
                "{name}: resource 2: unexpected diagnostics extracting depends_on:\n{diags}"
            );
            assert_eq!(
                exprs.len(),
                1,
                "{name}: resource 2: wrong number of depends_on exprs"
            );

            let (traversal, diags) = hcl::abs_traversal_for_expr(&*exprs[0]);
            assert_eq!(
                diags.len(),
                0,
                "{name}: resource 2: unexpected diagnostics decoding depends_on[0]:\n{diags}"
            );
            assert_eq!(
                traversal.0.len(),
                2,
                "{name}: resource 2: wrong number of depends_on traversal steps"
            );
            assert_eq!(
                traversal.root_name(),
                "happycloud_security_group",
                "{name}: resource 2: wrong depends_on traversal RootName"
            );
        }

        // Subtest "module" (Go: t.Run("module", ...)).
        {
            assert_eq!(root.modules.len(), 1, "{name}: wrong number of Modules");
            let module = &root.modules[0];
            assert_eq!(module.name, "foo", "{name}: module: wrong module name");

            let p_expr = &module.providers;
            let (pairs, diags) = hcl::expr_map(&**p_expr);
            assert_eq!(
                diags.len(),
                0,
                "{name}: module: unexpected diagnostics extracting providers:\n{diags}"
            );
            assert_eq!(
                pairs.len(),
                1,
                "{name}: module: wrong number of key/value pairs in providers"
            );

            let pair = &pairs[0];
            let (kt, diags) = hcl::abs_traversal_for_expr(&*pair.key);
            assert_eq!(
                diags.len(),
                0,
                "{name}: module: unexpected diagnostics extracting providers key {:?}:\n{diags}",
                pair.key
            );
            let (vt, diags) = hcl::abs_traversal_for_expr(&*pair.value);
            assert_eq!(
                diags.len(),
                0,
                "{name}: module: unexpected diagnostics extracting providers value  {:?}:\n{diags}",
                pair.value
            );

            assert_eq!(
                kt.0.len(),
                1,
                "{name}: module: wrong number of key traversal steps"
            );
            assert_eq!(
                vt.0.len(),
                2,
                "{name}: module: wrong number of value traversal steps"
            );

            assert_eq!(
                kt.root_name(),
                "null",
                "{name}: module: wrong number key traversal root"
            );
            assert_eq!(
                vt.root_name(),
                "null",
                "{name}: module: wrong number value traversal root"
            );
            if let Traverser::Attr {
                name: attr_name, ..
            } = &vt.0[1]
            {
                assert_eq!(
                    attr_name, "foo",
                    "{name}: module: wrong number value traversal attribute name"
                );
            } else {
                panic!(
                    "{name}: module: wrong value traversal [1] type {:?}; want Traverser::Attr",
                    vt.0[1]
                );
            }
        }

        // Subtest "locals" (Go: t.Run("locals", ...)).
        {
            let locals = &root.locals[0];
            let (attrs, diags) = locals.config.just_attributes();
            assert!(!diags.has_errors(), "{name}: locals: {diags}");

            let mut ctx = EvalContext::new();
            ctx.functions = HashMap::from([
                (
                    "func".to_string(),
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![Parameter {
                            ty: Some(Type::string()),
                            ..Default::default()
                        }],
                        var_param: None,
                        type_fn: static_return_type(Type::string()),
                        refine_result: None,
                        impl_fn: Box::new(|_args, _retty| Ok(Value::string("func_result"))),
                    }),
                ),
                (
                    "scoped::func".to_string(),
                    Function::new(Spec {
                        description: String::new(),
                        params: vec![Parameter {
                            ty: Some(Type::string()),
                            ..Default::default()
                        }],
                        var_param: None,
                        type_fn: static_return_type(Type::string()),
                        refine_result: None,
                        impl_fn: Box::new(|_args, _retty| Ok(Value::string("scoped::func_result"))),
                    }),
                ),
            ]);

            let res = &attrs["func_result"];
            let (func_val, diags) = res.expr.value(Some(&ctx));
            assert!(!diags.has_errors(), "{name}: locals: {diags}");

            let want_val = Value::string("func_result");

            assert!(
                func_val.raw_equals(&want_val),
                "{name}: locals: expected {want_val:?}, got {func_val:?}"
            );

            let res = &attrs["scoped_func_result"];
            let (func_val, diags) = res.expr.value(Some(&ctx));
            assert!(!diags.has_errors(), "{name}: locals: {diags}");

            let want_val = Value::string("scoped::func_result");

            assert!(
                func_val.raw_equals(&want_val),
                "{name}: locals: expected {want_val:?}, got {func_val:?}"
            );
        }
    }
}

// Go: the `terraformLikeNativeSyntax` const.
const TERRAFORM_LIKE_NATIVE_SYNTAX: &str = r#"

variable "image_id" {
}

locals {
  func_result        = func("arg")
  scoped_func_result = scoped::func("arg")
}

resource "happycloud_instance" "test" {
  instance_type = "z3.weedy"
  image_id      = var.image_id

  tags = {
  "Name" = "foo"
  "${"Environment"}" = "prod"
  }

  depends_on = [
    happycloud_security_group.public,
  ]
}

resource "happycloud_security_group" "public" {
  ingress {
    cidr_block = "0.0.0.0/0"
  }
}

resource "happycloud_security_group" "private" {
  ingress {
    cidr_block = "10.0.0.0/8"
  }
  ingress {
    cidr_block = "192.168.0.0/16"
  }
  dynamic "ingress" {
    for_each = var.extra_private_cidr_blocks
    content {
      cidr_block = ingress.value
    }
  }
}

module "foo" {
  providers = {
    null = null.foo
  }
}

"#;

// Go: the `terraformLikeJSON` const. The odd tab indentation on the
// `scoped_func_result` line is upstream's, preserved byte-for-byte.
const TERRAFORM_LIKE_JSON: &str = r#"
{
  "variable": {
    "image_id": {}
  },
  "locals": {
    "func_result": "${func(\"arg\")}",
	"scoped_func_result": "${scoped::func(\"arg\")}"
  },
  "resource": {
    "happycloud_instance": {
      "test": {
        "instance_type": "z3.weedy",
        "image_id": "${var.image_id}",
        "tags": {
            "Name": "foo",
            "${\"Environment\"}": "prod"
        },
        "depends_on": [
          "happycloud_security_group.public"
        ]
      }
    },
    "happycloud_security_group": {
      "public": {
        "ingress": {
          "cidr_block": "0.0.0.0/0"
        }
      },
      "private": {
        "ingress": [
          {
            "cidr_block": "10.0.0.0/8"
          },
          {
            "cidr_block": "192.168.0.0/16"
          }
        ],
        "dynamic": {
          "ingress": {
            "for_each": "${var.extra_private_cidr_blocks}",
            "iterator": "block",
            "content": {
              "cidr_block": "${block.value}"
            }
          }
        }
      }
    }
  },
  "module": {
    "foo": {
      "providers": {
        "null": "null.foo"
      }
    }
  }
}
"#;
