# KAIST-CS220 Programming Principles in Rust

This repository is built on the curriculum of **KAIST CS220: Programming Principles**, demonstrating solutions across systems design, concurrency, memory safety, type systems, and program correctness.

---

## 🚀 Key Highlights & Architectural Overview

This codebase features robust, self-contained implementations across core domains of modern computer science and software engineering:

### 1. ⚡ Concurrency & Parallel Systems
- **Multi-threaded Message Routing (`assignment12`)**: Custom channel demuxing (`demux`) and stream multiplexing (`funnel`) primitives utilizing Rust's `std::sync::mpsc` multi-producer single-consumer channels.
- **Data-Parallel Computation (`assignment13`)**: Custom parallel iterator primitives (`par_iter`) leveraging `rayon` for work-stealing data parallelism across multi-core processors.
- **Thread Synchronization Protocols (`assignment11`, `assignment12`)**: Concurrent resource allocation (`tv_room`) and state machines (`card`) with race-free multi-threaded synchronization.

### 2. 🎛 Language Engineering, Parsing & AST Construction
- **Grammar-Based AST Parser (`calc` binary / `assignment04`)**: Production-ready CLI parser built with `pest` / `pest_derive` and `clap` to parse, validate, and evaluate domain-specific syntax trees.
- **Unix Shell Lexer & Parser (`assignment03`)**: Custom tokenizer and parser for unix-like shell commands, supporting argument escaping, command chaining, and custom operator overloading.
- **Symbolic Differentiation & Semirings (`assignment06`)**: Computer algebra module capable of calculating symbolic calculus derivatives and generic semiring structures.

### 3. 🧠 Advanced Data Structures & Memory Ownership
- **Graphs & Labyrinth Pathfinding (`assignment10`, `assignment11`)**: Graph data structures and maze-solving pathfinders (`labyrinth`) navigating Rust's strict ownership model without unsafe reference cycles.
- **Ownership-Safe Linked Lists (`assignment11`)**: Custom linked lists engineered around Rust's borrow checker rules without memory leaks.
- **Custom Iterator Combinators (`assignment07`)**: Extension traits, custom generators, and lazy evaluation pipelines (`my_itertools`) mirroring `std::iter` standard idioms.

### 4. 🔢 High-Performance & Symbolic Computation
- **Arbitrary-Precision Arithmetic (`BigInt`) (`assignment09`)**: Multi-precision integer arithmetic engine supporting unbounded numeric values and optimized memory layout.
- **Optimized & Parallel Matrix Operations (`matmul`, `vec_and_mat`) (`assignment02`, `assignment09`)**: Vector and matrix algebra implementations featuring parallel matrix multiplication via `ndarray` and multithreading.
- **Functional Foundations & Church Encoding (`assignment08`)**: Lambda calculus abstractions built using pure functional programming patterns in Rust generics.

---

## 🛠 Tech Stack & Core Libraries

| Domain | Tools / Dependencies |
| :--- | :--- |
| **Language Toolchain** | Rust (Edition 2021), Cargo |
| **Concurrency & Multithreading** | `rayon`, `std::sync::mpsc`, `std::sync::Mutex`, Thread Pools |
| **Parsing & AST Execution** | `pest`, `pest_derive`, `clap` CLI parser |
| **Math & Data Science** | `ndarray`, `ndarray-rand`, `approx`, `itertools`, `anyhow` |
| **Quality & Assurance** | `ntest`, Strict `rustc` Lints (`#![deny(warnings)]`, `#![deny(rustdoc::all)]`) |

---

## 📂 Repository Structure

```text
.
├── Cargo.toml               # Package manifest and workspace dependencies
├── src/
│   ├── lib.rs               # Library root with strict safety & documentation flags
│   ├── bin/
│   │   ├── calc.rs          # AST Expression Calculator CLI executable
│   │   └── par_iter.rs      # Parallel iterator benchmark & demonstration
│   └── assignments/         # Module implementations:
│       ├── assignment01/    # Rust fundamentals & foundational exercises
│       ├── assignment02/    # Vector & Matrix linear algebra operations
│       ├── assignment03/    # Unix Shell parser & operator overloading
│       ├── assignment04/    # Context-aware AST parser & evaluator
│       ├── assignment06/    # Semiring algebraic structures & Symbolic Calculus
│       ├── assignment07/    # Custom Iterators, Generators & Combinators
│       ├── assignment08/    # Church encoding & Lambda calculus primitives
│       ├── assignment09/    # Arbitrary-precision BigInt & Parallel Matrix Multiplication
│       ├── assignment10/    # Graph pathfinding & Labyrinth algorithms
│       ├── assignment11/    # Ownership-safe Graphs, Linked Lists & Storage Engine
│       ├── assignment12/    # Multi-threaded channels (Demux/Funnel) & Concurrency
│       └── assignment13/    # Parallel iterators & stream processing
└── scripts/
    ├── build.sh             # Build script
    └── test.sh              # Comprehensive test runner
```

---

## 🧪 Building, Running & Testing

### Prerequisites
- **Rust Toolchain** (1.70 or newer): [Install Rust](https://www.rust-lang.org/tools/install)

### Commands

1. **Build Project**:
   ```bash
   cargo build
   ```

2. **Run All Unit & Integration Tests**:
   ```bash
   cargo test
   ```

3. **Execute AST Calculator CLI**:
   ```bash
   cargo run --bin calc --features build-calc -- <path-to-expression-file>
   ```

4. **Execute Parallel Iterator Benchmark**:
   ```bash
   cargo run --bin par_iter
   ```

5. **Generate & View API Documentation**:
   ```bash
   cargo doc --open
   ```

---
