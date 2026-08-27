//! Grapheme-aware scanning of buffers into ranges (hcl v2: `pos_scanner.go`).

use crate::pos::{Pos, Range};

/// The result of one step of a split function: how many bytes to advance and
/// the token found, if any (Go: the `(advance, token, err)` triple returned
/// by `bufio.SplitFunc`, without the error, which HCL's scanner never uses).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitResult {
    /// Number of input bytes consumed by this step.
    pub advance: usize,
    /// The token to emit, if a complete token was found.
    pub token: Option<Vec<u8>>,
}

/// A split function deciding how a [`RangeScanner`] chunks its input
/// (Go: `bufio.SplitFunc`). Called with the remaining data and whether the
/// end of input has been reached.
pub type SplitFunc = fn(data: &[u8], at_eof: bool) -> SplitResult;

/// A split function that emits one line per token, stripping the trailing
/// newline and any preceding carriage return (Go: `bufio.ScanLines`).
pub fn scan_lines(data: &[u8], at_eof: bool) -> SplitResult {
    todo!()
}

/// Scans a buffer, providing both the tokens of an arbitrary split function
/// and the source [`Range`] of each token (hcl: `hcl.RangeScanner`).
pub struct RangeScanner {
    _priv: (),
}

impl RangeScanner {
    /// A scanner over the whole buffer, starting at [`Pos::initial`]
    /// (hcl: `hcl.NewRangeScanner`).
    pub fn new(b: &[u8], filename: &str, cb: SplitFunc) -> RangeScanner {
        todo!()
    }

    /// A scanner over a fragment of a file, whose first byte is at the given
    /// start position (hcl: `hcl.NewRangeScannerFragment`).
    pub fn new_fragment(b: &[u8], filename: &str, start: Pos, cb: SplitFunc) -> RangeScanner {
        todo!()
    }

    /// Advances to the next token, returning `false` at the end of input
    /// (hcl: `RangeScanner.Scan`).
    pub fn scan(&mut self) -> bool {
        todo!()
    }

    /// The source range of the current token (hcl: `RangeScanner.Range`).
    pub fn range(&self) -> Range {
        todo!()
    }

    /// The bytes of the current token (hcl: `RangeScanner.Bytes`).
    pub fn bytes(&self) -> &[u8] {
        todo!()
    }
}
