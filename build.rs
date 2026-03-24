use std::env;
use std::path::Path;
use std::fs;

#[path = "src/transpiler/mod.rs"]
mod transpiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_all.rs");
    
    let mut all_rust_code = String::new();
    all_rust_code.push_str("#[allow(unused_imports)]\n");
    all_rust_code.push_str("use rusty_framework::prelude::*;\n\n");

    // HelloApp
    let hello_path = Path::new("demos/HelloDemo/HelloApp.cs");
    if hello_path.exists() {
        let code = transpiler::transpile_file_to_string(hello_path)?;
        all_rust_code.push_str(&code);
        all_rust_code.push_str("\n\n");
    }

    // Samples
    let samples_dir = Path::new("samples");
    if samples_dir.exists() {
        let mut entries: Vec<_> = fs::read_dir(samples_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort(); // Consistent order

        for path in entries {
            if path.extension().map_or(false, |ext| ext == "cs") {
                if let Ok(code) = transpiler::transpile_file_to_string(&path) {
                    all_rust_code.push_str(&format!("// Transpiled from {}\n", path.display()));
                    all_rust_code.push_str(&code);
                    all_rust_code.push_str("\n\n");
                }
            }
        }
    }

    fs::write(&dest_path, all_rust_code)?;

    println!("cargo:rerun-if-changed=demos/HelloDemo/HelloApp.cs");
    println!("cargo:rerun-if-changed=samples/");
    println!("cargo:rerun-if-changed=src/transpiler/mod.rs");
    Ok(())
}
