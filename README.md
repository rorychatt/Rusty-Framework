# Rusty-Framework

This is the planned direct 1:1 port of the C# `Ivy-Framework` into a new Rust-based `Rusty-Framework`, maintaining exactly the same structure, state handling, and frontend compatibility.

## Proposed Architecture

To mirror `Ivy-Framework` cleanly while leveraging Rust's safety and performance, the new `Rusty-Framework` project will be structured similarly:

### 1. Project Initialization

- Initialize a new Cargo Workspace with a primary crate (`rusty_framework` or `ivy_rust`).
- Setup `Cargo.toml` with necessary dependencies:
  - **Networking:** `axum` + `tokio` (for building the web framework).
  - **Serialization/Diffing:** `serde`, `serde_json`, `json-patch` (using the same blazing-fast logic originally extracted to `RustServer` cdylib).
  - **Concurrency:** `tokio` channels or `dashmap` for managing global state and concurrent SignalR connections.

### 2. Core Abstractions & State Handling

We will recreate the core C# structure natively in Rust:

- **Traits (C# Interfaces):**
  - `trait Widget` (Replaces `IWidget`)
  - `trait View` (Replaces `IView`)
  - `trait ContentBuilder` (Replaces `IContentBuilder`)
- **State Handling (`WidgetTree`):**
  - Implement a `WidgetTree` struct that mirrors `Ivy-Framework/src/Ivy/Core/WidgetTree.cs`.
  - It will construct the view hierarchy (`build()`), serialize it to JSON, and compute the mathematical `json-patch` diff exactly like the C# algorithm.
  - Use Rust's interior mutability (`Arc<Mutex<T>>` / `RwLock`) to represent stateful `View` components, which get re-rendered when events trigger (`RefreshView(viewId)`).

### 3. Server Architecture (`Server.rs`)

- Port `Ivy-Framework/src/Ivy/Server.cs` functionality.

- Setup an Axum `Router` mimicking the `Microsoft.AspNetCore.Builder` pattern.
- Setup HTTP endpoints for static assets (integrating directly with the unified React frontend).

### 4. Component Porting (Widgets)

- Recreate all fundamental widgets found in `Ivy-Framework/src/Ivy/Widgets` inside `Rusty-Framework/src/widgets/`.

- **Goal:** The end goal is to systematically port *all* ~40 widgets from C# to Rust.
- This includes:
  - `Button`, `Text`, `Badge`, `Card`, `Container`, `Column`, `Row`, `TextField`, `ListView`, etc.
- Widgets will be structs that serialize to the explicit JSON schema expected by the React frontend (`id`, `$$type`, `children`, style props).

## Open Questions for Implementation

Prior to writing the code, the following architectural directions will need to be decided:

1. **SignalR Compatibility:**
   The React frontend (`Ivy-Framework/src/frontend/src/hooks/use-backend.tsx`) heavily relies on `@microsoft/signalr`. Rust does not have a 1st-party officially supported SignalR server crate.
   *Decision required:* Should we write a basic custom SignalR socket emulator in Rust (so the frontend doesn't have to change at all), or refactor the frontend's networking to use standard WebSockets across both C# and Rust backend variants?

2. **View Definition Syntax (Developer Experience):**
   In C#, Ivy uses a fluent API builder pattern for View structures (e.g. `new Column { Children = { new Button() } }`).
   *Decision required:* Should we use a declarative macro (like `rsx!` / `html!`), or a standard Builder pattern logic (`Column::new().child(Button::new())`) to reflect the C# experience as closely as possible?
