use std::sync::Arc;
use parking_lot::RwLock;
use std::fmt::Debug;

pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    subscribers: Arc<RwLock<Vec<Box<dyn Fn() + Send + Sync>>>>,
}

impl<T: Clone + Send + Sync + 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(value)),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn use_state(value: T) -> Self {
        Self::new(value)
    }

    pub fn get(&self) -> T {
        self.value.read().clone()
    }

    pub fn set(&self, new_value: T) {
        {
            let mut val = self.value.write();
            *val = new_value;
        }
        self.notify();
    }

    pub fn subscribe<F>(&self, f: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.subscribers.write().push(Box::new(f));
    }

    fn notify(&self) {
        let subs = self.subscribers.read();
        for sub in subs.iter() {
            sub();
        }
    }
}

impl<T: Debug + Clone + Send + Sync + 'static> Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("value", &self.get())
            .finish()
    }
}
