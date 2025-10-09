# Kaiv utils

Utilities I use across projects.

---

## Features

- **`env_config_macro`** - adds the `env_config!` macro for easy environment and file-based configuration  
- **`bevy_support`** - adds Bevy types (`Vec2`, `Vec3`, `Vec4`) support for some operations

---

## What is this

`kaiv_utils` is a small collection of convenience utilities and helper macros reused across my personal Rust projects.  
It’s meant to stay **tiny**, **dependency-light**, and **Git-only** - not published to crates.io.

### Currently includes:

- `env_config!` - macro to define global configuration structs with lazy loading, env parsing, and defaults (`env_config_macro` feature)
- `wrappers!` - macro to create simple wrapper structs with automatic `Deref`, `DerefMut` and `From` implementations
- `exp_decay` - exponential decay interpolation for `f32` and (optionally) Bevy’s `Vec2`, `Vec3`, `Vec4`
- `ping_pong_rem` - "ping-pong" remainder for integer types
- `inspect_none` - call a fn if `Option` is `None`

---

## Why keep it Git-only

These utilities are small, experimental, and evolve quickly.  
I prefer to version them alongside projects rather than maintaining crates.io releases.

---

## Add to your project

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
kaiv_utils = { git = "https://github.com/kaiv-dev/kaiv_utils.git", branch = "main" }
