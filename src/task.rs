//! # Stage 1: Structs, enums and methods
//!
//! In this stage you build the `Task` type that the rest of the app is built on.
//!
//! Work through the exercises in order. Each one has a `todo!()` for you to
//! replace with real code. `todo!()` compiles, but it crashes when it runs, so
//! tests for exercises you haven't done yet fail with "not yet implemented".
//!
//! Run one exercise's tests with `cargo test ex1`, `cargo test ex2`, and so on.
//! Run everything with `cargo test`.

// While the exercises are full of `todo!()`, the function arguments are never
// used and the compiler warns about it. This line silences those warnings.

use std::fmt;

// ---------------------------------------------------------------------------
// Priority
// ---------------------------------------------------------------------------

/// How important a task is.
///
/// An `enum` is a type whose value is exactly one of a fixed set of variants.
/// It's like a TypeScript union `"low" | "medium" | "high"`, but the compiler
/// knows every possible value and makes you handle all of them in a `match`.
///
/// The `derive` line asks the compiler to write some common code for us:
/// - `Debug` lets you print it for debugging with `{:?}`.
/// - `Clone` and `Copy` let it be copied freely, like a number.
/// - `PartialEq` and `Eq` let you compare two priorities with `==`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    /// EXERCISE 1: turn text like `"high"` into `Some(Priority::High)`.
    ///
    /// - Accept `"low"`, `"medium"` and `"high"` in any case (`"HIGH"`, `"High"`, ...).
    /// - Return `None` for anything else.
    ///
    /// `Option<T>` is how Rust says "maybe a value". It is either `Some(value)`
    /// or `None`. Rust has no `null` or `undefined`, so whenever a value might
    /// be missing, the type says so and the caller has to deal with it.
    ///
    /// Hints:
    /// - `match` is like `switch`, but it must cover every possible value.
    ///   Use `_ => ...` as the catch-all arm.
    /// - `s.to_lowercase()` returns a new `String`. You can't match a `String`
    ///   against text literals directly: call `.as_str()` on it first to get a
    ///   `&str` you can match on.
    pub fn parse(s: &str) -> Option<Priority> {
    match s.to_lowercase().as_str() {
        "low" => Some(Priority::Low),
        "medium" => Some(Priority::Medium),
        "high" => Some(Priority::High),
        _ => None,
    }
}
}

// This is a worked example, not an exercise. Read it before Exercise 6.
//
// A *trait* is a set of methods a type can promise to have, a bit like an
// interface in TypeScript. `Display` is the trait that `{}` uses when it prints
// something. Implementing it is like writing `toString()` in JS or `__str__`
// in Python.
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `match` is an expression, so it produces a value. Here it picks
        // the text, and that value is stored in `label`.
        let label = match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        };
        // `write!` works like `format!`, but writes into the formatter `f`.
        // There's no semicolon, so its result is what `fmt` returns.
        write!(f, "{label}")
    }
}

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

/// A single thing to do.
///
/// A `struct` groups named fields together, like a plain JS object or a Python
/// dataclass. Unlike a JS object, its fields and their types are fixed.
///
/// `title` is a `String`, which means the task *owns* its text. When the task
/// goes away, its title is freed along with it. No garbage collector needed.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    /// EXERCISE 2: create a new task.
    ///
    /// New tasks are not done and have `Priority::Medium`.
    ///
    /// Hints:
    /// - `title` is a `&str`: a *borrowed* view of text that belongs to
    ///   somebody else. The task needs to own its own copy as a `String`.
    ///   Convert it with `title.to_string()`.
    /// - Build a struct with `Task { id: ..., title: ..., ... }`. If a variable
    ///   has the same name as the field, you can write `id` instead of `id: id`,
    ///   just like the JS object shorthand.
    /// - A function returns its last expression if that line has no semicolon.
    ///   You don't need to write `return`.
    pub fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: title.to_string(),
            priority: Priority::Medium,
            done: false,
        }
    }

    /// EXERCISE 3: report whether this task is done.
    ///
    /// `&self` means this method *borrows* the task in order to read it. It can
    /// look at the fields but it isn't allowed to change them.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// EXERCISE 4: mark the task as done.
    ///
    /// `&mut self` means this method borrows the task *mutably*, so it is
    /// allowed to change its fields. Look at the ex4 test to see that the
    /// caller must declare the variable with `let mut` before calling this.
    pub fn complete(&mut self) {
        self.done = true;
    }

    /// EXERCISE 5: return this task with its priority changed.
    ///
    /// This method takes `self` with no `&`, so it takes *ownership* of the
    /// task. The caller hands the task over, we change it, and we hand it back.
    /// That lets callers chain calls when they set up a task:
    ///
    /// ```text
    /// let task = Task::new(1, "Do taxes").with_priority(Priority::High);
    /// ```
    ///
    /// Hint: try `self.priority = priority;` first and read the compiler error
    /// carefully. It tells you exactly what to add to the signature.
    pub fn with_priority(self, priority: Priority) -> Task {
        Task {
            id: self.id,
            title: self.title,
            priority,
            done: self.done,
        }
    }
}

