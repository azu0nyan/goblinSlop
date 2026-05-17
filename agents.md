# 🧌 GoblinSlop — Agent Guide

> A Rust + Axum web service that serves goblin lore from in-memory SQLite and
> generates a unique goblin page for any unknown URL. **No 404s** — every path
> resolves to something goblin.

- **Language**: Rust 2024
- **Framework**: Axum 0.7
- **Storage**: SQLite via rusqlite 0.32 (in-memory, rebuilt from JSON on every boot)
- **Templating**: hand-rolled single-pass placeholder engine (no Tera / Askama)

---

## Architecture

The project follows a three-layer split adapted from the bulletproof-rust-web
guide. **Dependencies point inward**: HTTP and infra both depend on domain;
domain depends on nothing in the project.

```
src/
├── main.rs                  thin entry: tracing, config, init, serve
├── lib.rs                   re-exports for integration tests
├── config.rs                env-var Config
├── error.rs                 AppError + AppResult + IntoResponse
│
├── domain/                  PURE. no axum, no rusqlite, no fs.
│   ├── content.rs           ContentEntry, DynamicPage, SourceRef
│   ├── generator.rs         deterministic dynamic page generator
│   ├── references.rs        cross-reference link generator
│   └── templates_data.rs    title / intro / body / verdict constants
│
├── infra/                   I/O: rusqlite, fs, markdown
│   ├── db.rs                schema + queries + inserts (one file)
│   ├── content_loader.rs    JSON → markdown → ContentEntry → db
│   └── image_pool.rs        scan `static/images/` once at startup
│
└── http/                    axum-facing
    ├── mod.rs               AppState + create_router
    ├── response.rs          ApiResponse<T>
    ├── view/                HTML rendering
    │   ├── template_engine.rs   single-pass {KEY} substitution
    │   ├── layout.rs            BASE_HTML_HEAD/FOOT + build_head
    │   ├── components.rs        tags, category, card grid, preview
    │   └── pages.rs             render_content_page / _dynamic / _static
    └── handlers/            one file per route, ~30 lines each
```

### Layer rules

| Layer    | May depend on                                       | Must NOT import           |
|----------|-----------------------------------------------------|---------------------------|
| `domain` | `std`, `serde`, `rand`, `anyhow`, `thiserror`       | axum, rusqlite, tokio, fs |
| `infra`  | `domain`, rusqlite, fs, markdown, tracing           | axum                      |
| `http`   | `domain`, `infra`, axum, tower                      | —                         |

A `domain` change that pulls in axum or rusqlite is a code smell — extract the
HTTP/DB concern into the calling layer instead.

---

## Request flow

1. Client requests `/<path>`.
2. Axum matches a named route (`/`, `/search`, `/tag/:t`, `/category/:c`,
   `/raw/:slug`, `/sitemap.xml`, `/api/...`) or falls through to
   `dynamic_fallback`.
3. `dynamic_fallback`:
   - Empty slug → 308 to `/`.
   - Slug with `_` → 308 to the hyphen form (canonicalization).
   - Slug in DB → render `render_content_page`.
   - Otherwise → derive keywords from path, generate via seeded RNG, render
     `render_dynamic_page`.
4. **No dynamic-page caching.** Generation is deterministic from the path; the
   same URL always produces identical HTML, so no cache lookup or write is
   needed.

---

## Conventions

### Handlers (`src/http/handlers/`)

- 5–40 lines each. Extract → call service / db / generator → format response.
- Always return `AppResult<T>`. Never `unwrap()` / `expect()` in request paths.
- Lock the rusqlite `Mutex` once at the top, map poison to `AppError::Internal`.
- Drop the lock (`drop(conn);`) before doing CPU-heavy template work that
  doesn't need the database.

### Errors (`src/error.rs`)

- `AppError::NotFound(String)` → 404.
- `AppError::Database(rusqlite::Error)` → 500 + log.
- `AppError::Internal(anyhow::Error)` → 500 + log.
- `IntoResponse` is implemented on `AppError`, so handlers can `?` freely.

### Logging

- `tracing` only. No `println!` / `eprintln!` in library code.
- Use structured fields: `info!(slug = %entry.slug, "loaded")`.
- Default filter is `info`. Override with `RUST_LOG=debug`.

### Database

- Single in-memory SQLite connection behind `Arc<Mutex<Connection>>`. Adequate
  for a low-traffic content site. If traffic grows, swap to `r2d2` pool — the
  `infra::db` API does not need to change.
- Schema lives in `infra::db::init_db`. No migrations: the DB is wiped and
  rebuilt from `data/content/*.json` on every boot.
- Use parameterized queries. Never interpolate into SQL.

### Templating

- One engine: `http::view::template_engine::render`. Single-pass byte scan,
  longest-key-first, UTF-8 aware. Benchmarks (`cargo bench`) show ~5× speedup
  vs the chained-`.replace()` it replaced.
