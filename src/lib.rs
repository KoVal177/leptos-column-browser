//! `leptos-column-browser` — multi-pane hierarchical navigator for Leptos.
//!
//! Provides both a low-level [`BrowserView`] and a high-level
//! [`ColumnBrowser`] component for navigating deeply nested trees via an
//! async [`TopologyProvider`] you implement.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_column_browser::{
//!     ColumnBrowser, Node, NodeId, StaticTopologyProvider,
//! };
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let tree = vec![
//!         Node::container(NodeId::root("docs"), "Documents".into(), "folder")
//!             .with_child(Node::leaf(
//!                 NodeId::from_segments(["docs", "readme.md"]),
//!                 "readme.md".into(),
//!                 "file",
//!             )),
//!     ];
//!
//!     view! {
//!         <ColumnBrowser
//!             provider=StaticTopologyProvider { nodes: tree }
//!             root_id=NodeId::root("__root__")
//!             on_open=Callback::new(|id| log::info!("opened {id}"))
//!         />
//!     }
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unreachable_pub)]

/// High-level stateful components.
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub mod components;
/// Domain types and traits for hierarchical navigation.
pub mod topology;
/// Leptos UI components for column rendering.
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub mod ui;

// Flat re-exports for the happy path
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use components::column_browser::ColumnBrowser;
pub use topology::{
    BrowserError, Node, NodeFilter, NodeId, NodeKind, NodeView, NoopTopologyProvider,
    StaticTopologyProvider, TopologyProvider,
};
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use ui::column::ColumnSizeConfig;
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use ui::icons::{
    DEFAULT_CONTAINER_ICON, DEFAULT_LEAF_ICON, IconRenderer, container_leaf_icon_renderer,
    default_icon_renderer,
};
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use ui::{BrowserView, DrillPath};
