# Hello World

## What this project teaches

This is the smallest useful Rust program. It shows:

- where execution starts
- how to print text to the terminal
- what a Rust function looks like

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    println!("Hello, Rust!");
}
```

## Explanation

`fn` means you are defining a function.

`main` is the special function that Rust runs first when your program starts.

`println!` prints a line of text to the terminal.

The `!` is important. It tells you that `println!` is a macro, not a normal function.

The semicolon at the end of the `println!` line finishes that statement.

## Expected output

```text
Hello, Rust!
```
