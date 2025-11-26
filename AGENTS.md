# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the main binary entrypoint (`main.rs`) for cargo-based builds.
- Top-level feature folders (e.g., `array/`, `linked_list/`, `hot100/`) contain LeetCode-style solutions, often as standalone scripts.
- `rust_scripty_demo/` and several files under `array/` use `rust-script` with shebangs for quick, dependency-light execution.
- Supporting notes and templates live in `algorithm_templates.md`, `rust_cheatsheet.md`, and similar markdown files.

## Build, Test, and Development Commands
- `cargo run` — run the cargo binary in `src/main.rs`.
- `cargo test` — execute the cargo test suite (e.g., unit tests in `linked_list/lc206_reverse_list.rs`).
- `rust-script path/to/file.rs` — run standalone scripts that include a `#!/usr/bin/env rust-script` shebang (e.g., `array/remove_item/lc26.rs`, `hot100/lc160.rs`).
- `cargo fmt` (optional but recommended) — format Rust codebase-wide.

## Coding Style & Naming Conventions
- Use Rust defaults: 4-space indentation, snake_case for functions/variables, CamelCase for types.
- Prefer returning `Option`/`Result` where meaningful; avoid `unwrap()` in non-demo logic.
- Keep comments concise, focused on intent or edge cases. Follow existing module patterns for solution layout.
- When adding scripts, include a brief header describing the problem and approach.

## Testing Guidelines
- Place unit tests near implementations using Rust’s `#[cfg(test)]` modules (see `linked_list/lc206_reverse_list.rs`).
- Name tests after behavior (e.g., `test_reverse_list_iterative`).
- Run `cargo test` for cargo-managed code; use `rust-script` to manually exercise standalone solutions or add lightweight asserts in `main()`.

## Commit & Pull Request Guidelines
- Write commits in the style “area: summary” (e.g., `linked_list: add intersection finder`); keep them focused and descriptive.
- PRs should summarize the change set, list key commands run (`cargo test`, `rust-script ...`), and mention affected paths (e.g., `array/binary_search/...`).
- Include notes on edge cases handled or remaining gaps; attach screenshots only if a change affects output formatting or logged structure.

## 沟通与回答
- 所有回答使用中文，保持简洁明确，必要时可补充代码或命令示例，避免中英混杂。
