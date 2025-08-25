use std::{
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
    u64,
};

use bevy::{diagnostic::FrameCount, platform::hash::FixedState, prelude::*};

pub mod prelude {
    pub use crate::{ui_state_render, StateRender, StateUiAppExt};
}

#[derive(Component)]
pub struct RootNode<T> {
    phantom: PhantomData<T>,
}

pub trait StateRender {
    fn render(&self, commands: EntityCommands);
}

#[derive(Resource)]
pub struct RenderedHash<T> {
    phantom: PhantomData<T>,
    last_frame_check: u32,
    last_hash_value: u64,
}

pub trait StateUiAppExt {
    fn register_ui_state<TState>(&mut self) -> &mut Self
    where
        TState: Resource;
}

impl StateUiAppExt for App {
    fn register_ui_state<TState>(&mut self) -> &mut Self
    where
        TState: Resource,
    {
        self.insert_resource(RenderedHash::<TState> {
            phantom: PhantomData::default(),
            last_frame_check: 0,
            last_hash_value: 0,
        });
        self
    }
}

pub fn ui_state_render<TState>(
    mut commands: Commands,
    frame_count: Res<FrameCount>,
    mut hash: ResMut<RenderedHash<TState>>,
    state: Option<Res<TState>>,
    q_root: Query<Entity, With<RootNode<TState>>>,
) where
    TState: Resource + Hash + StateRender,
{
    let Some(state) = state else {
        for entity in q_root.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    if hash.last_frame_check == frame_count.0 {
        return;
    }

    let mut hasher = FixedState::default().build_hasher();
    state.hash(&mut hasher);
    let value = hasher.finish();

    if hash.last_hash_value == value {
        return;
    }

    hash.last_frame_check = frame_count.0;
    hash.last_hash_value = value;

    for entity in q_root.iter() {
        commands.entity(entity).despawn();
    }

    let commands = commands.spawn(RootNode::<TState> {
        phantom: PhantomData::default(),
    });

    state.render(commands);
}
