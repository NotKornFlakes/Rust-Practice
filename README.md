# tasks: learning Rust by building a to-do app

A command-line task tracker, built one stage at a time. Each stage teaches a
handful of Rust ideas. You write the code, and the tests tell you when it works.

## Setup

1. Install a C linker. Rust uses it to build programs: `sudo dnf install gcc`
2. Open a new terminal so `cargo` is on your PATH, or run `source ~/.cargo/env`.
3. In VS Code, install the **rust-analyzer** extension. It shows types, errors
   and documentation as you type.

## How to work through a stage

1. Open the stage's file and read it from top to bottom.
2. Find the next `todo!()` and replace it with your code.
3. Run that exercise's tests, for example `cargo test ex1`.
4. Repeat until `cargo test` passes, then try `cargo run`.

| Command          | What it does                                       |
| ---------------- | -------------------------------------------------- |
| `cargo test`     | Run every test                                     |
| `cargo test ex2` | Run only the tests with `ex2` in their name        |
| `cargo check`    | Check for errors quickly, without building         |
| `cargo run`      | Build and run the program                          |
| `cargo clippy`   | Suggest more idiomatic ways to write your code     |
| `cargo fmt`      | Format your code in the standard Rust style        |

**Read the compiler errors.** Rust's errors are unusually helpful and often say
exactly what to change. For a long explanation of any error code, run
`rustc --explain E0382` (with that error's code).

When you're stuck, ask Claude for a hint or a review rather than the answer.

## Roadmap

- [ ] **Stage 1: Structs and enums** (`src/task.rs`): `struct`, `enum`, `impl`,
      `&self` vs `&mut self` vs `self`, `String` vs `&str`, `match`, `Option`,
      the `Display` trait
- [ ] **Stage 2: A list of tasks**: `Vec`, borrowing in loops, iterators
      (`filter`, `map`, `find`), returning references
- [ ] **Stage 3: Errors**: `Result`, custom error types, the `?` operator,
      parsing user input
- [ ] **Stage 4: Saving to disk**: reading and writing files, adding crates,
      `serde` and JSON
- [ ] **Stage 5: A real CLI**: subcommands with `clap`, such as
      `tasks add "Buy milk" --priority high`, `tasks list` and `tasks done 3`
- [ ] **Stage 6: Polish**: your own traits, sorting and filtering, colored
      output, due dates
