//! Domain types and traits for hierarchical navigation.
//!
//! No Arrow, no network, no framework dependency.
//! Compiles on `wasm32-unknown-unknown` and native alike.

/// Declarative node filtering.
pub mod filter;
/// Node domain model and wire-format view.
pub mod node;
/// Hierarchical node identifiers.
pub mod node_id;
/// Topology provider trait and built-in implementations.
pub mod provider;

pub use filter::NodeFilter;
pub use node::{Node, NodeKind, NodeView};
pub use node_id::NodeId;
pub use provider::{BrowserError, NoopTopologyProvider, StaticTopologyProvider, TopologyProvider};
