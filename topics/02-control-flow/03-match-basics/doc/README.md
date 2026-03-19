# Match Basics

## What this project teaches

This project introduces:

- `match` for branching on different values
- match arms like `1 => "Monday"`
- `_` as a fallback arm for "everything else"

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let day_number = 3;

    let day_name = match day_number {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        _ => "Weekend",
    };

    println!("Day {day_number} is {day_name}.");
}
```

## Explanation

`match` compares one value (`day_number`) against several patterns.

Each pattern has an arm:

```rust
3 => "Wednesday"
```

If `day_number` is `3`, this arm runs and the result is `"Wednesday"`.

The `_` pattern means "any other value" and is used as a fallback.

In this example, we store the result of `match` in `day_name`.

## Expected output

```text
Day 3 is Wednesday.
```
