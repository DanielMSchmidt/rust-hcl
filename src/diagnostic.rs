//! Accumulating diagnostics (hcl v2: `diagnostic.go`,
//! `diagnostic_typeparams.go`).

use std::any::Any;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::eval_context::EvalContext;
use crate::pos::Range;
use crate::structure::ExprRef;

/// The severity of a [`Diagnostic`] (hcl: `hcl.DiagnosticSeverity`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiagnosticSeverity {
    /// The invalid zero value (hcl: `hcl.DiagInvalid`).
    #[default]
    Invalid,
    /// A problem that prevents further progress (hcl: `hcl.DiagError`).
    Error,
    /// A problem that warrants attention but does not prevent progress
    /// (hcl: `hcl.DiagWarning`).
    Warning,
}

/// Machine-readable extra information attached to a [`Diagnostic`]
/// (hcl: the `Diagnostic.Extra` field's informal contract, plus
/// `hcl.DiagnosticExtraUnwrapper`).
///
/// Recipients downcast via [`diagnostic_extra`], which walks the wrapper
/// chain exposed by [`DiagnosticExtra::unwrap_diagnostic_extra`].
pub trait DiagnosticExtra: Any + fmt::Debug {
    /// The wrapped inner extra value, if this value wraps another
    /// (hcl: `DiagnosticExtraUnwrapper.UnwrapDiagnosticExtra`).
    fn unwrap_diagnostic_extra(&self) -> Option<&dyn DiagnosticExtra> {
        None
    }

    /// This value as [`Any`], for downcasting (Go: implicit via
    /// `interface{}`).
    fn as_any(&self) -> &dyn Any;
}

/// Finds a `T` in the diagnostic's extra value, unwrapping nested extras as
/// needed; `T` may be a concrete extra type or one of the extra traits
/// (e.g. `dyn hclsyntax::FunctionCallDiagExtra`), matching Go's interface
/// query (hcl: `hcl.DiagnosticExtra[T]`).
pub fn diagnostic_extra<T: ?Sized + 'static>(diag: &Diagnostic) -> Option<&T> {
    todo!()
}

/// Information to present to a user about a problem in configuration
/// (hcl: `hcl.Diagnostic`).
#[derive(Debug, Clone, Default)]
pub struct Diagnostic {
    /// How severe the problem is (hcl: `Diagnostic.Severity`).
    pub severity: DiagnosticSeverity,
    /// Terse English-language description of the general problem
    /// (hcl: `Diagnostic.Summary`).
    pub summary: String,
    /// Elaborate English-language description of the problem
    /// (hcl: `Diagnostic.Detail`).
    pub detail: String,
    /// Tight source range for the problematic construct
    /// (hcl: `Diagnostic.Subject`; `nil` ⇒ `None`).
    pub subject: Option<Range>,
    /// Optional broader source range containing `subject`
    /// (hcl: `Diagnostic.Context`; `nil` ⇒ `None`).
    pub context: Option<Range>,
    /// For expression evaluation problems, the offending expression
    /// (hcl: `Diagnostic.Expression`; `nil` ⇒ `None`).
    pub expression: Option<ExprRef>,
    /// The evaluation context active when the problem occurred
    /// (hcl: `Diagnostic.EvalContext`; `nil` ⇒ `None`).
    pub eval_context: Option<Arc<EvalContext>>,
    /// Extension point for machine-readable extra information
    /// (hcl: `Diagnostic.Extra`; `nil` ⇒ `None`).
    pub extra: Option<Arc<dyn DiagnosticExtra>>,
}

/// Compares the user-facing data fields (severity, summary, detail, subject,
/// context). `expression`, `eval_context`, and `extra` are excluded: Go
/// tests never compare those by value.
impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Diagnostic {
    /// The error-string rendering, identical to Go's `Diagnostic.Error`:
    /// `<filename>:<line>,<column>: <summary>; <detail>` (or just
    /// `<summary>; <detail>` with no subject).
    pub fn error(&self) -> String {
        todo!()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// A list of [`Diagnostic`]s (hcl: `hcl.Diagnostics`).
///
/// Where Go's append-and-reassign idiom is `diags = diags.Append(d)` /
/// `diags = diags.Extend(more)`, the Rust methods mutate in place:
/// `diags.push(d)` / `diags.extend(more)`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    /// An empty diagnostics list (Go: `nil`).
    pub fn new() -> Diagnostics {
        Diagnostics(Vec::new())
    }

    /// Appends a single diagnostic (hcl: `Diagnostics.Append`).
    pub fn push(&mut self, diag: Diagnostic) {
        self.0.push(diag);
    }

    /// Appends all diagnostics from another list
    /// (hcl: `Diagnostics.Extend`).
    pub fn extend(&mut self, diags: Diagnostics) {
        self.0.extend(diags.0);
    }

    /// Whether the receiver contains any diagnostics of severity
    /// [`DiagnosticSeverity::Error`] (hcl: `Diagnostics.HasErrors`).
    pub fn has_errors(&self) -> bool {
        todo!()
    }

    /// The error diagnostics as individual error strings
    /// (hcl: `Diagnostics.Errs`, each rendered with `Diagnostic.Error`).
    pub fn errs(&self) -> Vec<String> {
        todo!()
    }
}

/// The error-string rendering, identical to Go's `Diagnostics.Error`:
/// `no diagnostics`, the single diagnostic's message, or
/// `<first>, and <n> other diagnostic(s)`.
impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Deref for Diagnostics {
    type Target = Vec<Diagnostic>;

    fn deref(&self) -> &Vec<Diagnostic> {
        &self.0
    }
}

impl DerefMut for Diagnostics {
    fn deref_mut(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.0
    }
}

impl IntoIterator for Diagnostics {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diagnostics {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<Diagnostic> for Diagnostics {
    fn from_iter<I: IntoIterator<Item = Diagnostic>>(iter: I) -> Diagnostics {
        Diagnostics(iter.into_iter().collect())
    }
}

impl From<Vec<Diagnostic>> for Diagnostics {
    fn from(v: Vec<Diagnostic>) -> Diagnostics {
        Diagnostics(v)
    }
}

/// A sink that can format and print diagnostics
/// (hcl: `hcl.DiagnosticWriter`).
pub trait DiagnosticWriter {
    /// Writes a single diagnostic (hcl: `DiagnosticWriter.WriteDiagnostic`).
    fn write_diagnostic(&mut self, diag: &Diagnostic) -> std::io::Result<()>;

    /// Writes all given diagnostics in sequence
    /// (hcl: `DiagnosticWriter.WriteDiagnostics`).
    fn write_diagnostics(&mut self, diags: &Diagnostics) -> std::io::Result<()>;
}
