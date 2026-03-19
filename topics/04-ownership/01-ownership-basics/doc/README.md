# Ownership Basics

## What this project teaches

This project introduces:

- ownership of heap data with `String`
- moving ownership into a function
- why a moved value cannot be used again

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn print_message(message: String) {
    println!("Message: {message}");
}

fn main() {
    let text = String::from("hello ownership");

    print_message(text);

    // text was moved into print_message, so it cannot be used here anymore.
}
```

## Explanation

`String` stores text on the heap, and it follows Rust ownership rules.

In this line:

```rust
print_message(text);
```

ownership of `text` moves into `print_message` because the parameter type is `String`.

After the move, `text` is no longer valid in `main`.

This is how Rust prevents use-after-free bugs at compile time.

## Expected output

```text
Message: hello ownership
```
