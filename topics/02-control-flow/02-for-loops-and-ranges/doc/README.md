# For Loops and Ranges

## What this project teaches

This project introduces:

- `for` loops to repeat code
- iterating through values in an array
- using a range like `1..=3` to count

## How to run it

From this project folder:

```bash
cargo run
```

## Code

```rust
fn main() {
    let scores = [10, 20, 30];

    println!("Reading values from an array:");
    for score in scores {
        println!("Score: {score}");
    }

    println!("Counting with a range:");
    for number in 1..=3 {
        println!("Number: {number}");
    }
}
```

## Explanation

`for score in scores` loops through each item in the `scores` array.

The loop runs once for each value: `10`, then `20`, then `30`.

`1..=3` is an inclusive range, so it includes both `1` and `3`.

That means this loop runs three times:

```rust
for number in 1..=3
```

## Expected output

```text
Reading values from an array:
Score: 10
Score: 20
Score: 30
Counting with a range:
Number: 1
Number: 2
Number: 3
```
