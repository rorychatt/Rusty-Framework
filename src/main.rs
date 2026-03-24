use rusty_framework::prelude::*;

// This macro includes the generated Rust code from the build script.
include!(concat!(env!("OUT_DIR"), "/generated_hello.rs"));

fn main() {
    println!("--- RUSTY NATIVE APP ---");
    
    // Boot the HelloApp (transpiled from C#)
    let app = HelloApp::build();
    
    println!("Serialized UI from HelloApp.cs:");
    println!("{}", serde_json::to_string_pretty(&app.serialize()).unwrap());
}
