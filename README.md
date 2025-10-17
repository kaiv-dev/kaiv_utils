# Kaiv utils

Utilities I use across projects.

---

## Features

- **`env_config_macro`** - adds the `env_config!` macro for easy environment and file-based configuration  
- **`bevy_support`** - adds Bevy types (`Vec2`, `Vec3`, `Vec4`) support for some operations
- **`trace_err`** - adds the `TraceError` trait for tracing `Result`'s `Err`

---

## What is this

`kaiv_utils` is a small collection of convenience utilities and helper macros reused across my personal Rust projects.  
It’s meant to stay **tiny**, **dependency-light**, and **Git-only** - not published to crates.io.

### Currently includes:

- `wrappers!` - macro to create simple wrapper structs with automatic `Deref`, `DerefMut` and `From` implementations
- `exp_decay` - exponential decay interpolation for `f32` and (optionally) Bevy’s `Vec2`, `Vec3`, `Vec4`
- `ping_pong_rem` - "ping-pong" remainder for integer types
- `inspect_none` - call a fn if `Option` is `None`
- `module_import_prelude!` - macro to make `prelude` module less painful

#### `env_config_macro` feature
- `env_config!` - macro to define global configuration structs with lazy loading, env parsing, and defaults

#### `trace_err` feature
- `TraceError` - trait, that provides tracing for `Result`'s `Err` with msgs and optional unwrapping `trace_err`, `trace_err_msg`, `expect_trace_err` `expect_trace_err_msg`

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
