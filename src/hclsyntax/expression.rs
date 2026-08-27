//! The native-syntax expression AST (hcl v2: `hclsyntax/expression.go`,
//! `expression_ops.go`, `expression_template.go`, `expression_vars.go`).
//!
//! Go models each node as a struct implementing the `hclsyntax.Expression`
//! interface; here the nodes are the same structs gathered into the
//! [`Expression`] enum so parse results can be compared structurally, as the
//! upstream tests do with `deep.Equal`. Each node struct converts into the
//! enum via `From`, so `&hclsyntax.LiteralValueExpr{..}` ports as
//! `LiteralValueExpr { .. }.into()`.

use std::any::Any;
use std::sync::Arc;

use cty::function::Function;
use cty::{Type, Value};

use crate::diagnostic::{DiagnosticExtra, Diagnostics};
use crate::eval_context::EvalContext;
use crate::expr_helpers::{KeyValuePair, StaticCall};
use crate::pos::Range;
use crate::structure::ExprRef;
use crate::traversal::Traversal;

/// A native-syntax expression (hclsyntax: `hclsyntax.Expression`).
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// (hclsyntax: `*hclsyntax.LiteralValueExpr`).
    LiteralValue(LiteralValueExpr),
    /// (hclsyntax: `*hclsyntax.ScopeTraversalExpr`).
    ScopeTraversal(ScopeTraversalExpr),
    /// (hclsyntax: `*hclsyntax.RelativeTraversalExpr`).
    RelativeTraversal(Box<RelativeTraversalExpr>),
    /// (hclsyntax: `*hclsyntax.FunctionCallExpr`).
    FunctionCall(Box<FunctionCallExpr>),
    /// (hclsyntax: `*hclsyntax.ConditionalExpr`).
    Conditional(Box<ConditionalExpr>),
    /// (hclsyntax: `*hclsyntax.IndexExpr`).
    Index(Box<IndexExpr>),
    /// (hclsyntax: `*hclsyntax.TupleConsExpr`).
    TupleCons(TupleConsExpr),
    /// (hclsyntax: `*hclsyntax.ObjectConsExpr`).
    ObjectCons(ObjectConsExpr),
    /// (hclsyntax: `*hclsyntax.ObjectConsKeyExpr`).
    ObjectConsKey(Box<ObjectConsKeyExpr>),
    /// (hclsyntax: `*hclsyntax.ForExpr`).
    For(Box<ForExpr>),
    /// (hclsyntax: `*hclsyntax.SplatExpr`).
    Splat(Box<SplatExpr>),
    /// (hclsyntax: `*hclsyntax.AnonSymbolExpr`). Shared between a
    /// [`SplatExpr`] and its `each` expression, hence the `Arc`.
    AnonSymbol(Arc<AnonSymbolExpr>),
    /// (hclsyntax: `*hclsyntax.BinaryOpExpr`).
    BinaryOp(Box<BinaryOpExpr>),
    /// (hclsyntax: `*hclsyntax.UnaryOpExpr`).
    UnaryOp(Box<UnaryOpExpr>),
    /// (hclsyntax: `*hclsyntax.TemplateExpr`).
    Template(TemplateExpr),
    /// (hclsyntax: `*hclsyntax.TemplateJoinExpr`).
    TemplateJoin(Box<TemplateJoinExpr>),
    /// (hclsyntax: `*hclsyntax.TemplateWrapExpr`).
    TemplateWrap(Box<TemplateWrapExpr>),
    /// (hclsyntax: `*hclsyntax.ParenthesesExpr`).
    Parentheses(Box<ParenthesesExpr>),
    /// (hclsyntax: `*hclsyntax.ExprSyntaxError`).
    SyntaxError(ExprSyntaxError),
}

impl Expression {
    /// The value of the expression in the given context; `None` mirrors a
    /// nil `*hcl.EvalContext` (hclsyntax: `Expression.Value`).
    pub fn value(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        todo!()
    }

