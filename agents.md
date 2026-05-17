# 🧌 GoblinSlop — Agent Guide

> A Rust + Axum web service that serves goblin lore from in-memory SQLite and
> generates a unique goblin page for any unknown URL. **No 404s** — every path
> resolves to something goblin.

- **Language**: Rust 2024
- **Framework**: Axum 0.7
- **Storage**: SQLite via rusqlite 0.32 (in-memory, rebuilt from JSON on every boot)
- **Templating**: hand-rolled single-pass placeholder engine (no Tera / Askama)

The structure follows the [bulletproof-rust-web] guide: three layers, ports +
services for repository access, dependencies pointing inward.

[bulletproof-rust-web]: https://gruberb.github.io/bulletproof-rust-web/

---

## Architecture

```
src/
├── main.rs                          thin entry: tracing, validated config, wire state, serve
├── lib.rs                           re-exports
├── config.rs                        env-var Config + validate()
├── error.rs                         AppError + AppResult + IntoResponse
│
├── domain/                          PURE. no axum, no rusqlite, no fs.
│   ├── content.rs                   ContentEntry, DynamicPage, SourceRef
│   ├── generator.rs                 deterministic dynamic page generator
│   ├── references.rs                cross-reference link generator
│   ├── templates_data.rs            title / intro / body / verdict constants
│   ├── ports/
│   │   └── content_repository.rs    ContentRepository trait (Send + Sync, sync methods)
│   └── services/
│       └── content_service.rs       ContentService<R> + resolve_path (find-or-generate)
│
├── infra/                           I/O: rusqlite, fs, markdown
│   ├── db.rs                        schema + queries (kept as raw functions)
│   ├── content_loader.rs            JSON → markdown → ContentEntry → db
│   ├── image_pool.rs                scan `static/images/` once at startup
│   └── repositories/
│       └── sqlite_content_repo.rs   SqliteContentRepo impl ContentRepository (wraps Arc<Mutex<Connection>>)
│
└── api/                             axum-facing
    ├── mod.rs                       create_router
    ├── state.rs                     AppState (holds ContentService<SqliteContentRepo>)
    ├── response.rs                  ApiResponse<T>
    ├── view/                        HTML rendering
    │   ├── template_engine.rs       single-pass {KEY} substitution
    │   ├── layout.rs                BASE_HTML_HEAD/FOOT + build_head
    │   ├── components.rs            tags, category, card grid, preview
    │   └── pages.rs                 render_content_page / _dynamic / _static
    └── handlers/                    one file per route, ~20 lines each, all return AppResult
```

### Layer rules

| Layer    | May depend on                                             | Must NOT import           |
|----------|-----------------------------------------------------------|---------------------------|
| `domain` | `std`, `serde`, `rand`, `anyhow`, `thiserror`             | axum, rusqlite, tokio, fs |
| `infra`  | `domain`, rusqlite, fs, markdown, tracing                 | axum                      |
| `api`    | `domain`, `infra`, axum, tower                            | —                         |

Dependencies point inward. Handlers depend on `ContentService<R>`, not on
`SqliteContentRepo` or `infra::db`. The concrete repo is wired in `main.rs`
(the composition root) and held inside the service.

---

## Request flow

1. Client requests `/<path>`.
2. Axum matches a named route (`/`, `/search`, `/tag/:t`, `/category/:c`,
   `/raw/:slug`, `/sitemap.xml`, `/api/...`) or falls through to
   `dynamic_fallback`.
3. `dynamic_fallback`:
   - Empty slug → 308 to `/`.
   - Slug with `_` → 308 to the hyphen form (canonicalization).
   - Otherwise → `state.content.resolve_path(slug)` returns either
     `PageContent::Static(entry)` (render content page) or
     `PageContent::Dynamic(page)` (render generated page).
4. **No dynamic-page caching.** Generation is deterministic from the path; the
   same URL always produces identical HTML.

---

## Conventions

### Handlers (`src/api/handlers/`)

- 5–40 lines each. Extract → call **service** → format response.
- Always return `AppResult<T>`. Never `unwrap()` / `expect()` in request paths.
- Handlers never see rusqlite or `infra::db` directly. The path is
  `handler → ContentService → ContentRepository → infra::db`.

### Errors (`src/error.rs`)

- `AppError::NotFound(String)` → 404.
- `AppError::Internal(#[from] anyhow::Error)` → 500 + log.
- `IntoResponse` is implemented on `AppError`, so handlers `?` freely.
- Per-rusqlite errors are wrapped at the repository boundary
  (`SqliteContentRepo` adds `anyhow::Context`), so the domain never depends on
  `rusqlite::Error`.

### Logging

- `tracing` only. No `println!` / `eprintln!` in library code.
- Structured fields: `info!(slug = %entry.slug, "loaded")`.
- Default filter is `info`. Override with `RUST_LOG=debug`.

