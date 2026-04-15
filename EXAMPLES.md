# Examples

## `file_explorer` — Static In-Memory Tree

Demonstrates `StaticTopologyProvider` to browse a fake filesystem hierarchy
without any async I/O. A good starting point before introducing a real async provider.

### Running

```bash
cd examples/file_explorer
trunk serve
# open http://localhost:8080
```

### Walkthrough

**Building the node tree** — nodes are constructed with `Node::container` and
`Node::leaf`, linked via `.with_child()`. `NodeId::from_segments` keeps paths
stable and slash-joined under the hood:

```rust
Node::container(NodeId::root("Documents"), "Documents".into(), "folder")
    .with_child(Node::leaf(
        NodeId::from_segments(["Documents", "notes.txt"]),
        "notes.txt".into(),
        "text",
    ))
```

**Icon renderer** — a plain closure maps `node_kind` (`"container"` or `"leaf"`) to
an SVG string. `DEFAULT_CONTAINER_ICON` and `DEFAULT_LEAF_ICON` are the
zero-effort starting point:

```rust
let icons: IconRenderer = Arc::new(|node_kind: &str| {
    if node_kind == "container" { DEFAULT_CONTAINER_ICON.to_owned() }
    else                        { DEFAULT_LEAF_ICON.to_owned() }
});
```

**Mounting** — `visible_cols=3` sets the initial column count; users can resize
freely at runtime. The `on_open` callback fires when the user presses Enter or
double-clicks an item:

```rust
<ColumnBrowser
    provider=StaticTopologyProvider { nodes: make_filesystem() }
    root_id=NodeId::root("__root__")
    visible_cols=3
    icon_renderer=icons
    on_open=Callback::new(move |id: String| set_last_opened.set(id))
/>
```

### Key Concepts

- `StaticTopologyProvider` — whole tree in memory; no futures needed.
- `NodeId::root` vs `NodeId::from_segments` — use `root` for top-level nodes,
  `from_segments` for all others. Segments may not contain `/`.
- `node_type` is free-form (e.g. `"pdf"`, `"folder"`) — your `IconRenderer`
  maps it to icons; the library has no opinion.
- `DEFAULT_CONTAINER_ICON` / `DEFAULT_LEAF_ICON` — ready-made SVG fallbacks.

### What to Try

- Add a deeply nested subtree (4+ levels) and navigate with the keyboard.
- Replace `DEFAULT_CONTAINER_ICON` with a custom SVG string for `"pdf"` nodes.
- Pass `external_path=path` and set it programmatically to jump to a specific item.

---

## `api_navigator` — Async Provider Pattern

Demonstrates the full `TopologyProvider` async contract using an in-memory Rust
crates ecosystem tree that returns data via `async fn`. A real provider would
perform HTTP requests, database queries, or other async I/O.

### Running

```bash
cd examples/api_navigator
trunk serve
# open http://localhost:8080
```

### Walkthrough

**Provider struct** — holds the tree in an `Arc<HashMap>` so it is cheap to
clone into reactive closures:

```rust
#[derive(Clone)]
pub struct CratesProvider {
    tree: Arc<HashMap<String, Vec<Node>>>,
}
```

**Implementing the trait** — look up the parent's canonical ID in the map and
return the child nodes. For real async I/O, replace `ready(Ok(...))` with an
`async` block that awaits an HTTP request:

```rust
impl TopologyProvider for CratesProvider {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
        let children = self.tree
            .get(parent_id.canonical())
            .cloned()
            .unwrap_or_default();
        std::future::ready(Ok(children))
    }
}
```

**Mounting** — `visible_cols=4` opens four columns by default, showing the
deeper tree immediately:

```rust
<ColumnBrowser
    provider=CratesProvider::new()
    root_id=NodeId::root("__root__")
    visible_cols=4
    icon_renderer=make_icon_renderer()
    on_open=Callback::new(move |id: String| set_last_opened.set(id))
/>
```

### Key Concepts

- `Arc<HashMap<String, Vec<Node>>>` — cheap `Clone` while sharing data across
  reactive closures; replace with a real Arc-wrapped client for HTTP.
- `parent_id.canonical()` — returns the slash-joined string ID to use as a
  lookup key or URL path segment.
- `std::future::ready(Ok(...))` — wraps a sync value in an immediately-resolved
  future; swap in an `async` block for real I/O.
- `BrowserError::fetch` / `BrowserError::parse` — the two error constructors for
  network and deserialisation failures.

### What to Try

- Replace `std::future::ready` with `gloo_net::http::Request` to fetch real data.
- Add a `NodeFilter` to hide leaf nodes when the user hasn't selected a category.
- Wire `visible_cols` to a reactive signal from a toolbar control.
