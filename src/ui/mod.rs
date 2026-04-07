//! Leptos UI components for the column browser.
//!
//! This module provides the rendering layer:
//!
//! - [`BrowserView`] — stateless multi-column layout with keyboard navigation.
//! - [`BrowserColumn`](crate::ui::column::BrowserColumn) — a single column with items, resize handles, and ARIA semantics.
//! - [`IconRenderer`](crate::ui::icons::IconRenderer) — type alias for custom icon callbacks.
//! - [`DrillPath`] — navigation state tracking.
//! - `style` — CSS class constants.

/// Multi-column browser view.
pub mod browser_view;
/// Column rendering components.
pub mod column;
/// Inline SVG icons for navigation items.
pub mod icons;
/// Navigation state management (drill path).
pub mod state;
/// CSS class constants for styling.
pub mod style;

pub use browser_view::BrowserView;
pub use state::DrillPath;
