use std::marker::PhantomData;

use bevy::prelude::*;

pub mod prelude {
    pub use crate::BevyStateUiPlugin;
    pub use crate::StateRender;
}

/// Marker component for the root UI entity managed by the plugin.
#[derive(Component)]
struct RootNode<T> {
    phantom: PhantomData<T>,
}

/// Implement this trait on a [`Resource`] to define how it renders as UI.
///
/// The provided [`EntityCommands`] refers to the root entity that the plugin
/// spawns. Insert UI components and children from here.
pub trait StateRender {
    fn render(&self, commands: EntityCommands);
}

/// Marker resource inserted when debug logging is enabled for a state type.
#[derive(Resource)]
struct DebugStateRender<T> {
    phantom: PhantomData<T>,
}

/// A Bevy plugin that re-renders UI whenever a [`Resource`] of type `T` changes.
///
/// The plugin registers an [`Update`] system that uses Bevy's change detection
/// to determine when `T` has been modified. On change, the existing root UI
/// entity is despawned and [`StateRender::render`] is called to rebuild it.
///
/// The resource does not need to exist at startup. The system handles
/// `Option<Res<T>>`, so the resource can be inserted or removed at any time.
///
/// # Usage
/// ```no_run
/// app.add_plugins(BevyStateUiPlugin::<MyState>::default());
/// ```
///
/// Enable debug logging with the builder method:
/// ```no_run
/// app.add_plugins(BevyStateUiPlugin::<MyState>::default().debug());
/// ```
pub struct BevyStateUiPlugin<T: Resource + std::fmt::Debug + StateRender> {
    phantom: PhantomData<T>,
    debug: bool,
}

impl<T: Resource + std::fmt::Debug + StateRender> BevyStateUiPlugin<T> {
    /// Enables debug logging. When active, prints the state value to stdout
    /// each time the UI is re-rendered.
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

impl<T: Resource + std::fmt::Debug + StateRender> Plugin for BevyStateUiPlugin<T> {
    fn build(&self, app: &mut App) {
        if self.debug {
            app.insert_resource(DebugStateRender::<T> {
                phantom: PhantomData,
            });
        }
        app.add_systems(Update, ui_state_render::<T>);
    }
}

impl<T: Resource + std::fmt::Debug + StateRender> Default for BevyStateUiPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            debug: false,
        }
    }
}

/// System that re-renders UI when the state resource changes.
///
/// If the resource is removed, the root entity is despawned (clearing the UI).
/// If the resource exists and has changed since the last run, the old root
/// entity is despawned and a new one is spawned via [`StateRender::render`].
fn ui_state_render<T>(
    mut commands: Commands,
    state: Option<Res<T>>,
    debug: Option<Res<DebugStateRender<T>>>,
    root: Query<Entity, With<RootNode<T>>>,
) where
    T: Resource + std::fmt::Debug + StateRender,
{
    let Some(state) = state else {
        for entity in root.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    if state.is_changed() {
        if debug.is_some() {
            println!("[View Rendered] {:?}", state.as_ref());
        }
    } else {
        return;
    }

    for entity in root.iter() {
        commands.entity(entity).despawn();
    }

    let commands = commands.spawn(RootNode::<T> {
        phantom: PhantomData,
    });

    state.render(commands);
}
