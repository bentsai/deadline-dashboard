wit_bindgen::generate!({
    world: "wasi:http/proxy@0.2.0",
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
  <title>Deadline Dashboard</title>
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <style>
    :root { color-scheme: light dark; --accent: #6366f1; --danger: #ef4444; --warn: #f59e0b; --ok: #10b981; }
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { height: 100%; }
    body {
      font: 16px/1.5 -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif;
      background: #f8fafc;
      color: #1e293b;
      padding: 2rem;
    }
    @media (prefers-color-scheme: dark) {
      body { background: #0f172a; color: #e2e8f0; }
    }
    h1 { font-size: 1.75rem; margin-bottom: 1.5rem; letter-spacing: -.02em; }
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
      gap: 1rem;
      margin-bottom: 2rem;
    }
    .card {
      background: white;
      border-radius: .75rem;
      padding: 1.25rem;
      box-shadow: 0 1px 3px rgba(0,0,0,.08);
      border-left: 4px solid var(--accent);
      display: flex;
      flex-direction: column;
      gap: .5rem;
    }
    @media (prefers-color-scheme: dark) {
      .card { background: #1e293b; box-shadow: 0 1px 3px rgba(0,0,0,.3); }
    }
    .card.urgent { border-left-color: var(--danger); }
    .card.soon { border-left-color: var(--warn); }
    .card.ok { border-left-color: var(--ok); }
    .card.past { border-left-color: #94a3b8; opacity: .6; }
    .card-name { font-weight: 600; font-size: 1rem; }
    .card-date { font-size: .8rem; color: #64748b; }
    @media (prefers-color-scheme: dark) { .card-date { color: #94a3b8; } }
    .card-days { font-size: 2rem; font-weight: 700; }
    .card-days .label { font-size: .8rem; font-weight: 400; color: #64748b; }
    @media (prefers-color-scheme: dark) { .card-days .label { color: #94a3b8; } }

    .add-form {
      display: flex; gap: .5rem; flex-wrap: wrap;
      padding: 1rem; background: white; border-radius: .75rem;
      box-shadow: 0 1px 3px rgba(0,0,0,.08);
    }
    @media (prefers-color-scheme: dark) { .add-form { background: #1e293b; } }
    .add-form input {
      padding: .5rem .75rem; border: 1px solid #cbd5e1; border-radius: .5rem;
      font: inherit; background: inherit; color: inherit;
    }
    @media (prefers-color-scheme: dark) { .add-form input { border-color: #475569; } }
    .add-form input[type="text"] { flex: 1; min-width: 150px; }
    .add-form button, .delete-btn {
      padding: .5rem 1rem; border: none; border-radius: .5rem;
      background: var(--accent); color: white; font: inherit;
      cursor: pointer; font-weight: 500;
    }
    .add-form button:hover { opacity: .9; }
    .delete-btn {
      background: transparent; color: #94a3b8; font-size: .75rem;
      padding: .25rem .5rem; align-self: flex-start; margin-left: auto;
    }
    .delete-btn:hover { color: var(--danger); }
  </style>
</head>
<body>
  <h1>Deadline Dashboard</h1>
  <div class="grid" id="grid"></div>
  <form class="add-form" id="add-form">
    <input type="text" id="name-input" placeholder="Deadline name" required>
    <input type="datetime-local" id="date-input" required>
    <button type="submit">Add</button>
  </form>
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
        card.innerHTML = '<div style="display:flex;align-items:start"><span class="card-name">' + escHtml(d.name) + '</span><button class="delete-btn" data-idx="' + i + '">&times;</button></div>'
          + '<div class="card-date">' + escHtml(formatted + " at " + timeStr) + '</div>'
          + '<div class="card-days">' + (days < 0 ? Math.abs(days) + ' <span class="label">days ago</span>' : days + ' <span class="label">days left</span>') + '</div>';
        grid.appendChild(card);
      });
      grid.querySelectorAll(".delete-btn").forEach(btn => {
        btn.addEventListener("click", () => {
          const deadlines = load();
          deadlines.splice(parseInt(btn.dataset.idx), 1);
          save(deadlines);
          render();
        });
      });
    }

    function escHtml(s) {
      const d = document.createElement("div");
      d.textContent = s;
      return d.innerHTML;
    }

    document.getElementById("add-form").addEventListener("submit", (e) => {
      e.preventDefault();
      const name = document.getElementById("name-input").value.trim();
      const date = document.getElementById("date-input").value;
      if (!name || !date) return;
      const deadlines = load();
      deadlines.push({name, date});
      save(deadlines);
      document.getElementById("name-input").value = "";
      document.getElementById("date-input").value = "";
      render();
    });

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
        stream.blocking_write_and_flush(PAGE).unwrap();
        drop(stream);
        OutgoingBody::finish(body, None).unwrap();
    }
}

export!(Component);
