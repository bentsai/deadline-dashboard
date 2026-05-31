# What and When

A place to organize top-of-mind projects and upcoming due dates.

With What and When, you can set countdowns to show how many days before something is due, and create cards that describe the big things. Finally, a scratch area to take notes.

This is not meant for task breakdowns or managing projects. No notifications. No details. Just big words.

![Screenshot](screenshot.png)

## Terminology

- **What** — cards for what's top of mind right now
- **When** — countdown cards with due dates
- **Words** — scratch space for notes

Add a card to the What or When section with the **+** button in that section's header.

## Running

Open `index.html` in your browser. That's it — no build step, no server required.

Or serve it locally:

```sh
python3 -m http.server 8080
```

## Data and sync

All data lives in your browser's localStorage. There is no cloud sync, no accounts, no server-side storage. Your data never leaves your machine.

## Export / Import

Click "export" in the top-right corner. This encodes all your data (When cards, What cards, and Words notes) into a URL with a base64 hash fragment. Copy the URL and open it in another browser to import everything. The import is a one-time snapshot — changes made afterward are independent in each browser.

## Features

- Countdown cards with urgency-based color coding (red/yellow/white)
- Mark items as done (green card with completion timestamp)
- "What" cards for what's top of mind
- Scratch space for daily notes
- Drag to reorder
- Natural language date parsing (see below)
- Export/import via URL for moving between browsers

## Date parsing

When adding a When card, type a name followed by a date. If no date is detected, it defaults to today at 5:00 PM. Examples:

| Input | Parsed as |
|-------|-----------|
| `Report due today` | Today at 9:00 AM |
| `Call mom tomorrow` | Tomorrow at 9:00 AM |
| `Meeting tomorrow at 3pm` | Tomorrow at 3:00 PM |
| `Ship feature friday` | Nearest Friday (including today) at 9:00 AM |
| `Demo next wednesday` | Wednesday after the nearest one, at 9:00 AM |
| `Proposal in 3 days` | 3 days from now at 9:00 AM |
| `Deploy in 2 weeks` | 14 days from now at 9:00 AM |
| `Reminder in 4 hours` | 4 hours from now (exact time) |
| `Submit report june 15` | June 15 at 9:00 AM |
| `Taxes 4/15` | April 15 at 9:00 AM |
| `Party 12/31 at 8pm` | December 31 at 8:00 PM |
| `Fix the bug` | Today at 5:00 PM (no date detected) |

Day names must be spelled out in full (e.g., "friday" not "fri").
