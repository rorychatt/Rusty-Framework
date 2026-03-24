use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de>> Signal<T> {
    pub fn use_state(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.write() = new_value;
    }
}
