use app_ext::{Render, StateUiAppExt};
use bevy::{ecs::system::EntityCommands, prelude::*};
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
        .add_systems(Update, update_button_interactions)
        .register_ui::<State>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(State {
        interaction: Interaction::None,
    });
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct State {
    pub interaction: Interaction,
}

impl Render for State {
    fn root(&self) -> impl Bundle {
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
        info!("Rendering");

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
                    background_color: match self.interaction {
                        Interaction::Pressed => Color::srgb(1.0, 1.0, 1.0).into(),
                        Interaction::Hovered => Color::srgb(0.5, 0.5, 0.5).into(),
                        Interaction::None => Color::srgb(0.0, 0.0, 0.0).into(),
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
        if interaction == &state.interaction {
            continue;
        }
        state.interaction = *interaction;
    }
}
