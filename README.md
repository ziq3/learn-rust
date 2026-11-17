# learn-rust

A quick path through the Rust ideas that matter. Syntax you'll pick up as you go. These are the concepts that make Rust *Rust*.

## Setup

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # installs rustc + cargo
```
Editor: VS Code + **rust-analyzer** (shows types inline, which is half the learning).

## How to use

Each lesson is one file in `examples/`. Read it top to bottom, run it, then replace every `todo!()` until its tests pass.

```sh
cargo run  --example 01_ownership   # see it work
cargo test --example 01_ownership   # do the exercises
cargo clippy --examples             # the linter teaches idiomatic Rust; read its suggestions
```

Read the compiler errors all the way through. They're the best Rust teacher you have.

## Lessons

| #  | File               | Why it matters |
|----|--------------------|----------------|
| 01 | `ownership`        | Memory safety without a GC. Everything else builds on this. |
| 02 | `types`            | Enums + `match` + `Option`: make invalid states unrepresentable. |
| 03 | `errors`           | `Result` + `?`: errors are values, not exceptions. |
| 04 | `traits`           | Generics vs `dyn`: polymorphism without inheritance. |
| 05 | `iterators`        | Closures + iterator chains: how idiomatic Rust code looks. |
| 06 | `lifetimes`        | Read the borrow checker's mind (and when to just clone). |
| 07 | `smart_pointers`   | Box / Rc / RefCell / Arc: when single ownership isn't enough. |
| 08 | `concurrency`      | Threads, `Arc<Mutex>`, channels, with data races caught at compile time. |
| 09 | `async`            | tokio, `join!`, `spawn`, `select!`: how modern Rust services run. |

## Skip for now

Writing macros, `unsafe`, `Pin`, advanced lifetime variance, trait object safety rules. You'll know when you need them.

## Next: build something real

The modern crates that cover most of what you'll build:

- **CLI**: `clap` (args), `anyhow` (errors), `serde` + `serde_json` (data)
- **Web API**: `axum` + `tokio` + `serde` + `sqlx` (Postgres/SQLite)
- **HTTP client**: `reqwest`
- **Library errors**: `thiserror`. **Logging**: `tracing`. **Parallel CPU work**: `rayon`

Suggested capstone: a todo REST API with axum + sqlx + SQLite, with tests.

Reference: [The Book](https://doc.rust-lang.org/book/) · [Rust by Example](https://doc.rust-lang.org/rust-by-example/) · [std docs](https://doc.rust-lang.org/std/)
