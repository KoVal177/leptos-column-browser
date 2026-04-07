//! CSS class name constants for the column browser.
//!
//! The consuming application is expected to provide CSS for these classes,
//! or link the bundled `style/column-browser.css`.

/// Root wrapper element for the column browser container.
pub const BROWSER_ROOT: &str = "lcb-root";
/// Individual column wrapper.
pub const BROWSER_COLUMN: &str = "lcb-column";
/// List element inside a column.
pub const BROWSER_LIST: &str = "lcb-list";
/// Individual item (row) in a column.
pub const BROWSER_ITEM: &str = "lcb-item";
/// Modifier class for the currently selected item.
pub const BROWSER_SELECTED: &str = "lcb-item--selected";
/// Modifier class for container items (have children).
pub const BROWSER_CONTAINER: &str = "lcb-item--container";
/// Modifier class for leaf items (no children).
pub const BROWSER_LEAF: &str = "lcb-item--leaf";
/// Inline SVG icon span.
pub const BROWSER_ICON: &str = "lcb-item-icon";
/// Chevron indicator for drillable items.
pub const BROWSER_CHEVRON: &str = "lcb-item-chevron";
/// Span wrapping the item's text label (for styling / a11y targeting).
pub const BROWSER_LABEL: &str = "lcb-item-label";
/// Drag-to-resize handle at the right edge of a column.
pub const BROWSER_RESIZE_HANDLE: &str = "lcb-resize-handle";
