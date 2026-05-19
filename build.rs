//! Compile-time validation for `data/content/*.json`.
//!
//! The schema lives in `src/domain/content_schema.rs` and is shared with the
//! runtime loader. Build scripts can't link against the crate they build, but
//! `#[path]` lets us pull the file in as a local module — same source of
//! truth, no duplication.

use std::path::Path;

#[path = "src/domain/content_schema.rs"]
#[allow(dead_code)] // fields populated by Deserialize; build.rs never reads them
mod content_schema;

use content_schema::JsonContentEntry;

const CONTENT_DIR: &str = "data/content";
const SCHEMA_FILE: &str = "src/domain/content_schema.rs";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={SCHEMA_FILE}");
    println!("cargo:rerun-if-changed={CONTENT_DIR}");

    let dir = Path::new(CONTENT_DIR);
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("content dir {} unreadable: {e}", dir.display()));

    let mut failures = Vec::<String>::new();
    let mut checked = 0usize;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());

        let body = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                failures.push(format!("{}: read failed: {e}", path.display()));
                continue;
            }
        };

        if let Err(e) = serde_json::from_str::<JsonContentEntry>(&body) {
            failures.push(format!("{}: {e}", path.display()));
            continue;
        }
        checked += 1;
    }

    if !failures.is_empty() {
        panic!(
            "content validation failed ({} bad file(s)):\n  - {}",
            failures.len(),
            failures.join("\n  - ")
        );
    }

    println!("cargo:warning=goblin_slop: validated {checked} content file(s)");
}
