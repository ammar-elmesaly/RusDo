# RusDo - A minimalist to-do list CLI tool

This is my very first Rust project. I was messing around with the language recently and decided to enhance my Rust skills—and more broadly, my programming skills—by building a project in this memory-safe, low-level language.

## Description

### Brief Introduction

The project consists of several modules, with main ones like `add_task`, `ui`, and `menu`. It utilizes the Rust **ratatui** library together with **crossterm** (a well-known terminal backend used to communicate with the shell).

### About the Project

`main.rs` contains all the module definitions and calls `menu::run_loop`, which runs the main menu loop. The main menu loop handles all menu items enumerated in the `MenuAction` enum. Menu items are stored in the `MenuItem` struct, which holds the content and the action, while the `Menu` struct holds a vector of `MenuItems` and the `selected` index (useful for navigation).

The `menu.rs` file contains all menu-related structures, functions, and modules. The project uses SQLite3, provided by the **rusqlite** crate.

Each window in the project (view tasks, delete a task, about, etc.) has its own `run_loop` and `handle_events` function. The `run_loop` of each window returns to its parent window. If you’re at the top-level `run_loop`, the program simply quits after returning.

### Note

This project would probably have been much simpler to implement in a language like Python (since it isn’t performance-critical), but it serves as a practice project, and I’ve learned a lot from it.
