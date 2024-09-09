# CompOrg Engager - a small tool that brings classroom engagement to the next level

Copyright (C) 2023,2024 Dr. Nick Ivanov @ Rowan University

e-mail: ivanov@rowan.edu, LICENSE: GNU GPL 3

Version: 0.3

## Usage

The program can operate in two modes: I) Lucky Student Mode and II) Group Forming Mode

### I. Lucky Student Mode

This mode allows to pick a random attending student.

1. Add students to a JSON file using `sample.json` as an example; mark the attendance (e.g., `class-cs06205-1.json`).
2. Run this command:
```
cargo run class-cs06205-1.json
```

### II. Group Forming Mode

This mode is used to make groups of desired size randomly from the attending students. The group captain in each group is chosen randomly.

1. Add students to a JSON file using `sample.json` as an example; mark the attendance (e.g., `class-cs06205-1.json`).
2. Run this command:
```
cargo run groups <N> class-cs06205-1.json
```
Where N is an integer number between 2 and 32 indicating the desired group size. The algorithm will create maximum number of groups of size N, except possibly for the last group, which will be filled with the remaining students. If the there is only one remaining student, they will be added to the last group, effectively making its size N+1.

## Frequently Asked Questions

### Can I build it to avoid re-compilation?
Yes. Just run `cargo build --release`, and get the executable in `target/release/`.

### I don't have cargo, what should I do?

Go to https://rustup.rs/ and follow the instructions. It's safe.