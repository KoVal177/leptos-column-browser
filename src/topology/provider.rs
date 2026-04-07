use std::future::Future;

use crate::topology::node::Node;
use crate::topology::node_id::NodeId;

/// Errors returned by `TopologyProvider`.
#[derive(Debug, Clone, thiserror::Error)]
pub enum BrowserError {
    /// Failed to connect to the data source.
    #[error("connection failed: {0}")]
    Connection(String),

    /// The requested parent node was not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Any other error.
    #[error("{0}")]
    Other(String),
}

/// Port for tree navigation.
///
/// Implement this trait to connect any data source to the column browser.
/// The implementation is responsible for:
///
/// - Fetching the immediate children of any given `NodeId`.
/// - Returning `Ok(Vec::new())` for leaf nodes (never an error for "no children").
/// - Being `Clone` so the component can share the provider across async tasks.
///
/// # Example
///
/// ```rust
/// use leptos_column_browser::{TopologyProvider, BrowserError, Node, NodeId};
/// use std::future::Future;
///
/// #[derive(Clone)]
/// struct FlatProvider(Vec<Node>);
///
/// impl TopologyProvider for FlatProvider {
///     fn get_children(
///         &self,
///         _parent_id: &NodeId,
///     ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
///         let nodes = self.0.clone();
///         async move { Ok(nodes) }
///     }
/// }
/// ```
pub trait TopologyProvider: Send + Sync {
    /// Return the immediate children of `parent_id`.
    ///
    /// Return an empty `Vec` for leaf nodes (do not return an error).
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>>;
}

/// A no-op provider that always returns an empty child list.
///
/// Useful as a default or placeholder in tests and examples.
#[derive(Clone)]
pub struct NoopTopologyProvider;

impl TopologyProvider for NoopTopologyProvider {
    async fn get_children(&self, _parent_id: &NodeId) -> Result<Vec<Node>, BrowserError> {
        Ok(Vec::new())
    }
}

/// A static in-memory provider that walks a pre-built `Node` tree.
///
/// `get_children` returns the immediate children of `parent_id`
/// by traversing the root nodes. The root sentinel id (`"__root__"` by
/// convention) returns the top-level nodes directly.
#[derive(Clone)]
pub struct StaticTopologyProvider {
    /// Top-level nodes in the tree.
    pub nodes: Vec<Node>,
}

impl TopologyProvider for StaticTopologyProvider {
    async fn get_children(&self, parent_id: &NodeId) -> Result<Vec<Node>, BrowserError> {
        // Root sentinel — return top-level nodes.
        if parent_id.segments().len() == 1 && parent_id.canonical() == "__root__" {
            return Ok(self.nodes.clone());
        }
        // Walk the tree to find the parent, then return its children.
        for root in &self.nodes {
            if let Some(node) = root.find(parent_id) {
                return Ok(node.children.clone());
            }
        }
        Err(BrowserError::NotFound(parent_id.canonical()))
    }
}
