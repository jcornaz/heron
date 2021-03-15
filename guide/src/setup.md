# Setup

## Add the dependency in your `Cargo.toml`

**For a 3d game:**
```toml
bevy = "^0.4.0"
heron = "0.2.0"
```

**For a 2d game:**
```toml
bevy = "^0.4.0"
heron = { version = "0.2.0", default-features = false, features = ["2d"] }
```

## Add the plugin

```rust,no_run
use bevy::prelude::*;
use heron::prelude::*;
fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin::default())
    // ... Add your resources and systems
    .run();
}
```
