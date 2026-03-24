use rusty_framework::prelude::*;
use std::sync::Arc;
use std::collections::HashMap;

// This will be replaced by the build script with 'mod generated_all;'
include!(concat!(env!("OUT_DIR"), "/generated_all.rs"));

pub struct SampleRegistry {
    pub apps: HashMap<String, Arc<dyn IvyApp>>,
}

impl SampleRegistry {
    pub fn new() -> Self {
        let mut apps = HashMap::new();
        apps.insert("HelloApp".to_string(), Arc::new(HelloApp::new()) as Arc<dyn IvyApp>);
        apps.insert("HelloTextInput".to_string(), Arc::new(HelloTextInput::new()) as Arc<dyn IvyApp>);
        apps.insert("HelloBoolInput".to_string(), Arc::new(HelloBoolInput::new()) as Arc<dyn IvyApp>);
        Self { apps }
    }
}

impl rusty_framework::server::AppProvider for SampleRegistry {
    fn get_app(&self, app_id: &str) -> Arc<dyn IvyApp> {
        self.apps.get(app_id).cloned().unwrap_or_else(|| self.apps.get("HelloApp").unwrap().clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Arc::new(SampleRegistry::new());
    rusty_framework::server::start_server_with_provider(registry).await;
    Ok(())
}
