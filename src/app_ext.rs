use std::{
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
    sync::{Arc, Mutex},
    u64,
};

use bevy::{prelude::*, utils::AHasher};

#[derive(Resource)]
pub struct RenderedHash<T> {
    phantom: PhantomData<T>,
    hash: Arc<Mutex<u64>>,
}

pub trait StateUiAppExt {
    fn register_ui_state<T>(&mut self) -> &mut Self
    where
        T: Resource;
}

impl StateUiAppExt for App {
    fn register_ui_state<T>(&mut self) -> &mut Self
    where
        T: Resource,
    {
        self.insert_resource(RenderedHash::<T> {
            phantom: PhantomData::default(),
            hash: Arc::new(Mutex::new(0)),
        });
        self
    }
}

pub fn ui_state_changed<T>(previous_hash: Res<RenderedHash<T>>, state: Option<Res<T>>) -> bool
where
    T: Resource + Hash,
{
    let Some(state) = state else { return false };

    if !state.is_added() && !state.is_changed() {
        return false;
    }

    let mut hasher: AHasher = BuildHasherDefault::default().build_hasher();
    state.hash(&mut hasher);
    let hash = hasher.finish();

    let Ok(mut previous_hash) = previous_hash.hash.lock() else {
        warn!("Failed to lock hash mutex");
        return false;
    };

    if *previous_hash == hash {
        debug!("State hashes match, skipping rerender");
        return false;
    }

    *previous_hash = hash;

    true
}
