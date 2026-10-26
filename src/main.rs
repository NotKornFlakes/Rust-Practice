// The command-line program. For now it just creates and prints a few tasks so
// you can see your Stage 1 code working. Run it with `cargo run`.
//
// In later stages this becomes a real CLI that saves your tasks to a file.

// The library is called `tasks` (from Cargo.toml), and `task` is the module in
// src/task.rs. This line brings two of its types into scope.
use tasks::task::{Priority, Task};

fn main() {
    // `let` variables can't be changed by default. `mut` makes this one
    // changeable, which is required to call `complete()` on it.
    let mut milk = Task::new(1, "Buy milk");
    milk.complete();

    let taxes = Task::new(2, "Do taxes").with_priority(Priority::High);

    println!("{milk}");
    println!("{taxes}");

    // `parse` returns an Option, so we have to handle the `None` case too.
    for text in ["low", "urgent"] {
        match Priority::parse(text) {
            Some(priority) => println!("'{text}' is a priority: {priority}"),
            None => println!("'{text}' is not a priority"),
        }
    }
}
