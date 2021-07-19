# 100_days_of_rust
100 Days of learning Rust
# Day 1 
## Setting up Rust
Installing Rust from 
* [Rust Official](https://www.rust-lang.org/tools/install)
* Command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 
* Update via `rustup update`
VS-Code
* rust-lang.rust Extension
* Rust Language Server
## Getting Started with Rust
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

### Chapter III - Variables, Functions, Control Flow
Content is stored in `./variables`, `./functions`, `./branches`

Learnings
* Variables are immutable but can be made mutable with the `mut` keyword `let mut x = 5`
* Constants are declared with the `const` keyword replacing `let`. Capitalcase with spaces as underscores
* You can insert underscores in numerals for readability `const TEN_THOUSAND=10_000`
* With shadowing you can overwrite variables, you are able to change type with this
```rust
let x = 5
println!("x={}", x) // out: "x=5"
let x = x*2
println!("x={}", x) // out: "x=10"
```
* Scalars are _integers, floating-points, numbers, Booleans, and characters_
* Explicit type annotation via colon `let t: bool = true`
* Compound Types
```rust
// Tuple construction
let tup: (i32, f64, u8) = (500, 6.4, 1);
// Tuple deconstruction
let (x, y, z) = tup
let x = tup.0

// Array construction
let array = [1,2,3,4,5,6]
// Create an array of length 5 with type i32
let array : [i32; 5] = [1,2,3,4,5]
// Create an array of length 5 filled with 3
let array = [3;5]
```
* Arrays are static in length, Vectors are variable in length
* Statements `let y= 6` have semicolons at the end
* Expressions evaluate to something `3+4`; do not have semicolons at the end
* Functions return the last Expression implicitly, the following function returns 6 without a return statement
```rust
fn six() -> i8 {
  6
}
```
* for elements in an Array
```rust
for element in array.iter() {
  //dostuff
}
```
* Creating a range `(1..4)` creates all numbers from 1-4 you can reverse them with `(1..4).rev()`

## Day 2
code is stored in `./ownership`
### Ownership in rust

(I remembered its better to ask questions in order to remember things)
Questions?
<details><summary>What are the 3 ownership rules</summary>
* Each value in Rust has a variable that’s called its owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped
</details>
<details><summary>What is generally faster writing to heap or stack?</summary>> stack, because you have to follow a pointer to the heap</details>
<details><summary>What properties does data have on the stack?</summary>> It is in a known fixed size</details>
<details><summary>What happens to the content of a variable when it leaves scope?</summary>> It gets dropped; the memory gets cleared</details>
<details><summary>What happens to an object (in the heap) if you try to shallow copy in rust?</summary>> it will move to the new variable instead of being shallow copied. this is done in order to prevent dropping a piece of memory twice.</details>
<details><summary>What happens if you pass a string into a function as parameter?</summary>> The string variable will not be usable after the function call. This happens because the ownership moves to the function parameter and dropps after the function is called.</details>
<details><summary>How can you borrow a variable?</summary>> by using an & infront of the variable. This would then be called a reference</details>
<details><summary>Are can you change the content of a Reference?</summary>> References are by default immutable but can be made mutable by adding the mut keyword after the & `function_name(&mut variable_name)`.</details>
<details><summary>What are slices?</summary>> Quite similar to python slices; are written with the rust range indicator `..`. For example if you want to have only a slice of a String you would say `string[2..5]` same would apply for arrays.</details>

# Interesting Repositories

* [Rustlings](https://github.com/rust-lang/rustlings) lovely curated learning exercises for rust
* [BAT](https://github.com/sharkdp/bat) cat command line tool written in rust and with wings
* [The Algorithms](https://github.com/TheAlgorithms/Rust) algorithms in Rust