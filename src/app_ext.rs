use std::{
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
};

use bevy::{ecs::system::EntityCommands, prelude::*, utils::AHasher};

#[derive(Component)]
pub struct Root<T> {
    phantom: PhantomData<T>,
}

#[derive(Component)]
pub struct RootHash<T> {
    phantom: PhantomData<T>,
    hash: u64,
}

pub trait Render {
    fn root(&self) -> impl Bundle;
    fn render(&self, commands: EntityCommands);
}

pub trait StateUiAppExt {
    fn register_ui<T>(&mut self) -> &mut Self
    where
        T: Resource + Render;

    fn register_ui_with_hash<T>(&mut self) -> &mut Self
    where
        T: Resource + Render + Hash;
}

impl StateUiAppExt for App {
    fn register_ui<T>(&mut self) -> &mut Self
    where
        T: Resource + Render,
    {
        self.add_systems(Update, update::<T>.run_if(resource_changed::<T>));
        self
    }

    fn register_ui_with_hash<T>(&mut self) -> &mut Self
    where
        T: Resource + Render + Hash,
    {
        self.add_systems(Update, update_with_hash::<T>.run_if(resource_changed::<T>));
        self
    }
}

pub fn update<T>(mut commands: Commands, query: Query<Entity, With<Root<T>>>, state: Res<T>)
where
    T: Resource + Render,
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    state.render(commands.spawn((
        state.root(),
        Root {
            phantom: PhantomData::<T>,
        },
    )));
}

pub fn update_with_hash<T>(
    mut commands: Commands,
    query: Query<(Entity, &RootHash<T>)>,
    state: Res<T>,
) where
    T: Resource + Render + Hash,
{
    let mut hasher: AHasher = BuildHasherDefault::default().build_hasher();
    state.hash(&mut hasher);
    let hash = hasher.finish();

    if let Ok((entity, root)) = query.get_single() {
        if root.hash == hash {
            debug!("State hashes match, skipping rerender");
            return;
        }

        commands.entity(entity).despawn_recursive();
    } else {
        if !query.is_empty() {
            warn!("Multiple root entities detected, skipping rerender");
            return;
        }
    }

    state.render(commands.spawn((
        state.root(),
        RootHash {
            phantom: PhantomData::<T>,
            hash,
        },
    )));
}
