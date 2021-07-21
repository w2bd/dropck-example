# dropck-example

This repository is to list drop check(aka dropck) problems that we can encounter
programming in Rust and reason about the relevant compile errors. There might be
wrong information here and it will be really appreciated if you make an issue or
a pull request if you find something wrong.

## Rust compiler version
rustc 1.53.0-nightly (392ba2ba1 2021-04-17)

## How to test
Just do `cargo check` to check if the test cases compile successfully.  
If you want to test a specific case, just comment out the other cases in
`main.rs`. This is an example to test `case_3`.
```rust
// mod case_1;
// mod case_2;
mod case_3;

fn main() {}
```

## References
#### Drop Check
https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md

#### Rust Subtyping and Variance
https://doc.rust-lang.org/nomicon/subtyping.html

#### Non-Lexical Lifetimes
https://rust-lang.github.io/rfcs/2094-nll.html
