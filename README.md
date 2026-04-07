# leptos-column-browser

[![Crates.io](https://img.shields.io/crates/v/leptos-column-browser)](https://crates.io/crates/leptos-column-browser)
[![Docs.rs](https://docs.rs/leptos-column-browser/badge.svg)](https://docs.rs/leptos-column-browser)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

> Finder-style column navigation for Leptos, with async loading, keyboard support,
> resizable columns, and CSS-variable theming.

A multi-pane column browser for [Leptos](https://leptos.dev/), modeled on
macOS Finder's Miller-column navigation. Navigate any deeply-nested hierarchy
via an async `TopologyProvider` trait you implement — filesystems, APIs, org
charts, or anything tree-shaped.

---

## Features

- **Domain-agnostic** — `TopologyProvider` is your contract; the library has
  no opinion about what "node types" mean.
- **Async-first** — children are fetched lazily on drill-down via
  `async fn get_children`.
- **Keyboard accessible** — ↑/↓ within columns, → to drill, ← to pop back.
- **ARIA compliant** — `role="tree"`, `role="treeitem"`, `aria-selected`,
  `aria-expanded`.
- **Resizable columns** — drag the right edge of any column.
- **Custom icons** — supply an `IconRenderer` closure; no built-in opinions.
- **Controlled or uncontrolled state** — pass an `RwSignal<DrillPath>` for
  deep-link routing, or let the component manage state internally.
- **Theming** — pure CSS variables, no style injection.

---

## Quick Start

### Requirements

- Rust `1.94+`
- `leptos = "0.8"`
- Default `ui` feature enabled
- `wasm32-unknown-unknown` target for browser builds

### Install

Add to `Cargo.toml`:

```toml
[dependencies]
leptos-column-browser = "0.1"
```

Link the bundled stylesheet from `style/column-browser.css` in your `index.html`
(or import it from your bundler):

```html
<link rel="stylesheet" href="path/to/column-browser.css" />
```

Mount a static browser:

```rust
use leptos::prelude::*;
use leptos_column_browser::{
    ColumnBrowser, Node, NodeId, StaticTopologyProvider,
};

#[component]
pub fn App() -> impl IntoView {
    let tree = vec![
        Node::container(NodeId::root("docs"), "Documents".into(), "folder")
            .with_child(Node::leaf(
                NodeId::from_segments(["docs", "readme.md"]),
                "readme.md".into(),
                "file",
            )),
    ];

    view! {
        <ColumnBrowser
            provider=StaticTopologyProvider { nodes: tree }
            root_id=NodeId::root("__root__")
            on_open=Callback::new(|id| log::info!("opened {id}"))
        />
    }
}
```

---

## Implementing `TopologyProvider`

`TopologyProvider` is the primary contract between your data and the UI:

```rust
use std::future::Future;
use leptos_column_browser::{TopologyProvider, BrowserError, Node, NodeId};

#[derive(Clone)]
pub struct MyProvider;

impl TopologyProvider for MyProvider {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
        let parent = parent_id.canonical();
        async move {
            // Fetch children from your backend / in-memory tree.
            // Return Ok(vec![]) for leaf nodes — do NOT return an error.
            todo!("fetch children for {parent}")
        }
    }
}
```

Key points:

- `get_children` is called once per drill-down.
- Return `Ok(Vec::new())` for leaf nodes — the UI distinguishes leaves by
  `NodeKind`, not by an empty child list.
- `Node::container` / `Node::leaf` control `NodeKind`.
- `node_type` is a free-form string you define; it is passed to your
  `IconRenderer` so you can map it to icons.

---

## Custom Icons

Supply an `IconRenderer` to `ColumnBrowser`:

```rust
use leptos_column_browser::{IconRenderer, DEFAULT_CONTAINER_ICON, DEFAULT_LEAF_ICON};
use std::sync::Arc;

let icons: IconRenderer = Arc::new(|node_kind: &str| match node_kind {
    "container" => DEFAULT_CONTAINER_ICON.to_owned(),
    _           => DEFAULT_LEAF_ICON.to_owned(),
});
```

The renderer receives `node_kind` (`"container"` or `"leaf"`).

---

## Controlled Navigation State

For URL-based routing or programmatic navigation, supply an external signal:

```rust
use leptos::prelude::*;
use leptos_column_browser::DrillPath;

let path = RwSignal::new(DrillPath::empty());

// Navigate programmatically:
path.set(DrillPath::empty().select(0, leptos_column_browser::NodeId::root("docs")));
```

Pass `external_path=path` to `ColumnBrowser` to enable controlled mode.

---

## Feature Flags

- `ui` (default): enables the Leptos UI components and web bindings

---

## CSS Theming

Link `style/column-browser.css` and override the `--lcb-*` variables on
`:root` or any ancestor element:

```css
:root {
    --lcb-bg:           #1e1e2e;  /* outer background */
    --lcb-column-bg:    #181825;  /* per-column background */
    --lcb-border:       #313244;  /* column separator */
    --lcb-item-hover:   #313244;  /* hover highlight */
    --lcb-selected:     #45475a;  /* selected item background */
    --lcb-text:         #cdd6f4;  /* primary text */
    --lcb-text-muted:   #6c7086;  /* secondary / muted text */
    --lcb-accent:       #89b4fa;  /* icons, focus ring, resize handle */
}
```

| Variable | Default | Description |
|---|---|---|
| `--lcb-bg` | `#1e1e2e` | Root container background |
| `--lcb-column-bg` | `#181825` | Per-column background |
| `--lcb-border` | `#313244` | Column divider colour |
| `--lcb-item-hover` | `#313244` | Item hover background |
| `--lcb-selected` | `#45475a` | Selected item background |
| `--lcb-text` | `#cdd6f4` | Primary text colour |
| `--lcb-text-muted` | `#6c7086` | Secondary / muted text colour |
| `--lcb-accent` | `#89b4fa` | Icon colour, focus ring, resize handle |

All colours use Catppuccin Mocha by default. A light-theme override:

```css
.lcb-root {
    --lcb-bg:         #eff1f5;
    --lcb-column-bg:  #e6e9ef;
    --lcb-border:     #ccd0da;
    --lcb-item-hover: #dce0e8;
    --lcb-selected:   #bcc0cc;
    --lcb-text:       #4c4f69;
    --lcb-text-muted: #8c8fa1;
    --lcb-accent:     #1e66f5;
}
```

---

## Examples

Run the examples with [Trunk](https://trunkrs.dev/):

```sh
cd examples/file_explorer && trunk serve
cd examples/api_navigator && trunk serve
```

`file_explorer` demonstrates a static in-memory tree.
`api_navigator` demonstrates async loading against the crates.io/GitHub ecosystem.

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
