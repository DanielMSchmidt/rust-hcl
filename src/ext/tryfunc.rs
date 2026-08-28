//! The `try` and `can` functions (hcl v2: the `ext/tryfunc` package).

use cty::function::Function;

/// The `try(...)` function: the result of the first argument that succeeds
/// (tryfunc: `tryfunc.TryFunc`).
pub fn try_func() -> Function {
    todo!()
}

/// The `can(...)` function: whether its argument evaluates without error
/// (tryfunc: `tryfunc.CanFunc`).
pub fn can_func() -> Function {
    todo!()
}
