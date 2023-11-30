# RowanEngager - a small tool that makes classroom engagement to the next level

Copyright (C) 2023 Dr. Nick Ivanov @ Rowan University

e-mail: ivanov@rowan.edu, LICENSE: GNU GPL 3

## Usage

1. Add students to a JSON file using `sample.json` as an example; mark the attendance.
2. Run this command:
```
cargo run sample.json
```
3. Put the Lucky One on the spot by asking question, asking to write something on the board, etc.
4. Give the Lucky One a participation bonus point.

## Frequently Asked Questions

### Can I build it to avoid re-compilation?
Yes. Just run `cargo build --release`, and get the executable in `target/release/`.

### I don't have cargo, what should I do?

Go to https://rustup.rs/ and follow the instructions. It's safe.