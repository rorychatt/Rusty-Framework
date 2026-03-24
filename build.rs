use std::env;
use std::path::Path;

#[path = "src/transpiler/mod.rs"]
mod transpiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_hello.rs");
    
    // Transpile the demo app
    transpiler::transpile_file(Path::new("HelloApp.cs"), &dest_path)?;

    println!("cargo:rerun-if-changed=HelloApp.cs");
    println!("cargo:rerun-if-changed=src/transpiler/mod.rs");
    Ok(())
}
