# hcl v2 → rust-hcl API mapping

The contract between the upstream Go API and the Rust surface that the
conformance tests compile against. Pinned upstream:
`github.com/hashicorp/hcl` (v2, `main` branch) @
`6abbb088cdb82416d1b3d9fcbaab29534133567a`.

The cty side of the API follows rust-cty's own `docs/api-mapping.md`
(`cty.StringVal("x")` → `Value::string("x")`, `cty.NumberIntVal(1)` →
`Value::number_int(1)`, `cty.List(cty.String)` →
`Type::list(Type::string())`, and so on).

## General rules

- **Naming.** `CamelCase` → `snake_case`. Go package-level functions become
  module-level functions (`hclsyntax.ParseConfig` →
  `hclsyntax::parse_config`); methods stay methods. Go packages become
  modules of the single `hcl` crate: `hcl` root package → crate root,
  `hclsyntax` → `hcl::hclsyntax`, `ext/dynblock` → `hcl::ext::dynblock`,
  etc.
- **Diagnostics accumulate.** Where Go returns `(T, hcl.Diagnostics)`, Rust
  returns `(T, Diagnostics)` — **not** `Result` — because tests assert
  partial results and diagnostics together. `Diagnostics::has_errors()`
  mirrors `HasErrors`. Summary and detail strings are asserted
  byte-for-byte wherever upstream asserts them. The Go append-and-reassign
  idiom `diags = diags.Append(d)` / `diags = diags.Extend(more)` becomes
  in-place `diags.push(d)` / `diags.extend(more)`.
- **Severities.** `hcl.DiagInvalid/DiagError/DiagWarning` →
  `DiagnosticSeverity::{Invalid, Error, Warning}`.
- **Positions are literal.** `Pos { line, column, byte }`: 1-based line,
  1-based column counted in **grapheme clusters**, 0-based byte offset.
  `hcl.InitialPos` → `Pos::initial()`. Port every position expectation
  literally — never convert to a different indexing.
- **Ranges.** `hcl.Range{Filename, Start, End}` →
  `Range { filename, start, end }`. Free functions become associated:
  `hcl.RangeBetween(a, b)` → `Range::between(a, b)`, `hcl.RangeOver(a, b)`
  → `Range::over(a, b)`. `Range.String()` → `Display`
  (`format!("{r}")`), with byte-identical output (e.g. `file.tf:1,5-12`).
  `Range.Ptr()` has no analogue (`Option<Range>` at use sites).
- **`nil` pointers/interfaces → `Option`.** A Go `*hcl.Range` field is
  `Option<Range>`, a nil `*hcl.EvalContext` argument is
  `Option<&EvalContext>` (pass `None` where Go passes `nil`). Go
  zero-value-only cases with no Rust analogue are omitted with a
  `NOTE(port)` comment.
- **Panics vs. errors.** Operations that panic on misuse in Go panic in
  Rust; tests assert them with `std::panic::catch_unwind`. Go `error`
  returns become `Result` only where the Go API returns a bare `error`
  (e.g. `hclsimple.Decode` → `Result<T, Diagnostics>`).
- **No `Must*` variants**: use the fallible form plus `.unwrap()`.
- **Equality.** `PartialEq` on AST/data types implements whatever deep
  equality upstream tests compare with (`reflect.DeepEqual` /
  `deep.Equal`, which look at exported fields). Interior handles
  (`BodyRef`, `ExprRef`) compare via the `eq_dyn` trait hook.
  `Diagnostic: PartialEq` compares severity/summary/detail/subject/context
  only — Go tests never compare `Expression`/`EvalContext`/`Extra` by
  value.
