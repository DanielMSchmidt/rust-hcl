//! Schemas describing what a caller expects in a body (hcl v2: `schema.go`).

/// A block type expected in a body, with its label names
/// (hcl: `hcl.BlockHeaderSchema`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockHeaderSchema {
    /// The block type name (hcl: `BlockHeaderSchema.Type`).
    pub block_type: String,
    /// The names of the labels the block type requires, in order
    /// (hcl: `BlockHeaderSchema.LabelNames`).
    pub label_names: Vec<String>,
}

/// An attribute expected in a body (hcl: `hcl.AttributeSchema`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AttributeSchema {
    /// The attribute name (hcl: `AttributeSchema.Name`).
    pub name: String,
    /// Whether omitting the attribute is an error
    /// (hcl: `AttributeSchema.Required`).
    pub required: bool,
}

/// The desired shallow structure of a body
/// (hcl: `hcl.BodySchema`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BodySchema {
    /// The expected attributes (hcl: `BodySchema.Attributes`).
    pub attributes: Vec<AttributeSchema>,
    /// The expected block types (hcl: `BodySchema.Blocks`).
    pub blocks: Vec<BlockHeaderSchema>,
}
