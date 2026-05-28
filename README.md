# What and When

A place to organize top-of-mind projects and upcoming due dates.

With What and When, you can set countdowns to show how many days before something is due, and create cards that describe the big things. Finally, a scratch area to take notes.

This is not meant for task breakdowns or managing projects. No notifications. No details. Just big words.

![Screenshot](screenshot.png)

## Terminology

- **When** — countdown cards with due dates
- **What** — cards for what's top of mind right now
- **Words** — scratch space for notes

## Running

Open `index.html` in your browser. That's it — no build step, no server required.

Or serve it locally:

```sh
python3 -m http.server 8080
```

## Data and sync

All data lives in your browser's localStorage. There is no cloud sync, no accounts, no server-side storage. Your data never leaves your machine.

## Export / Import

Click "export" in the bottom-right corner. This encodes all your data (When cards, What cards, and Words notes) into a URL with a base64 hash fragment. Copy the URL and open it in another browser to import everything. The import is a one-time snapshot — changes made afterward are independent in each browser.

## Features

- Countdown cards with urgency-based color coding (red/yellow/white)
- Mark items as done (green card with completion timestamp)
- "What" cards for what's top of mind
- Scratch space for daily notes
- Drag to reorder
- Natural language date parsing ("next friday", "next week", "june 15 9am")
- Export/import via URL for moving between browsers
