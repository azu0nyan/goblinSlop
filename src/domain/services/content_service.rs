use anyhow::Result;

use crate::domain::generator::{generate_dynamic_page_content, parse_path_into_keywords};
use crate::domain::ports::ContentRepository;
use crate::domain::{ContentEntry, DynamicPage};

/// Outcome of resolving an unknown URL path: either a stored entry or a
/// freshly generated dynamic page.
pub enum PageContent {
    Static(ContentEntry),
    Dynamic(DynamicPage),
}

/// Application service for content. Holds a repository implementation and
/// orchestrates the "find or generate" rule that defines GoblinSlop's
/// no-404 behavior. Pass-through accessors exist for handlers that just
/// want a query result.
#[derive(Clone)]
pub struct ContentService<R: ContentRepository> {
    repo: R,
}

impl<R: ContentRepository> ContentService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Look up `slug` in the repository; if missing, derive keywords and
    /// generate a deterministic dynamic page.
    pub fn resolve_path(&self, slug: &str) -> Result<PageContent> {
        if let Some(entry) = self.repo.get_by_slug(slug)? {
            return Ok(PageContent::Static(entry));
        }
        let keywords = parse_path_into_keywords(slug);
        let final_keywords = if keywords.is_empty() {
            vec!["goblin".into(), "mystery".into(), slug.to_string()]
        } else {
            keywords
        };
        Ok(PageContent::Dynamic(generate_dynamic_page_content(
            slug,
            &final_keywords,
        )))
    }

    pub fn find_by_slug(&self, slug: &str) -> Result<Option<ContentEntry>> {
        self.repo.get_by_slug(slug)
    }

    pub fn paginated(&self, page: u64, per_page: u64) -> Result<(Vec<ContentEntry>, u64)> {
        let entries = self.repo.get_paginated(page, per_page)?;
        let total = self.repo.count_all()?;
        Ok((entries, total))
    }

    pub fn all(&self) -> Result<Vec<ContentEntry>> {
        self.repo.get_all()
    }

    pub fn search(&self, query: &str) -> Result<Vec<ContentEntry>> {
        self.repo.search(query)
    }

    pub fn by_tag(&self, tag: &str) -> Result<Vec<ContentEntry>> {
        self.repo.get_by_tag(tag)
    }

    pub fn by_category(&self, category: &str) -> Result<Vec<ContentEntry>> {
        self.repo.get_by_category(category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct FakeRepo {
        entries: Mutex<Vec<ContentEntry>>,
    }

    impl FakeRepo {
        fn with(entries: Vec<ContentEntry>) -> Self {
            Self {
                entries: Mutex::new(entries),
            }
        }
    }

    impl ContentRepository for FakeRepo {
        fn get_by_slug(&self, slug: &str) -> Result<Option<ContentEntry>> {
            Ok(self
                .entries
                .lock()
                .unwrap()
                .iter()
                .find(|e| e.slug == slug)
                .cloned())
        }
        fn get_paginated(&self, _page: u64, _per_page: u64) -> Result<Vec<ContentEntry>> {
            Ok(self.entries.lock().unwrap().clone())
        }
        fn count_all(&self) -> Result<u64> {
            Ok(self.entries.lock().unwrap().len() as u64)
        }
        fn get_all(&self) -> Result<Vec<ContentEntry>> {
            Ok(self.entries.lock().unwrap().clone())
        }
        fn search(&self, _query: &str) -> Result<Vec<ContentEntry>> {
            Ok(self.entries.lock().unwrap().clone())
        }
        fn get_by_tag(&self, _tag: &str) -> Result<Vec<ContentEntry>> {
            Ok(self.entries.lock().unwrap().clone())
        }
        fn get_by_category(&self, _category: &str) -> Result<Vec<ContentEntry>> {
            Ok(self.entries.lock().unwrap().clone())
        }
    }

    fn sample() -> ContentEntry {
        ContentEntry {
            id: 1,
            slug: "goblin-lore".into(),
            title: "Goblin Lore".into(),
            body_markdown: "body".into(),
            body_html: "<p>body</p>".into(),
            category: "lore".into(),
            tags: vec!["goblin".into()],
            references: vec![],
            sources: vec![],
            is_dynamic: false,
            date_added: "2026-01-01T00:00:00Z".into(),
            image: None,
        }
    }

    #[test]
    fn resolve_path_returns_static_when_present() {
        let svc = ContentService::new(FakeRepo::with(vec![sample()]));
        match svc.resolve_path("goblin-lore").unwrap() {
            PageContent::Static(e) => assert_eq!(e.slug, "goblin-lore"),
            PageContent::Dynamic(_) => panic!("expected static"),
        }
    }

    #[test]
    fn resolve_path_generates_dynamic_when_missing() {
        let svc = ContentService::new(FakeRepo::with(vec![]));
        match svc.resolve_path("unknown-goblin-path").unwrap() {
            PageContent::Dynamic(p) => {
                assert_eq!(p.path, "unknown-goblin-path");
                assert!(!p.title.is_empty());
                assert!(!p.keywords.is_empty());
            }
            PageContent::Static(_) => panic!("expected dynamic"),
        }
    }

    #[test]
    fn resolve_path_dynamic_is_deterministic() {
        let svc = ContentService::new(FakeRepo::with(vec![]));
        let a = match svc.resolve_path("x-y-z").unwrap() {
            PageContent::Dynamic(p) => p,
            _ => panic!(),
        };
        let b = match svc.resolve_path("x-y-z").unwrap() {
            PageContent::Dynamic(p) => p,
            _ => panic!(),
        };
        assert_eq!(a.title, b.title);
        assert_eq!(a.content, b.content);
    }
}
