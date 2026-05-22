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
  <title>DEADLINES</title>
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { height: 100%; }
    body {
      font-family: "Arial Black", "Arial Bold", sans-serif;
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
      font-family: Arial, sans-serif;
      font-size: .85rem;
      font-weight: 400;
      color: #555;
    }
    .card-days {
      font-size: 7rem;
      font-weight: 900;
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
    .add-form {
      display: flex;
      gap: .75rem;
      flex-wrap: wrap;
      padding: 1.5rem;
      background: #111;
      border: 4px solid #fff;
    }
    .add-form input {
      padding: .75rem 1rem;
      border: 3px solid #fff;
      background: #000;
      color: #fff;
      font: 900 1rem "Arial Black", sans-serif;
      text-transform: uppercase;
    }
    .add-form input::placeholder { color: #555; }
    .add-form input[type="text"] { flex: 1; min-width: 200px; }
    .add-form button {
      padding: .75rem 1.5rem;
      border: 3px solid #fff;
      background: #fff;
      color: #000;
      font: 900 1rem "Arial Black", sans-serif;
      text-transform: uppercase;
      letter-spacing: .05em;
      cursor: pointer;
    }
    .add-form button:hover { background: #ffcc00; border-color: #ffcc00; }
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
          + '<div class="card-days">' + (days < 0 ? Math.abs(days) : days) + '</div>'
          + '<span class="label">' + (days < 0 ? "days ago" : "days left") + '</span>';
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
