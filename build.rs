use std::env;
use std::path::Path;

#[path = "src/transpiler/mod.rs"]
mod transpiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_hello.rs");
    
    // Transpile the demo app (located in demos/HelloDemo/)
    let source_path = Path::new("demos/HelloDemo/HelloApp.cs");
    if source_path.exists() {
        transpiler::transpile_file(source_path, &dest_path)?;
    }

    println!("cargo:rerun-if-changed=demos/HelloDemo/HelloApp.cs");
    println!("cargo:rerun-if-changed=src/transpiler/mod.rs");
    Ok(())
}
