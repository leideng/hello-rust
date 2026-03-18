# AGENTS.md

This repository is a beginner-first Rust learning workspace.

## Goal

- Keep examples small, runnable, and easy to read.
- Teach one topic at a time.
- Prefer clear explanation over clever Rust.
- Assume the reader is new to Rust and new to systems programming.

## Repository Structure

- Put each topic in its own folder under `topics/`.
- Put each Rust project in a subfolder inside that topic.
- Every project must contain:
  - `src/` for Rust source code
  - `doc/` for Markdown explanations
  - `Cargo.toml` for the crate itself

Example:

```text
topics/
  01-getting-started/
    01-hello-world/
      Cargo.toml
      src/
      doc/
```

## Content Rules

- Start with fundamentals before moving to ownership, structs, enums, traits, and lifetimes.
- Each project should focus on a single main idea or one tightly related pair of ideas.
- Keep examples practical but very small.
- Prefer standard library features before introducing external crates.
- Add comments only when they help a beginner understand something non-obvious.
- Keep code ASCII unless there is a strong reason not to.

## Documentation Rules

- Every project should have at least one Markdown file in `doc/`.
- Documentation should explain:
  - what the example teaches
  - how to run it
  - the important syntax in the code
  - what output to expect
- Write in simple language.
- Avoid assuming prior Rust knowledge.
- When introducing new keywords, explain them briefly.

## Naming

- Topic folders should use ordered prefixes such as `01-`, `02-`, `03-`.
- Project folders should also use ordered prefixes such as `01-hello-world`.
- Use descriptive names that make the learning path obvious.

## Workspace

- Keep the root `Cargo.toml` as a workspace manifest that includes all learning projects.
- Each learning project should remain independently runnable with `cargo run` from its own folder.

## Change Guidance

- Do not replace simple examples with more advanced or more compact Rust unless the lesson requires it.
- If you add a new topic, place it at the correct point in the learning sequence.
- If you change code, update the matching documentation in the same project.
