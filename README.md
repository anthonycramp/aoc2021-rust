# Advent of Code 2015 in Rust

Rust solutions for Advent of Code 2015 problems

## Setting up for a new day

This repo is structured as a cargo workspace. Each day's problem is solved in a
separate binary crate, named dayNN. There are also library crates. The following
steps set me up with a baseline to work on a new day's problem. When I learn
how, I'll write an app that plugs in to cargo so I can just say something like
`cargo aoc NN`.

- `cargo new dayNN`
- Add `dayNN` to the workspace members list in `./Cargo.toml`
- If the problem has a large puzzle input (most days do), create file
  `dayNN/src/dayNN.txt` and copy in the puzzle input
- Add `const INPUT: &str = include_str!("dayNN.txt");` to the top of
  `dayNN/main.rs`. If the puzzle input is short (e.g., a single line), then it
  can be assigned directly to `INPUT` instead of reading it out of `dayNN.txt`
  with `include_str!`.
- Replace the rest of `main.rs` with:

```rust
// const INPUT: &str = include_str!("dayNN.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day NN Part 1: {:?}", part1(INPUT));
    println!("Day NN Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase { input: "...", expected: 123 },
            TestCase { input: "abc", expected: 345 },
        ];
        for TestCase{input, expected} in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [
            TestCase { input: "...", expected: 123 },
            TestCase { input: "abc", expected: 345 },
        ];
        for TestCase{input, expected} in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
```

- Add `test_support = {path="../test_support"}` to the `dependencies` in
  `dayNN/Cargo.toml` (depending on the problem, this might not be used)
- Add `dayNN/notes.md` to capture thoughts and reflections on the problem and
  Rust features
- Run `cargo build -p dayNN` and `cargo test -p dayNN` to trigger an update to
  `Cargo.lock`
- `git add dayNN Cargo.toml Cargo.lock` and
  `git commit -m "Added Day NN skeleton` and `git push`
