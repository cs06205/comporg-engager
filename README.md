# CompOrg Engager - a small tool that brings classroom engagement to the next level

Copyright (C) 2023 Dr. Nick Ivanov @ Rowan University

e-mail: ivanov@rowan.edu, LICENSE: GNU GPL 3

Version: 0.2

## Usage

1. Add students to a JSON file (prefixed with `class-`) using `sample.json` as an example; mark the attendance (e.g., `class-cs06205-1.json`).
2. Run this command:
```
cargo run class-cs06205-1.json
```
3. Put the Lucky One on the spot by asking question, asking to write something on the board, etc.
4. Give the Lucky One a participation bonus point.

## Frequently Asked Questions

### Can I build it to avoid re-compilation?
Yes. Just run `cargo build --release`, and get the executable in `target/release/`.

### I don't have cargo, what should I do?

Go to https://rustup.rs/ and follow the instructions. It's safe.