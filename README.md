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
* `println!` macro for stdout, `!` signals a macro

### Chapter II - Guessing Game
Content is stored in `./guessing game`.

Learnings 
* `use std::io` import-statement io from standard
* `println!("You guessed: {}", guess);` you can use `{}` as placeholder in println! for variables
* `let` to create variables
* variables are per default immutable add `let mut varname` mut to make them mutable
* `String::new()`in this case `::` is used to invoce a static function 
* Some methods return the `Result` type which is an `Ok` or `Err` enum you can evaluate them with the `.expect()` method which will exit the program on `Err`
* packages are called `crates`
* `cargo doc --open` to generate documentation files for your project and open them
* Introduction of the arms pattern `match (expression) {ARM1 => stuff; ARM2 => stuff}`
* Checkout Comments in `./guessing_game/src/main.rs` on how to use arms pattern for error handling 
* `_` is used to take every value
* Introduction of the keywords `loop`, `break` and `continue` for looping
# Interesting Repositories

* [Rustlings](https://github.com/rust-lang/rustlings) lovely curated learning exercises for rust
* [BAT](https://github.com/sharkdp/bat) cat command line tool written in rust and with wings
* [The Algorithms](https://github.com/TheAlgorithms/Rust) algorithms in Rust