    /// The variables needed to evaluate the expression
    /// (hclsyntax: `Expression.Variables`).
    pub fn variables(&self) -> Vec<Traversal> {
        todo!()
    }

    /// The range of the whole expression (hclsyntax: `Expression.Range`).
    pub fn range(&self) -> Range {
        todo!()
    }

    /// A shorter range at the start of the expression
    /// (hclsyntax: `Expression.StartRange`).
    pub fn start_range(&self) -> Range {
        todo!()
    }

    /// The expression as an absolute traversal, if it is one
    /// (hclsyntax: the `AsTraversal` methods behind
    /// `hcl.AbsTraversalForExpr`).
    pub fn as_traversal(&self) -> Option<Traversal> {
        todo!()
    }

    /// The expression as a static function call, if it is one
    /// (hclsyntax: `FunctionCallExpr.ExprCall`).
    pub fn expr_call(&self) -> Option<StaticCall> {
        todo!()
    }

    /// The expression as a static list, if it is one
    /// (hclsyntax: `TupleConsExpr.ExprList`).
    pub fn expr_list(&self) -> Option<Vec<ExprRef>> {
        todo!()
    }

    /// The expression as static key–value pairs, if it is one
    /// (hclsyntax: `ObjectConsExpr.ExprMap`).
    pub fn expr_map(&self) -> Option<Vec<KeyValuePair>> {
        todo!()
    }

    /// This expression wrapped in a shared [`ExprRef`] handle, for calling
    /// the `hcl`-level helpers (Go: implicit interface conversion).
    pub fn into_expr_ref(self) -> ExprRef {
        ExprRef::new(self)
    }
}

impl crate::structure::Expression for Expression {
    fn value(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        Expression::value(self, ctx)
    }

    fn variables(&self) -> Vec<Traversal> {
        Expression::variables(self)
    }

    fn range(&self) -> Range {
        Expression::range(self)
    }

    fn start_range(&self) -> Range {
        Expression::start_range(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_traversal(&self) -> Option<Traversal> {
        Expression::as_traversal(self)
    }

    fn expr_call(&self) -> Option<StaticCall> {
        Expression::expr_call(self)
    }

    fn expr_list(&self) -> Option<Vec<ExprRef>> {
        Expression::expr_list(self)
    }

    fn expr_map(&self) -> Option<Vec<KeyValuePair>> {
        Expression::expr_map(self)
    }
}

/// An expression that was written in parentheses
/// (hclsyntax: `hclsyntax.ParenthesesExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesesExpr {
    /// The wrapped expression (hclsyntax: the embedded `Expression`).
    pub expression: Expression,
    /// The source range including the parentheses
    /// (hclsyntax: `ParenthesesExpr.SrcRange`).
    pub src_range: Range,
}

/// A literal value (hclsyntax: `hclsyntax.LiteralValueExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralValueExpr {
    /// The value (hclsyntax: `LiteralValueExpr.Val`).
    pub val: Value,
    /// The source range (hclsyntax: `LiteralValueExpr.SrcRange`).
    pub src_range: Range,
}

/// An absolute traversal from the evaluation scope
/// (hclsyntax: `hclsyntax.ScopeTraversalExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ScopeTraversalExpr {
    /// The traversal (hclsyntax: `ScopeTraversalExpr.Traversal`).
    pub traversal: Traversal,
    /// The source range (hclsyntax: `ScopeTraversalExpr.SrcRange`).
    pub src_range: Range,
}

/// A traversal applied to the result of another expression
/// (hclsyntax: `hclsyntax.RelativeTraversalExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct RelativeTraversalExpr {
    /// The expression being traversed
    /// (hclsyntax: `RelativeTraversalExpr.Source`).
    pub source: Expression,
    /// The relative traversal
    /// (hclsyntax: `RelativeTraversalExpr.Traversal`).
    pub traversal: Traversal,
    /// The source range (hclsyntax: `RelativeTraversalExpr.SrcRange`).
    pub src_range: Range,
}

