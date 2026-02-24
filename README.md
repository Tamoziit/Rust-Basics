# Rust-Basics
---
Rust Yaayyyyyyyyy

## Theory
---
- Rust is an **ahead-of-time** compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. (Unlike py, js, etc.)
- **Cargo**: Rust’s build system and package manager. Cargo handles a lot of tasks, such as building the code, downloading the libraries which the code depends on, and building those libraries. [Analogous: npm]
- **Cargo.toml**: Package config --> like package.json
- **cargo CMDs**:

```bash
cargo build  # Builds a debug build for development
```
```bash
cargo build --release # For production release/build
```
```bash
cargo check # Compiles the Rust code, but doesn't build an executable --> faster to check for a safe build in dev, rather than building the executable at each step
```