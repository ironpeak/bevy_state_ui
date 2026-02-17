use bevy::{prelude::*, window::PresentMode};
use bevy_state_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simple'".to_string(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .add_systems(Update, ui_state_render::<State>)
        .register_ui_state::<State>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<State>();
}

#[derive(Resource, Hash, Debug, Default)]
pub struct State {
    pub count: u128,
}

impl StateRender for State {
    fn render(&self, mut commands: EntityCommands) {
        info!("UI Rendered!");
        info!("{self:?}");

        commands
            .insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("Count: {}", self.count)),
                    TextColor::WHITE,
                    Node {
                        position_type: PositionType::Absolute,
                        top: percent(42.5),
                        left: percent(35),
                        ..default()
                    },
                ));

                parent
                    .spawn((
                        Text::new("Click me to increment the Count!"),
                        Node {
                            position_type: PositionType::Absolute,
                            top: percent(50),
                            left: percent(35),
                            ..default()
                        },
                    ))
                    .observe(on_click)
                    .observe(|out: On<Pointer<Out>>, mut texts: Query<&mut TextColor>| {
                        let mut text_color = texts.get_mut(out.entity).unwrap();
                        text_color.0 = Color::WHITE;
                    })
                    .observe(
                        |over: On<Pointer<Over>>, mut texts: Query<&mut TextColor>| {
                            let mut color = texts.get_mut(over.entity).unwrap();
                            color.0 = bevy::color::palettes::tailwind::CYAN_400.into();
                        },
                    );
            });
    }
}

fn on_click(_click: On<Pointer<Click>>, mut state: ResMut<State>) {
    state.count += 1;
}
