# rust_2025_repeat

A comprehensive Rust language primitives exercise demonstrating fundamental concepts in Rust programming.

## Overview

This project contains practical examples and exercises covering the core primitive types and language constructs in Rust. It's designed as a learning resource for understanding Rust's type system, ownership model, and basic programming constructs.

## What's Covered

The exercise demonstrates the following Rust language primitives and concepts:

### 1. Basic Data Types
- Integer types (`i8`, `i32`, `i64`, `u8`, `u32`, `u64`, etc.)
- Floating point types (`f32`, `f64`)
- Boolean type (`bool`)
- Character type (`char`) with Unicode support
- Type inference

### 2. String Types
- String literals (`&str`)
- Owned strings (`String`)
- String manipulation (push, formatting)
- String slicing

### 3. Collections
- Arrays (fixed-size collections)
- Vectors (dynamic arrays)
- Tuples (heterogeneous collections)
- Unit type

### 4. Custom Types
- Structs (named field structs, tuple structs, unit structs)
- Enums with different variant types
- Pattern matching

### 5. Control Flow
- Conditional expressions (`if`/`else`)
- Loops (`loop`, `while`, `for`)
- Pattern matching with `match`
- Break and continue statements

### 6. Functions
- Function definitions and calls
- Return values and multiple returns
- Closures (anonymous functions)
- Higher-order functions (`map`, `filter`)

### 7. Ownership and Borrowing
- Move semantics
- Borrowing (immutable and mutable references)
- Copy vs. Clone
- String slicing

### 8. Error Handling
- `Option<T>` type for nullable values
- `Result<T, E>` type for error handling
- Pattern matching for error handling
- Combinators (`map`, `unwrap_or`)

## Running the Exercise

To run this exercise, you'll need Rust installed on your system. If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/).

```bash
# Clone this repository
git clone https://github.com/mudriyjo/rust_2025_repeat.git
cd rust_2025_repeat

# Build the project
cargo build

# Run the exercise
cargo run
```

## Expected Output

The program will output demonstrations of each primitive type and concept, with clear explanations and examples. Each section is labeled and shows practical usage of the language features.

## Learning Objectives

After reviewing and running this exercise, you should understand:
- Rust's type system and primitive types
- How ownership and borrowing work in Rust
- Basic control flow constructs
- Error handling patterns
- Function definitions and closures
- Collection types and their usage

## Further Learning

This exercise covers the fundamentals, but Rust has many more advanced features to explore:
- Lifetimes and advanced borrowing
- Traits and generics
- Modules and packages
- Concurrency and async programming
- Memory management and smart pointers

Happy learning! 🦀