- Page renderers compose: `build_head(...)` → page body → `BASE_HTML_FOOT`.
- The duplicated article card (4 call-sites previously) lives in
  `view::components::render_card` / `render_card_grid`.

### Tests

- Unit tests live next to the code they cover (`#[cfg(test)] mod tests`).
- `infra::db` tests use `:memory:` databases.
- `infra::content_loader` tests read real `data/content/*.json` — they verify
  the schema contract.
- `domain::generator` tests assert determinism: same path → same output.

---

## Configuration

All env vars are optional; sensible defaults are baked in.

| Env var               | Default                    | Notes                          |
|-----------------------|----------------------------|--------------------------------|
| `GOBLIN_HOST`         | `0.0.0.0`                  |                                |
| `GOBLIN_PORT`         | `3000`                     |                                |
| `GOBLIN_CONTENT_DIR`  | `data/content`             | source JSON files              |
| `GOBLIN_STATIC_DIR`   | `static`                   | served at `/static/*`          |
| `GOBLIN_BASE_URL`     | `https://goblin.geno.su`   | canonical URLs, sitemap, OG    |
| `GOBLIN_DB_PATH`      | `goblin_slop.db`           | currently unused — DB is `:memory:` |
| `RUST_LOG`            | `info`                     | tracing-subscriber EnvFilter   |

---

## Content schema (`data/content/*.json`)

```json
{
  "id": "...",
  "title": "...",
  "slug": "kebab-case-unique",
  "body_markdown": "# Heading\n\nBody text...",
  "category": "lore",
  "tags": ["goblin", "lore"],
  "references": ["other-slug", ...],
  "sources": [{"name": "MAL", "url": "https://..."}],
  "is_dynamic": false,
  "date_added": "2026-01-01T00:00:00Z",
  "image": "optional-image.jpg"
}
```

`tags`, `references`, `sources` default to safe values when omitted.
Slugs must be hyphen-separated; `_` variants 308-redirect to the hyphen form.

---

## API

| Method | Path                       | Returns          | Status |
|--------|----------------------------|------------------|--------|
| GET    | `/`                        | HTML home + grid | 200    |
| GET    | `/search?q=`               | HTML search page | 200    |
| GET    | `/tag/:tag`                | HTML tag page    | 200    |
| GET    | `/category/:category`      | HTML cat page    | 200    |
| GET    | `/raw/:slug`               | text/markdown    | 200/404|
| GET    | `/sitemap.xml`             | XML              | 200    |
| GET    | `/api/all`                 | JSON list        | 200    |
| GET    | `/api/content/:slug`       | JSON entry/null  | 200    |
| GET    | `/api/dynamic/*path`       | JSON page        | 200    |
| GET    | `/api/search?q=`           | JSON list        | 200    |
| GET    | `/<anything>`              | HTML page        | 200    |
| GET    | `/<anything_with_underscores>` | 308 → hyphen | 308    |

---

## Verification

A change is done when these four pass:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

For changes touching HTTP, also smoke-test a running server against `/`,
`/api/all`, `/tag/goblin`, and an unknown path.

---

## Recent refactor (2026-05-17)

Repo was reshaped from a flat `src/{db, routes, json_content_loader}` layout
into the `domain` / `infra` / `http` split documented above. Key changes:

- **`lib.rs` added** so handlers and renderers are reachable from integration
  tests.
- **`AppError` + `IntoResponse`** replaces ad-hoc `(StatusCode, String)`
  tuples and silent `.unwrap_or_default()` swallowing.
- **`main` returns `anyhow::Result`** — startup failures now surface with
  context instead of `panic!`.
- **`tracing`** replaces every `println!` / `eprintln!` (incl. content loader).
- **`use_new_template_engine` flag removed.** The single-pass engine is
  validated; the chained-`.replace()` fallback was dead code.
- **Card markup deduplicated.** The article-card HTML was copy-pasted across
  home / search / tag / category; it now lives in `view::components`.
- **`ContentEntry` / `DynamicPage` / `SourceRef`** moved to `domain::content`,
  decoupling them from rusqlite.

What was deliberately **not** changed (kept simple per
[bulletproof-rust-web]'s "incremental complexity" principle):

- Still rusqlite + `Arc<Mutex<Connection>>`; no async DB, no pool.
- No repository traits or service generics — the call graph is small enough
  that direct function calls in `infra::db` are clearer than ports/adapters.
- No middleware stack (CORS, timeout, rate limit). Add when there's a real
  reason, not preemptively.
- No CLAUDE.md, no validator crate, no DTO/entity split — the schema is
  read-mostly and there are no user-supplied bodies to validate.

[bulletproof-rust-web]: https://gruberb.github.io/bulletproof-rust-web/