/// EXERCISE 6: decide how a task looks when it is printed with `{}`.
///
/// The format is a checkbox, the id, the title and the priority:
///
/// ```text
/// [ ] 1: Buy milk (medium)
/// [x] 2: Do taxes (high)
/// ```
///
/// Hints:
/// - Copy the shape of `Display for Priority` above.
/// - `if` is an expression in Rust, so it can produce a value, like the
///   `cond ? a : b` operator in JS.
/// - `Priority` already implements `Display`, so you can put `self.priority`
///   straight into `write!`.
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let checkbox = if self.done { "[x]" } else { "[ ]" };
        write!(f, "{} {}: {} ({})", checkbox, self.id, self.title, self.priority)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
//
// `#[cfg(test)]` means this module is only compiled when you run `cargo test`.
// Each `#[test]` function is one test, and `assert_eq!(a, b)` fails the test if
// `a` and `b` are different.
//
// You don't need to change anything below, but do read the tests: they show
// exactly how your code is going to be used.

#[cfg(test)]
mod tests {
    // Bring everything from the parent module (this file) into scope.
    use super::*;

    #[test]
    fn ex1_parse_lowercase() {
        assert_eq!(Priority::parse("low"), Some(Priority::Low));
        assert_eq!(Priority::parse("medium"), Some(Priority::Medium));
        assert_eq!(Priority::parse("high"), Some(Priority::High));
    }

    #[test]
    fn ex1_parse_ignores_case() {
        assert_eq!(Priority::parse("HIGH"), Some(Priority::High));
        assert_eq!(Priority::parse("Low"), Some(Priority::Low));
    }

    #[test]
    fn ex1_parse_rejects_unknown_text() {
        assert_eq!(Priority::parse("urgent"), None);
        assert_eq!(Priority::parse(""), None);
    }

    #[test]
    fn ex2_new_task_has_defaults() {
        let task = Task::new(1, "Buy milk");
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Buy milk");
        assert_eq!(task.priority, Priority::Medium);
        assert!(!task.done);
    }

    #[test]
    fn ex3_is_done_reads_the_field() {
        let mut task = Task::new(1, "Buy milk");
        assert!(!task.is_done());
        task.done = true;
        assert!(task.is_done());
    }

    #[test]
    fn ex4_complete_marks_task_done() {
        let mut task = Task::new(1, "Buy milk");
        task.complete();
        assert!(task.is_done());
    }

    #[test]
    fn ex4_completing_twice_keeps_it_done() {
        let mut task = Task::new(1, "Buy milk");
        task.complete();
        task.complete();
        assert!(task.is_done());
    }

    #[test]
    fn ex5_with_priority_changes_only_priority() {
        let task = Task::new(7, "Do taxes").with_priority(Priority::High);
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.id, 7);
        assert_eq!(task.title, "Do taxes");
        assert!(!task.done);
    }

    #[test]
    fn ex6_display_open_task() {
        let task = Task::new(1, "Buy milk");
        assert_eq!(task.to_string(), "[ ] 1: Buy milk (medium)");
    }

    #[test]
    fn ex6_display_done_task() {
        let mut task = Task::new(2, "Do taxes").with_priority(Priority::High);
        task.complete();
        assert_eq!(task.to_string(), "[x] 2: Do taxes (high)");
    }
}
