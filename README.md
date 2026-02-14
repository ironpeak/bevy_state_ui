# bevy_state_ui

A simple Bevy plugin that re-renders UI from application state. Define your UI as a function of state, and the plugin handles the rest.

## Features

- **Declarative** - define UI based on a state struct, not imperative entity management.
- **Reactive** - only re-renders when the state actually changes, using Bevy's built-in change detection.
- **Simple** - minimal API surface: one trait, one plugin.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_state_ui = "0.8"
```

## Usage

1. Define a state struct that implements `Resource`, `Debug`, and `StateRender`:

```rust
#[derive(Resource, Debug, Default)]
struct GameState {
    count: usize,
}

impl StateRender for GameState {
    fn render(&self, mut commands: EntityCommands) {
        commands
            .insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(Text::new(format!("Count: {}", self.count)));
            });
    }
}
```

2. Add the plugin and insert your resource whenever you're ready:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyStateUiPlugin::<GameState>::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<GameState>();
}
```

The resource doesn't need to exist at startup. The plugin handles `Option<Res<T>>`, so you can insert or remove the resource at any time and the UI will appear or disappear accordingly.

### Debug logging

Chain `.debug()` to log state changes to stdout:

```rust
.add_plugins(BevyStateUiPlugin::<GameState>::default().debug())
```

### Full example

See [examples/simple.rs](examples/simple.rs). Run it with:

```bash
cargo run --example simple
```

## How it works

- `BevyStateUiPlugin<T>` registers an `Update` system for your state type.
- Each frame, the system checks `Res<T>::is_changed()`.
- If the state changed, the previous root UI entity is despawned and `StateRender::render` is called to rebuild the UI tree.
- If the resource is removed, the UI is despawned and nothing is rendered.

## When not to use this

This plugin takes a full teardown-and-rebuild approach to UI updates. This is a deliberate tradeoff for simplicity, but it comes with limitations:

- **Frequent state changes** - If your state updates every frame (e.g. a real-time HUD tracking position, health, scores, etc.), the entire UI tree is despawned and respawned every frame. For large or deep UI hierarchies, this can be expensive.
- **Animations and transitions** - Since the UI is rebuilt from scratch on each change, there is no continuity between frames. CSS-like transitions or entity-based animations won't carry over.
- **Large UI trees** - The cost of despawn/respawn scales with the size of the UI hierarchy. For complex layouts with many nodes, the overhead may be noticeable.

This plugin works best for UI that changes infrequently in response to discrete events (menu screens, settings panels, inventory views, dialogue boxes, etc.) rather than UI that needs to update continuously.
