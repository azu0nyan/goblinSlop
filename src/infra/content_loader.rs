use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use pulldown_cmark::{Parser, html};
use rusqlite::Connection;
use serde::Deserialize;
use tracing::{info, warn};

use crate::domain::{ContentEntry, SourceRef};
use crate::infra::db::insert_content;

/// Unified content entry loaded from individual JSON files in `data/content/`.
#[derive(Debug, Deserialize)]
struct JsonContentEntry {
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

fn default_category() -> String {
    "general".to_string()
}

fn default_tags() -> Vec<String> {
    vec!["goblin".to_string()]
}

fn default_date_added() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

/// Load every `.json` file from `content_dir` into the supplied connection.
pub fn load_all_content_into_conn(conn: &Connection, content_dir: &str) -> Result<()> {
    let content_path = Path::new(content_dir);
    if !content_path.exists() {
        return Err(anyhow!("Content directory not found: {}", content_dir));
    }

    let mut entries: Vec<fs::DirEntry> = fs::read_dir(content_path)
        .with_context(|| format!("reading content dir {content_dir}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    info!(count = entries.len(), dir = %content_path.display(), "loading content");

    for entry in &entries {
        let path = entry.path();
        let json_content = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                warn!(path = %path.display(), error = %e, "read failed, skipped");
                continue;
            }
        };

        match serde_json::from_str::<JsonContentEntry>(&json_content) {
            Ok(json_entry) => {
                let body_html = markdown_to_html(&json_entry.body_markdown);
                let content_entry = ContentEntry {
                    id: 0,
                    slug: json_entry.slug,
                    title: json_entry.title,
                    body_markdown: json_entry.body_markdown,
                    body_html,
                    category: json_entry.category,
                    tags: json_entry.tags,
                    references: json_entry.references,
                    sources: json_entry.sources,
                    is_dynamic: json_entry.is_dynamic,
                    date_added: json_entry.date_added,
                    image: json_entry.image,
                };

                match insert_content(conn, &content_entry) {
                    Ok(_) => info!(slug = %content_entry.slug, "loaded"),
                    Err(e) => warn!(id = %json_entry.id, error = %e, "insert failed"),
                }
            }
            Err(e) => warn!(path = %path.display(), error = %e, "invalid JSON, skipped"),
        }
    }

    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::db::{get_content_by_slug, init_db};

    #[test]
    fn deserialize_single_content_unit() {
        let test_file = std::path::PathBuf::from("data/content/goblin-lore.json");
        assert!(
            test_file.exists(),
            "data/content/goblin-lore.json must exist"
        );
        let json_str = fs::read_to_string(&test_file).unwrap();
        let entry: JsonContentEntry = serde_json::from_str(&json_str).unwrap();
        assert_eq!(entry.slug, "goblin-lore");
        assert!(entry.body_markdown.starts_with("# Goblin Lore"));
        assert_eq!(entry.category, "lore");
        assert!(!entry.references.is_empty());
        assert!(entry.tags.iter().any(|t| t == "goblin" || t == "lore"));
    }

    #[test]
    fn load_and_read_entry_with_all_fields() {
        let conn = init_db(":memory:").unwrap();
        load_all_content_into_conn(&conn, "data/content").unwrap();

        let entry = get_content_by_slug(&conn, "goblin-slayer-anime")
            .unwrap()
            .expect("entry must exist");

        assert!(!entry.title.is_empty());
        assert!(entry.tags.contains(&"goblin".to_string()));
        assert!(!entry.references.is_empty());
        assert!(!entry.sources.is_empty());
        for s in &entry.sources {
            assert!(s.url.starts_with("http"));
        }
        assert_eq!(entry.date_added.len(), 20);
        assert!(entry.date_added.ends_with('Z'));
    }
}
