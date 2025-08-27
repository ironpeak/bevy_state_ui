# bevy_state_ui

A simple UI library for [Bevy](https://bevy.org) that renders UI directly from application state.

Instead of manually managing UI entities, you declare your state and its `render` function, and `bevy_state_ui` will automatically keep the UI in sync whenever the state changes.

## Features

* Declarative: define your UI based on a state struct.
* Efficient: only re-renders UI when the state hash changes.
* Familiar: integrates seamlessly with Bevy’s ECS and UI system.
* Simple: minimal API, easy to get started.

## Installation

Add to your `Cargo.toml`:

~~~toml
[dependencies]
bevy_state_ui = "0.5"
~~~

## Example

Here’s a minimal app with a button that changes appearance when hovered:

[examples/simple.rs](examples/simple.rs)


Run it with:

~~~bash
cargo run --example simple
~~~

## Why use this instead of plain Bevy UI?

Normally in Bevy, building UI means:

* Spawning a hierarchy of `Node`/`Text`/`Button` entities in startup systems.
* Keeping track of `Entity` IDs or `Querys` to update them later.
* Writing update logic that mutates styles, colors, and text when game state changes.

This often leads to boilerplate and imperative code. For example:

* You check a `Res<State>` inside systems.
* You manually update the `BackgroundColor` of a button when `state.hovered` changes.
* You may need to despawn/respawn UI trees when state transitions are large.

With bevy_state_ui, you flip the model:

* Define your UI as a pure function of state (`impl StateRender for State`).
* The library automatically detects when state changes (via hashing).
* The old UI is despawned and re-rendered from the new state.

This means:

✅ Less boilerplate
✅ More predictable UI (no stale entity state)
✅ A workflow similar to React/Elm/SwiftUI for Bevy

If your mental model of UI is "render(state) → tree of UI nodes", this library gives you exactly that.

## How it works

* You define a `State` struct that implements:
  * `Resource` (so it can live in the ECS world).
  * `Hash` (to efficiently detect when state changes).
  * `StateRender` (your declarative UI description).
* The system `ui_state_render::<State>`:
  * Computes a hash of the state each frame.
  * If it changed, despawns the previous UI root and calls your `render` function.
  * Keeps the UI automatically in sync with your state.

This lets you think of UI as a pure function of state, much like React, Elm, or SwiftUI.