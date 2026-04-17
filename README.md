# bevy_state_ui

A simple UI library for [Bevy](https://bevy.org) that renders UI directly from application state.

Instead of manually managing UI entities, you declare your state and its `render` function, and `bevy_state_ui` will automatically keep the UI in sync whenever the state changes.

## Features

- Declarative: define your UI based on a state struct.
- Efficient: only re-renders UI when the state actually changes (PartialEq comparison).
- Familiar: integrates seamlessly with Bevy's ECS and UI system.
- Simple: minimal API, easy to get started.
- Configurable: choose which schedule to run in and add run conditions.
- Queryable: `RootNode<T>` is public for direct entity access and manual cleanup.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_state_ui = "0.8"
```

## Examples

Here's a minimal app with clickable text that increments a counter:

[examples/simple.rs](examples/simple.rs)

```bash
cargo run --example simple
```

A more advanced example with a pause menu, demonstrating custom schedules, run conditions, and `RootNode<T>` cleanup:

[examples/pause.rs](examples/pause.rs)

```bash
cargo run --example pause
```

## Why use this instead of plain Bevy UI?

Normally in Bevy, building UI means:

- Spawning a hierarchy of `Node`/`Text`/`Button` entities in startup systems.
- Keeping track of `Entity` IDs or `Querys` to update them later.
- Writing update logic that mutates styles, colors, and text when game state changes.

This often leads to boilerplate and imperative code. For example:

- You check a `Res<State>` inside systems.
- You manually update the `BackgroundColor` of a button when `state.hovered` changes.
- You may need to despawn/respawn UI trees when state transitions are large.

With bevy_state_ui, you flip the model:

- Define your UI as a pure function of state (`impl StateRender for State`).
- The library automatically detects when state changes (via PartialEq comparison).
- The old UI is despawned and re-rendered from the new state.

This means:

✅ Less boilerplate
✅ More predictable UI (no stale entity state)
✅ A workflow similar to React/Elm/SwiftUI for Bevy

If your mental model of UI is "render(state) → tree of UI nodes", this library gives you exactly that.

## Configuration

The plugin supports a builder pattern for advanced configuration:

```rust
use bevy::prelude::*;
use bevy_state_ui::prelude::*;

app.add_plugins(
    BevyStateUiPlugin::<MyState>::default()
        .schedule(FixedUpdate)                      // run in FixedUpdate instead of Update
        .run_if(in_state(GameState::Running))       // only render when the game is running
        .debug()                                    // log re-renders
);
```

- **`.schedule(schedule)`** — configures which Bevy schedule the render system runs in (default: `Update`). Useful for syncing UI updates with `FixedUpdate` logic.
- **`.run_if(condition)`** — adds a run condition to the render system. When the condition is false, the system won't run at all. Best used with state resources that persist across the condition (e.g., a timer that stays visible but frozen). For UI that should appear/disappear based on app state, prefer inserting/removing the resource instead (see [`examples/pause.rs`](examples/pause.rs)).
- **`.debug()`** — enables `info!` logging on every re-render.

### RootNode\<T\>

Each UI tree is rooted under a `RootNode<T>` component. Since it's public, you can query for it directly:

```rust
fn count_active_uis(q: Query<Entity, With<RootNode<MyState>>>) {
    info!("Active UI roots: {}", q.iter().count());
}
```

## How it works

- You define a state struct that implements:
  - `Resource` (so it can live in the ECS world).
  - `Clone + PartialEq` (to detect when the state actually changes).
  - `Debug` (for optional debug logging).
  - `StateRender` (your declarative UI description).
- Register the plugin: `app.add_plugins(BevyStateUiPlugin::<MyState>::default())` (optionally configure with `.schedule()`, `.run_if()`, `.debug()`).
- The plugin adds a render system (to the configured schedule, default `Update`) that:
  - Uses Bevy's `is_changed()` as a fast path (zero cost when nothing was mutated).
  - Compares the current state against a stored previous value via `PartialEq`.
  - If the state truly changed, despawns the previous `RootNode<T>` and calls your `render` function.
  - This two-layer approach avoids false re-renders from `DerefMut` access that doesn't change the value.

This lets you think of UI as a pure function of state, much like React, Elm, or SwiftUI.
- The UI tree is rooted under a `RootNode<T>` component, which is public — you can query for it to inspect or manually despawn the UI.
