use std::{fmt::Debug, marker::PhantomData, sync::Mutex};

use bevy::{
    ecs::schedule::{BoxedCondition, ScheduleLabel},
    prelude::*,
};

pub mod prelude {
    pub use crate::{BevyStateUiPlugin, StateRender};
}

#[derive(Component)]
pub struct RootNode<T> {
    phantom: PhantomData<T>,
}

pub trait StateRender {
    fn render(&self, commands: EntityCommands);
}

#[derive(Resource)]
struct PreviousState<T> {
    value: Option<T>,
}

#[derive(Resource)]
struct DebugStateRender<T> {
    phantom: PhantomData<T>,
}

pub struct BevyStateUiPlugin<T, S: ScheduleLabel = Update> {
    phantom: PhantomData<T>,
    debug: bool,
    schedule: S,
    run_conditions: Mutex<Vec<BoxedCondition>>,
}

impl<T> Default for BevyStateUiPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            debug: false,
            schedule: Update,
            run_conditions: Mutex::new(Vec::new()),
        }
    }
}

impl<T, S: ScheduleLabel> BevyStateUiPlugin<T, S> {
    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn schedule<S2: ScheduleLabel>(self, schedule: S2) -> BevyStateUiPlugin<T, S2> {
        BevyStateUiPlugin {
            phantom: self.phantom,
            debug: self.debug,
            schedule,
            run_conditions: Mutex::new(self.run_conditions.into_inner().unwrap()),
        }
    }

    pub fn run_if<M>(self, condition: impl SystemCondition<M>) -> Self {
        self.run_conditions
            .lock()
            .unwrap()
            .push(Box::new(IntoSystem::into_system(condition)));
        self
    }
}

impl<T: Resource + Clone + PartialEq + Debug + StateRender, S: ScheduleLabel + Clone> Plugin
    for BevyStateUiPlugin<T, S>
{
    fn build(&self, app: &mut App) {
        app.insert_resource(PreviousState::<T> { value: None });
        if self.debug {
            app.insert_resource(DebugStateRender::<T> {
                phantom: PhantomData,
            });
        }
        let run_conditions = std::mem::take(&mut *self.run_conditions.lock().unwrap());
        let mut configs = ui_state_render::<T>.into_configs();
        for condition in run_conditions {
            configs.run_if_dyn(condition);
        }
        app.add_systems(self.schedule.clone(), configs);
    }
}

fn ui_state_render<T: Resource + Clone + PartialEq + Debug + StateRender>(
    mut commands: Commands,
    mut previous: ResMut<PreviousState<T>>,
    state: Option<Res<T>>,
    q_root: Query<Entity, With<RootNode<T>>>,
    debug: Option<Res<DebugStateRender<T>>>,
) {
    let Some(state) = state else {
        for entity in q_root.iter() {
            commands.entity(entity).despawn();
        }
        previous.value = None;
        return;
    };

    if !state.is_changed() {
        return;
    }

    if previous.value.as_ref() == Some(&*state) {
        return;
    }

    if debug.is_some() {
        info!("UI re-render: {:?}", &*state);
    }

    previous.value = Some((*state).clone());

    for entity in q_root.iter() {
        commands.entity(entity).despawn();
    }

    let commands = commands.spawn(RootNode::<T> {
        phantom: PhantomData,
    });

    state.render(commands);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Clone, PartialEq, Debug, Default)]
    struct TestState {
        value: i32,
    }

    impl StateRender for TestState {
        fn render(&self, mut commands: EntityCommands) {
            commands.insert(Node::default());
        }
    }

    #[derive(Resource, Clone, PartialEq, Debug, Default)]
    struct OtherState {
        name: String,
    }

    impl StateRender for OtherState {
        fn render(&self, mut commands: EntityCommands) {
            commands.insert(Node::default());
        }
    }

    #[derive(Resource, Clone, PartialEq, Debug, Default)]
    struct FloatState {
        position: f32,
    }

    impl StateRender for FloatState {
        fn render(&self, mut commands: EntityCommands) {
            commands.insert(Node::default());
        }
    }

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app
    }

    fn root_node_count<T: Send + Sync + 'static>(app: &mut App) -> usize {
        app.world_mut()
            .query_filtered::<Entity, With<RootNode<T>>>()
            .iter(app.world())
            .count()
    }

    fn root_node_entity<T: Send + Sync + 'static>(app: &mut App) -> Entity {
        app.world_mut()
            .query_filtered::<Entity, With<RootNode<T>>>()
            .single(app.world())
            .unwrap()
    }

    #[test]
    fn first_render_on_initial_state() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.insert_resource(TestState { value: 42 });

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 1);
    }

    #[test]
    fn re_render_on_actual_change() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.insert_resource(TestState { value: 1 });

        app.update();

        let entity_before = root_node_entity::<TestState>(&mut app);

        app.world_mut().resource_mut::<TestState>().value = 2;

        app.update();

        let entity_after = root_node_entity::<TestState>(&mut app);
        assert_ne!(entity_before, entity_after);

        let prev = app.world().resource::<PreviousState<TestState>>();
        assert_eq!(prev.value.as_ref().unwrap().value, 2);
    }

    #[test]
    fn no_re_render_on_same_value_mutation() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.insert_resource(TestState { value: 10 });

        app.update();

        let entity_before = root_node_entity::<TestState>(&mut app);

        // Access via DerefMut but write the same value
        app.world_mut().resource_mut::<TestState>().value = 10;

        app.update();

        let entity_after = root_node_entity::<TestState>(&mut app);
        assert_eq!(entity_before, entity_after);
    }

    #[test]
    fn ui_despawned_on_state_removal() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.insert_resource(TestState { value: 1 });

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 1);

        app.world_mut().remove_resource::<TestState>();

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 0);

        let prev = app.world().resource::<PreviousState<TestState>>();
        assert!(prev.value.is_none());
    }

    #[test]
    fn multiple_state_types() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.add_plugins(BevyStateUiPlugin::<OtherState>::default());
        app.insert_resource(TestState { value: 1 });
        app.insert_resource(OtherState {
            name: "hello".into(),
        });

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 1);
        assert_eq!(root_node_count::<OtherState>(&mut app), 1);
    }

    #[test]
    fn debug_mode_does_not_panic() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default().debug());
        app.insert_resource(TestState { value: 1 });

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 1);
    }

    #[test]
    fn float_state_works() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<FloatState>::default());
        app.insert_resource(FloatState { position: 3.14 });

        app.update();

        assert_eq!(root_node_count::<FloatState>(&mut app), 1);

        app.world_mut().resource_mut::<FloatState>().position = 2.71;

        app.update();

        assert_eq!(root_node_count::<FloatState>(&mut app), 1);

        let prev = app.world().resource::<PreviousState<FloatState>>();
        assert_eq!(prev.value.as_ref().unwrap().position, 2.71);
    }

    #[test]
    fn no_render_without_state() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());

        app.update();

        assert_eq!(root_node_count::<TestState>(&mut app), 0);
    }

    #[test]
    fn no_re_render_on_idle_frames() {
        let mut app = test_app();
        app.add_plugins(BevyStateUiPlugin::<TestState>::default());
        app.insert_resource(TestState { value: 5 });

        app.update();

        let entity = root_node_entity::<TestState>(&mut app);

        // Run several more updates with no mutations
        app.update();
        app.update();
        app.update();

        let entity_after = root_node_entity::<TestState>(&mut app);
        assert_eq!(entity, entity_after);
    }
}
