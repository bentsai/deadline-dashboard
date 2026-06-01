// Tier 1 — fast logic tests. No browser, no deps. Run: `node --test test.mjs`.
//
// These cover the pure logic (date parsing, urgency, countdown, export codec)
// that agents were previously re-verifying through the browser every cycle.
// DOM-dependent interactions (inline edit, drag-reorder, the wireInput blur
// latch, the setInterval edit-guard) are NOT covered here — those are Tier 2,
// exercised in a real browser. See AGENTS.md.
//
// No-drift design: instead of copying logic out of index.html, we evaluate the
// actual shipped <script> in a vm sandbox with just-permissive-enough stubs for
// document/localStorage/location, then call its functions directly. A change to
// parseDate in index.html is reflected here with zero edits.

import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import vm from "node:vm";

const here = dirname(fileURLToPath(import.meta.url));
const html = readFileSync(join(here, "index.html"), "utf8");

// Grab the main app <script> (the one defining the logic, not the pre-paint theme script).
const scripts = [...html.matchAll(/<script>([\s\S]*?)<\/script>/g)].map((m) => m[1]);
const source = scripts.find((s) => s.includes("function parseDate"));
assert.ok(source, "could not find the app script in index.html");

// A stub DOM element that absorbs any property read or method call without
// throwing. We never assert on the DOM here — we only need render()/renderWhat()
// and the top-level init to run to completion so the functions become available.
function stubEl() {
  const el = {
    innerHTML: "", textContent: "", value: "", className: "", title: "",
    style: {}, dataset: {},
    classList: { add() {}, remove() {}, toggle() { return false; }, contains() { return false; } },
    addEventListener() {}, removeEventListener() {},
    appendChild() {}, prepend() {}, remove() {},
    setAttribute() {}, removeAttribute() {}, getAttribute() { return null; },
    setSelectionRange() {}, focus() {},
    querySelector() { return null; }, querySelectorAll() { return []; },
  };
  return el;
}

const sandbox = {
  console: { warn() {}, log() {}, error() {} },
  document: {
    createElement: () => stubEl(),
    getElementById: () => stubEl(),
    querySelector: () => stubEl(),       // e.g. the theme-color <meta>; a stub absorbs setAttribute
    querySelectorAll: () => [],
    documentElement: stubEl(),
    title: "",
  },
  location: { hash: "", pathname: "/", origin: "https://test", reload() {}, replace() {} },
  history: { replaceState() {} },
  navigator: { clipboard: { writeText: () => Promise.resolve() } },
  localStorage: (() => {
    const m = new Map();
    return {
      getItem: (k) => (m.has(k) ? m.get(k) : null),
      setItem: (k, v) => m.set(k, String(v)),
      removeItem: (k) => m.delete(k),
    };
  })(),
  setInterval: () => 0,                  // no-op so the 60s redraw doesn't keep the process alive
  setTimeout: () => 0,
  clearTimeout: () => {},
  addEventListener: () => {},
  btoa, atob, escape, unescape,
  encodeURIComponent, decodeURIComponent,
};
sandbox.window = sandbox;

vm.createContext(sandbox);
vm.runInContext(source, sandbox, { filename: "index.html#script" });

// Top-level `function` declarations land on the sandbox; pull out what we test.
const { parseDate, splitInput, extractTime, urgency, timeUntil, formatForEdit, encodeData, decodeData } = sandbox;

// --- helpers for relative-date assertions (avoid hardcoding today) -----------
const MS_HOUR = 3600000;
const dayOf = (n) => { const d = new Date(); d.setDate(d.getDate() + n); return d; };
const sameDay = (iso, ref) => {
  const d = new Date(iso);
  return d.getFullYear() === ref.getFullYear() && d.getMonth() === ref.getMonth() && d.getDate() === ref.getDate();
};
const hm = (iso) => { const d = new Date(iso); return [d.getHours(), d.getMinutes()]; };

// --- parseDate ---------------------------------------------------------------
test("parseDate: today defaults to 9am", () => {
  const r = parseDate("today");
  assert.ok(sameDay(r, new Date()));
  assert.deepEqual(hm(r), [9, 0]);
});

test("parseDate: tomorrow is +1 day", () => {
  assert.ok(sameDay(parseDate("tomorrow"), dayOf(1)));
});

test("parseDate: next week is +7 days", () => {
  assert.ok(sameDay(parseDate("next week"), dayOf(7)));
});

test("parseDate: 'today at 3pm' applies pm time", () => {
  assert.deepEqual(hm(parseDate("today at 3pm")), [15, 0]);
});

test("parseDate: 'today at 9:30am' applies minutes", () => {
  assert.deepEqual(hm(parseDate("today at 9:30am")), [9, 30]);
});

test("parseDate: 12am midnight, 12pm noon", () => {
  assert.deepEqual(hm(parseDate("today at 12am")), [0, 0]);
  assert.deepEqual(hm(parseDate("today at 12pm")), [12, 0]);
});

test("parseDate: 'in 2 days' / 'in 1 week'", () => {
  assert.ok(sameDay(parseDate("in 2 days"), dayOf(2)));
  assert.ok(sameDay(parseDate("in 1 week"), dayOf(7)));
});

test("parseDate: 'in 3 hours' offsets from now (not 9am)", () => {
  const diff = new Date(parseDate("in 3 hours")) - new Date();
  assert.ok(Math.abs(diff - 3 * MS_HOUR) < 2 * 60000, `expected ~3h, got ${diff}ms`);
});

