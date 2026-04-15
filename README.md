# leptos-column-browser

[![Crates.io version](https://img.shields.io/crates/v/leptos-column-browser.svg)](https://crates.io/crates/leptos-column-browser)
[![Docs.rs](https://img.shields.io/docsrs/leptos-column-browser)](https://docs.rs/leptos-column-browser)
[![CI](https://github.com/KoVal177/leptos-column-browser/actions/workflows/ci.yml/badge.svg)](https://github.com/KoVal177/leptos-column-browser/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/rustc-1.94+-blue.svg)](https://blog.rust-lang.org/)
[![Downloads](https://img.shields.io/crates/d/leptos-column-browser.svg)](https://crates.io/crates/leptos-column-browser)

Finder-style column navigation for [Leptos](https://leptos.dev/) with async loading,
keyboard support, resizable columns, and CSS-variable theming.

## Overview

`leptos-column-browser` renders a multi-pane Miller-column navigator for any
deeply-nested hierarchy you can represent as a tree. You implement one async
trait — `TopologyProvider` — and the component handles rendering, keyboard
navigation, ARIA, and column management.

## Features

- **Domain-agnostic** — `TopologyProvider` is your contract; no opinion on node types
- **Async-first** — children fetched lazily on drill-down via `async fn get_children`
- **Keyboard accessible** — ↑/↓ within columns, → to drill, ← to pop back
- **ARIA compliant** — `role="tree"`, `role="treeitem"`, `aria-selected`, `aria-expanded`
- **Resizable columns** — drag the right edge; min/default widths configurable
- **Custom icons** — `IconRenderer` closure; no built-in icon opinions
- **Controlled state** — supply `RwSignal<DrillPath>` for URL routing or leave internal
- **CSS-variable theming** — override `--lcb-*` variables; zero style injection

## Installation

```toml
[dependencies]
leptos-column-browser = "0.1"
```

Link the stylesheet in `index.html`:

```html
<link rel="stylesheet" href="path/to/column-browser.css" />
```

Requires `wasm32-unknown-unknown` and [`trunk`](https://trunkrs.dev/):

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Quick Start

```rust
use leptos::prelude::*;
use leptos_column_browser::{ColumnBrowser, Node, NodeId, StaticTopologyProvider};

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

For async providers (HTTP, database) see [`docs/provider.md`](docs/provider.md).

## Examples

See [`EXAMPLES.md`](EXAMPLES.md) for full walkthroughs.

| Example | What it shows |
|---|---|
| [`file_explorer`](examples/file_explorer/) | Static in-memory tree with `StaticTopologyProvider` |
| [`api_navigator`](examples/api_navigator/) | Async provider pattern with lazy child loading |

## Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `ui` | ✅ | Enables Leptos UI components and web bindings |

## Further Reading

- [`docs/provider.md`](docs/provider.md) — implementing `TopologyProvider`, `NodeId`, `NodeFilter`, controlled navigation
- [`docs/theming.md`](docs/theming.md) — CSS variable reference, light/dark themes, scoped themes

## MSRV

Rust **1.94** (edition 2024). Requires `wasm32-unknown-unknown` for browser use.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

MIT OR Apache-2.0

