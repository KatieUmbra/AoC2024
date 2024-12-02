# Advent of code year 2024
By Katherine, [AOC24](https://adventofcode.com/2024)

## How to run

### Requirements

I'm doing this project on Artix 6.12.1 but it should work on any other OS
- Rust/Cargo (1.82)

### Setup

- Create a folder in the project root called inputs `mkdir inputs`.
- Create a file for the day(s) you wanna find the solution for like `{day}.txt` eg. `1.txt` you can use the touch command like `touch 1.txt`.
- Paste the input in the file, making sure there are no trailing lines at the end or beginning.

### Building

- Run `cargo build` to build the project
- Run `cargo run` to run the project

**EVENTUALLY I'M GONNA ADD CLI ARGUMENTS TO SELECT THE DAY, FOR NOW JUST CHANGE THE IMPORT IN `src/main.rs` LIKE `use day::{cardinal day}::solution;`**

Example

```sh
echo "${your puzzle input}" >> inputs/"${day}".txt
# change the first line in the file to use the day you want
vim src/main.rs
cargo run
```

### Current State

- 1 \*\*
- 2 \*
