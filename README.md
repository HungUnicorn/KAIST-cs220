# KAIST CS220: Programming Principles in Rust

Solutions for KAIST CS220 covering type systems, memory safety, concurrency, parsing, and program correctness in Rust.

## Topics

| Assignment | Topic |
|:-----------|:------|
| 01 | Rust fundamentals |
| 02 | Vector & matrix operations |
| 03 | Shell lexer/parser, operator overloading |
| 04 | AST parser & evaluator (`pest`, `clap`) |
| 06 | Semirings, symbolic differentiation |
| 07 | Custom iterators & combinators |
| 08 | Church encoding, lambda calculus |
| 09 | Arbitrary-precision `BigInt`, matrix multiplication |
| 10 | Graph pathfinding, labyrinth solver |
| 11 | Ownership-safe graphs & linked lists |
| 12 | Multi-threaded channels (`demux`/`funnel`) |
| 13 | Parallel iterators via `rayon` |

## Dependencies

- **Toolchain**: Rust Edition 2021
- **Concurrency**: `rayon`, `std::sync::mpsc`, `Mutex`
- **Parsing**: `pest`, `pest_derive`, `clap`
- **Numerics**: `ndarray`, `ndarray-rand`, `approx`, `itertools`
- **Testing**: `ntest`, strict `rustc` lints (`#![deny(warnings)]`)

## Usage

```bash
cargo build                  # build
cargo test                   # run all tests
cargo doc --open             # generate docs

# binaries
cargo run --bin calc --features build-calc -- <expr-file>
cargo run --bin par_iter
```
