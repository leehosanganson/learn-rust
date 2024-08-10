# learn-rust

Progress so far: Chapter 3 <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>

### Cargo Commands

1. cargo new
2. cargo build
    - cargo build --release
3. cargo run
4. cargo check

Cargo Conventions

``` bash
git clone example.org/someproject
cd someproject
cargo build
```

### Crate

A crate is a collection of Rust source code files. The project we built is a binary crate, which is an executable.
`rand` crate is a library crate, which is intended to be used by other programs and can't be executed on its own.

Modify `Cargo.toml` to add a dependecy. Use `cargo build` to build again. Cargo fetches the latest version of the dependency from the registrry, which is a copy of data from `Crates.io` (open source Rust projects).
