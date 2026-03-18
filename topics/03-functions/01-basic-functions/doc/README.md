# Basic Functions

## What this project teaches

This project introduces:

- how to define your own functions
- function parameters
- return values
- a simple string slice parameter with `&str`

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    greet("Leo");

    let result = add(2, 3);
    println!("2 + 3 = {result}");
}
```

## Explanation

`fn greet(name: &str)` defines a function named `greet`.

`name: &str` means the function accepts a string slice. For now, you can think of `&str` as a borrowed view of text.

`fn add(a: i32, b: i32) -> i32` defines a function that takes two `i32` integers and returns an `i32`.

In Rust, the last expression in a function can become the return value if it does not end with a semicolon.

That is why this works:

```rust
a + b
```

If you wrote `a + b;` with a semicolon, it would become a statement and would not be returned.

## Expected output

```text
Hello, Leo!
2 + 3 = 5
```
