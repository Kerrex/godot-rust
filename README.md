# GDNative bindings for Rust

Collection of crates to let you easily write code for Godot games in Rust. Actual documentation coming soon™

Crates:

* `gdnative-core`: Core crate, depend on this one to make a game

* `gdnative-macros`: Procedural macros (specifically the `#[godot_export]` macro) for automatically generating some code behind the scenes 👀

* `gdnative-sys`: Automatically generated C bindings and relevant code (such as trait implementations) 🐲

* `gdnative-api` *(coming soon)* Godot's API, which will let you use existing Godot types and methods/functions in Rust
