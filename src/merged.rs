//! Merging multiple bodies into one (hcl v2: `merged.go`).

use crate::structure::{BodyRef, File};

/// A body with the contents of all the given files' bodies
/// (hcl: `hcl.MergeFiles`).
pub fn merge_files(files: Vec<File>) -> BodyRef {
    todo!()
}

/// A body with the contents of all the given bodies
/// (hcl: `hcl.MergeBodies`).
pub fn merge_bodies(bodies: Vec<BodyRef>) -> BodyRef {
    todo!()
}

/// A body with no content at all (hcl: `hcl.EmptyBody`).
pub fn empty_body() -> BodyRef {
    todo!()
}
