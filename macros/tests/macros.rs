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
#[case(MyLayer::World)]
#[case(MyLayer::Player)]
#[case(MyLayer::Enemies)]
fn from_bits_is_inverse_of_to_bits(#[case] layer: MyLayer) {
    assert_eq!(MyLayer::from_bits(layer.to_bits()), layer)
}

#[rstest]
#[case(MyLayer::World, 1)]
#[case(MyLayer::Player, 2)]
#[case(MyLayer::Enemies, 4)]
fn returns_expected_bits(#[case] layer: MyLayer, #[case] expected_bits: u16) {
    assert_eq!(layer.to_bits(), expected_bits)
}

#[rstest]
#[case(MyLayer::World, 1)]
#[case(MyLayer::Player, 2)]
#[case(MyLayer::Enemies, 4)]
fn returns_expected_layer_from_bits(#[case] expected_layer: MyLayer, #[case] bits: u16) {
    assert_eq!(MyLayer::from_bits(bits), expected_layer)
}