- **`GoString`/`%#v`.** Where upstream asserts Go-syntax renderings, port
  byte-for-byte to a `go_string()` method, plus a twin test pinning
  `Display`/`to_string()` to the equivalent Rust constructor syntax
  (see rust-cty's `type_go_string`/`type_display` pair).
- **Maps.** Go `map[string]X` → `HashMap<String, X>`. Order-dependent
  assertions must sort exactly where upstream sorts.
- **Interface values → handle newtypes.** `hcl.Body` and `hcl.Expression`
  are traits; interface *values* are the `BodyRef`/`ExprRef` newtypes
  around `Arc<dyn ...>`. Wrap with `BodyRef::new(b)` / `ExprRef::new(e)`.
  Downcasting a Go type assertion `x.(*hclsyntax.Body)` becomes
  `x.as_any().downcast_ref::<hclsyntax::Body>()`.
- **Optional Go interfaces → default trait methods.** Go's pattern of
  asserting side interfaces (`exprList`, `exprCall`, `exprMap`,
  `unwrapExpression`, hcldec's `UnknownBody`/`MarkedBody`, hcled's
  `contextStringer`) becomes default trait methods returning
  `Option`/`None` on `Expression`/`Body`, and the `FileNav` trait for
  `File.Nav`.

## Core type correspondence

| hcl v2 (Go) | rust-hcl |
|---|---|
| `hcl.Pos{Line, Column, Byte}` | `Pos { line, column, byte }` |
| `hcl.InitialPos` | `Pos::initial()` |
| `hcl.Range{Filename, Start, End}` | `Range { filename, start, end }` |
| `hcl.RangeBetween/RangeOver` | `Range::between/over` |
| `r.ContainsPos/ContainsOffset/Overlaps/Overlap/PartitionAround/Empty/String()` | `contains_pos/contains_offset/overlaps/overlap/partition_around/empty/Display` |
| `hcl.RangeScanner`, `bufio.ScanLines` | `RangeScanner`, `scan_lines` |
| `hcl.Diagnostic{...}` | `Diagnostic { severity, summary, detail, subject, context, expression, eval_context, extra }` (strings are `String`, ranges `Option<Range>`) |
| `hcl.Diagnostics` | `Diagnostics` (newtype over `Vec<Diagnostic>`; `Deref` to the vec) |
| `diags.HasErrors()/Errs()/Error()` | `has_errors()/errs()/Display` |
| `hcl.DiagnosticExtra[T](diag)` | `diagnostic_extra::<T>(&diag)` |
| `hcl.NewDiagnosticTextWriter(w, files, width, color)` | `DiagnosticTextWriter::new(w, files, width, color)` |
| `hcl.File{Body, Bytes, Nav}` | `File { body: BodyRef, bytes, nav: Option<Arc<dyn FileNav>> }` |
| `hcl.Block{Type, Labels, Body, DefRange, TypeRange, LabelRanges}` | `Block { block_type, labels, body, def_range, type_range, label_ranges }` |
| `hcl.Blocks` + `OfType/ByType` | `Blocks` (newtype) + `of_type/by_type` |
| `hcl.Attribute{Name, Expr, Range, NameRange}` | `Attribute { name, expr, range, name_range }` |
| `hcl.Attributes` | `Attributes` = `HashMap<String, Attribute>` |
| `hcl.BodyContent{Attributes, Blocks, MissingItemRange}` | `BodyContent { attributes, blocks, missing_item_range }` |
| `hcl.Body` (interface) | `trait Body` (`content/partial_content/just_attributes/missing_item_range`); values are `BodyRef` |
| `hcl.Expression` (interface) | `trait Expression` (`value/variables/range/start_range`); values are `ExprRef` |
| `expr.Value(nil)` | `expr.value(None)` |
| `hcl.EvalContext{Variables, Functions}` | `EvalContext { variables, functions }` |
| `ctx.NewChild()` | `EvalContext::new_child(&arc_ctx)` (wrap the parent in `Arc` first) |
| `hcl.BodySchema{Attributes, Blocks}` | `BodySchema { attributes, blocks }` |
| `hcl.AttributeSchema{Name, Required}` | `AttributeSchema { name, required }` |
| `hcl.BlockHeaderSchema{Type, LabelNames}` | `BlockHeaderSchema { block_type, label_names }` |
| `hcl.Traversal{...}` | `Traversal(vec![...])` |
| `hcl.TraverseRoot{Name, SrcRange}` | `Traverser::Root { name, src_range }` |
| `hcl.TraverseAttr/TraverseIndex/TraverseSplat` | `Traverser::Attr { .. } / Index { key, .. } / Splat { each, .. }` |
| `hcl.TraversalJoin(a, r)` | `Traversal::join(a, r)` |
| `t.TraverseRel/TraverseAbs/IsRelative/SimpleSplit/RootName/SourceRange` | same, snake_case; `traverse_abs` takes `Option<&EvalContext>` |
| `hcl.TraversalSplit{Abs, Rel}` | `TraversalSplit { abs, rel }` |
| traversal rendering in diagnostics | `Traversal: Display` (`foo.bar[0]`) |
| `hcl.AbsTraversalForExpr/RelTraversalForExpr/ExprAsKeyword` | `abs_traversal_for_expr/rel_traversal_for_expr/expr_as_keyword` (take `&dyn Expression`) |
| `hcl.ExprCall/ExprList/ExprMap` | `expr_call/expr_list/expr_map` |
| `hcl.StaticCall{Name, NameRange, Arguments, ArgsRange}` | `StaticCall { name, name_range, arguments, args_range }` |
| `hcl.KeyValuePair{Key, Value}` | `KeyValuePair { key, value }` |
| `hcl.UnwrapExpression/UnwrapExpressionUntil` | `unwrap_expression/unwrap_expression_until` |
| `hcl.StaticExpr(val, rng)` | `static_expr(val, rng)` |
| `hcl.MergeFiles/MergeBodies/EmptyBody` | `merge_files/merge_bodies/empty_body` |
| `hcl.Index/GetAttr/ApplyPath(v, ..., *Range)` | `index/get_attr/apply_path(..., Option<&Range>)` |

## hclsyntax

| hcl v2 (Go) | rust-hcl |
|---|---|
| `hclsyntax.ParseConfig/ParseExpression/ParseTemplate` | `hclsyntax::parse_config/parse_expression/parse_template` |
| `hclsyntax.ParseTraversalAbs/ParseTraversalPartial` | `parse_traversal_abs/parse_traversal_partial` |
| `hclsyntax.LexConfig/LexExpression/LexTemplate` | `lex_config/lex_expression/lex_template` |
| `hclsyntax.ValidIdentifier` | `valid_identifier` |
| `hclsyntax.ParseStringLiteralToken` | `parse_string_literal_token` |
| `hclsyntax.Token{Type, Bytes, Range}` | `Token { ty, bytes, range }` |
| `hclsyntax.TokenType` constants | `TokenType` enum, Go name minus prefix: `TokenOBrace` → `TokenType::OBrace`, `TokenEOF` → `TokenType::EOF`, `TokenNil` → `TokenType::Nil` |
| `TokenType.GoString()` | `go_string()` (`"hclsyntax.TokenOBrace"`) |
| unexported `scanTokens(data, fn, start, mode)` | `scan_tokens(data, filename, start, ScanMode::{Normal, Template, IdentOnly})` |
| unexported `scanStringLit(data, quoted)` | `scan_string_lit(data, quoted) -> Vec<Vec<u8>>` |
| unexported `checkInvalidTokens` | `check_invalid_tokens(&[Token])` |
| unexported `nameSuggestion` | `name_suggestion(given, &[&str])` |
| unexported `peeker` | `Peeker` (`new/peek/read/next_range/prev_range/push_include_newlines/pop_include_newlines`) |
| `hclsyntax.Body{Attributes, Blocks, SrcRange, EndRange}` | `hclsyntax::Body { attributes, blocks, src_range, end_range }` (a struct, also `impl hcl::Body`) |
| `hclsyntax.Attribute{..., EqualsRange}` | `hclsyntax::Attribute { name, expr, src_range, name_range, equals_range }` |
| `hclsyntax.Block{Type, ..., OpenBraceRange, CloseBraceRange}` | `hclsyntax::Block { block_type, labels, body, type_range, label_ranges, open_brace_range, close_brace_range }` |
| `file.Body.(*hclsyntax.Body)` | `file.body.as_any().downcast_ref::<hclsyntax::Body>().unwrap()` |
| `hclsyntax.Expression` (interface) | `hclsyntax::Expression` **enum**; variant per node type, name minus `Expr`: `Expression::LiteralValue(..)`, `ScopeTraversal`, `RelativeTraversal`, `FunctionCall`, `Conditional`, `Index`, `TupleCons`, `ObjectCons`, `ObjectConsKey`, `For`, `Splat`, `AnonSymbol`, `BinaryOp`, `UnaryOp`, `Template`, `TemplateJoin`, `TemplateWrap`, `Parentheses`, `SyntaxError` |
| `&hclsyntax.LiteralValueExpr{...}` (as an Expression) | `LiteralValueExpr { .. }.into()` (every node struct has `From<..> for Expression`) |
| nil `KeyExpr`/`CondExpr` in `ForExpr` | `key_expr/cond_expr: Option<Expression>` |
| `hclsyntax.OpAdd` etc. (`*Operation` identity) | `Operation::{Add, Subtract, Multiply, Divide, Modulo, Negate, Equal, NotEqual, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, LogicalAnd, LogicalOr, LogicalNot}` |
| `TemplateExpr.IsStringLiteral()` | `is_string_literal()` on `TemplateExpr` (match on the enum first) |
| `hclsyntax.Variables(expr)` | `hclsyntax::variables(&expr)` |
| `hclsyntax.Walk/VisitAll`, `Node`, `Walker` | `walk/visit_all`, `Node<'_>` enum of borrows, `trait Walker { enter, exit }` |
| `hclsyntax.ChildScope` | `ChildScope { local_names, expr }` |
| `hclsyntax.File{Body, Bytes}` / `AsHCLFile` | `hclsyntax::File { body, bytes }` / `as_hcl_file()` |
| `hclsyntax.FunctionCallDiagExtra` / `FunctionCallUnknownDiagExtra` | same-named traits (via `diagnostic_extra`) |
| navigation (`file.Nav`) | `file.nav` (`Option<Arc<dyn FileNav>>`), `context_string`/`context_def_range` |

## json

| hcl v2 (Go) | rust-hcl |
|---|---|
| `json.Parse/ParseWithStartPos/ParseExpression/ParseExpressionWithStartPos/ParseFile` | `json::parse/parse_with_start_pos/parse_expression/parse_expression_with_start_pos/parse_file` |
| `json.IsJSONExpression/IsJSONBody` | `is_json_expression/is_json_body` |
| unexported `parseFileContent/parseExpression` (raw AST) | `parse_file_content/parse_value` |
| unexported `objectVal/arrayVal/...` nodes | `json::Node::{Object, Array, Boolean, Number, String, Null, Invalid}` (struct variants; `objectAttr` → `ObjectAttr`) |
| unexported scanner `token`/`tokenType`/`scan`/`pos` | `json::scanner::{Token, TokenType, scan, ScannerPos}`; `tokenBraceO` → `TokenType::BraceO`, etc. |
| unexported `keywordSuggestion` | `keyword_suggestion` |

## hclparse / hcldec / hclwrite / gohcl / hcled / hcltest / hclsimple

| hcl v2 (Go) | rust-hcl |
|---|---|
| `hclparse.NewParser()` | `hclparse::Parser::new()` (methods take `&mut self`) |
| `hcldec.Decode/PartialDecode/ImpliedType/SourceRange/ChildBlockTypes/ImpliedSchema/Variables` | `hcldec::{decode, partial_decode, implied_type, source_range, child_block_types, implied_schema, variables}` (specs pass as `&dyn Spec`) |
| `hcldec.Spec` (interface) | `trait Spec`; composition uses `SpecRef::new(..)` handles |
| `hcldec.ObjectSpec{"a": s}` | `ObjectSpec::from_iter([("a", SpecRef::new(s))])` (or the `FromIterator` collect idiom) |
| `hcldec.TupleSpec{...}` | `TupleSpec(vec![SpecRef::new(..), ..])` |
| `&hcldec.AttrSpec{Name, Type, Required}` | `AttrSpec { name, ty, required }` |
| `BlockSpec/BlockListSpec/BlockTupleSpec/BlockSetSpec{TypeName, Nested, ...}` | same names; `type_name`, `nested: SpecRef`, `min_items`/`max_items: usize` |
| `BlockMapSpec/BlockObjectSpec{LabelNames}` | `label_names` |
| `BlockAttrsSpec{ElementType}` | `element_type` |
| `DefaultSpec{Primary, Default}` | `DefaultSpec { primary, default }` |
| `RefineValueSpec.Refine` / `ValidateSpec.Func` | boxed closures (`RefineFunc` / `ValidateFunc`) |
| `hclwrite.NewEmptyFile()/NewFile()` | `hclwrite::File::new()` |
| `hclwrite.ParseConfig/Format` | `hclwrite::parse_config/format` |
| `hclwrite` nodes (`*File/*Body/*Block/*Attribute/*Expression`) | cloneable shared **handles**; `&self` methods mutate the shared node like Go pointer methods |
| `body.SetAttributeValue/SetAttributeTraversal/SetAttributeRaw/...` | same, snake_case; "not found" `nil` returns are `Option` |
| `hclwrite.Token{Type, Bytes, SpacesBefore}` | `Token { ty, bytes, spaces_before }` |
| `hclwrite.Tokens` (`[]*Token`) | `Tokens` (newtype over `Vec<Token>`); `toks.Bytes()` → `bytes()` |
| `hclwrite.TokensForValue/...Traversal/...Identifier/...Tuple/...Object/...FunctionCall` | `tokens_for_*` |
| `hclwrite.ObjectAttrTokens{Name, Value}` | `ObjectAttrTokens { name, value }` |
| unexported `format` internals (`linesForFormat`, `partitionTokens`, `partitionLeadCommentTokens`, `lexConfig`) | `lines_for_format/partition_tokens/partition_lead_comment_tokens/lex_config`, `FormatLine { lead, assign, comment }` |
| `TestTreeNode`/`makeTestTree` (upstream test helper) | `TestTreeNode { node_type, val, children }` / `make_test_tree(&file)` |
| `gohcl.DecodeBody(body, ctx, &target)` | `gohcl::decode_body::<T>(body, ctx) -> (T, Diagnostics)` |
| `gohcl.DecodeExpression(expr, ctx, &target)` | `gohcl::decode_expression::<T>(expr, ctx) -> (T, Diagnostics)` |
| `gohcl.ImpliedBodySchema(val)` | `gohcl::implied_body_schema::<T>() -> (BodySchema, bool)` |
| `gohcl.EncodeIntoBody/EncodeAsBlock` | `gohcl::encode_into_body/encode_as_block` |
| Go struct with `hcl:` tags | Rust struct with `#[derive(FromBody)]` (and/or `#[derive(EncodeBody)]`) + `#[hcl(...)]` attributes (grammar below) |
| `hcled.ContextString/ContextDefRange` | `hcled::context_string/context_def_range` |
| `hcltest.MockBody/MockExprLiteral/MockExprVariable/MockExprTraversal/MockExprTraversalSrc/MockExprList/MockAttrs` | `hcltest::mock_*` |
| `hclsimple.Decode/DecodeFile` (returns `error`) | `hclsimple::decode/decode_file::<T>() -> Result<T, Diagnostics>` |

## ext packages

| hcl v2 (Go) | rust-hcl |
|---|---|
| `dynblock.Expand(body, ctx, opts...)` | `dynblock::expand(body, ctx, vec![opts...])` (`vec![]` for none) |
| `dynblock.OptCheckForEach(f)` | `ExpandOption::CheckForEach(Arc::new(f))` |
| `dynblock.WalkVariables/WalkExpandVariables`, node/child types | `walk_variables/walk_expand_variables`, `WalkVariablesNode::visit`, `WalkVariablesChild { block_type_name, node }` |
| `dynblock.VariablesHCLDec/ExpandVariablesHCLDec` | `variables_hcldec/expand_variables_hcldec` |
| `typeexpr.Type(expr)` | `typeexpr::ty(expr)` |
| `typeexpr.TypeConstraint/TypeConstraintWithDefaults/TypeString` | `type_constraint/type_constraint_with_defaults/type_string` |
| `typeexpr.TypeConstraintType` (a var) | `type_constraint_type()` |
| `typeexpr.TypeConstraintVal/TypeConstraintFromVal/ConvertFunc` | `type_constraint_val/type_constraint_from_val/convert_func()` |
| `typeexpr.Defaults{Type, DefaultValues, Children}` | `Defaults { ty, default_values, children }` (children by value, not pointer) |
| `tryfunc.TryFunc/CanFunc` (vars) | `tryfunc::try_func()/can_func()` |
| `customdecode.ExpressionType/ExpressionClosureType` (vars) | `customdecode::expression_type()/expression_closure_type()` |
| `customdecode.ExpressionVal/FromVal`, closures | `expression_val/expression_from_val`, `ExpressionClosure { expression, eval_context }`, `expression_closure_val/from_val` |
| `customdecode.CustomExpressionDecoder` (capsule extension key) | `custom_expression_decoder_key()` |
| `transform.Shallow/Deep/Chain/NewErrorBody/BodyWithDiagnostics`, `Transformer(Func)` | same, snake_case; `TransformerFunc(Arc<dyn Fn(BodyRef) -> BodyRef>)` |
| `userfunc.DecodeUserFunctions(body, type, contextFunc)` | `userfunc::decode_user_functions(body, block_type, Option<ContextFunc>)` |

## The `#[hcl(...)]` attribute grammar (gohcl derive)

`hcl-derive` provides `#[derive(FromBody)]` and `#[derive(EncodeBody)]`,
both accepting `#[hcl(...)]` on named struct fields. In the stub phase the
derives fully parse and validate this grammar but emit `todo!()` bodies.

| Go struct tag | Rust attribute | Meaning |
|---|---|---|
| `hcl:"name"` / `hcl:"name,attr"` | `#[hcl(attr = "name")]` | required attribute |
| `hcl:"name,optional"` | `#[hcl(attr = "name", optional)]` | optional attribute |
| `hcl:"type,block"` | `#[hcl(block = "type")]` | nested block(s); field type decides arity (`T`, `Option<T>`, `Vec<T>`) |
| `hcl:"name,label"` | `#[hcl(label = "name")]` | block label |
| `hcl:",remain"` | `#[hcl(remain)]` | leftover body (field of type `BodyRef` or `Attributes`) |
| `hcl:",body"` | `#[hcl(body)]` | the whole body (field of type `BodyRef`) |
| `hcl:",range"` | `#[hcl(range)]` | the body/block's range (field of type `Range`) |
| `hcl:"name,label_range"` | `#[hcl(label_range = "name")]` | the named label's range (field of type `Range`) |
| `hcl:",def_range"` | `#[hcl(def_range)]` | the block's definition range (field of type `Range`) |
| `hcl:",type_range"` | `#[hcl(type_range)]` | the block's type-name range (field of type `Range`) |
| `hcl:"name,attr_range"` | `#[hcl(attr_range = "name")]` | the named attribute's range (field of type `Range`) |
| `hcl:"name,attr_name_range"` | `#[hcl(attr_name_range = "name")]` | the named attribute's name range (field of type `Range`) |
| `hcl:"name,attr_value_range"` | `#[hcl(attr_value_range = "name")]` | the named attribute's value-expression range (field of type `Range`) |
| — | `#[hcl(optional)]` | shorthand for `#[hcl(attr = "<field name>", optional)]` |
| untagged field | no attribute | ignored, as gohcl ignores untagged fields |

Go field-type conventions carry over: `*T` → `Option<T>`, slices → `Vec<T>`,
`hcl.Body` → `BodyRef`, `hcl.Expression` → `ExprRef`,
`hcl.Attributes` → `Attributes`, `*hcl.Attribute` → `Option<Attribute>`.

## Conformance test conventions

Each test file under `tests/conformance/` starts with a header comment
naming the upstream file(s) it transcribes, the pinned commit SHA, and the
MPL-2.0 notice (the transcriptions are derivative works of MPL-2.0 code —
see `README.md`). Every test carries `#[ignore = "not yet implemented"]`
and a permalink comment of the exact shape:

```rust
// Ported from TestXxx:
// https://github.com/hashicorp/hcl/blob/6abbb088cdb82416d1b3d9fcbaab29534133567a/<path>#L<line-of-func-decl>
```

Table-driven Go tests become loops over case arrays preserving upstream
order and literal expected values, with the case index (and input, when
short) in assertion messages. Cases that cannot be expressed are kept in
place as `NOTE(port):` comments rather than silently dropped.
