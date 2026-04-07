#![allow(missing_docs)]

use leptos_column_browser::NodeId;

#[test]
fn root_creates_single_segment() {
    let id = NodeId::root("data");
    assert_eq!(id.canonical(), "data");
    assert_eq!(id.depth(), 1);
    assert_eq!(id.name(), "data");
}

#[test]
fn from_segments_builds_path() {
    let id = NodeId::from_segments(["a", "b", "c.item"]);
    assert_eq!(id.canonical(), "a/b/c.item");
    assert_eq!(id.depth(), 3);
    assert_eq!(id.name(), "c.item");
}

#[test]
fn parse_roundtrips() {
    let original = NodeId::from_segments(["a", "b", "c"]);
    let parsed = NodeId::parse(&original.canonical());
    assert_eq!(original, parsed);
}

#[test]
fn parent_returns_none_for_root() {
    let root = NodeId::root("root");
    assert!(root.parent().is_none());
}

#[test]
fn parent_strips_last_segment() {
    let id = NodeId::from_segments(["a", "b", "c"]);
    let parent = id.parent().expect("should have a parent");
    assert_eq!(parent.canonical(), "a/b");
}

#[test]
fn child_appends_segment() {
    let parent = NodeId::root("a");
    let child = parent.child("b");
    assert_eq!(child.canonical(), "a/b");
    assert_eq!(child.depth(), 2);
}

#[test]
fn segments_accessor() {
    let id = NodeId::from_segments(["a", "b", "c"]);
    assert_eq!(id.segments(), &["a", "b", "c"]);
}

#[test]
fn display_matches_canonical() {
    let id = NodeId::from_segments(["x", "y"]);
    assert_eq!(format!("{id}"), "x/y");
}

#[test]
fn serde_roundtrip() {
    let id = NodeId::from_segments(["a", "b.item"]);
    let json = serde_json::to_string(&id).expect("serialize");
    let back: NodeId = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(id, back);
}

#[test]
fn equality_and_hash() {
    use std::collections::HashSet;
    let a = NodeId::parse("a/b/c");
    let b = NodeId::parse("a/b/c");
    let c = NodeId::parse("a/b/d");
    assert_eq!(a, b);
    assert_ne!(a, c);

    let mut set = HashSet::new();
    set.insert(a.clone());
    assert!(set.contains(&b));
    assert!(!set.contains(&c));
}

#[test]
#[should_panic(expected = "at least one segment")]
fn empty_segments_panics() {
    NodeId::from_segments(Vec::<String>::new());
}

#[test]
#[should_panic(expected = "must not be empty")]
fn empty_segment_panics() {
    NodeId::from_segments(["a", "", "b"]);
}

#[test]
#[should_panic(expected = "must not contain '/'")]
fn slash_in_segment_panics() {
    NodeId::from_segments(["a/b"]);
}
