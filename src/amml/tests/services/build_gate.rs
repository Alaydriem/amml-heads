use alaydriem_heads::services::BuildGate;

#[test]
fn only_the_2660_mesh_size_passes() {
    assert!(BuildGate::accepts(0x260));
    assert!(!BuildGate::accepts(0x278));
    assert!(!BuildGate::accepts(0));
}
