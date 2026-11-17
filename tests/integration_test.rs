// Integration test for SigmaOS-ethereum-cex
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use ethereum_cex as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
