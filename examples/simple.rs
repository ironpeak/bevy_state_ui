use app_ext::{Render, StateUiAppExt};
use bevy::prelude::*;
use bevy_state_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Simple'".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .register_ui::<State>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(State {});
}

#[derive(Resource)]
pub struct State {}

impl Render for State {
    fn render(&self, _commands: Commands) {
        info!("Rendering");
    }
}
