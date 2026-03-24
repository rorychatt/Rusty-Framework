# HelloDemo

A demonstration of writing Ivy-style code in C# that is transpiled and executed in native Rust.

## Prerequisites

- [.NET 8.0 SDK](https://dotnet.microsoft.com/download)
- [Rust Toolchain](https://rustup.rs/)

## How to Run

1.  Open a terminal in this directory (`demos/HelloDemo`).
2.  Run the following command:
    ```bash
    dotnet build
    ```

## What happens?

1.  **MSBuild triggers**: The `.csproj` file identifies the build target.
2.  **Transpilation**: The `Rusty-Framework` build process (via `cargo run`) parses `HelloApp.cs` using `tree-sitter`.
3.  **Rust Generation**: It generates a native Rust implementation of your UI in `target/debug/build/.../out/generated_hello.rs`.
4.  **Native Execution**: The Rust binary is compiled and executed, serializing the resulting widget tree to the console.

## Project Structure

- `HelloApp.cs`: Your UI definition in C#.
- `HelloDemo.csproj`: The .NET project file that bridges to the Rust backend.
- `../../src/`: The Rust framework and transpiler source.
