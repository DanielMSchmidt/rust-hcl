//! Walking the native-syntax AST (hcl v2: `hclsyntax/walk.go`,
//! `hclsyntax/node.go`, `hclsyntax/variables.go`).

use std::collections::HashSet;

use crate::diagnostic::Diagnostics;
use crate::hclsyntax::expression::Expression;
use crate::hclsyntax::structure::{Attribute, Attributes, Block, Blocks, Body};
use crate::pos::Range;
use crate::traversal::Traversal;

/// A borrowed AST node handed to walkers (hclsyntax: the `hclsyntax.Node`
/// interface, implemented by bodies, attributes, blocks, and expressions).
#[derive(Debug, Clone, Copy)]
pub enum Node<'a> {
    /// (hclsyntax: `*hclsyntax.Body`).
    Body(&'a Body),
    /// (hclsyntax: `hclsyntax.Attributes`).
    Attributes(&'a Attributes),
    /// (hclsyntax: `*hclsyntax.Attribute`).
    Attribute(&'a Attribute),
    /// (hclsyntax: `hclsyntax.Blocks`).
    Blocks(&'a Blocks),
    /// (hclsyntax: `*hclsyntax.Block`).
    Block(&'a Block),
    /// Any expression node (hclsyntax: the expression types).
    Expr(&'a Expression),
    /// (hclsyntax: `hclsyntax.ChildScope`).
    ChildScope(&'a ChildScope),
}

impl Node<'_> {
    /// The node's source range (hclsyntax: `Node.Range`).
    pub fn range(&self) -> Range {
        todo!()
    }
}

/// A virtual node marking a child scope with local symbol names, emitted
/// while walking `for` expressions and splats
/// (hclsyntax: `hclsyntax.ChildScope`).
#[derive(Debug, Clone)]
pub struct ChildScope {
    /// The names defined locally in this scope
    /// (hclsyntax: `ChildScope.LocalNames`).
    pub local_names: HashSet<String>,
    /// The expression the scope covers (hclsyntax: `ChildScope.Expr`).
    pub expr: Expression,
}

/// A callback for [`visit_all`] (hclsyntax: `hclsyntax.VisitFunc`).
pub type VisitFunc<'a> = &'a mut dyn FnMut(Node<'_>) -> Diagnostics;

/// Calls the callback once for each node in the AST, in lexical order
/// (hclsyntax: `hclsyntax.VisitAll`).
pub fn visit_all(node: Node<'_>, f: VisitFunc<'_>) -> Diagnostics {
    todo!()
}

/// An object called into while walking an AST (hclsyntax:
/// `hclsyntax.Walker`).
pub trait Walker {
    /// Called on the way into each node (hclsyntax: `Walker.Enter`).
    fn enter(&mut self, node: Node<'_>) -> Diagnostics;

    /// Called on the way out of each node (hclsyntax: `Walker.Exit`).
    fn exit(&mut self, node: Node<'_>) -> Diagnostics;
}

/// Walks the AST, calling the walker's enter and exit methods around each
/// node's children (hclsyntax: `hclsyntax.Walk`).
pub fn walk(node: Node<'_>, w: &mut dyn Walker) -> Diagnostics {
    todo!()
}

/// The variables referenced by an expression, before evaluation
/// (hclsyntax: `hclsyntax.Variables`).
pub fn variables(expr: &Expression) -> Vec<Traversal> {
    todo!()
}
