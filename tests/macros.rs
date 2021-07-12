#![cfg(any(dim2, dim3))]

use rstest::rstest;

use heron_core::PhysicsLayer;
use heron_macros::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PhysicsLayer)]
enum MyLayer {
    World,
    Player,
    Enemies,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PhysicsLayer)]
#[allow(unused)]
enum MaxLayerCount {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Aa,
    Ab,
    Ac,
    Ad,
    Ae,
    Af,
}

#[rstest]
#[case(MyLayer::World, 1)]
#[case(MyLayer::Player, 2)]
#[case(MyLayer::Enemies, 4)]
fn returns_expected_bits(#[case] layer: MyLayer, #[case] expected_bits: u32) {
    assert_eq!(layer.to_bits(), expected_bits)
}

#[test]
fn returns_expected_all_bits_mask() {
    assert_eq!(MyLayer::all_bits(), 0b111)
}

#[test]
fn max_layers_bits() {
    assert_eq!(MaxLayerCount::all_bits(), u32::MAX);
    assert_eq!(MaxLayerCount::all_bits(), 0xffffffff);
}
