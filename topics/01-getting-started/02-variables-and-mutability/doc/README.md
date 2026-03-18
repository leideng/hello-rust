# Variables and Mutability

## What this project teaches

This project introduces:

- `let` for creating variables
- immutable variables
- mutable variables with `mut`
- shadowing with another `let`

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let language = "Rust";
    println!("Learning {language}");

    let mut lesson_count = 1;
    println!("Finished lessons: {lesson_count}");

    lesson_count = lesson_count + 1;
    println!("Finished lessons now: {lesson_count}");

    let lesson_count = lesson_count * 2;
    println!("After review, counted lessons: {lesson_count}");
}
```

## Explanation

`let language = "Rust";` creates a variable named `language`.

By default, Rust variables are immutable. That means you cannot change them after creating them.

`let mut lesson_count = 1;` creates a mutable variable. The `mut` keyword allows you to change its value later.

This line changes the value:

```rust
lesson_count = lesson_count + 1;
```

Later, this line creates a new variable with the same name:

```rust
let lesson_count = lesson_count * 2;
```

This is called shadowing. Shadowing is different from mutation:

- mutation changes the same variable
- shadowing creates a new variable with the same name

## Expected output

```text
Learning Rust
Finished lessons: 1
Finished lessons now: 2
After review, counted lessons: 4
```