### Database

- One in-memory SQLite connection behind `Arc<Mutex<Connection>>`, owned by
  `SqliteContentRepo`. The mutex is held only for the duration of a single
  query — never across `await` points.
- Schema lives in `infra::db::init_db`. No migrations: the DB is wiped and
  rebuilt from `data/content/*.json` on every boot.
- Use parameterized queries. Never interpolate into SQL.

### Domain port + service

- `ContentRepository` (in `domain::ports`) defines every read the service uses.
  Methods are synchronous because the only impl wraps sync rusqlite.
- `ContentService<R: ContentRepository>` (in `domain::services`) is generic
  over the port. Tests can swap in a fake repo with no I/O.
- The service is the only place that knows the "find static, else generate
  dynamic" rule. Handlers and the `/api/dynamic/*` route both go through it.

### Templating

- One engine: `api::view::template_engine::render`. Single-pass byte scan,
  longest-key-first, UTF-8 aware. Benchmarks (`cargo bench`) show ~5× speedup
  vs chained-`.replace()`.
- Page renderers compose: `build_head(...)` → page body → `BASE_HTML_FOOT`.
- The article card (4 call-sites previously) lives in
  `view::components::render_card_grid`.

### Configuration

- `Config::from_env()` reads env vars with defaults.
- `Config::validate()` runs at startup before any I/O — surfaces missing
  content dirs, malformed `base_url`, empty host, etc. immediately.

### Tests

- Unit tests live next to the code they cover.
- `infra::db` tests use `:memory:` databases.
- `infra::content_loader` tests read real `data/content/*.json` — they verify
  the schema contract.
- `domain::generator` tests assert determinism.
- `domain::services::content_service` tests use an in-process `FakeRepo` —
  proving the port abstraction pays off.
- `config::tests` cover validate-rejects-* cases.

---

## Configuration vars

All optional; sensible defaults baked in.

| Env var               | Default                    | Notes                          |
|-----------------------|----------------------------|--------------------------------|
| `GOBLIN_HOST`         | `0.0.0.0`                  | non-empty enforced             |
| `GOBLIN_PORT`         | `3000`                     |                                |
| `GOBLIN_CONTENT_DIR`  | `data/content`             | dir-exists enforced            |
| `GOBLIN_STATIC_DIR`   | `static`                   | dir-exists enforced            |
| `GOBLIN_BASE_URL`     | `https://goblin.geno.su`   | http(s):// enforced            |
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
`/api/all`, `/tag/goblin`, an unknown path, and `/some_underscored_path`.

---

## Alignment with bulletproof-rust-web

| Guide rule                                         | Implementation                              |
|----------------------------------------------------|---------------------------------------------|
| Three layers, deps point inward                    | `domain/` ← `infra/`, `api/`                |
| Layer names `api`/`domain`/`infra`                 | matches guide                               |
| Domain has zero framework imports                  | `domain/*` only uses std/serde/rand/anyhow  |
| AppError + AppResult + IntoResponse                | `src/error.rs`                              |
| `Internal(#[from] anyhow::Error)` catch-all        | `error.rs`                                  |
| `thiserror` structured, `anyhow` catch-all         | `error.rs` + repo layer                     |
| Repository trait (port) in `domain/ports/`         | `ContentRepository`                         |
| Service generic over port in `domain/services/`    | `ContentService<R>`                         |
| Repository impl in `infra/repositories/`           | `SqliteContentRepo`                         |
| Service stored in AppState as concrete type        | `ContentService<SqliteContentRepo>`         |
| AppState single Clone struct, shared via `Arc`     | `api/state.rs`                              |
| AppState in its own file                           | `api/state.rs`                              |
| Composition root in `main.rs`                      | `main.rs`                                   |
| Eager config validation at startup                 | `Config::validate()`                        |
| `lib.rs` + `main.rs` split                         | both present                                |
| Single-crate layout (start here)                   | not a workspace, correct per guide          |

### Deliberate deviations (with rationale)

- **`Arc<Mutex<Connection>>` in repo.** Guide prefers no `Mutex<T>` in
  AppState. Our `Mutex` is *inside* the repo (an infra detail), not exposed
  to handlers, and is required because rusqlite is sync. Migrating to
  `deadpool-sqlite` would remove the lock entirely — a planned-but-not-done
  follow-up.
- **No `api/dtos/`.** The site has no client-supplied bodies (all GET), and
  `ContentEntry` is structurally identical to the JSON response shape. The
  guide explicitly permits sharing types when "structurally identical with
  identical invariants."
- **No `secrecy` / `SecretString`.** No secrets in this config.
- **No middleware stack (CORS / timeout / rate limit).** Not warranted for
  a read-only public content site; add when there's a concrete reason.
