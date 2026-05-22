wit_bindgen::generate!({
    world: "wasi:http/proxy@0.2.8",
    path: "wit",
    generate_all,
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

const PAGE: &[u8] = br##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>DEADLINES</title>
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Rubik:wght@400;700;900&family=Rubik+Mono+One&display=swap" rel="stylesheet">
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { height: 100%; }
    body {
      font-family: "Rubik", sans-serif;
      background: #000;
      color: #fff;
      padding: 2rem;
    }
    h1 {
      font-size: 3rem;
      text-transform: uppercase;
      letter-spacing: .1em;
      margin-bottom: 2rem;
      border-bottom: 6px solid #fff;
      padding-bottom: 1rem;
    }
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
      gap: 1.5rem;
      margin-bottom: 2.5rem;
    }
    .card {
      background: #fff;
      color: #000;
      padding: 2rem;
      border: 4px solid #000;
      outline: 4px solid #fff;
      display: flex;
      flex-direction: column;
      gap: .75rem;
      min-height: 220px;
      justify-content: space-between;
    }
    .card.urgent { background: #ff0000; color: #fff; border-color: #ff0000; outline-color: #ff0000; }
    .card.urgent .card-date { color: rgba(255,255,255,.7); }
    .card.urgent .label { color: rgba(255,255,255,.7); }
    .card.urgent .delete-btn { color: rgba(255,255,255,.5); }
    .card.urgent .delete-btn:hover { color: #fff; }
    .card.soon { background: #ffcc00; color: #000; border-color: #ffcc00; outline-color: #ffcc00; }
    .card.past { background: #333; color: #999; border-color: #333; outline-color: #555; }
    .card.past .card-days { color: #666; }
    .card.past .label { color: #666; }
    .card-name {
      font-size: 1.1rem;
      font-weight: 900;
      text-transform: uppercase;
      letter-spacing: .05em;
    }
    .card-date {
      font-family: "Rubik", sans-serif;
      font-size: .85rem;
      font-weight: 400;
      color: #555;
    }
    .card-days {
      font-family: "Rubik Mono One", monospace;
      font-size: 7rem;
      font-weight: 400;
      line-height: 1;
      letter-spacing: -.04em;
    }
    .label {
      display: block;
      font-size: 1rem;
      font-weight: 900;
      text-transform: uppercase;
      letter-spacing: .1em;
      color: #555;
    }
    .add-card {
      background: transparent;
      border: 2px solid #333;
      padding: 2rem;
      min-height: 220px;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      transition: border-color .15s;
    }
    .add-card:hover { border-color: #666; }
    .add-card .plus {
      font-size: 1.5rem;
      font-weight: 400;
      color: #333;
      font-family: "Rubik", sans-serif;
      transition: color .15s;
    }
    .add-card:hover .plus { color: #666; }
    .add-card.active {
      border-color: #fff;
      flex-direction: column;
      align-items: stretch;
      justify-content: start;
      gap: .75rem;
      cursor: default;
      padding: 1.5rem;
    }
    .inline-input {
      width: 100%;
      padding: .75rem;
      border: 3px solid #fff;
      background: #000;
      color: #fff;
      font: 900 1rem "Rubik", sans-serif;
    }
    .inline-input::placeholder { color: #444; text-transform: none; font-weight: 400; font-family: "Rubik", sans-serif; font-size: .9rem; }
    .inline-input:focus { outline: none; border-color: #ffcc00; }
    .inline-hint {
      font-family: "Rubik", sans-serif;
      font-size: .7rem;
      font-weight: 400;
      color: #444;
    }
    .inline-error {
      font-family: "Rubik", sans-serif;
      font-size: .8rem;
      font-weight: 700;
      color: #ff0000;
    }
    .inline-actions { display: flex; gap: .5rem; margin-top: .25rem; }
    .inline-actions button {
      padding: .5rem 1rem;
      border: 2px solid #555;
      background: transparent;
      color: #999;
      font: 700 .75rem "Rubik", sans-serif;
      text-transform: uppercase;
      letter-spacing: .05em;
      cursor: pointer;
    }
    .inline-actions button:hover { border-color: #fff; color: #fff; }
    .inline-actions button.primary { border-color: #fff; color: #fff; }
    .inline-actions button.primary:hover { background: #fff; color: #000; }
    .inline-actions button.danger:hover { border-color: #ff0000; color: #ff0000; }
    .card.ok .inline-actions button { border-color: #999; color: #333; }
    .card.ok .inline-actions button:hover { border-color: #000; color: #000; }
    .card.ok .inline-actions button.primary { border-color: #000; color: #000; }
    .card.ok .inline-actions button.primary:hover { background: #000; color: #fff; }
    .card.ok .inline-actions button.danger:hover { border-color: #ff0000; color: #ff0000; }
    .card.ok .inline-input { border-color: #000; background: #fff; color: #000; }
    .card.ok .inline-input:focus { border-color: #000; }
    .card.ok .inline-error { color: #ff0000; }
    .card.soon .inline-actions button { border-color: #666; color: #333; }
    .card.soon .inline-actions button:hover { border-color: #000; color: #000; }
    .card.soon .inline-actions button.primary { border-color: #000; color: #000; }
    .card.soon .inline-actions button.primary:hover { background: #000; color: #ffcc00; }
    .card.soon .inline-actions button.danger:hover { border-color: #ff0000; color: #ff0000; }
    .card.soon .inline-input { border-color: #000; background: #ffcc00; color: #000; }
    .card.soon .inline-input:focus { border-color: #000; }
    .card { cursor: pointer; transition: outline-color .1s; }
    .card:hover { outline-color: #ffcc00; }
    .card.editing { outline-color: #fff; cursor: default; }
    .delete-btn {
      background: none;
      border: none;
      color: #999;
      font-size: 1.5rem;
      font-weight: 900;
      cursor: pointer;
      padding: 0 .5rem;
      line-height: 1;
      margin-left: auto;
    }
    .delete-btn:hover { color: #ff0000; }
  </style>
</head>
<body>
  <h1>Deadlines</h1>
  <div class="grid" id="grid"></div>
  <script>
    const SEED = [{name:"Performance reflection due",date:"2026-05-29T09:00"}];
    const KEY = "deadline-dashboard-v1";

    function load() {
      const raw = localStorage.getItem(KEY);
      if (raw) return JSON.parse(raw);
      save(SEED);
      return SEED;
    }
    function save(deadlines) { localStorage.setItem(KEY, JSON.stringify(deadlines)); }

    function daysUntil(dateStr) {
      const target = new Date(dateStr);
      const now = new Date();
      return Math.ceil((target - now) / (1000 * 60 * 60 * 24));
    }

    const MONTHS = {jan:0,feb:1,mar:2,apr:3,may:4,jun:5,jul:6,aug:7,sep:8,oct:9,nov:10,dec:11,
      january:0,february:1,march:2,april:3,june:5,july:6,august:7,september:8,october:9,november:10,december:11};
    const DAYS = {sun:0,mon:1,tue:2,wed:3,thu:4,fri:5,sat:6,
      sunday:0,monday:1,tuesday:2,wednesday:3,thursday:4,friday:5,saturday:6};

    function parseDate(str) {
      const s = str.trim().toLowerCase();
      const now = new Date();
      let hour = 9, min = 0;

      const timeMatch = s.match(/(?:at\s+)?(\d{1,2})(?::(\d{2}))?\s*(am|pm)?/);
      if (timeMatch) {
        hour = parseInt(timeMatch[1]);
        min = timeMatch[2] ? parseInt(timeMatch[2]) : 0;
        if (timeMatch[3] === "pm" && hour < 12) hour += 12;
        if (timeMatch[3] === "am" && hour === 12) hour = 0;
      }
      const hasTime = !!timeMatch && (timeMatch[3] || timeMatch[2] || parseInt(timeMatch[1]) !== parseInt(s.match(/\d+/)?.[0]));

      if (s === "today") {
        return makeDate(now.getFullYear(), now.getMonth(), now.getDate(), hour, min);
      }
      if (s === "tomorrow") {
        const d = new Date(now); d.setDate(d.getDate() + 1);
        return makeDate(d.getFullYear(), d.getMonth(), d.getDate(), hour, min);
      }

      const nextDay = s.match(/^next\s+(\w+)/);
      if (nextDay && DAYS[nextDay[1]] !== undefined) {
        const target = DAYS[nextDay[1]];
        const d = new Date(now);
        let diff = target - d.getDay();
        if (diff <= 0) diff += 7;
        d.setDate(d.getDate() + diff);
        return makeDate(d.getFullYear(), d.getMonth(), d.getDate(), hour, min);
      }

      for (const [name, idx] of Object.entries(MONTHS)) {
        const re = new RegExp(name + "\\s+(\\d{1,2})(?:[,\\s]+(\\d{4}))?");
        const m = s.match(re);
        if (m) {
          const day = parseInt(m[1]);
          let year = m[2] ? parseInt(m[2]) : now.getFullYear();
          const candidate = new Date(year, idx, day);
          if (candidate < now && !m[2]) year++;
          if (!hasTime) { hour = 9; min = 0; }
          return makeDate(year, idx, day, hour, min);
        }
        const re2 = new RegExp("(\\d{1,2})\\s+" + name + "(?:[,\\s]+(\\d{4}))?");
        const m2 = s.match(re2);
        if (m2) {
          const day = parseInt(m2[1]);
          let year = m2[2] ? parseInt(m2[2]) : now.getFullYear();
          const candidate = new Date(year, idx, day);
          if (candidate < now && !m2[2]) year++;
          if (!hasTime) { hour = 9; min = 0; }
          return makeDate(year, idx, day, hour, min);
        }
      }

      const slashDate = s.match(/(\d{1,2})\/(\d{1,2})(?:\/(\d{2,4}))?/);
      if (slashDate) {
        const mo = parseInt(slashDate[1]) - 1;
        const day = parseInt(slashDate[2]);
        let year = slashDate[3] ? parseInt(slashDate[3]) : now.getFullYear();
        if (year < 100) year += 2000;
        if (!hasTime) { hour = 9; min = 0; }
        return makeDate(year, mo, day, hour, min);
      }

      return null;
    }

    function makeDate(y, m, d, h, min) {
      const dt = new Date(y, m, d, h, min);
      const pad = n => String(n).padStart(2, "0");
      return dt.getFullYear() + "-" + pad(dt.getMonth()+1) + "-" + pad(dt.getDate()) + "T" + pad(dt.getHours()) + ":" + pad(dt.getMinutes());
    }

    function splitInput(str) {
      const s = str.trim();
      const patterns = [
        /^(.+?)\s+((?:next\s+)?(?:mon|tue|wed|thu|fri|sat|sun)\w*(?:\s+at\s+\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
        /^(.+?)\s+(tomorrow(?:\s+at\s+\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
        /^(.+?)\s+(today(?:\s+at\s+\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
        /^(.+?)\s+(\d{1,2}\/\d{1,2}(?:\/\d{2,4})?(?:\s+(?:at\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
        /^(.+?)\s+((?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)\w*\s+\d{1,2}(?:[,\s]+\d{4})?(?:\s+(?:at\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
        /^(.+?)\s+(\d{1,2}\s+(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)\w*(?:[,\s]+\d{4})?(?:\s+(?:at\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm)?)?)\s*$/i,
      ];
      for (const re of patterns) {
        const m = s.match(re);
        if (m && m[1].trim() && parseDate(m[2].trim())) {
          return { name: m[1].trim(), date: parseDate(m[2].trim()) };
        }
      }
      return null;
    }

    function formatForEdit(d) {
      const dateObj = new Date(d.date);
      const months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
      const mo = months[dateObj.getMonth()];
      const day = dateObj.getDate();
      const year = dateObj.getFullYear();
      const h = dateObj.getHours();
      const m = dateObj.getMinutes();
      let time = "";
      if (h !== 9 || m !== 0) {
        const suffix = h >= 12 ? "pm" : "am";
        const h12 = h > 12 ? h - 12 : (h === 0 ? 12 : h);
        time = " at " + h12 + (m > 0 ? ":" + String(m).padStart(2,"0") : "") + suffix;
      }
      return d.name + " " + mo + " " + day + " " + year + time;
    }

    function render() {
      const deadlines = load();
      deadlines.sort((a, b) => new Date(a.date) - new Date(b.date));
      const grid = document.getElementById("grid");
      grid.innerHTML = "";
      deadlines.forEach((d, i) => {
        const days = daysUntil(d.date);
        let cls = "ok";
        if (days < 0) cls = "past";
        else if (days <= 2) cls = "urgent";
        else if (days <= 7) cls = "soon";

        const card = document.createElement("div");
        card.className = "card " + cls;
        const dateObj = new Date(d.date);
        const formatted = dateObj.toLocaleDateString(undefined, {weekday:"short", month:"short", day:"numeric", year:"numeric"});
        const timeStr = dateObj.toLocaleTimeString(undefined, {hour:"numeric", minute:"2-digit"});
        card.innerHTML = '<div style="display:flex;align-items:start"><span class="card-name">' + escHtml(d.name) + '</span></div>'
          + '<div class="card-date">' + escHtml(formatted + " at " + timeStr) + '</div>'
          + '<div class="card-days">' + (days < 0 ? Math.abs(days) : days) + '</div>'
          + '<span class="label">' + (days < 0 ? "days ago" : "days left") + '</span>';
        card.addEventListener("click", () => startEdit(card, i));
        grid.appendChild(card);
      });

      const addCard = document.createElement("div");
      addCard.className = "add-card";
      addCard.innerHTML = '<span class="plus">+ new deadline</span>';
      addCard.addEventListener("click", function handler() {
        addCard.removeEventListener("click", handler);
        addCard.classList.add("active");
        addCard.innerHTML = '<input class="inline-input" id="add-input" placeholder="Project proposal due June 15">'
          + '<span class="inline-hint">Enter to save. Esc to cancel.</span>'
          + '<span class="inline-error" id="add-error"></span>';
        const input = document.getElementById("add-input");
        input.focus();
        input.addEventListener("keydown", (e) => { if (e.key === "Enter") doAdd(); if (e.key === "Escape") render(); });
        input.addEventListener("blur", () => { if (input.value.trim()) doAdd(); else render(); });
      });
      grid.appendChild(addCard);
    }

    function doAdd() {
      const input = document.getElementById("add-input");
      if (!input) return;
      const val = input.value.trim();
      const err = document.getElementById("add-error");
      if (!val) { render(); return; }
      const result = splitInput(val);
      if (!result) { if (err) err.textContent = "Include a date, e.g. \"Report due May 30\""; return; }
      const deadlines = load();
      deadlines.push({name: result.name, date: result.date});
      save(deadlines);
      render();
    }

    function startEdit(card, idx) {
      if (card.classList.contains("editing")) return;
      card.classList.add("editing");
      const deadlines = load();
      const d = deadlines[idx];
      card.innerHTML = '<input class="inline-input" id="edit-input" value="' + escAttr(formatForEdit(d)) + '">'
        + '<span class="inline-error" id="edit-error"></span>'
        + '<span class="inline-hint">Enter to save. Esc to cancel. Clear to delete.</span>';
      const input = document.getElementById("edit-input");
      input.focus();
      input.setSelectionRange(0, input.value.length);
      let saved = false;
      function trySave() {
        if (saved) return;
        saved = true;
        const val = input.value.trim();
        if (!val) { deadlines.splice(idx, 1); save(deadlines); render(); return; }
        const result = splitInput(val);
        if (!result) { saved = false; document.getElementById("edit-error").textContent = "Include a date"; return; }
        deadlines[idx] = {name: result.name, date: result.date};
        save(deadlines);
        render();
      }
      input.addEventListener("keydown", (e) => { if (e.key === "Enter") trySave(); if (e.key === "Escape") render(); });
      input.addEventListener("blur", () => setTimeout(trySave, 100));
      card.addEventListener("click", (e) => e.stopPropagation());
    }

    function escAttr(s) { return s.replace(/&/g,"&amp;").replace(/"/g,"&quot;").replace(/</g,"&lt;"); }

    function escHtml(s) {
      const d = document.createElement("div");
      d.textContent = s;
      return d.innerHTML;
    }

    render();
    setInterval(render, 60000);
  </script>
</body>
</html>
"##;

impl Guest for Component {
    fn handle(_req: IncomingRequest, response_out: ResponseOutparam) {
        let headers = Fields::new();
        let _ = headers.set(&"content-type".to_string(), &[b"text/html; charset=utf-8".to_vec()]);
        let _ = headers.set(&"cache-control".to_string(), &[b"no-cache".to_vec()]);

        let response = OutgoingResponse::new(headers);
        let body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

        let stream = body.write().unwrap();
        for chunk in PAGE.chunks(4096) {
            stream.blocking_write_and_flush(chunk).unwrap();
        }
        drop(stream);
        OutgoingBody::finish(body, None).unwrap();
    }
}

export!(Component);
