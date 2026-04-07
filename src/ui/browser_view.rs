use leptos::prelude::*;

use crate::topology::{NodeId, NodeView};
use crate::ui::column::{BrowserColumn, ColumnSizeConfig};
use crate::ui::icons::{IconRenderer, container_leaf_icon_renderer};
use crate::ui::state::DrillPath;
use crate::ui::style;

/// Low-level multi-column browser view.
///
/// Renders a horizontal sequence of columns representing each level of
/// the tree. The component maintains a sliding window showing at most
/// `visible_cols` columns at once (defaulting to 3).
///
/// Stateless — state is managed by the parent. For auto-managed state,
/// use `ColumnBrowser` from `components`.
///
/// # Keyboard navigation
///
/// The root `<div>` intercepts **ArrowRight** (move focus to the next
/// column) and **ArrowLeft / Escape** (pop back one level via the
/// `on_pop` callback). Intra-column navigation (↑ / ↓ / Enter / Space)
/// is handled within each `BrowserColumn`.
///
/// # Signal contract
///
/// - `columns_data`: reactive signal holding the list of columns. Each entry
///   is a `Vec<NodeView>` representing one column's items. Updated by
///   the parent when the topology changes.
/// - `path`: reactive signal holding the current [`DrillPath`].
/// - `on_select`: callback fired when a node is clicked: `(col_idx, canonical_id)`.
/// - `on_open`: callback fired when a leaf is double-clicked: `canonical_id`.
/// - `on_pop`: callback fired on ArrowLeft / Escape to navigate up one level.
#[allow(unreachable_pub, clippy::needless_pass_by_value)]
#[component]
pub fn BrowserView(
    /// Reactive column data — one inner `Vec` per column.
    columns_data: Signal<Vec<Vec<NodeView>>>,
    /// The current navigation path (selection state).
    path: Signal<DrillPath>,
    /// Maximum number of columns visible at once.
    #[prop(default = 3)]
    visible_cols: usize,
    /// Callback: user clicked an item at `(col_idx, canonical_id)`.
    on_select: Callback<(usize, String)>,
    /// Callback: user double-clicked a leaf `canonical_id`.
    on_open: Callback<String>,
    /// Callback: navigate up one level (ArrowLeft / Escape).
    on_pop: Callback<()>,
    /// Icon renderer forwarded to every column.
    /// Defaults to the built-in container/leaf renderer.
    #[prop(default = container_leaf_icon_renderer())]
    icon_renderer: IconRenderer,
    /// Column sizing config forwarded to every column.
    #[prop(default = ColumnSizeConfig::default())]
    size_config: ColumnSizeConfig,
) -> impl IntoView {
    // Shared signal: when set to Some(col_idx), the target BrowserColumn
    // focuses its first item and clears the signal.
    let focus_request: RwSignal<Option<usize>> = RwSignal::new(None);

    let on_root_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowRight" => {
                ev.prevent_default();
                // Focus the first item of the rightmost visible column.
                let total = columns_data.get_untracked().len();
                if total > 0 {
                    focus_request.set(Some(total - 1));
                }
            }
            "ArrowLeft" | "Escape" => {
                ev.prevent_default();
                on_pop.run(());
            }
            _ => {}
        }
    };

    view! {
        <div
            class={style::BROWSER_ROOT}
            on:keydown=on_root_keydown
        >
            {move || {
                let data = columns_data.get();
                let total = data.len();
                let start = total.saturating_sub(visible_cols);
                data.into_iter()
                    .enumerate()
                    .skip(start)
                    .map(|(col_idx, items)| {
                        let selected_id = Memo::new(move |_| {
                            path.get().at(col_idx).map(NodeId::canonical)
                        });
                        view! {
                            <BrowserColumn
                                items=items
                                col_idx=col_idx
                                selected_id=selected_id.into()
                                on_select=on_select
                                on_open=on_open
                                icon_renderer=icon_renderer.clone()
                                size_config=size_config
                                focus_request=focus_request
                            />
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </div>
    }
}
