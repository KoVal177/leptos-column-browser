#![allow(missing_docs)]

use leptos_column_browser::{NodeFilter, NodeView};

fn leaf_view(id: &str, label: &str, node_type: &str) -> NodeView {
    NodeView {
        id: id.into(),
        label: label.into(),
        node_type: node_type.into(),
        node_kind: "leaf".into(),
        children: vec![],
    }
}

fn container_view(id: &str, label: &str, node_type: &str, children: Vec<NodeView>) -> NodeView {
    NodeView {
        id: id.into(),
        label: label.into(),
        node_type: node_type.into(),
        node_kind: "container".into(),
        children,
    }
}

fn sample_views() -> Vec<NodeView> {
    vec![
        container_view(
            "root",
            "Root",
            "group",
            vec![
                leaf_view("root/a.item", "a.item", "item"),
                leaf_view("root/notes.doc", "notes.doc", "doc"),
            ],
        ),
        leaf_view("top.item", "top.item", "item"),
    ]
}

#[test]
fn empty_filter_returns_all() {
    let nodes = sample_views();
    let result = NodeFilter::all().apply(&nodes);
    assert_eq!(result, nodes);
}

#[test]
fn is_empty_on_default() {
    assert!(NodeFilter::all().is_empty());
}

#[test]
fn filter_by_single_type() {
    let nodes = sample_views();
    let result = NodeFilter::by_type("item").apply(&nodes);
    // Container survives because it has an item child.
    assert_eq!(result.len(), 2);
    // Container only keeps the item child.
    assert_eq!(result[0].children.len(), 1);
    assert_eq!(result[0].children[0].node_type, "item");
    // Root-level leaf item survives.
    assert_eq!(result[1].id, "top.item");
}

#[test]
fn filter_by_type_prunes_empty_containers() {
    let nodes = vec![container_view(
        "empty",
        "Empty",
        "group",
        vec![leaf_view("empty/a.doc", "a.doc", "doc")],
    )];
    let result = NodeFilter::by_type("item").apply(&nodes);
    assert!(result.is_empty());
}

#[test]
fn filter_by_label_case_insensitive() {
    let nodes = vec![
        leaf_view("a", "Alpha Report", "item"),
        leaf_view("b", "Beta Plan", "doc"),
    ];
    let filter = NodeFilter {
        label_contains: Some("ALPHA".into()),
        ..NodeFilter::default()
    };
    let result = filter.apply(&nodes);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "a");
}

#[test]
fn filter_by_multiple_types() {
    let nodes = vec![
        leaf_view("a", "A", "item"),
        leaf_view("b", "B", "doc"),
        leaf_view("c", "C", "db"),
    ];
    let result = NodeFilter::by_types(vec!["item".into(), "db".into()]).apply(&nodes);
    assert_eq!(result.len(), 2);
}

#[test]
fn combined_type_and_label() {
    let nodes = vec![
        leaf_view("a", "Alpha Q1", "item"),
        leaf_view("b", "Beta Q1", "item"),
        leaf_view("c", "Alpha Q2", "doc"),
    ];
    let filter = NodeFilter {
        node_types: vec!["item".into()],
        label_contains: Some("alpha".into()),
        ..NodeFilter::default()
    };
    let result = filter.apply(&nodes);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "a");
}

#[test]
fn deep_tree_pruning() {
    let deep = container_view(
        "l1",
        "L1",
        "group",
        vec![container_view(
            "l1/l2",
            "L2",
            "group",
            vec![
                leaf_view("l1/l2/a.item", "a.item", "item"),
                leaf_view("l1/l2/b.doc", "b.doc", "doc"),
            ],
        )],
    );
    let result = NodeFilter::by_type("item").apply(&[deep]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].children.len(), 1);
    assert_eq!(result[0].children[0].children.len(), 1);
    assert_eq!(result[0].children[0].children[0].id, "l1/l2/a.item");
}

#[test]
fn serde_roundtrip() {
    let filter = NodeFilter {
        node_types: vec!["item".into()],
        label_contains: Some("test".into()),
        leaves_only: true,
    };
    let json = serde_json::to_string(&filter).expect("serialize");
    let back: NodeFilter = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(filter, back);
}
