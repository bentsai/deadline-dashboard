# Deadline Dashboard

A place to organize top-of-mind projects and upcoming due dates.

With DD, you can set deadlines to show how many days before the due date, and create cards that describe the big things. Finally, a scratch area to take notes.

This is not meant for task breakdowns or managing projects. No notifications. No details. Just big words.

## Running

Requires [wasmtime](https://wasmtime.dev/) and a Rust toolchain with the `wasm32-wasip2` target.

```sh
cargo build --target wasm32-wasip2 --release
wasmtime serve --addr 127.0.0.1:8080 -S cli=y target/wasm32-wasip2/release/deadline_dashboard.wasm
```

Open http://127.0.0.1:8080 in your browser. All data is stored in localStorage.

## Features

- Countdown cards with urgency-based color coding (red/yellow/white)
- Mark deadlines as done (green card with completion timestamp)
- "Now" cards for what's top of mind
- Scratch space for daily notes
- Drag to reorder
- Natural language date parsing ("next friday", "next week", "june 15 9am")
- Export/import via URL for moving between browsers
