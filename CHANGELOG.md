# Changelog

All notable changes to `leptos-column-browser` are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] — 2026-04-15

### Added
- CI workflow (`.github/workflows/ci.yml`) — lint, test, WASM compile check, msrv gate.
- `CONTRIBUTING.md` — development setup, test commands, PR checklist.
- `EXAMPLES.md` — full walkthrough of `file_explorer` and `api_navigator` examples.
- `docs/provider.md` — `TopologyProvider` implementation guide.
- `docs/theming.md` — CSS variable reference with light and dark palettes.
- `rustfmt.toml` — consistent formatting config (edition 2024, max_width 100).

### Fixed
- README badges updated to shields.io format with CI status and MSRV badge.

## [0.1.0] — 2026-04-07

### Added
- `ColumnBrowser` stateful component with async `TopologyProvider` integration.
- `BrowserView` stateless low-level component.
- `BrowserColumn` with pointer-capture resize handles.
- `IconRenderer` callback prop — consumer controls icon rendering.
- `ColumnSizeConfig` prop for configurable initial and minimum column widths.
- Controlled `DrillPath` via `external_path: Option<RwSignal<DrillPath>>`.
- `on_pop` callback for "navigate back one level" (keyboard ← / Escape).
- Keyboard navigation: ↑/↓ within columns, Enter to select, ← to pop.
- ARIA roles: `role="tree"`, `role="treeitem"`, `aria-selected`, `aria-expanded`.
- `NodeFilter` — declarative, composable filtering of `NodeView` lists.
- `StaticTopologyProvider` — navigable in-memory tree provider.
- `NoopTopologyProvider` — placeholder / test stub.
- CSS variable theming system (`--lcb-*`).
- Two examples: `file_explorer` (static) and `api_navigator` (async GitHub).

### Changed
- `NodeView::source_type` renamed to `NodeView::node_type` (breaking).
- `Node::source_type` renamed to `Node::node_type` (breaking).
- `StaticTopologyProvider` now walks the node tree for `get_children` (breaking).

### Removed
- `src/topology/type_ids.rs` and all data-engineering type constants (breaking).
- `icons::icon_for_kind()` function (breaking) — replaced by `IconRenderer` prop.
- `icons::PARQUET_ICON`, `icons::CSV_ICON`, `icons::SCHEMA_ICON` (breaking).
- Hard-coded column width (`160px`) and minimum width (`80px`) — now configurable.
