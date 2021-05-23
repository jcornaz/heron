use rstest::rstest;

use heron_core::Layer;
use heron_macros::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Layer)]
enum MyLayer {
    World,
    Player,
    Enemies,
}

#[rstest]
#[case(MyLayer::World, 1)]
#[case(MyLayer::Player, 2)]
#[case(MyLayer::Enemies, 4)]
fn returns_expected_bits(#[case] layer: MyLayer, #[case] expected_bits: u16) {
    assert_eq!(layer.to_bits(), expected_bits)
}

#[test]
fn returns_expected_all_bits_mask() {
    assert_eq!(MyLayer::all_bits(), 7)
}
