# hello-rust

My first journey to learn Rust.

This repository is organized as a beginner-friendly Rust workspace. Each topic has its own folder, and each small Rust project lives inside that topic with:

- `src/` for source code
- `doc/` for explanations

## Topics

### 01 Getting Started

- `topics/01-getting-started/01-hello-world`
- `topics/01-getting-started/02-variables-and-mutability`
- `topics/01-getting-started/03-basic-data-types`
- `topics/01-getting-started/04-tuples-and-arrays`

### 02 Control Flow

- `topics/02-control-flow/01-if-and-loops`

### 03 Functions

- `topics/03-functions/01-basic-functions`

## How to run a project

Change into any project folder and run:

```bash
cargo run
```

Example:

```bash
cd topics/01-getting-started/01-hello-world
cargo run
```

## How to check the whole workspace

From the repository root:

```bash
cargo check --workspace
```