/// A function call (hclsyntax: `hclsyntax.FunctionCallExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpr {
    /// The function name, including any namespace prefix
    /// (hclsyntax: `FunctionCallExpr.Name`).
    pub name: String,
    /// The argument expressions (hclsyntax: `FunctionCallExpr.Args`).
    pub args: Vec<Expression>,
    /// Whether the final argument carries `...` expansion
    /// (hclsyntax: `FunctionCallExpr.ExpandFinal`).
    pub expand_final: bool,
    /// The range of the function name
    /// (hclsyntax: `FunctionCallExpr.NameRange`).
    pub name_range: Range,
    /// The range of the opening parenthesis
    /// (hclsyntax: `FunctionCallExpr.OpenParenRange`).
    pub open_paren_range: Range,
    /// The range of the closing parenthesis
    /// (hclsyntax: `FunctionCallExpr.CloseParenRange`).
    pub close_paren_range: Range,
}

/// Extra information attached to diagnostics from evaluating a function
/// call (hclsyntax: `hclsyntax.FunctionCallDiagExtra`).
pub trait FunctionCallDiagExtra: DiagnosticExtra {
    /// The name of the function that was being called
    /// (hclsyntax: `FunctionCallDiagExtra.CalledFunctionName`).
    fn called_function_name(&self) -> String;

    /// The error returned by the function itself, if the call failed
    /// during execution; the error's `Display` matches Go's `err.Error()`
    /// (hclsyntax: `FunctionCallDiagExtra.FunctionCallError`).
    fn function_call_error(&self) -> Option<cty::Error>;
}

/// Extra information attached to diagnostics reporting a call to an unknown
/// function (hclsyntax: `hclsyntax.FunctionCallUnknownDiagExtra`).
pub trait FunctionCallUnknownDiagExtra: DiagnosticExtra {
    /// The local name of the function that was called
    /// (hclsyntax: `FunctionCallUnknownDiagExtra.CalledFunctionName`).
    fn called_function_name(&self) -> String;

    /// The namespace prefix of the call, empty for the global namespace
    /// (hclsyntax: `FunctionCallUnknownDiagExtra.CalledFunctionNamespace`).
    fn called_function_namespace(&self) -> String;
}

/// A ternary conditional (hclsyntax: `hclsyntax.ConditionalExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalExpr {
    /// The condition (hclsyntax: `ConditionalExpr.Condition`).
    pub condition: Expression,
    /// The result when the condition is true
    /// (hclsyntax: `ConditionalExpr.TrueResult`).
    pub true_result: Expression,
    /// The result when the condition is false
    /// (hclsyntax: `ConditionalExpr.FalseResult`).
    pub false_result: Expression,
    /// The source range (hclsyntax: `ConditionalExpr.SrcRange`).
    pub src_range: Range,
}

/// An index operation `collection[key]` (hclsyntax: `hclsyntax.IndexExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    /// The collection being indexed (hclsyntax: `IndexExpr.Collection`).
    pub collection: Expression,
    /// The key expression (hclsyntax: `IndexExpr.Key`).
    pub key: Expression,
    /// The source range of the whole operation
    /// (hclsyntax: `IndexExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the collection expression
    /// (hclsyntax: `IndexExpr.OpenRange`).
    pub open_range: Range,
    /// The range of the brackets and key
    /// (hclsyntax: `IndexExpr.BracketRange`).
    pub bracket_range: Range,
}

/// A tuple constructor `[...]` (hclsyntax: `hclsyntax.TupleConsExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct TupleConsExpr {
    /// The element expressions (hclsyntax: `TupleConsExpr.Exprs`).
    pub exprs: Vec<Expression>,
    /// The source range (hclsyntax: `TupleConsExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the opening bracket
    /// (hclsyntax: `TupleConsExpr.OpenRange`).
    pub open_range: Range,
}

/// An object constructor `{...}` (hclsyntax: `hclsyntax.ObjectConsExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectConsExpr {
    /// The key–value items (hclsyntax: `ObjectConsExpr.Items`).
    pub items: Vec<ObjectConsItem>,
    /// The source range (hclsyntax: `ObjectConsExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the opening brace
    /// (hclsyntax: `ObjectConsExpr.OpenRange`).
    pub open_range: Range,
}

