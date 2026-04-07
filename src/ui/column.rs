use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::topology::NodeView;
use crate::ui::icons::{CHEVRON_RIGHT, IconRenderer};
use crate::ui::style;

/// Configuration for column width sizing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColumnSizeConfig {
    /// Initial column width in pixels.
    pub initial_width: f64,
    /// Minimum column width in pixels (drag clamp).
    pub min_width: f64,
}

impl Default for ColumnSizeConfig {
    fn default() -> Self {
        Self {
            initial_width: 160.0,
            min_width: 80.0,
        }
    }
}

/// A single column in the browser layout.
///
/// Renders a `<ul role="tree">` of items with ARIA tree-widget semantics.
/// Keyboard navigation (↑ / ↓ / Enter / Space) is handled via a single
/// `keydown` listener on the `<ul>` (WAI-ARIA roving-tabindex pattern).
///
/// Cross-column navigation (→ / ← / Escape) is coordinated by the parent
/// `BrowserView` via the `focus_request` signal.
#[allow(
    unreachable_pub,
    clippy::too_many_lines,
    clippy::needless_pass_by_value
)]
#[component]
pub fn BrowserColumn(
    /// The items to display in this column.
    items: Vec<NodeView>,
    /// The absolute column index into `DrillPath::segments`.
    col_idx: usize,
    /// The canonical id of the currently selected item (if any).
    selected_id: Signal<Option<String>>,
    /// Callback fired when a user clicks an item: `(col_idx, canonical_id)`.
    on_select: Callback<(usize, String)>,
    /// Callback fired when a user double-clicks a leaf item: `canonical_id`.
    on_open: Callback<String>,
    /// Icon renderer — maps `node_kind` to an SVG string.
    icon_renderer: IconRenderer,
    /// Column sizing configuration.
    #[prop(default = ColumnSizeConfig::default())]
    size_config: ColumnSizeConfig,
    /// Shared signal for cross-column focus requests from `BrowserView`.
    /// When set to `Some(col_idx)`, this column focuses its first item
    /// and clears the signal.
    focus_request: RwSignal<Option<usize>>,
) -> impl IntoView {
    // ── resize state ─────────────────────────────────────────────────────────
    let col_width: RwSignal<f64> = RwSignal::new(size_config.initial_width);
    let drag_start_x: RwSignal<f64> = RwSignal::new(0.0);
    let drag_start_w: RwSignal<f64> = RwSignal::new(0.0);
    let dragging: RwSignal<bool> = RwSignal::new(false);
    let min_width = size_config.min_width;

    let on_handle_down = move |ev: leptos::ev::PointerEvent| {
        ev.prevent_default();
        dragging.set(true);
        #[allow(clippy::cast_precision_loss)]
        let x = f64::from(ev.client_x());
        drag_start_x.set(x);
        drag_start_w.set(col_width.get_untracked());
        if let Some(target) = ev.target() {
            let el: web_sys::Element = target.unchecked_into();
            let _ = el.set_pointer_capture(ev.pointer_id());
        }
    };
    let on_handle_move = move |ev: leptos::ev::PointerEvent| {
        if !dragging.get_untracked() {
            return;
        }
        #[allow(clippy::cast_precision_loss)]
        let new_w = (drag_start_w.get_untracked() + f64::from(ev.client_x())
            - drag_start_x.get_untracked())
        .max(min_width);
        col_width.set(new_w);
    };
    let on_handle_up = move |_: leptos::ev::PointerEvent| {
        dragging.set(false);
    };

    // ── keyboard / focus state ───────────────────────────────────────────────
    let item_count = items.len();
    let focus_idx: RwSignal<Option<usize>> = RwSignal::new(None);
    let list_ref = NodeRef::<leptos::html::Ul>::new();

    // Pre-extract item IDs for the keydown handler (avoids borrowing `items`).
    let item_ids: Vec<String> = items.iter().map(|n| n.id.clone()).collect();

    // Helper: focus the <li> at `idx` within this column's <ul>.
    let focus_item = move |idx: usize| {
        if let Some(ul) = list_ref.get() {
            let children = ul.children();
            #[allow(clippy::cast_possible_truncation)]
            let idx_u32 = idx as u32; // Column item counts are well below u32::MAX.
            if let Some(li) = children.item(idx_u32)
                && let Ok(el) = li.dyn_into::<HtmlElement>()
            {
                let _ = el.focus();
            }
        }
        focus_idx.set(Some(idx));
    };

    // Watch for cross-column focus requests targeting this column.
    Effect::new(move |_| {
        if focus_request.get() == Some(col_idx) {
            focus_item(0);
            focus_request.set(None);
        }
    });

    let on_keydown = {
        let item_ids = item_ids.clone();
        move |ev: leptos::ev::KeyboardEvent| {
            let current = focus_idx.get_untracked().unwrap_or(0);
            match ev.key().as_str() {
                "ArrowDown" => {
                    ev.prevent_default();
                    let next = (current + 1).min(item_count.saturating_sub(1));
                    focus_item(next);
                }
                "ArrowUp" => {
                    ev.prevent_default();
                    let prev = current.saturating_sub(1);
                    focus_item(prev);
                }
                "Enter" | " " => {
                    ev.prevent_default();
                    if let Some(id) = item_ids.get(current) {
                        on_select.run((col_idx, id.clone()));
                    }
                }
                // ArrowRight / ArrowLeft / Escape are handled at BrowserView
                // level for cross-column coordination.
                _ => {}
            }
        }
    };

    view! {
        <div
            class={style::BROWSER_COLUMN}
            style:width=move || format!("{}px", col_width.get())
            style:min-width=move || format!("{}px", col_width.get())
        >
            <ul
                node_ref=list_ref
                class={style::BROWSER_LIST}
                role="tree"
                aria-label=format!("Column {}", col_idx + 1)
                tabindex="0"
                on:keydown=on_keydown
            >
                {items
                    .into_iter()
                    .enumerate()
                    .map(|(item_idx, item)| {
                        let item_id = item.id.clone();
                        let item_id_click = item_id.clone();
                        let item_id_dbl = item_id.clone();
                        let is_leaf = item.is_leaf();
                        let node_kind_str = item.node_kind.clone();
                        let kind_class = if is_leaf {
                            style::BROWSER_LEAF
                        } else {
                            style::BROWSER_CONTAINER
                        };
                        let icon_svg = icon_renderer(&node_kind_str);

                        let is_selected = {
                            let item_id = item_id.clone();
                            Signal::derive(move || {
                                selected_id.get().as_deref() == Some(item_id.as_str())
                            })
                        };

                        // aria-expanded: only on containers, true when drilled.
                        // NOTE: This column browser uses separate <ul> columns
                        // rather than nested <ul role="group">. The aria-expanded
                        // attribute is technically for items with nested groups,
                        // but we use it here to convey drill state to assistive
                        // technology — an accepted deviation for the Miller
                        // column pattern.
                        let is_expanded = {
                            let item_id = item_id.clone();
                            Signal::derive(move || {
                                selected_id.get().as_deref() == Some(item_id.as_str())
                            })
                        };

                        view! {
                            <li
                                class=move || {
                                    let mut cls = String::from(style::BROWSER_ITEM);
                                    cls.push(' ');
                                    cls.push_str(kind_class);
                                    if is_selected.get() {
                                        cls.push(' ');
                                        cls.push_str(style::BROWSER_SELECTED);
                                    }
                                    cls
                                }
                                role="treeitem"
                                aria-selected=move || is_selected.get().to_string()
                                aria-expanded=move || {
                                    if is_leaf { None } else { Some(is_expanded.get().to_string()) }
                                }
                                tabindex="-1"
                                on:click=move |_| {
                                    focus_idx.set(Some(item_idx));
                                    on_select.run((col_idx, item_id_click.clone()));
                                }
                                on:dblclick=move |_| {
                                    if is_leaf { on_open.run(item_id_dbl.clone()); }
                                }
                                on:focus=move |_| { focus_idx.set(Some(item_idx)); }
                            >
                                <span class={style::BROWSER_ICON} aria-hidden="true" inner_html=icon_svg />
                                <span class={style::BROWSER_LABEL}>{item.label.clone()}</span>
                                {(!is_leaf).then(|| view! {
                                    <span
                                        class={style::BROWSER_CHEVRON}
                                        aria-hidden="true"
                                        inner_html=CHEVRON_RIGHT
                                    />
                                })}
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
            <div
                class={style::BROWSER_RESIZE_HANDLE}
                aria-hidden="true"
                on:pointerdown=on_handle_down
                on:pointermove=on_handle_move
                on:pointerup=on_handle_up
                on:lostpointercapture=on_handle_up
            />
        </div>
    }
}
