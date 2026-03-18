# If and Loops

## What this project teaches

This project introduces:

- `if` for decisions
- `while` for repeating code
- comparison operators such as `>`, `<`, and `==`

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let target_score = 3;
    let mut score = 0;

    if target_score > 0 {
        println!("The game has started.");
    }

    while score < target_score {
        score += 1;
        println!("Score is now {score}");
    }

    if score == target_score {
        println!("Target reached.");
    }
}
```

## Explanation

The first `if` checks whether `target_score` is greater than `0`.

If the condition is true, the code inside the braces runs.

The `while` loop keeps running as long as `score < target_score` stays true.

Inside the loop, this line increases the score:

```rust
score += 1;
```

That is a shorter way to write:

```rust
score = score + 1;
```

When `score` reaches `3`, the loop stops. Then the final `if` prints a message.

## Expected output

```text
The game has started.
Score is now 1
Score is now 2
Score is now 3
Target reached.
```
