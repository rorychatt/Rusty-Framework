# Hello App Demo

This is a demonstration of the **Rusty-Framework** transpilation pipeline. It takes an Ivy-style C# application and transpiles it into native Rust for high-performance execution.

## Contents
- `HelloApp.cs`: The source Ivy application written in C#.

## How it works
1. The `build.rs` script in the root directory detects this file.
2. It uses the `Rusty-Framework` transpiler (in `src/transpiler/`) to convert the C# code into Rust.
3. The generated Rust code is then compiled and executed.

## How to run
From the root of the `Rusty-Framework` repository, run:

```bash
cargo run
```

This will:
1. Transpile `HelloApp.cs`.
2. Compile the resulting native Rust.
3. Print the serialized JSON representation of the widget tree to the console.

## Expected Output
You should see a JSON representation of a `Layout` containing a `Card`, which in turn contains a vertical stack of widgets (Logo, Text, etc.).
