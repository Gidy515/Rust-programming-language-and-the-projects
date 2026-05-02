# 📖 The Rust Programming Language — Book Progress

Working through [The Rust Programming Language](https://doc.rust-lang.org/book/) (a.k.a. "the Rust book") and building all the projects along the way.

---

## 🗂️ Progress

| Chapter | Title | Status |
|---------|-------|--------|
| 1 | Getting Started | ✅ Done |
| 2 | Programming a Guessing Game | ✅ Done |
| 3 | Common Programming Concepts | ✅ Done |
| 4 | Understanding Ownership | ✅ Done |
| 5 | Using Structs to Structure Related Data | ✅ Done |
| 6 | Enums and Pattern Matching | ✅ Done |
| 7 | Managing Growing Projects with Packages, Crates, and Modules | ✅ Done |
| 8 | Common Collections | ✅ Done |
| 9 | Error Handling | ✅ Done |
| 10 | Generic Types, Traits, and Lifetimes | ✅ Done |
| 11 | Writing Automated Tests | ✅ Done |
| 12 | An I/O Project: Building a Command Line Program | ✅ Done |
| 13 | Functional Language Features: Iterators and Closures | 🔄 In Progress |
| 14 | More About Cargo and Crates.io | ⬜ Not Started |
| 15 | Smart Pointers | ⬜ Not Started |
| 16 | Fearless Concurrency | ⬜ Not Started |
| 17 | Object-Oriented Programming Features | ⬜ Not Started |
| 18 | Patterns and Matching | ⬜ Not Started |
| 19 | Advanced Features | ⬜ Not Started |
| 20 | Final Project: Building a Multithreaded Web Server | ⬜ Not Started |

**Status key:** ⬜ Not Started · 🔄 In Progress · ✅ Done

---

## 🛠️ Projects

### Project 1 — Guessing Game *(Chapter 2)*
> A CLI number guessing game using random numbers, user input, and basic error handling.

- **Path:** `projects/guessing_game/`
- **Status:** ✅ Done
- **Notes:** —
 Used the `rand` crate for the first time — good intro to adding external dependencies via `Cargo.toml`
  - `loop`, `break`, and `continue` feel natural for game logic
  - `match` on a `Result` type is the idiomatic way to handle errors in Rust — no exceptions
  - `String::new()` + `read_line()` pattern is the standard way to capture user input
  - Shadowing a variable (re-using `guess` as both `String` and `u32`) was a surprising but clean Rust pattern
---

### Project 2 — `grep` CLI Tool: `minigrep` *(Chapter 12)*
> A minimal clone of `grep` — searches for a string in a file from the command line.

- **Path:** `projects/minigrep/`
- **Status:** ✅ Done
- **Notes:**
- `std::env::args()` is the standard way to collect CLI arguments
  - Separating logic into a `Config` struct and a `run()` function keeps `main` clean — good separation of concerns
  - Returning `Box<dyn Error>` from `run()` is a flexible way to bubble up different error types
  - `eprintln!` writes to stderr instead of stdout — important for CLI tools so errors don't pollute piped output
  - The environment variable trick (`IGNORE_CASE`) was a clean intro to `std::env::var()`
  - Iterators and closures in the refactor (Chapter 13 tie-in) made `search()` much more concise

---

### Project 3 — Multithreaded Web Server *(Chapter 20)*
> A basic HTTP web server built from scratch using threads and a thread pool.

- **Path:** `projects/web_server/`
- **Status:** 🔄 Not Done yet
- **Notes:** —

---

## 📁 Repo Structure
