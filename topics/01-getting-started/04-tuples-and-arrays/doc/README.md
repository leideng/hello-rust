# Tuples and Arrays

## What this project teaches

This project introduces two compound data types:

- tuples, which can store values of different types
- arrays, which store multiple values of the same type

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let person = ("Leo", 1);
    let (name, completed_lessons) = person;

    println!("Name: {name}");
    println!("Completed lessons: {completed_lessons}");

    let scores = [80, 85, 90];
    println!("First score: {}", scores[0]);
    println!("All scores: {:?}", scores);
}
```

## Explanation

The tuple `("Leo", 1)` stores two values together. The first is text, and the second is a number.

This line:

```rust
let (name, completed_lessons) = person;
```

takes the tuple apart and puts each value into its own variable. This is called destructuring.

The array `[80, 85, 90]` stores three integers of the same type.

You can access one item in an array by index:

```rust
scores[0]
```

Rust array indexes start at `0`, so `scores[0]` means the first item.

`{:?}` is a debug format. It lets `println!` print values like arrays in a useful way.

## Expected output

```text
Name: Leo
Completed lessons: 1
First score: 80
All scores: [80, 85, 90]
```
