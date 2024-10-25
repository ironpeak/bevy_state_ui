use bevy::{ecs::system::EntityCommands, prelude::*, window::PresentMode};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_state_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Simple Hash'".to_string(),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                }),
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_button_interactions)
        .register_ui_with_hash::<State>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(State { hovered: false });
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Hash)]
pub struct State {
    pub hovered: bool,
}

impl Render for State {
    fn root(&self) -> impl Bundle {
        info!("root");
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }
    }

    fn render(&self, mut commands: EntityCommands) {
        info!("render");
        commands.with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(15.0),
                        top: Val::Percent(42.5),
                        left: Val::Percent(30.0),
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: if self.hovered {
                        Color::srgb(1.0, 1.0, 1.0).into()
                    } else {
                        Color::srgb(0.0, 0.0, 0.0).into()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "I am a button",
                        TextStyle {
                            color: Color::srgb(0.0, 0.0, 0.0),
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                });
        });
    }
}

fn update_button_interactions(
    mut state: ResMut<State>,
    q_interaction: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    if q_interaction.is_empty() {
        return;
    }
    for interaction in &q_interaction {
        match interaction {
            Interaction::None | Interaction::Pressed => {
                state.hovered = false;
            }
            Interaction::Hovered => {
                state.hovered = true;
            }
        }
    }
}
