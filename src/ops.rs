//! Operations on `cty` values with HCL-flavored diagnostics
//! (hcl v2: `ops.go`).

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::pos::Range;

/// The result of indexing `collection` with `key`, with any failure
/// reported as diagnostics against `src_range` (hcl: `hcl.Index`).
pub fn index(collection: &Value, key: &Value, src_range: Option<&Range>) -> (Value, Diagnostics) {
    todo!()
}

/// The value of the given attribute of `obj`, with any failure reported as
/// diagnostics against `src_range` (hcl: `hcl.GetAttr`).
pub fn get_attr(obj: &Value, attr_name: &str, src_range: Option<&Range>) -> (Value, Diagnostics) {
    todo!()
}

/// Applies a `cty` path to a value, with any failure reported as
/// diagnostics against `src_range` (hcl: `hcl.ApplyPath`).
pub fn apply_path(
    val: &Value,
    path: &cty::Path,
    src_range: Option<&Range>,
) -> (Value, Diagnostics) {
    todo!()
}
