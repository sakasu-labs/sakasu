use sakasu_core::MerkleTree;

#[test]
fn empty_tree_root_is_zero() {
    let t = MerkleTree::new();
    assert_eq!(t.root(), [0u8; 32]);
}

#[test]
fn root_changes_on_insert() {
    let mut t = MerkleTree::new();
    let r0 = t.root();
    t.insert([1u8; 32]);
    let r1 = t.root();
    t.insert([2u8; 32]);
    let r2 = t.root();
    assert_ne!(r0, r1);
    assert_ne!(r1, r2);
}

#[test]
fn root_deterministic_under_same_inputs() {
    let mut a = MerkleTree::new();
    let mut b = MerkleTree::new();
    for i in 0..7u8 {
        a.insert([i; 32]);
        b.insert([i; 32]);
    }
    assert_eq!(a.root(), b.root());
}
