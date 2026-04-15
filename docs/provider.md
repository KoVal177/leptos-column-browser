# Implementing `TopologyProvider`

`TopologyProvider` is the single trait that connects your data to
`leptos-column-browser`. The browser calls `get_children` on every
drill-down and expects you to return the items to display in the new column.

## Trait Definition

```rust
pub trait TopologyProvider: Clone + 'static {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>>;
}
```

Key obligations:

- **Return `Ok(vec![])` for leaf nodes** — the browser distinguishes leaves by
  `Node::kind`, not by an empty child list. A `BrowserError` for a leaf node
  will show an error state.
- The future must be `Send` if your async runtime requires it; for WASM
  (single-threaded) this is relaxed.
- `Clone` is required because the provider is cloned into each reactive closure.

## Node and NodeId

```rust
// Containers (can have children):
Node::container(
    NodeId::root("docs"),        // unique stable identifier
    "Documents".into(),          // display label
    "folder",                    // node_type — passed to your IconRenderer
)

// Leaves (no children):
Node::leaf(
    NodeId::from_segments(["docs", "readme.md"]),
    "readme.md".into(),
    "file",
)
```

`NodeId` rules:
- `NodeId::root("x")` — top-level node, canonical form `"x"`.
- `NodeId::from_segments(["a", "b", "c"])` — canonical form `"a/b/c"`.
- Segments may not contain `/`.
- IDs must be stable — the browser uses them as keys for selection and path
  restoration.

## Minimal Synchronous Provider

```rust
#[derive(Clone)]
pub struct DemoProvider {
    nodes: Vec<Node>,
}

impl TopologyProvider for DemoProvider {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
        let children: Vec<Node> = self
            .nodes
            .iter()
            .filter(|n| n.id().parent_canonical() == Some(parent_id.canonical()))
            .cloned()
            .collect();
        std::future::ready(Ok(children))
    }
}
```

When the entire tree fits in memory, use the built-in `StaticTopologyProvider`
instead — it does the parent lookup for you:

```rust
StaticTopologyProvider { nodes: my_vec_of_nodes }
```

## Async Provider (HTTP / Database)

```rust
#[derive(Clone)]
pub struct ApiProvider {
    base_url: String,
}

impl TopologyProvider for ApiProvider {
    fn get_children(
        &self,
        parent_id: &NodeId,
    ) -> impl Future<Output = Result<Vec<Node>, BrowserError>> {
        let url = format!("{}/children/{}", self.base_url, parent_id.canonical());
        async move {
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| BrowserError::fetch(e.to_string()))?;
            let items: Vec<ApiItem> = resp
                .json()
                .await
                .map_err(|e| BrowserError::parse(e.to_string()))?;
            Ok(items.into_iter().map(|i| i.into_node()).collect())
        }
    }
}
```

## NodeFilter

`NodeFilter` composes declarative predicates over `NodeView` slices before
they are rendered:

```rust
use leptos_column_browser::NodeFilter;

// Show only containers:
let filter = NodeFilter::by_kind(NodeKind::Container);
// Show only items whose label contains a search term:
let filter = NodeFilter::label_contains("doc");
// Combine:
let filter = NodeFilter::label_contains("doc").and(NodeFilter::by_kind(NodeKind::Leaf));
```

Pass the filter as the `node_filter` prop on `ColumnBrowser` or `BrowserView`.

## Controlled Navigation

Supply an external `RwSignal<DrillPath>` to lock the browser into a specific path
or to navigate programmatically (e.g. from a URL router):

```rust
let path = RwSignal::new(DrillPath::empty());
// Select the first item in the root column:
path.update(|p| *p = p.select(0, NodeId::root("docs")));
// Mount:
view! { <ColumnBrowser external_path=path ... /> }
```
