use bevy::prelude::*;
use bevy_state_ui::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Running,
    Paused,
}

#[derive(Resource, Clone, PartialEq, Debug)]
struct GameTimer {
    elapsed: f32,
}

#[derive(Resource, Clone, PartialEq, Debug)]
struct PauseMenu;

impl StateRender for GameTimer {
    fn render(&self, mut commands: EntityCommands) {
        let minutes = (self.elapsed / 60.0) as u32;
        let seconds = self.elapsed % 60.0;

        commands
            .insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new(format!("Time: {:02}:{:05.2}", minutes, seconds)),
                    TextFont {
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ));

                parent.spawn((
                    Text::new("Press ESC to pause"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                    Node {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                ));
            });
    }
}

impl StateRender for PauseMenu {
    fn render(&self, mut commands: EntityCommands) {
        commands
            .insert(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            })
            .insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("PAUSED"),
                    TextFont {
                        font_size: 72.0,
                        ..default()
                    },
                    TextColor::WHITE,
                ));

                parent.spawn((
                    Text::new("Press ESC to resume"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                    Node {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                ));
            });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(
            BevyStateUiPlugin::<GameTimer>::default()
                .schedule(FixedUpdate)
                .run_if(in_state(GameState::Running)),
        )
        .add_plugins(BevyStateUiPlugin::<PauseMenu>::default())
        .insert_resource(GameTimer { elapsed: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_pause)
        .add_systems(
            FixedUpdate,
            tick_timer.run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::Paused), insert_pause_menu)
        .add_systems(OnExit(GameState::Paused), remove_pause_menu)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
        }
    }
}

fn tick_timer(mut timer: ResMut<GameTimer>, time: Res<Time>) {
    timer.elapsed += time.delta_secs();
}

fn insert_pause_menu(mut commands: Commands) {
    commands.insert_resource(PauseMenu);
}

fn remove_pause_menu(mut commands: Commands) {
    commands.remove_resource::<PauseMenu>();
}