/// One item in an [`ObjectConsExpr`]
/// (hclsyntax: `hclsyntax.ObjectConsItem`).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectConsItem {
    /// The key expression (hclsyntax: `ObjectConsItem.KeyExpr`).
    pub key_expr: Expression,
    /// The value expression (hclsyntax: `ObjectConsItem.ValueExpr`).
    pub value_expr: Expression,
}

/// The key of an object-constructor item, handling the "naked identifier as
/// string" rule (hclsyntax: `hclsyntax.ObjectConsKeyExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectConsKeyExpr {
    /// The wrapped key expression
    /// (hclsyntax: `ObjectConsKeyExpr.Wrapped`).
    pub wrapped: Expression,
    /// Whether to disable the literal-name interpretation
    /// (hclsyntax: `ObjectConsKeyExpr.ForceNonLiteral`).
    pub force_non_literal: bool,
}

/// A `for` expression (hclsyntax: `hclsyntax.ForExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct ForExpr {
    /// The key variable name; empty if the key is ignored
    /// (hclsyntax: `ForExpr.KeyVar`).
    pub key_var: String,
    /// The value variable name (hclsyntax: `ForExpr.ValVar`).
    pub val_var: String,
    /// The collection being iterated (hclsyntax: `ForExpr.CollExpr`).
    pub coll_expr: Expression,
    /// The result key expression; `None` when producing a tuple
    /// (hclsyntax: `ForExpr.KeyExpr`, `nil` ⇒ `None`).
    pub key_expr: Option<Expression>,
    /// The result value expression (hclsyntax: `ForExpr.ValExpr`).
    pub val_expr: Expression,
    /// The `if` clause condition; `None` when absent
    /// (hclsyntax: `ForExpr.CondExpr`, `nil` ⇒ `None`).
    pub cond_expr: Option<Expression>,
    /// Whether the value has the `...` grouping ellipsis
    /// (hclsyntax: `ForExpr.Group`).
    pub group: bool,
    /// The source range (hclsyntax: `ForExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the opening bracket/brace
    /// (hclsyntax: `ForExpr.OpenRange`).
    pub open_range: Range,
    /// The range of the closing bracket/brace
    /// (hclsyntax: `ForExpr.CloseRange`).
    pub close_range: Range,
}

/// A splat expression `expr.*` / `expr[*]`
/// (hclsyntax: `hclsyntax.SplatExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct SplatExpr {
    /// The expression being splatted (hclsyntax: `SplatExpr.Source`).
    pub source: Expression,
    /// The per-element expression, which refers to `item`
    /// (hclsyntax: `SplatExpr.Each`).
    pub each: Expression,
    /// The anonymous symbol standing for the current element; shared with
    /// occurrences inside `each` (hclsyntax: `SplatExpr.Item`).
    pub item: Arc<AnonSymbolExpr>,
    /// The source range (hclsyntax: `SplatExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the `*` marker (hclsyntax: `SplatExpr.MarkerRange`).
    pub marker_range: Range,
}

/// An anonymous symbol carrying per-context values during splat evaluation
/// (hclsyntax: `hclsyntax.AnonSymbolExpr`).
#[derive(Debug, Default)]
pub struct AnonSymbolExpr {
    /// The source range (hclsyntax: `AnonSymbolExpr.SrcRange`).
    pub src_range: Range,
}

/// Compares only the source range; the per-context evaluation state is
/// transient (Go: unexported `values` map, ignored by `deep.Equal`).
impl PartialEq for AnonSymbolExpr {
    fn eq(&self, other: &Self) -> bool {
        self.src_range == other.src_range
    }
}

