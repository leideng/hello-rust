# Basic Data Types

## What this project teaches

This project introduces four common Rust scalar types:

- integers such as `u32`
- floating-point numbers such as `f64`
- booleans with `bool`
- characters with `char`

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let age: u32 = 20;
    let temperature: f64 = 23.5;
    let is_learning_rust: bool = true;
    let grade: char = 'A';

    println!("Age: {age}");
    println!("Temperature: {temperature}");
    println!("Learning Rust: {is_learning_rust}");
    println!("Grade: {grade}");
}
```

## Explanation

The `: type` syntax after a variable name is called a type annotation.

`u32` means an unsigned 32-bit integer. Unsigned means it cannot store negative numbers.

`f64` means a 64-bit floating-point number. It is used for decimal values.

`bool` stores either `true` or `false`.

`char` stores a single character and uses single quotes like `'A'`.

Rust can often figure out types by itself, but writing the type explicitly is useful while learning.

## Expected output

```text
Age: 20
Temperature: 23.5
Learning Rust: true
Grade: A
```
