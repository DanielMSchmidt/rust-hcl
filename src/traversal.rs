//! Traversals: sequences of variable-access operations
//! (hcl v2: `traversal.go`).

use std::fmt;
use std::ops::Deref;

use cty::Value;

use crate::diagnostic::Diagnostics;
use crate::eval_context::EvalContext;
use crate::pos::Range;

/// One step in a [`Traversal`] (hcl: `hcl.Traverser`, i.e. `TraverseRoot`,
/// `TraverseAttr`, `TraverseIndex`, or `TraverseSplat`).
#[derive(Debug, Clone)]
pub enum Traverser {
    /// The starting symbol of an absolute traversal
    /// (hcl: `hcl.TraverseRoot`).
    Root {
        /// The root variable name (hcl: `TraverseRoot.Name`).
        name: String,
        /// The step's source range (hcl: `TraverseRoot.SrcRange`).
        src_range: Range,
    },
    /// Accessing an attribute of an object (hcl: `hcl.TraverseAttr`).
    Attr {
        /// The attribute name (hcl: `TraverseAttr.Name`).
        name: String,
        /// The step's source range (hcl: `TraverseAttr.SrcRange`).
        src_range: Range,
    },
    /// Indexing into a collection with a key (hcl: `hcl.TraverseIndex`).
    Index {
        /// The index key (hcl: `TraverseIndex.Key`).
        key: Value,
        /// The step's source range (hcl: `TraverseIndex.SrcRange`).
        src_range: Range,
    },
    /// A splat, applying the rest of the traversal to each element
    /// (hcl: `hcl.TraverseSplat`).
    Splat {
        /// The traversal applied to each element
        /// (hcl: `TraverseSplat.Each`).
        each: Traversal,
        /// The step's source range (hcl: `TraverseSplat.SrcRange`).
        src_range: Range,
    },
}

impl Traverser {
    /// Applies this single step to a value
    /// (hcl: `Traverser.TraversalStep`).
    pub fn traversal_step(&self, val: &Value) -> (Value, Diagnostics) {
        todo!()
    }

    /// The step's source range (hcl: `Traverser.SourceRange`).
    pub fn source_range(&self) -> Range {
        todo!()
    }
}

impl PartialEq for Traverser {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// A description of traversing through a value through a series of
/// operations (hcl: `hcl.Traversal`).
///
/// Constructed as `Traversal(vec![Traverser::Root { .. }, ..])`, mirroring
/// Go's `hcl.Traversal{hcl.TraverseRoot{..}, ..}` composite literals.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Traversal(pub Vec<Traverser>);

impl Traversal {
    /// An absolute traversal extended by the steps of a relative one
    /// (hcl: `hcl.TraversalJoin`).
    pub fn join(abs: Traversal, rel: Traversal) -> Traversal {
        todo!()
    }

    /// Applies a relative traversal to the given value
    /// (hcl: `Traversal.TraverseRel`).
    pub fn traverse_rel(&self, val: &Value) -> (Value, Diagnostics) {
        todo!()
    }

    /// Applies an absolute traversal starting from the given context; `None`
    /// mirrors a nil `*hcl.EvalContext` (hcl: `Traversal.TraverseAbs`).
    pub fn traverse_abs(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        todo!()
    }

    /// Whether the receiver is a relative traversal
    /// (hcl: `Traversal.IsRelative`).
    pub fn is_relative(&self) -> bool {
        todo!()
    }

    /// Splits an absolute traversal into its root-only absolute part and the
    /// relative remainder (hcl: `Traversal.SimpleSplit`).
    pub fn simple_split(&self) -> TraversalSplit {
        todo!()
    }

    /// The root name of an absolute traversal
    /// (hcl: `Traversal.RootName`).
    pub fn root_name(&self) -> String {
        todo!()
    }

    /// The source range covering the whole traversal
    /// (hcl: `Traversal.SourceRange`).
    pub fn source_range(&self) -> Range {
        todo!()
    }
}

/// The user-facing rendering of a traversal, e.g. `foo.bar[0]`, identical to
/// the string built by the diagnostic text writer (Go: unexported
/// `diagnosticTextWriter.traversalStr`).
impl fmt::Display for Traversal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Deref for Traversal {
    type Target = Vec<Traverser>;

    fn deref(&self) -> &Vec<Traverser> {
        &self.0
    }
}

impl From<Vec<Traverser>> for Traversal {
    fn from(v: Vec<Traverser>) -> Traversal {
        Traversal(v)
    }
}

impl IntoIterator for Traversal {
    type Item = Traverser;
    type IntoIter = std::vec::IntoIter<Traverser>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// An absolute traversal broken into its absolute prefix and relative
/// remainder (hcl: `hcl.TraversalSplit`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraversalSplit {
    /// The absolute prefix (hcl: `TraversalSplit.Abs`).
    pub abs: Traversal,
    /// The relative remainder (hcl: `TraversalSplit.Rel`).
    pub rel: Traversal,
}

impl TraversalSplit {
    /// Traverses the absolute part only (hcl: `TraversalSplit.TraverseAbs`).
    pub fn traverse_abs(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        todo!()
    }

    /// Applies the relative part to the given value
    /// (hcl: `TraversalSplit.TraverseRel`).
    pub fn traverse_rel(&self, val: &Value) -> (Value, Diagnostics) {
        todo!()
    }

    /// Traverses the absolute part and then applies the relative part
    /// (hcl: `TraversalSplit.Traverse`).
    pub fn traverse(&self, ctx: Option<&EvalContext>) -> (Value, Diagnostics) {
        todo!()
    }

    /// Recombines the two parts into a single absolute traversal
    /// (hcl: `TraversalSplit.Join`).
    pub fn join(&self) -> Traversal {
        todo!()
    }

    /// The root name of the absolute part (hcl: `TraversalSplit.RootName`).
    pub fn root_name(&self) -> String {
        todo!()
    }
}
