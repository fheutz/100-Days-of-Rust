# 100_days_of_rust
100 Days of learning Rust
# Table of Content
- [100_days_of_rust](#100-days-of-rust)
- [Table of Content](#Table-of-Content)
- [Day 1](#day-1)
  * [Setting up Rust](#setting-up-rust)
  * [Getting Started on Rust](#getting-started-on-rust)
    + [Chapter I - Hello Cargo](#chapter-i---hello-cargo)
    + [Chapter II - Guessing Game](#chapter-ii---guessing-game)
- [Interesting Repositories](#interesting-repositories)
# Day 1 
## Setting up Rust
Installing Rust from 
* [Rust Official](https://www.rust-lang.org/tools/install)
* Command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
* Update via `rustup update`
VS-Code
* rust-lang.rust Extension
* Rust Language Server
## Getting Started on Rust
[Rust Getting Started Book](https://doc.rust-lang.org/book)
### Chapter I - Hello Cargo
Content is stored in `./hello_cargo`.

Learnings
* `cargo` is Rusts build + package manager
* `cargo new $PROJECTNAME` for creating a new project with cargo
* `cargo.toml` contains package information, `cargo.lock` exact version and dependencies, like `package.lock`
* Compile + run `cargo run`
* Compile code with `cargo build` for production `cargo build --release`
* Quick run `cargo check`

### Chapter II - Guessing Game
Content is stored in `./guessing game`.

# Interesting Repositories

* [Rustlings](https://github.com/rust-lang/rustlings) lovely curated learning exercises for rust
* [BAT](https://github.com/sharkdp/bat) cat command line tool written in rust and with wings
* [The Algorithms](https://github.com/TheAlgorithms/Rust) algorithms in Rust
* 