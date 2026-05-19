//! Wire format for `data/content/*.json` and the shared `SourceRef` type.
//!
//! This module is included from `build.rs` via `#[path]` to drive compile-time
//! validation. It must therefore stay self-contained: no `crate::` paths, only
//! types from `std` and `serde` (which is both a runtime and build dependency).

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceRef {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonContentEntry {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    #[serde(default)]
    pub is_dynamic: bool,
    #[serde(default = "default_date_added")]
    pub date_added: String,
    #[serde(default)]
    pub image: Option<String>,
}

pub fn default_category() -> String {
    "general".to_string()
}

pub fn default_tags() -> Vec<String> {
    vec!["goblin".to_string()]
}

pub fn default_date_added() -> String {
    "1970-01-01T00:00:00Z".to_string()
}
