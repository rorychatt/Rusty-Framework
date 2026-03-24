use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct Signal<T> {
    id: String,
    value: Arc<RwLock<T>>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de>> Signal<T> {
    pub fn use_state(id: String, initial: T) -> Self {
        Self {
            id,
            value: Arc::new(RwLock::new(initial)),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.write() = new_value;
    }
}