/// A placeholder for an expression that failed to parse, evaluated
/// gracefully (hclsyntax: `hclsyntax.ExprSyntaxError`).
#[derive(Debug, Clone, PartialEq)]
pub struct ExprSyntaxError {
    /// The placeholder value to produce
    /// (hclsyntax: `ExprSyntaxError.Placeholder`).
    pub placeholder: Value,
    /// The parse diagnostics that led here
    /// (hclsyntax: `ExprSyntaxError.ParseDiags`).
    pub parse_diags: Diagnostics,
    /// The source range (hclsyntax: `ExprSyntaxError.SrcRange`).
    pub src_range: Range,
}

/// An operation to apply to operand values (hclsyntax: the pointers to the
/// package-level `hclsyntax.Operation` values; `OpAdd` → `Operation::Add`,
/// etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// `||` (hclsyntax: `OpLogicalOr`).
    LogicalOr,
    /// `&&` (hclsyntax: `OpLogicalAnd`).
    LogicalAnd,
    /// `!` (hclsyntax: `OpLogicalNot`).
    LogicalNot,
    /// `==` (hclsyntax: `OpEqual`).
    Equal,
    /// `!=` (hclsyntax: `OpNotEqual`).
    NotEqual,
    /// `>` (hclsyntax: `OpGreaterThan`).
    GreaterThan,
    /// `>=` (hclsyntax: `OpGreaterThanOrEqual`).
    GreaterThanOrEqual,
    /// `<` (hclsyntax: `OpLessThan`).
    LessThan,
    /// `<=` (hclsyntax: `OpLessThanOrEqual`).
    LessThanOrEqual,
    /// `+` (hclsyntax: `OpAdd`).
    Add,
    /// binary `-` (hclsyntax: `OpSubtract`).
    Subtract,
    /// `*` (hclsyntax: `OpMultiply`).
    Multiply,
    /// `/` (hclsyntax: `OpDivide`).
    Divide,
    /// `%` (hclsyntax: `OpModulo`).
    Modulo,
    /// unary `-` (hclsyntax: `OpNegate`).
    Negate,
}

impl Operation {
    /// The function implementing the operation
    /// (hclsyntax: `Operation.Impl`).
    pub fn impl_fn(&self) -> Function {
        todo!()
    }

    /// The operation's result type constraint
    /// (hclsyntax: `Operation.Type`).
    pub fn result_type(&self) -> Type {
        todo!()
    }
}

/// A binary operator application (hclsyntax: `hclsyntax.BinaryOpExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOpExpr {
    /// The left operand (hclsyntax: `BinaryOpExpr.LHS`).
    pub lhs: Expression,
    /// The operation (hclsyntax: `BinaryOpExpr.Op`).
    pub op: Operation,
    /// The right operand (hclsyntax: `BinaryOpExpr.RHS`).
    pub rhs: Expression,
    /// The source range (hclsyntax: `BinaryOpExpr.SrcRange`).
    pub src_range: Range,
}

/// A unary operator application (hclsyntax: `hclsyntax.UnaryOpExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpExpr {
    /// The operation (hclsyntax: `UnaryOpExpr.Op`).
    pub op: Operation,
    /// The operand (hclsyntax: `UnaryOpExpr.Val`).
    pub val: Expression,
    /// The source range (hclsyntax: `UnaryOpExpr.SrcRange`).
    pub src_range: Range,
    /// The range of the operator symbol
    /// (hclsyntax: `UnaryOpExpr.SymbolRange`).
    pub symbol_range: Range,
}

/// A string template, possibly with interpolations and directives
/// (hclsyntax: `hclsyntax.TemplateExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateExpr {
    /// The template's literal and interpolated parts, in order
    /// (hclsyntax: `TemplateExpr.Parts`).
    pub parts: Vec<Expression>,
    /// The source range (hclsyntax: `TemplateExpr.SrcRange`).
    pub src_range: Range,
}

impl TemplateExpr {
    /// Whether the template consists only of a single string literal
    /// (hclsyntax: `TemplateExpr.IsStringLiteral`).
    pub fn is_string_literal(&self) -> bool {
        todo!()
    }
}

