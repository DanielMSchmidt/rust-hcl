//! Language-server-style helpers for HCL editor integrations (hcl v2: the
//! `hcled` package).

use crate::pos::Range;
use crate::structure::File;

/// A human-readable description of the context at the given byte offset,
/// e.g. which block the offset is inside (hcled: `hcled.ContextString`).
pub fn context_string(file: &File, offset: usize) -> String {
    todo!()
}

/// The definition range of the context at the given byte offset
/// (hcled: `hcled.ContextDefRange`).
pub fn context_def_range(file: &File, offset: usize) -> Range {
    todo!()
}
