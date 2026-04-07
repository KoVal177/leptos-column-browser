//! Rust Crates Explorer — demonstrates async `TopologyProvider` without network I/O.

use std::sync::Arc;

use leptos::prelude::*;
use leptos_column_browser::{
    ColumnBrowser, DEFAULT_CONTAINER_ICON, DEFAULT_LEAF_ICON, IconRenderer, NodeId,
};

mod crates_provider;
use crates_provider::CratesProvider;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App);
}

fn make_icon_renderer() -> IconRenderer {
    Arc::new(|node_kind: &str| {
        if node_kind == "category" || node_kind == "container" {
            DEFAULT_CONTAINER_ICON.to_owned()
        } else {
            DEFAULT_LEAF_ICON.to_owned()
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let (last_opened, set_last_opened) = signal(String::from("(none)"));

    let provider = CratesProvider::new();

    view! {
        <div style="height: 100vh; display: flex; flex-direction: column;">
            <div style="padding: 0.75rem 1rem; background: #181825; color: #cdd6f4; font: 13px monospace; border-bottom: 1px solid #313244; display: flex; justify-content: space-between; align-items: center;">
                <span>"Rust Crates Explorer — async TopologyProvider demo"</span>
                <span style="color: #6c7086;">"Opened: " {move || last_opened.get()}</span>
            </div>
            <ColumnBrowser
                provider=provider
                root_id=NodeId::root("__root__")
                visible_cols=4
                icon_renderer=make_icon_renderer()
                on_open=Callback::new(move |id: String| set_last_opened.set(id))
            />
        </div>
    }
}
