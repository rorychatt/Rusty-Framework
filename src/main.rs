use std::env;
use std::path::Path;
use rusty_framework::transpiler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: rusty-framework <input.cs> <output.rs>");
        return Ok(());
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    println!("Transpiling {} to {}...", input_path.display(), output_path.display());
    transpiler::transpile_file(input_path, output_path)?;
    println!("Done!");

    Ok(())
}
