//! Tests the snapshotting function.
//!
//! Requires its own binary so that other tests running simultaneously don't
//! affect the environment.

extern crate kankyo;

use kankyo::*;

#[test]
fn test_snapshot() {
    utils::set_variables(&[("A", "B")]);
    let snap = snapshot();
    assert!(snap.contains_key("A"));
    let snap_length = snap.len();

    // Add in the new key and test that the old snap didn't change in length,
    // and that the new snap has only one extra key
    utils::set_variables(&[("C", "D")]);

    assert_eq!(snap.len(), snap_length);

    assert_eq!(snapshot().len(), snap_length + 1);
}