test("parseDate: bare weekday resolves to that weekday in the future-or-today", () => {
  const r = new Date(parseDate("friday"));
  assert.equal(r.getDay(), 5);
  assert.ok(r >= dayOf(0).setHours(0, 0, 0, 0));
});

test("parseDate: 'next friday' is at least 7 days out from a same-day match", () => {
  const bare = new Date(parseDate("friday"));
  const next = new Date(parseDate("next friday"));
  assert.equal(next.getDay(), 5);
  assert.equal((next - bare) / 86400000, 7);
});

test("parseDate: 'dec 25' uses month/day, rolls year if past", () => {
  const r = new Date(parseDate("dec 25"));
  assert.equal(r.getMonth(), 11);
  assert.equal(r.getDate(), 25);
  assert.ok(r >= new Date());
});

test("parseDate: explicit year is honored even if past", () => {
  const r = new Date(parseDate("jan 1 2020"));
  assert.equal(r.getFullYear(), 2020);
  assert.equal(r.getMonth(), 0);
});

test("parseDate: slash date m/d/yyyy", () => {
  const r = new Date(parseDate("12/25/2026"));
  assert.equal(r.getFullYear(), 2026);
  assert.equal(r.getMonth(), 11);
  assert.equal(r.getDate(), 25);
});

test("parseDate: 2-digit year expands to 20xx", () => {
  assert.equal(new Date(parseDate("1/1/27")).getFullYear(), 2027);
});

test("parseDate: garbage and abbreviated day names return null (strict by design)", () => {
  assert.equal(parseDate("garbage"), null);
  assert.equal(parseDate("fri"), null);     // strict: full weekday names only
  assert.equal(parseDate(""), null);
});

// --- splitInput --------------------------------------------------------------
test("splitInput: separates name from trailing date phrase", () => {
  const r = splitInput("Submit report friday");
  assert.equal(r.name, "Submit report");
  assert.equal(new Date(r.date).getDay(), 5);
});

test("splitInput: handles 'in N days' tail", () => {
  const r = splitInput("Renew domain in 30 days");
  assert.equal(r.name, "Renew domain");
  assert.ok(sameDay(r.date, dayOf(30)));
});

test("splitInput: no recognizable date returns null", () => {
  assert.equal(splitInput("just a note with no date"), null);
});

// --- extractTime -------------------------------------------------------------
test("extractTime: peels trailing 'at 3pm'", () => {
  const r = extractTime("dec 25 at 3pm");
  assert.equal(r.hour, 15);
  assert.equal(r.min, 0);
  assert.equal(r.rest, "dec 25");
});

// --- formatForEdit -----------------------------------------------------------
// Guards the inline-edit path: a card's stored date is rendered into the edit
// textarea by formatForEdit, then re-parsed on save. The displayed phrase must
// parse back to the same instant. formatForEdit always emits an explicit year,
// so parseDate honors it verbatim (no rollover) and we can compare exactly.
const roundTrip = (date) => {
  const phrase = formatForEdit({ name: "Task", date });   // e.g. "Task Jun 1 2026 at 3pm"
  const split = splitInput(phrase);                        // strips the "Task " name back off
  return split && split.date;
};

test("formatForEdit: 9am date round-trips through splitInput unchanged", () => {
  const date = "2026-06-15T09:00";   // 9am ⇒ formatForEdit omits the time, parseDate defaults to 9am
  assert.equal(roundTrip(date), date);
});

test("formatForEdit: non-9am time round-trips (pm)", () => {
  const date = "2026-12-25T15:30";
  assert.equal(roundTrip(date), date);
});

test("formatForEdit: midnight round-trips", () => {
  const date = "2026-03-01T00:00";
  assert.equal(roundTrip(date), date);
});

test("formatForEdit: on-the-hour pm with no minutes round-trips", () => {
  const date = "2026-07-04T13:00";
  assert.equal(roundTrip(date), date);
});

// --- urgency -----------------------------------------------------------------
test("urgency: classifies by days/done/ms", () => {
  assert.equal(urgency(5, true, 0).cls, "done");       // done wins regardless
  assert.equal(urgency(-1, false, -1000).cls, "past");
  assert.equal(urgency(0, false, 6 * MS_HOUR).cls, "urgent");  // ≤12h
  assert.equal(urgency(3, false, 3 * 86400000).cls, "soon");   // ≤5d
  assert.equal(urgency(10, false, 10 * 86400000).cls, "ok");
});

test("urgency: 12h boundary is urgent, just over is soon", () => {
  assert.equal(urgency(0, false, 12 * MS_HOUR).cls, "urgent");
  assert.equal(urgency(1, false, 12 * MS_HOUR + 1).cls, "soon");
});

// --- timeUntil ---------------------------------------------------------------
test("timeUntil: past dates report 'days ago'", () => {
  const past = new Date(Date.now() - 3 * 86400000).toISOString();
  assert.equal(timeUntil(past).unit, "days ago");
});

test("timeUntil: sub-hour reports minutes left (min 1)", () => {
  const soon = new Date(Date.now() + 90000).toISOString();   // 1.5 min
  const r = timeUntil(soon);
  assert.equal(r.unit, "minutes left");
  assert.ok(r.value >= 1);
});

// --- export/import codec -----------------------------------------------------
test("encodeData/decodeData: round-trips unicode payloads", () => {
  const payload = { domain: "work", when: [{ name: "café date ☕", date: "2026-06-01T09:00" }], what: [], words: "naïve — résumé" };
  // Compare as JSON: decodeData parses inside the vm realm, so the result's
  // prototypes differ from the host's and trip deepEqual's prototype check.
  assert.equal(JSON.stringify(decodeData(encodeData(payload))), JSON.stringify(payload));
});
