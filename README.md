# miketang84-todomvc-107

TodoMVC implemented with Leptos SSR + hydration, Axum, SQLx, and SQLite.

## Prerequisites

- Rust toolchain with the `wasm32-unknown-unknown` target
- `cargo-leptos`
- SQLite runtime support

## Local development

1. Copy the example environment file:

```bash
cp .env.example .env
```

2. Start the Leptos development server:

```bash
cargo leptos watch
```

3. Open the app at `http://127.0.0.1:8080`.

The app listens on `0.0.0.0:8080` by default and exposes a health check at `GET /healthz`.

## Environment variables

The server reads configuration from `.env` via `dotenvy`.

| Variable | Required | Default in `.env.example` | Purpose |
| --- | --- | --- | --- |
| `DATABASE_URL` | Yes | `sqlite://data/todomvc.db` | SQLite connection string used by SQLx. |
| `LEPTOS_SITE_ADDR` | Yes | `0.0.0.0:8080` | Bind address for the Axum/Leptos server. |
| `LEPTOS_SITE_ROOT` | Yes | `target/site` | Directory containing the generated site assets. |
| `RUST_LOG` | No | `info` | Tracing filter for server logs. |

Notes:

- For local development, `LEPTOS_SITE_ROOT=target/site` matches the `cargo-leptos` build output.
- For the container image, the runtime value is `LEPTOS_SITE_ROOT=site` because the built assets are copied into `/app/site`.
- `DATABASE_URL` should stay environment-driven; the app creates the SQLite file if it does not already exist.

## Docker

Build the image:

```bash
docker build -t miketang84-todomvc-107 .
```

Run it directly with a persistent SQLite directory:

```bash
mkdir -p data
docker run --rm \
  -p 8080:8080 \
  -e DATABASE_URL=sqlite://data/todomvc.db \
  -e LEPTOS_SITE_ADDR=0.0.0.0:8080 \
  -e LEPTOS_SITE_ROOT=site \
  -e RUST_LOG=info \
  -v "$(pwd)/data:/app/data" \
  miketang84-todomvc-107
```

Or use Compose:

```bash
docker compose up --build
```

The compose setup:

- builds from the local `Dockerfile`
- publishes `8080:8080`
- mounts a named volume at `/app/data` for SQLite persistence

## TodoMVC coverage

Implemented today:

- SSR page render with client hydration
- add todo on Enter
- list rendering with active/completed/all hash filters
- toggle a single todo
- inline edit on double-click
- Enter and blur save edits
- Escape cancels edits
- empty trimmed edit deletes the todo
- delete a single todo
- clear completed
- toggle all
- remaining item counter and selected footer filter state

Still tracked separately:

- final TodoMVC spec conformance pass in issue `#22`

