# 2024 Advent of code solutions

Following along with the challenges on [adventofcode.com](https://adventofcode.com/).

Decided (for better or for worse) to try to cycle around a range of languages, testing how well I logic the puzzle and use GitHub Copilot within VSCode to help me figure out the syntax for what I want to write across different languages. Coming at it from an R background I'm assuming it might be trickier than I expect to translate the way I program in R to other languages, and doing it as a way to expose myself to a range of different ways of approaching code very quickly.

Plan is to run through the most popular languages (or at least open source ones that I can easily install and run within VSCode) in order until I fail a day (or until I decide it's a silly idea) and then revert to cycling between R (what I know best), Python and TypeScript (what I'm currently learning in other projects).

[Top languages](https://jeroenheijmans.github.io/advent-of-code-surveys/) in 2023:

| Day  | Language          | Users in 2023 | Success? |
|------|-------------------|------|----------|
| 1    | Python 3          | 1225 | Yes      | 
| 2    | Rust              | 530  | Yes      |
| 3    | JavaScript        | 237  | Yes      |
| 4    | C++               | 235  | |
| 5    | C#                | 230  | |
| 6    | Java              | 182  | |
| 7    | Go                | 160  | |
| 8    | TypeScript        | 142  | |
| 9    | Kotlin            | 123  | |
| 10   | C                 | 92   | |
| 11   | Haskell           | 91   | |
| 12   | Ruby              | 57   | |
| 13   | Scala             | 51   | |
| 14   | PHP               | 48   | |
| 15   | F#                | 43   | |
| 16   | Elixir            | 40   | |
| 17   | Julia             | 39   | |
| 18   | R                 | 34   | |
| 19   | Perl 5            | 31   | |
| 20   | Swift             | 28   | |
| 21   | Zig               | 28   | |
| 22   | Bash/Shell        | 27   | |
|      | Excel (skipped)   | 27   | |
| 23   | Clojure           | 26   | |
| 24   | OCaml             | 24   | |
| 25   | Lua               | 21   | |

# Day 1 - Python

To run day one's script, ensure you:
1. [Install Python](https://www.python.org/downloads/)
2. Install any dependencies using `pip install ...`
3. `python 2024/day1.py`

# Day 2 - Rust

I used `cargo new ...` to create a project folder, then edited the `main.rs` script.

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Open terminal within rust project e.g. `cd 2024/day2_rust`
2. `cargo run`

# Day 3 - JavaScript

1. [Install npm](https://nodejs.org/en/download/package-manager)
2. `node 2024/day3.js`