use bevy::prelude::*;
use bevy_state_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyStateUiPlugin::<GameState>::default().debug())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.init_resource::<GameState>();
}

#[derive(Resource, Debug, Default)]
struct GameState {
    count: usize,
}

impl StateRender for GameState {
    fn render(&self, mut commands: EntityCommands) {
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

fn on_click(_click: On<Pointer<Click>>, mut state: ResMut<GameState>) {
    state.count += 1;
}
