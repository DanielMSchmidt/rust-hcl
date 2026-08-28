//! Interpreting expressions as type constraints (hcl v2: the
//! `ext/typeexpr` package).

use std::collections::HashMap;

use cty::function::Function;
use cty::{Type, Value};

use crate::diagnostic::Diagnostics;
use crate::structure::Expression;

/// Interprets the expression as an exact type, with no `any` keyword
/// allowed (typeexpr: `typeexpr.Type`).
pub fn ty(expr: &dyn Expression) -> (Type, Diagnostics) {
    todo!()
}

/// Interprets the expression as a type constraint, allowing the `any`
/// keyword (typeexpr: `typeexpr.TypeConstraint`).
pub fn type_constraint(expr: &dyn Expression) -> (Type, Diagnostics) {
    todo!()
}

/// Like [`type_constraint`], but also extracting `optional(..., default)`
/// default values (typeexpr: `typeexpr.TypeConstraintWithDefaults`).
pub fn type_constraint_with_defaults(
    expr: &dyn Expression,
) -> (Type, Option<Defaults>, Diagnostics) {
    todo!()
}

/// A string rendering of the type that would parse back to it
/// (typeexpr: `typeexpr.TypeString`).
pub fn type_string(ty: &Type) -> String {
    todo!()
}

/// The capsule type wrapping type constraints as values
/// (typeexpr: `typeexpr.TypeConstraintType`).
pub fn type_constraint_type() -> Type {
    todo!()
}

/// The given type constraint wrapped as a value of
/// [`type_constraint_type`] (typeexpr: `typeexpr.TypeConstraintVal`).
pub fn type_constraint_val(ty: Type) -> Value {
    todo!()
}

/// The type constraint wrapped by the given value
/// (typeexpr: `typeexpr.TypeConstraintFromVal`).
pub fn type_constraint_from_val(v: &Value) -> Type {
    todo!()
}

/// The `convert` function, taking a value and a type constraint
/// (typeexpr: `typeexpr.ConvertFunc`).
pub fn convert_func() -> Function {
    todo!()
}

/// Default values extracted from a type constraint with `optional(...)`
/// attributes (typeexpr: `typeexpr.Defaults`).
#[derive(Debug, Clone, PartialEq)]
pub struct Defaults {
    /// The type the defaults were derived from
    /// (typeexpr: `Defaults.Type`).
    pub ty: Type,
    /// Default values by attribute name
    /// (typeexpr: `Defaults.DefaultValues`).
    pub default_values: HashMap<String, Value>,
    /// Defaults for nested constructs, by attribute or element key
    /// (typeexpr: `Defaults.Children`).
    pub children: HashMap<String, Defaults>,
}

impl Defaults {
    /// The given value with defaults inserted for null or missing optional
    /// attributes (typeexpr: `Defaults.Apply`).
    pub fn apply(&self, val: &Value) -> Value {
        todo!()
    }
}
