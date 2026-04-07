//! File explorer example — demonstrates `StaticTopologyProvider`.

use std::sync::Arc;

use leptos::prelude::*;
use leptos_column_browser::{
    ColumnBrowser, DEFAULT_CONTAINER_ICON, DEFAULT_LEAF_ICON, IconRenderer, Node, NodeId,
    StaticTopologyProvider,
};

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App);
}

/// Build a realistic fake filesystem tree.
fn make_filesystem() -> Vec<Node> {
    vec![
        Node::container(NodeId::root("Documents"), "Documents".into(), "folder")
            .with_child(
                Node::container(
                    NodeId::from_segments(["Documents", "Work"]),
                    "Work".into(),
                    "folder",
                )
                .with_child(Node::leaf(
                    NodeId::from_segments(["Documents", "Work", "report.pdf"]),
                    "report.pdf".into(),
                    "pdf",
                ))
                .with_child(Node::leaf(
                    NodeId::from_segments(["Documents", "Work", "budget.xlsx"]),
                    "budget.xlsx".into(),
                    "spreadsheet",
                )),
            )
            .with_child(Node::leaf(
                NodeId::from_segments(["Documents", "notes.txt"]),
                "notes.txt".into(),
                "text",
            )),
        Node::container(NodeId::root("Pictures"), "Pictures".into(), "folder").with_child(
            Node::leaf(
                NodeId::from_segments(["Pictures", "holiday.jpg"]),
                "holiday.jpg".into(),
                "image",
            ),
        ),
        Node::leaf(NodeId::root("README.md"), "README.md".into(), "text"),
    ]
}

/// Consumer icon renderer — maps `node_kind` ("container"/"leaf") to SVG.
fn make_icon_renderer() -> IconRenderer {
    Arc::new(|node_kind: &str| {
        if node_kind == "container" {
            DEFAULT_CONTAINER_ICON.to_owned()
        } else {
            DEFAULT_LEAF_ICON.to_owned()
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let (last_opened, set_last_opened) = signal(String::from("(none)"));

    let provider = StaticTopologyProvider {
        nodes: make_filesystem(),
    };

    view! {
        <div id="root">
            <div class="toolbar">
                "Last opened: " {move || last_opened.get()}
            </div>
            <ColumnBrowser
                provider=provider
                root_id=NodeId::root("__root__")
                visible_cols=3
                icon_renderer=make_icon_renderer()
                on_open=Callback::new(move |id: String| set_last_opened.set(id))
            />
        </div>
    }
}
