//! Generic SVG icons for the column browser.
//!
//! All icons use a 16×16 `viewBox`. Colors inherit from the consumer's theme
//! via `currentColor` (which responds to `--lcb-*` CSS variables).
//!
//! # Custom Icons
//!
//! Pass an [`IconRenderer`](crate::ui::icons::IconRenderer) to [`ColumnBrowser`](crate::components::column_browser::ColumnBrowser)
//! to supply your own icons keyed on `node_kind`:
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use leptos_column_browser::IconRenderer;
//!
//! let icons: IconRenderer = Arc::new(|node_kind: &str| match node_kind {
//!     "container" => include_str!("icons/folder.svg").to_owned(),
//!     _ => include_str!("icons/file.svg").to_owned(),
//! });
//! ```

use std::sync::Arc;

/// A callback that maps a node's `node_kind` string to an SVG string.
///
/// The returned string is injected via `inner_html` on a `<span>` element.
/// It must be a complete, self-contained `<svg>` element.
pub type IconRenderer = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// Default icon for container nodes (directory / folder shape).
///
/// 16×16, stroke uses `currentColor` so it inherits the text colour.
pub const DEFAULT_CONTAINER_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M2 4v8a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1H8L6.5 3H3a1 1 0 0 0-1 1z"/></svg>"#;

/// Default icon for leaf nodes (generic document / file shape).
///
/// 16×16, stroke uses `currentColor`.
pub const DEFAULT_LEAF_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="12" height="12" rx="1"/><line x1="2" y1="6" x2="14" y2="6"/><line x1="2" y1="10" x2="14" y2="10"/><line x1="6" y1="6" x2="6" y2="14"/></svg>"#;

/// Chevron right — rendered beside drillable (container) items.
pub const CHEVRON_RIGHT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 2 8 6 4 10"/></svg>"#;

/// Sidebar toggle icon (hamburger-split layout).
pub const SIDEBAR_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="12" height="12" rx="1"/><line x1="6" y1="2" x2="6" y2="14"/></svg>"#;

/// Build the default [`IconRenderer`].
///
/// Returns `DEFAULT_LEAF_ICON` for every `node_kind`. Consumers who want
/// type-specific icons should supply their own `IconRenderer` to
/// `ColumnBrowser`.
#[must_use]
pub fn default_icon_renderer() -> IconRenderer {
    Arc::new(|_node_kind: &str| DEFAULT_LEAF_ICON.to_owned())
}

/// Build an [`IconRenderer`] that distinguishes containers from leaves.
///
/// Returns `DEFAULT_CONTAINER_ICON` when `node_kind == "container"` and
/// `DEFAULT_LEAF_ICON` for everything else.
#[must_use]
pub fn container_leaf_icon_renderer() -> IconRenderer {
    Arc::new(|node_kind: &str| {
        if node_kind == "container" {
            DEFAULT_CONTAINER_ICON.to_owned()
        } else {
            DEFAULT_LEAF_ICON.to_owned()
        }
    })
}
