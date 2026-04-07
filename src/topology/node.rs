use serde::{Deserialize, Serialize};

use crate::topology::node_id::NodeId;

/// Whether a node is a navigable container or a selectable leaf.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    /// A node that contains children (directory, schema, etc.).
    Container,
    /// A terminal node that can be opened for data exploration.
    Leaf,
}

impl NodeKind {
    /// Returns `true` if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf)
    }

    /// Returns `true` if this is a container node.
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Container)
    }
}

/// Domain model for a node in the navigation tree.
///
/// `Node` is the rich domain type used server-side.
/// Use `NodeView` for browser transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Unique hierarchical identifier.
    pub id: NodeId,
    /// Human-readable display name.
    pub label: String,
    /// Consumer-defined type tag (e.g. `"folder"`, `"document"`, `"user"`).
    pub node_type: String,
    /// Whether the node is a container or leaf.
    pub node_kind: NodeKind,
    /// Immediate children (populated for in-memory trees).
    pub children: Vec<Node>,
}

impl Node {
    /// Construct a container node with no children.
    pub fn container(id: NodeId, label: String, node_type: &str) -> Self {
        Self {
            id,
            label,
            node_type: node_type.to_owned(),
            node_kind: NodeKind::Container,
            children: Vec::new(),
        }
    }

    /// Construct a leaf node.
    pub fn leaf(id: NodeId, label: String, node_type: &str) -> Self {
        Self {
            id,
            label,
            node_type: node_type.to_owned(),
            node_kind: NodeKind::Leaf,
            children: Vec::new(),
        }
    }

    /// Convenience: root-level container (wraps `container`).
    pub fn root(id: NodeId, label: String, node_type: &str) -> Self {
        Self::container(id, label, node_type)
    }

    /// Builder: add a child node and return `self`.
    #[must_use]
    pub fn with_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// Builder: add multiple children.
    #[must_use]
    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children.extend(children);
        self
    }

    /// Total node count in this subtree (self + all descendants).
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(Node::count).sum::<usize>()
    }

    /// All leaf nodes in this subtree (depth-first).
    pub fn leaf_nodes(&self) -> Vec<&Node> {
        if self.node_kind.is_leaf() {
            return vec![self];
        }
        self.children.iter().flat_map(Node::leaf_nodes).collect()
    }

    /// Find a node by `NodeId` in this subtree (depth-first).
    pub fn find(&self, id: &NodeId) -> Option<&Node> {
        if &self.id == id {
            return Some(self);
        }
        self.children.iter().find_map(|c| c.find(id))
    }

    /// Convert to the wire-format view (non-recursive — children must be
    /// fetched lazily via `TopologyProvider`).
    pub fn to_view(&self) -> NodeView {
        NodeView {
            id: self.id.canonical(),
            label: self.label.clone(),
            node_type: self.node_type.clone(),
            node_kind: match self.node_kind {
                NodeKind::Container => "container".to_owned(),
                NodeKind::Leaf => "leaf".to_owned(),
            },
            children: Vec::new(),
        }
    }
}

/// Flat, serialisable node representation for browser transfer.
///
/// Children are populated lazily by the UI — `Vec` is empty for most
/// nodes and filled only when the UI requests children for a specific node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeView {
    /// Canonical string form of `NodeId`.
    pub id: String,
    /// Human-readable display name.
    pub label: String,
    /// Consumer-defined type tag — matches `Node::node_type`.
    pub node_type: String,
    /// `"container"` or `"leaf"` — kept as string for easy JS interop.
    pub node_kind: String,
    /// Lazily populated child nodes.
    pub children: Vec<NodeView>,
}

impl NodeView {
    /// Returns `true` if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.node_kind == "leaf"
    }

    /// Returns `true` if this is a container node.
    pub fn is_container(&self) -> bool {
        self.node_kind == "container"
    }
}

impl From<Node> for NodeView {
    fn from(node: Node) -> Self {
        node.to_view()
    }
}
