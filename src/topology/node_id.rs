use serde::{Deserialize, Serialize};

/// Hierarchical path identity for a navigation node.
///
/// `NodeId` is an ordered sequence of non-empty, slash-free segments that
/// uniquely identify a node within a tree. The canonical string form joins
/// segments with `/`.
///
/// # Invariants
///
/// - At least one segment.
/// - No segment is empty.
/// - No segment contains `/`.
///
/// # Examples
///
/// ```
/// use leptos_column_browser::NodeId;
///
/// let id = NodeId::from_segments(["docs", "guide", "intro.md"]);
/// assert_eq!(id.canonical(), "docs/guide/intro.md");
/// assert_eq!(id.depth(), 3);
/// assert_eq!(id.name(), "intro.md");
///
/// let parent = id.parent().unwrap();
/// assert_eq!(parent.canonical(), "docs/guide");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId {
    segments: Vec<String>,
}

impl NodeId {
    /// Construct from an iterator of segment strings.
    ///
    /// # Panics
    ///
    /// Panics if any segment is empty or contains `/`.
    pub fn from_segments<I, S>(segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let segments: Vec<String> = segments.into_iter().map(Into::into).collect();
        assert!(!segments.is_empty(), "NodeId requires at least one segment");
        for seg in &segments {
            assert!(!seg.is_empty(), "NodeId segment must not be empty");
            assert!(!seg.contains('/'), "NodeId segment must not contain '/'");
        }
        Self { segments }
    }

    /// Construct a single-segment (root-level) id.
    pub fn root(name: impl Into<String>) -> Self {
        Self::from_segments([name.into()])
    }

    /// Parse from the canonical slash-separated string form.
    ///
    /// Panics if the string is empty or any segment is empty.
    pub fn parse(canonical: &str) -> Self {
        Self::from_segments(canonical.split('/'))
    }

    /// The canonical slash-separated string representation.
    pub fn canonical(&self) -> String {
        self.segments.join("/")
    }

    /// The last segment (the node's own name within its parent).
    ///
    /// # Panics
    ///
    /// Panics if segments is somehow empty (invariant violation).
    pub fn name(&self) -> &str {
        self.segments
            .last()
            .expect("segments is non-empty by invariant")
    }

    /// Number of segments (depth in the tree, 1-based).
    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// Returns the parent `NodeId`, or `None` if this is a root node.
    pub fn parent(&self) -> Option<Self> {
        if self.segments.len() == 1 {
            return None;
        }
        Some(Self {
            segments: self.segments[..self.segments.len() - 1].to_vec(),
        })
    }

    /// Returns a child id by appending `segment` to this id.
    #[must_use]
    pub fn child(&self, segment: impl Into<String>) -> Self {
        let mut new = self.segments.clone();
        new.push(segment.into());
        Self { segments: new }
    }

    /// Access the raw segments.
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.canonical())
    }
}
