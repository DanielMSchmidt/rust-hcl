//! Source positions and ranges (hcl v2: `pos.go`).

use std::fmt;

/// A single location in a source file (hcl: `hcl.Pos`).
///
/// `line` is 1-based; `column` is 1-based and counted in grapheme clusters
/// (characters as they appear visually), and `byte` is the 0-based byte
/// offset of the position. All three are kept byte-for-byte identical to the
/// Go implementation's values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos {
    /// 1-based source line (hcl: `Pos.Line`).
    pub line: usize,
    /// 1-based column in grapheme clusters (hcl: `Pos.Column`).
    pub column: usize,
    /// 0-based byte offset (hcl: `Pos.Byte`).
    pub byte: usize,
}

impl Pos {
    /// The position of the first character in a file: line 1, column 1,
    /// byte 0 (hcl: `hcl.InitialPos`).
    pub fn initial() -> Pos {
        Pos {
            line: 1,
            column: 1,
            byte: 0,
        }
    }
}

/// A span between two positions in a specific file (hcl: `hcl.Range`).
///
/// `start` is inclusive and `end` is exclusive.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Range {
    /// The name of the file the positions point into (hcl: `Range.Filename`).
    pub filename: String,
    /// The inclusive start of the range (hcl: `Range.Start`).
    pub start: Pos,
    /// The exclusive end of the range (hcl: `Range.End`).
    pub end: Pos,
}

impl Range {
    /// A range from the start of one range to the end of another
    /// (hcl: `hcl.RangeBetween`).
    pub fn between(start: Range, end: Range) -> Range {
        todo!()
    }

    /// The smallest range that contains both given ranges
    /// (hcl: `hcl.RangeOver`).
    pub fn over(a: Range, b: Range) -> Range {
        todo!()
    }

    /// Whether the receiver contains the given position: at or after start,
    /// before end (hcl: `Range.ContainsPos`).
    pub fn contains_pos(&self, pos: Pos) -> bool {
        todo!()
    }

    /// Whether the given byte offset is within the receiver
    /// (hcl: `Range.ContainsOffset`).
    pub fn contains_offset(&self, offset: usize) -> bool {
        todo!()
    }

    /// Whether the range covers zero characters (hcl: `Range.Empty`).
    pub fn empty(&self) -> bool {
        todo!()
    }

    /// Whether the byte offsets of this range can slice the given buffer
    /// (hcl: `Range.CanSliceBytes`).
    pub fn can_slice_bytes(&self, b: &[u8]) -> bool {
        todo!()
    }

    /// The sub-slice of the buffer that the range covers
    /// (hcl: `Range.SliceBytes`).
    pub fn slice_bytes<'a>(&self, b: &'a [u8]) -> &'a [u8] {
        todo!()
    }

    /// Whether the receiver and the other range have any characters in
    /// common (hcl: `Range.Overlaps`).
    pub fn overlaps(&self, other: &Range) -> bool {
        todo!()
    }

    /// The range covered by both the receiver and the other range; an empty
    /// range at the receiver's start if they do not overlap
    /// (hcl: `Range.Overlap`).
    pub fn overlap(&self, other: &Range) -> Range {
        todo!()
    }

    /// Splits the receiver into the parts before, overlapping, and after the
    /// other range (hcl: `Range.PartitionAround`).
    pub fn partition_around(&self, other: &Range) -> (Range, Range, Range) {
        todo!()
    }
}

/// The compact user-facing rendering, identical to Go's `Range.String`:
/// e.g. `file.tf:1,5-12` or `file.tf:1,5-3,2`.
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
