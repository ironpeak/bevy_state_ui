use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component)]
pub struct Root<T> {
    phantom: PhantomData<T>,
}

pub trait Render {
    fn render(&self, commands: Commands);
}

pub trait StateUiAppExt {
    fn register_ui<T>(&mut self) -> &mut Self
    where
        T: Resource + Render;
}

impl StateUiAppExt for App {
    fn register_ui<T>(&mut self) -> &mut Self
    where
        T: Resource + Render,
    {
        self.add_systems(Update, update::<T>.run_if(resource_changed::<T>));
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

    state.render(commands);
}
