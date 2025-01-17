use std::{
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
    sync::{Arc, Mutex},
    u64,
};

use bevy::{core::FrameCount, prelude::*, utils::AHasher};

#[derive(Resource)]
pub struct RenderedHash<T> {
    phantom: PhantomData<T>,
    hash: Arc<Mutex<(u64, u32)>>,
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
            hash: Arc::new(Mutex::new((0, 0))),
        });
        self
    }
}

pub fn ui_state_changed<T>(
    frame_count: Res<FrameCount>,
    previous_hash: Res<RenderedHash<T>>,
    state: Option<Res<T>>,
) -> bool
where
    T: Resource + Hash,
{
    let Some(state) = state else { return false };

    let Ok(mut mutex) = previous_hash.hash.lock() else {
        warn!("Failed to lock hash mutex");
        return false;
    };

    if mutex.1 == frame_count.0 {
        return true;
    }

    let mut hasher: AHasher = BuildHasherDefault::default().build_hasher();
    state.hash(&mut hasher);
    let hash = hasher.finish();

    if mutex.0 == hash {
        debug!("State hashes match, skipping rerender");
        return false;
    }

    mutex.1 = frame_count.0;
    mutex.0 = hash;

    true
}
