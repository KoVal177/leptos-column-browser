//! Self-contained async topology provider — no network required.
//!
//! Demonstrates how any async data source can implement [`TopologyProvider`].
//! This mock returns in-memory data immediately; a real provider would
//! perform HTTP requests, database queries, etc.

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use leptos_column_browser::{BrowserError, Node, NodeId, TopologyProvider};

/// An in-memory provider that ships a static Rust ecosystem category tree.
///
/// Implements [`TopologyProvider`] with `async` returning immediately,
/// demonstrating the full async contract without any network I/O.
#[derive(Clone)]
pub struct CratesProvider {
    tree: Arc<HashMap<String, Vec<Node>>>,
}

impl CratesProvider {
    /// Build the Rust ecosystem demo tree.
    pub fn new() -> Self {
        let mut tree: HashMap<String, Vec<Node>> = HashMap::new();

        // ── root ──────────────────────────────────────────────────────────────
        tree.insert(
            "__root__".to_owned(),
            vec![
                Node::container(NodeId::root("async"), "Async Runtime".into(), "category"),
                Node::container(NodeId::root("web"), "Web Frameworks".into(), "category"),
                Node::container(NodeId::root("frontend"), "Frontend UI".into(), "category"),
                Node::container(NodeId::root("database"), "Database".into(), "category"),
                Node::container(NodeId::root("utils"), "Utilities".into(), "category"),
            ],
        );

        // ── async ─────────────────────────────────────────────────────────────
        tree.insert(
            "async".to_owned(),
            vec![
                Node::leaf(
                    NodeId::from_segments(["async", "tokio"]),
                    "tokio".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["async", "async-std"]),
                    "async-std".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["async", "smol"]),
                    "smol".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["async", "futures"]),
                    "futures".into(),
                    "crate",
                ),
            ],
        );

        // ── web ───────────────────────────────────────────────────────────────
        tree.insert(
            "web".to_owned(),
            vec![
                Node::leaf(
                    NodeId::from_segments(["web", "axum"]),
                    "axum".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["web", "actix-web"]),
                    "actix-web".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["web", "warp"]),
                    "warp".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["web", "rocket"]),
                    "rocket".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["web", "tower"]),
                    "tower".into(),
                    "crate",
                ),
            ],
        );

        // ── frontend ──────────────────────────────────────────────────────────
        tree.insert(
            "frontend".to_owned(),
            vec![
                Node::leaf(
                    NodeId::from_segments(["frontend", "leptos"]),
                    "leptos".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["frontend", "dioxus"]),
                    "dioxus".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["frontend", "yew"]),
                    "yew".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["frontend", "sycamore"]),
                    "sycamore".into(),
                    "crate",
                ),
            ],
        );

        // ── database ──────────────────────────────────────────────────────────
        tree.insert(
            "database".to_owned(),
            vec![
                Node::leaf(
                    NodeId::from_segments(["database", "sqlx"]),
                    "sqlx".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["database", "diesel"]),
                    "diesel".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["database", "sea-orm"]),
                    "sea-orm".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["database", "surrealdb"]),
                    "surrealdb".into(),
                    "crate",
                ),
            ],
        );

        // ── utils ─────────────────────────────────────────────────────────────
        tree.insert(
            "utils".to_owned(),
            vec![
                Node::leaf(
                    NodeId::from_segments(["utils", "serde"]),
                    "serde".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["utils", "tracing"]),
                    "tracing".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["utils", "clap"]),
                    "clap".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["utils", "rayon"]),
                    "rayon".into(),
                    "crate",
                ),
                Node::leaf(
                    NodeId::from_segments(["utils", "anyhow"]),
                    "anyhow".into(),
                    "crate",
                ),
            ],
        );

        Self {
            tree: Arc::new(tree),
        }
    }
}

impl TopologyProvider for CratesProvider {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
        let key = parent_id.canonical().to_owned();
        // Leaf nodes simply return an empty list — no error needed.
        let result = self.tree.get(&key).cloned().unwrap_or_default();
        async move { Ok(result) }
    }
}
