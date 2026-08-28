//! Generating write-tokens from values and traversals (hcl v2:
//! `hclwrite/generate.go`).

use cty::Value;

use crate::hclwrite::Tokens;
use crate::traversal::Traversal;

/// Tokens rendering the given constant value
/// (hclwrite: `hclwrite.TokensForValue`).
pub fn tokens_for_value(val: &Value) -> Tokens {
    todo!()
}

/// Tokens rendering the given traversal
/// (hclwrite: `hclwrite.TokensForTraversal`).
pub fn tokens_for_traversal(traversal: &Traversal) -> Tokens {
    todo!()
}

/// Tokens rendering the given name as an identifier
/// (hclwrite: `hclwrite.TokensForIdentifier`).
pub fn tokens_for_identifier(name: &str) -> Tokens {
    todo!()
}

/// Tokens rendering a tuple constructor with the given element renderings
/// (hclwrite: `hclwrite.TokensForTuple`).
pub fn tokens_for_tuple(elems: Vec<Tokens>) -> Tokens {
    todo!()
}

/// A name–value pair of token renderings for [`tokens_for_object`]
/// (hclwrite: `hclwrite.ObjectAttrTokens`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAttrTokens {
    /// Tokens rendering the attribute name
    /// (hclwrite: `ObjectAttrTokens.Name`).
    pub name: Tokens,
    /// Tokens rendering the attribute value
    /// (hclwrite: `ObjectAttrTokens.Value`).
    pub value: Tokens,
}

/// Tokens rendering an object constructor with the given attribute
/// renderings (hclwrite: `hclwrite.TokensForObject`).
pub fn tokens_for_object(attrs: Vec<ObjectAttrTokens>) -> Tokens {
    todo!()
}

/// Tokens rendering a call to the named function with the given argument
/// renderings (hclwrite: `hclwrite.TokensForFunctionCall`).
pub fn tokens_for_function_call(func_name: &str, args: Vec<Tokens>) -> Tokens {
    todo!()
}
