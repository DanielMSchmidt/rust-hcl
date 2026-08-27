//! Human-readable text rendering of diagnostics (hcl v2:
//! `diagnostic_text.go`).

use std::collections::HashMap;
use std::io;

use crate::diagnostic::{Diagnostic, DiagnosticWriter, Diagnostics};
use crate::structure::File;

/// A [`DiagnosticWriter`] that writes diagnostics as prose with source-code
/// snippets, wrapped to a given width and optionally colored
/// (hcl: `hcl.NewDiagnosticTextWriter`'s unexported `diagnosticTextWriter`).
pub struct DiagnosticTextWriter<W: io::Write> {
    _priv: std::marker::PhantomData<W>,
}

impl<W: io::Write> DiagnosticTextWriter<W> {
    /// A new text writer over `wr`, consulting `files` for source snippets;
    /// `width` is the wrap column (0 disables wrapping) and `color` enables
    /// terminal escapes (hcl: `hcl.NewDiagnosticTextWriter`).
    pub fn new(wr: W, files: HashMap<String, File>, width: usize, color: bool) -> Self {
        todo!()
    }
}

impl<W: io::Write> DiagnosticWriter for DiagnosticTextWriter<W> {
    fn write_diagnostic(&mut self, diag: &Diagnostic) -> io::Result<()> {
        todo!()
    }

    fn write_diagnostics(&mut self, diags: &Diagnostics) -> io::Result<()> {
        todo!()
    }
}
