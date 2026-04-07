use crate::topology::NodeId;

/// The current drill path through the hierarchy.
///
/// A flat `Vec<NodeId>` where index `i` is the selected node in column `i`.
/// Survives DOM teardown and rebuild — it is the single source of truth for
/// navigation state.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrillPath {
    /// Selected `NodeId` at each column depth.
    pub segments: Vec<NodeId>,
}

impl DrillPath {
    /// An empty path (nothing selected).
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the selected `NodeId` at `column_index`, if any.
    pub fn at(&self, column_index: usize) -> Option<&NodeId> {
        self.segments.get(column_index)
    }

    /// Return a new `DrillPath` with `id` selected at `column_index`.
    ///
    /// All segments at depth > `column_index` are truncated (selecting a
    /// node in an ancestor column resets the path beyond it).
    #[must_use]
    pub fn select(&self, column_index: usize, id: NodeId) -> Self {
        let mut new_segments = self.segments[..column_index.min(self.segments.len())].to_vec();
        new_segments.push(id);
        Self {
            segments: new_segments,
        }
    }

    /// Return a new `DrillPath` with the last selection removed.
    ///
    /// Returns an empty path if already empty.
    #[must_use]
    pub fn pop(&self) -> Self {
        if self.segments.is_empty() {
            return Self::empty();
        }
        Self {
            segments: self.segments[..self.segments.len() - 1].to_vec(),
        }
    }

    /// Depth of the current selection (number of selected nodes).
    pub fn depth(&self) -> usize {
        self.segments.len()
    }
}
