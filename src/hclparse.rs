//! A parser with a file cache for diagnostics (hcl v2: the `hclparse`
//! package).

use std::collections::HashMap;

use crate::diagnostic::Diagnostics;
use crate::structure::File;

/// The main interface for parsing files, remembering their sources so
/// diagnostics can quote them (hclparse: `hclparse.Parser`).
#[derive(Debug, Default)]
pub struct Parser {
    _priv: (),
}

impl Parser {
    /// A new parser with an empty cache (hclparse: `hclparse.NewParser`).
    pub fn new() -> Parser {
        todo!()
    }

    /// Parses the given buffer as native syntax
    /// (hclparse: `Parser.ParseHCL`).
    pub fn parse_hcl(&mut self, src: &[u8], filename: &str) -> (File, Diagnostics) {
        todo!()
    }

    /// Reads and parses the given file as native syntax
    /// (hclparse: `Parser.ParseHCLFile`).
    pub fn parse_hcl_file(&mut self, filename: &str) -> (File, Diagnostics) {
        todo!()
    }

    /// Parses the given buffer as JSON syntax
    /// (hclparse: `Parser.ParseJSON`).
    pub fn parse_json(&mut self, src: &[u8], filename: &str) -> (File, Diagnostics) {
        todo!()
    }

    /// Reads and parses the given file as JSON syntax
    /// (hclparse: `Parser.ParseJSONFile`).
    pub fn parse_json_file(&mut self, filename: &str) -> (File, Diagnostics) {
        todo!()
    }

    /// Adds a file directly to the cache, for files parsed elsewhere
    /// (hclparse: `Parser.AddFile`).
    pub fn add_file(&mut self, filename: &str, file: File) {
        todo!()
    }

    /// The source buffers of all files parsed so far
    /// (hclparse: `Parser.Sources`).
    pub fn sources(&self) -> HashMap<String, Vec<u8>> {
        todo!()
    }

    /// All files parsed so far (hclparse: `Parser.Files`).
    pub fn files(&self) -> HashMap<String, File> {
        todo!()
    }
}
