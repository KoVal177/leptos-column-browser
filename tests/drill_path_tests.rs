#![allow(missing_docs)]

use leptos_column_browser::{DrillPath, NodeId};

#[test]
fn empty_path_has_zero_depth() {
    assert_eq!(DrillPath::empty().depth(), 0);
}

#[test]
fn select_grows_depth() {
    let path = DrillPath::empty().select(0, NodeId::root("a"));
    assert_eq!(path.depth(), 1);
}

#[test]
fn select_at_same_column_replaces() {
    let path = DrillPath::empty()
        .select(0, NodeId::root("a"))
        .select(0, NodeId::root("b"));
    assert_eq!(path.depth(), 1);
    assert_eq!(path.at(0).unwrap().canonical(), "b");
}

#[test]
fn select_truncates_deeper_segments() {
    let path = DrillPath::empty()
        .select(0, NodeId::root("a"))
        .select(1, NodeId::from_segments(["a", "b"]))
        .select(0, NodeId::root("x"));
    assert_eq!(path.depth(), 1);
    assert_eq!(path.at(0).unwrap().canonical(), "x");
}

#[test]
fn pop_removes_last_segment() {
    let path = DrillPath::empty()
        .select(0, NodeId::root("a"))
        .select(1, NodeId::from_segments(["a", "b"]));
    let popped = path.pop();
    assert_eq!(popped.depth(), 1);
    assert_eq!(popped.at(0).unwrap().canonical(), "a");
}

#[test]
fn pop_on_empty_is_noop() {
    assert_eq!(DrillPath::empty().pop(), DrillPath::empty());
}
