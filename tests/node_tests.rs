#![allow(missing_docs)]

use leptos_column_browser::{Node, NodeId, NodeKind};

fn sample_tree() -> Node {
    Node::root(NodeId::root("root"), "Root".into(), "group")
        .with_child(
            Node::container(
                NodeId::from_segments(["root", "alpha"]),
                "Alpha".into(),
                "group",
            )
            .with_child(Node::leaf(
                NodeId::from_segments(["root", "alpha", "a.item"]),
                "a.item".into(),
                "item",
            ))
            .with_child(Node::leaf(
                NodeId::from_segments(["root", "alpha", "b.item"]),
                "b.item".into(),
                "item",
            )),
        )
        .with_child(Node::leaf(
            NodeId::from_segments(["root", "notes.doc"]),
            "notes.doc".into(),
            "doc",
        ))
}

#[test]
fn container_constructor() {
    let node = Node::container(NodeId::root("x"), "X".into(), "group");
    assert!(node.node_kind.is_container());
    assert!(node.children.is_empty());
}

#[test]
fn leaf_constructor() {
    let node = Node::leaf(NodeId::root("x"), "X".into(), "item");
    assert!(node.node_kind.is_leaf());
}

#[test]
fn count_includes_all_descendants() {
    let tree = sample_tree();
    assert_eq!(tree.count(), 5);
}

#[test]
fn leaf_nodes_returns_only_leaves() {
    let tree = sample_tree();
    let leaves = tree.leaf_nodes();
    assert_eq!(leaves.len(), 3);
    assert!(leaves.iter().all(|n| n.node_kind.is_leaf()));
}

#[test]
fn find_returns_self() {
    let tree = sample_tree();
    let found = tree.find(&NodeId::root("root")).expect("should find root");
    assert_eq!(found.label, "Root");
}

#[test]
fn find_returns_deep_child() {
    let tree = sample_tree();
    let id = NodeId::from_segments(["root", "alpha", "b.item"]);
    let found = tree.find(&id).expect("should find b.item");
    assert_eq!(found.label, "b.item");
}

#[test]
fn find_returns_none_for_missing() {
    let tree = sample_tree();
    assert!(tree.find(&NodeId::parse("root/nope")).is_none());
}

#[test]
fn to_view_creates_flat_node() {
    let node = Node::leaf(NodeId::root("test"), "Test".into(), "item");
    let view = node.to_view();
    assert_eq!(view.id, "test");
    assert_eq!(view.label, "Test");
    assert_eq!(view.node_type, "item");
    assert_eq!(view.node_kind, "leaf");
    assert!(view.children.is_empty());
}

#[test]
fn to_view_reports_container_kind() {
    let node = Node::container(NodeId::root("dir"), "Dir".into(), "group");
    let view = node.to_view();
    assert_eq!(view.node_kind, "container");
}

#[test]
fn node_kind_methods() {
    assert!(NodeKind::Leaf.is_leaf());
    assert!(!NodeKind::Leaf.is_container());
    assert!(NodeKind::Container.is_container());
    assert!(!NodeKind::Container.is_leaf());
}

#[test]
fn with_children_extends() {
    let children = vec![
        Node::leaf(NodeId::root("a"), "A".into(), "item"),
        Node::leaf(NodeId::root("b"), "B".into(), "doc"),
    ];
    let root = Node::container(NodeId::root("r"), "R".into(), "group").with_children(children);
    assert_eq!(root.children.len(), 2);
}

#[test]
fn node_view_helpers() {
    let view = leptos_column_browser::NodeView {
        id: "test".into(),
        label: "Test".into(),
        node_type: "item".into(),
        node_kind: "leaf".into(),
        children: vec![],
    };
    assert!(view.is_leaf());
    assert!(!view.is_container());
}
