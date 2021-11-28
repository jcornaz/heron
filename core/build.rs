fn main() {
    cfg_aliases::cfg_aliases! {
        // 2D feature is only enabled if 3D is not enabled
        dim2: { all(feature = "2d", not(feature = "3d")) },
        // 3D feature takes precedence over 2D feature
        dim3: { all(feature = "3d") }
    }
}
