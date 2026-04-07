use serde::{Deserialize, Serialize};

use crate::topology::node::NodeView;

/// Declarative filter applied to a list of `NodeView`s.
///
/// Conditions are `ANDed`. An empty filter matches everything.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeFilter {
    /// Keep only nodes whose `node_type` is in this list.
    /// Empty = no type filtering.
    #[serde(default)]
    pub node_types: Vec<String>,

    /// Keep only nodes whose `label` contains this substring (case-insensitive).
    pub label_contains: Option<String>,

    /// Keep only leaf nodes (containers survive if they have surviving children).
    #[serde(default)]
    pub leaves_only: bool,
}

impl NodeFilter {
    /// A filter that matches everything.
    pub fn all() -> Self {
        Self::default()
    }

    /// Filter by a single node type.
    pub fn by_type(node_type: impl Into<String>) -> Self {
        Self {
            node_types: vec![node_type.into()],
            ..Self::default()
        }
    }

    /// Filter by multiple node types.
    pub fn by_types(types: Vec<String>) -> Self {
        Self {
            node_types: types,
            ..Self::default()
        }
    }

    /// True if this filter matches everything (no conditions set).
    pub fn is_empty(&self) -> bool {
        self.node_types.is_empty() && self.label_contains.is_none() && !self.leaves_only
    }

    /// Apply the filter to a list of nodes, returning a new list.
    ///
    /// Container nodes survive if at least one descendant passes all conditions.
    /// The returned tree preserves the original hierarchy with non-matching
    /// subtrees pruned.
    pub fn apply(&self, nodes: &[NodeView]) -> Vec<NodeView> {
        if self.is_empty() {
            return nodes.to_vec();
        }
        nodes
            .iter()
            .filter_map(|node| self.filter_node(node))
            .collect()
    }

    fn filter_node(&self, node: &NodeView) -> Option<NodeView> {
        if node.is_leaf() {
            if self.matches_leaf(node) {
                return Some(node.clone());
            }
            return None;
        }

        // Container: recurse; survive only if children list is non-empty after filter.
        let filtered_children: Vec<NodeView> = node
            .children
            .iter()
            .filter_map(|c| self.filter_node(c))
            .collect();

        if filtered_children.is_empty() {
            return None;
        }

        let mut result = node.clone();
        result.children = filtered_children;
        Some(result)
    }

    fn matches_leaf(&self, node: &NodeView) -> bool {
        if !self.node_types.is_empty() && !self.node_types.contains(&node.node_type) {
            return false;
        }
        if let Some(ref label_substr) = self.label_contains
            && !node
                .label
                .to_lowercase()
                .contains(&label_substr.to_lowercase())
        {
            return false;
        }
        true
    }
}
