use rusty_framework::prelude::*;
use std::sync::Arc;

// This will be replaced by the build script with 'mod generated_hello;'
include!(concat!(env!("OUT_DIR"), "/generated_hello.rs"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(HelloApp::new());
    rusty_framework::server::start_server(app).await;
    Ok(())
}