/// Joining the results of a tuple of templates, from `for` directives in
/// templates (hclsyntax: `hclsyntax.TemplateJoinExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateJoinExpr {
    /// The tuple of results to join (hclsyntax: `TemplateJoinExpr.Tuple`).
    pub tuple: Expression,
}

/// A template consisting of a single interpolation, delegating to the
/// wrapped expression (hclsyntax: `hclsyntax.TemplateWrapExpr`).
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateWrapExpr {
    /// The wrapped expression (hclsyntax: `TemplateWrapExpr.Wrapped`).
    pub wrapped: Expression,
    /// The source range (hclsyntax: `TemplateWrapExpr.SrcRange`).
    pub src_range: Range,
}

impl From<LiteralValueExpr> for Expression {
    fn from(e: LiteralValueExpr) -> Expression {
        Expression::LiteralValue(e)
    }
}

impl From<ScopeTraversalExpr> for Expression {
    fn from(e: ScopeTraversalExpr) -> Expression {
        Expression::ScopeTraversal(e)
    }
}

impl From<TupleConsExpr> for Expression {
    fn from(e: TupleConsExpr) -> Expression {
        Expression::TupleCons(e)
    }
}

impl From<ObjectConsExpr> for Expression {
    fn from(e: ObjectConsExpr) -> Expression {
        Expression::ObjectCons(e)
    }
}

impl From<TemplateExpr> for Expression {
    fn from(e: TemplateExpr) -> Expression {
        Expression::Template(e)
    }
}

impl From<ExprSyntaxError> for Expression {
    fn from(e: ExprSyntaxError) -> Expression {
        Expression::SyntaxError(e)
    }
}

impl From<AnonSymbolExpr> for Expression {
    fn from(e: AnonSymbolExpr) -> Expression {
        Expression::AnonSymbol(Arc::new(e))
    }
}

impl From<Arc<AnonSymbolExpr>> for Expression {
    fn from(e: Arc<AnonSymbolExpr>) -> Expression {
        Expression::AnonSymbol(e)
    }
}

impl From<RelativeTraversalExpr> for Expression {
    fn from(e: RelativeTraversalExpr) -> Expression {
        Expression::RelativeTraversal(Box::new(e))
    }
}

impl From<FunctionCallExpr> for Expression {
    fn from(e: FunctionCallExpr) -> Expression {
        Expression::FunctionCall(Box::new(e))
    }
}

impl From<ConditionalExpr> for Expression {
    fn from(e: ConditionalExpr) -> Expression {
        Expression::Conditional(Box::new(e))
    }
}

impl From<IndexExpr> for Expression {
    fn from(e: IndexExpr) -> Expression {
        Expression::Index(Box::new(e))
    }
}

impl From<ObjectConsKeyExpr> for Expression {
    fn from(e: ObjectConsKeyExpr) -> Expression {
        Expression::ObjectConsKey(Box::new(e))
    }
}

impl From<ForExpr> for Expression {
    fn from(e: ForExpr) -> Expression {
        Expression::For(Box::new(e))
    }
}

impl From<SplatExpr> for Expression {
    fn from(e: SplatExpr) -> Expression {
        Expression::Splat(Box::new(e))
    }
}

impl From<BinaryOpExpr> for Expression {
    fn from(e: BinaryOpExpr) -> Expression {
        Expression::BinaryOp(Box::new(e))
    }
}

impl From<UnaryOpExpr> for Expression {
    fn from(e: UnaryOpExpr) -> Expression {
        Expression::UnaryOp(Box::new(e))
    }
}

impl From<TemplateJoinExpr> for Expression {
    fn from(e: TemplateJoinExpr) -> Expression {
        Expression::TemplateJoin(Box::new(e))
    }
}

impl From<TemplateWrapExpr> for Expression {
    fn from(e: TemplateWrapExpr) -> Expression {
        Expression::TemplateWrap(Box::new(e))
    }
}

impl From<ParenthesesExpr> for Expression {
    fn from(e: ParenthesesExpr) -> Expression {
        Expression::Parentheses(Box::new(e))
    }
}
