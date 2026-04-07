//! High-level `ColumnBrowser` component — owns state and drives the provider.

use leptos::prelude::*;

use crate::topology::{BrowserError, NodeId, NodeView, TopologyProvider};
use crate::ui::browser_view::BrowserView;
use crate::ui::column::ColumnSizeConfig;
use crate::ui::icons::{IconRenderer, container_leaf_icon_renderer};
use crate::ui::state::DrillPath;

/// A fully stateful column browser.
///
/// # Controlled mode
///
/// Pass `external_path` to manage [`DrillPath`] from outside the component.
/// The component will read from and write to the provided signal, enabling
/// deep-link routing, undo/redo, and programmatic navigation.
///
/// # Uncontrolled mode
///
/// Omit `external_path`. The component manages its own internal `DrillPath`.
#[allow(unreachable_pub, clippy::needless_pass_by_value)]
#[component]
pub fn ColumnBrowser<P>(
    /// The topology data source.
    provider: P,
    /// Root node — children of this node form the first column.
    root_id: NodeId,
    /// Maximum visible columns at once.
    #[prop(default = 3)]
    visible_cols: usize,
    /// Fired when the user double-clicks a leaf node.
    on_open: Callback<String>,
    /// Icon renderer — maps `node_kind` to an SVG string.
    /// Defaults to the built-in container/leaf renderer.
    #[prop(default = container_leaf_icon_renderer())]
    icon_renderer: IconRenderer,
    /// Column sizing configuration.
    #[prop(default = ColumnSizeConfig::default())]
    size_config: ColumnSizeConfig,
    /// External drill-path signal (controlled mode).
    ///
    /// When provided, the component reads/writes through this signal instead
    /// of its internal state. The caller is responsible for persisting and
    /// restoring the `DrillPath` (e.g., to/from the URL).
    #[prop(optional)]
    external_path: Option<RwSignal<DrillPath>>,
) -> impl IntoView
where
    P: TopologyProvider + Clone + 'static,
{
    // ── state ─────────────────────────────────────────────────────────────────
    // Always create an internal signal; use external one when provided.
    let internal_path = RwSignal::new(DrillPath::empty());
    let path_signal: RwSignal<DrillPath> = external_path.unwrap_or(internal_path);

    let (columns, set_columns) = signal(Vec::<Vec<NodeView>>::new());
    let (error, set_error) = signal(Option::<BrowserError>::None);

    // ── root fetch ────────────────────────────────────────────────────────────
    let provider_root = provider.clone();
    let root_id_clone = root_id.clone();

    Effect::new(move |_| {
        let p = provider_root.clone();
        let rid = root_id_clone.clone();
        leptos::task::spawn_local(async move {
            match p.get_children(&rid).await {
                Ok(nodes) => {
                    let views: Vec<NodeView> = nodes.into_iter().map(NodeView::from).collect();
                    set_columns.update(|cols| {
                        cols.clear();
                        cols.push(views);
                    });
                }
                Err(e) => set_error.set(Some(e)),
            }
        });
    });

    // ── select handler ────────────────────────────────────────────────────────
    let on_select = Callback::new(move |(col_idx, id_str): (usize, String)| {
        let new_path = path_signal
            .get_untracked()
            .select(col_idx, NodeId::parse(&id_str));
        let depth = new_path.depth();
        let parent_id = new_path.segments[depth - 1].clone();
        path_signal.set(new_path);

        let p = provider.clone();
        leptos::task::spawn_local(async move {
            match p.get_children(&parent_id).await {
                Ok(nodes) => {
                    let views: Vec<NodeView> = nodes.into_iter().map(NodeView::from).collect();
                    set_columns.update(|cols| {
                        cols.truncate(depth);
                        cols.push(views);
                    });
                }
                Err(e) => set_error.set(Some(e)),
            }
        });
    });

    // ── pop handler (keyboard ArrowLeft / Escape) ─────────────────────────────
    let on_pop = Callback::new(move |()| {
        let popped = path_signal.get_untracked().pop();
        let new_depth = popped.depth();
        path_signal.set(popped);
        // Keep root column + one column per remaining segment.
        set_columns.update(|cols| {
            cols.truncate(new_depth + 1);
        });
    });

    view! {
        {move || error.get().map(|e| view! {
            <div class="lcb-error" role="alert">{e.to_string()}</div>
        })}
        <BrowserView
            columns_data=columns.into()
            path=path_signal.into()
            visible_cols=visible_cols
            on_select=on_select
            on_open=on_open
            on_pop=on_pop
            icon_renderer=icon_renderer.clone()
            size_config=size_config
        />
    }
}
