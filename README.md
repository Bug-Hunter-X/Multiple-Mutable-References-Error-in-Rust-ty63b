# Multiple Mutable References in Rust
This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same data simultaneously.  This is prevented by the Rust compiler to avoid data races and ensure memory safety.

The `bug.rs` file contains code that attempts to create multiple mutable references.  The `bugSolution.rs` shows how to resolve the issue either by using a single mutable reference or by cloning the data if necessary. 

This example is useful for learning about Rust's borrowing system, which is a core aspect of the language's safety and concurrency features.