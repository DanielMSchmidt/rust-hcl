//! One-shot decoding of a configuration file into a struct (hcl v2: the
//! `hclsimple` package).

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::gohcl::FromBody;

/// Parses the given buffer — as native or JSON syntax depending on the
/// filename suffix — and decodes it into a new `T`
/// (hclsimple: `hclsimple.Decode`; the Go out-parameter becomes the return
/// value, and the Go `error` result is `Err(Diagnostics)`).
pub fn decode<T: FromBody>(
    filename: &str,
    src: &[u8],
    ctx: Option<&EvalContext>,
) -> Result<T, Diagnostics> {
    todo!()
}

/// Reads, parses, and decodes the given file into a new `T`
/// (hclsimple: `hclsimple.DecodeFile`).
pub fn decode_file<T: FromBody>(
    filename: &str,
    ctx: Option<&EvalContext>,
) -> Result<T, Diagnostics> {
    todo!()
}
