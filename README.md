# clitracker
CLITracker is a Rust-based terminal productivity tool with an ncurses UI. It integrates with Pivotal Tracker’s API to fetch, update, and manage tasks from your command line. Designed for developers who live in the terminal, it keeps you focused and efficient without switching to a browser.

## Development Setup

1. Copy `.env.example` to `.env` and add your Pivotal Tracker API token.
2. Build and run the application with `cargo run`.

This project uses ncurses for a terminal UI and reqwest for interacting with the Pivotal Tracker API. The current implementation contains stub functions and a basic menu layout.
