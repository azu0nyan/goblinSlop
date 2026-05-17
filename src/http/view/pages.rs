use rand::Rng;

use super::components::{render_category, render_tags};
use super::layout::{BASE_HTML_FOOT, build_head};
use crate::domain::references::generate_references_html_thread_rng;
use crate::domain::{ContentEntry, DynamicPage};

/// Render a standard content page with JSON-LD metadata.
pub fn render_content_page(entry: &ContentEntry, canonical_path: &str, base_url: &str) -> String {
    let mut html = String::new();
    let tags_str = entry.tags.join(", ");
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
    let og_image = format!(
        "{}/static/images/{}",
        base_url.trim_end_matches('/'),
        img_file
    );
    let og_desc = format!("Goblin content: {}", entry.title);

    let head = build_head(
        &format!("{} - GoblinSlop", entry.title),
        &og_desc,
        canonical_path,
        base_url,
        "index, follow",
        "Article",
        &entry.title,
        &og_desc,
        &tags_str,
        "Article",
        &entry.title,
        &og_desc,
        &og_image,
    );
    html.push_str(&head);

    let image_html = format!(
        r#"<div class="article-image">
            <img src="/static/images/{}" alt="{}" class="article-img">
        </div>"#,
        img_file, entry.title
    );

    let cat_link = render_category(&entry.category);
    let tag_links = render_tags(&entry.tags);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        <div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>
    </header>
    <div class="page-body">
        {image}
        {body}
    </div>
</article>"#,
        title = entry.title,
        category = cat_link,
        tags = tag_links,
        image = image_html,
        body = entry.body_html,
    ));

    let mut refs_keywords: Vec<String> = entry.tags.clone();
    refs_keywords.extend(entry.slug.split('-').map(|s| s.to_string()));
    html.push_str(&generate_references_html_thread_rng(
        &refs_keywords,
        Some(&entry.slug),
        &entry.references,
    ));

    if !entry.sources.is_empty() {
        html.push_str("<section class='sources-section'><h2>Sources</h2><ul class='sources-list'>");
        for src in &entry.sources {
            if src.url.is_empty() {
                html.push_str(&format!("<li>{}</li>", src.name));
            } else {
                html.push_str(&format!(
                    "<li><a href='{}' target='_blank' rel='noopener noreferrer'>{}</a></li>",
                    src.url, src.name
                ));
            }
        }
        html.push_str("</ul></section>");
    }

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a dynamically generated goblin page — picks random images from the pool per request.
pub fn render_dynamic_page<R: Rng>(
    dyn_page: &DynamicPage,
    canonical_path: &str,
    base_url: &str,
    rng: &mut R,
    image_pool: &[String],
) -> String {
    let keywords_str = dyn_page.keywords.join(", ");

    let selected_img1 = &image_pool[rng.gen_range(0..image_pool.len())];
    let selected_img2 = &image_pool[rng.gen_range(0..image_pool.len())];
    let og_image = format!(
        "{}/static/images/{}",
        base_url.trim_end_matches('/'),
        selected_img1
    );

    let mut html = String::new();

    let head = build_head(
        &format!("{} - GoblinSlop", dyn_page.title),
        &format!("Goblin content about: {}", keywords_str),
        canonical_path,
        base_url,
        "index, follow",
        "WebPage",
        &dyn_page.title,
        &format!("Goblin content related to: {}", keywords_str),
        &keywords_str,
        "WebPage",
        &dyn_page.title,
        &format!("Goblin content about: {}", keywords_str),
        &og_image,
    );
    html.push_str(&head);

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
    </header>
    <div class="page-body">
        <div class="article-image">
            <img src="/static/images/{}" alt="{title}" class="article-img">
        </div>
        {content}
    </div>
</article>"#,
        selected_img2,
        title = dyn_page.title,
        content = dyn_page.content,
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

/// Render a static page from raw HTML body (home, search, list pages).
pub fn render_static_page(
    title: &str,
    body_html: &str,
    category: &str,
    tags: &str,
    canonical_path: &str,
    base_url: &str,
) -> String {
    let og_image = format!(
        "{}/static/images/default.jpg",
        base_url.trim_end_matches('/')
    );
    let og_desc = if title.len() > 150 {
        &title[..150]
    } else {
        title
    };

    let mut html = build_head(
        &format!("{} - GoblinSlop", title),
        title,
        canonical_path,
        base_url,
        "index, follow",
        "CollectionPage",
        title,
        og_desc,
        tags,
        "WebPage",
        &format!("{} - GoblinSlop", title),
        og_desc,
        &og_image,
    );

    let meta = if category == "home" {
        String::new()
    } else {
        format!(
            r#"<div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>"#,
        )
    };

    html.push_str(&format!(
        r#"<article class="content-page">
    <header class="page-header">
        <h1>{title}</h1>
        {meta}
    </header>
    <div class="page-body">
        {body}
    </div>
</article>"#,
        title = title,
        meta = meta,
        body = body_html
    ));

    html.push_str(BASE_HTML_FOOT);
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::image_pool;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn render_content_page_output_valid_html() {
        let entry = ContentEntry {
            id: 1,
            title: "Test Article".to_string(),
            slug: "test-article".to_string(),
            body_markdown: "# Hello\n\nWorld content.".to_string(),
            body_html: "<h1>Hello</h1><p>World content.</p>".to_string(),
            category: "schizophrenia".to_string(),
            tags: vec!["goblin".to_string(), "test".to_string()],
            references: vec![],
            sources: vec![],
            is_dynamic: false,
            date_added: "2026-05-17T00:00:00Z".to_string(),
            image: Some("goblin-schizophrenia.jpg".to_string()),
        };

        let html = render_content_page(&entry, "/test-article", "https://goblin.geno.su");
        assert!(html.contains("<title>Test Article - GoblinSlop</title>"));
        assert!(html.contains("og:title"));
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn render_dynamic_page_picks_from_pool() {
        let images = image_pool::scan("static/images");
        assert!(!images.is_empty(), "need at least one image in pool");

        let dyn_page = DynamicPage {
            path: "/random-goblin-path".to_string(),
            title: "Random Goblin Thoughts".to_string(),
            content: "<p>Goblin thinking...</p>".to_string(),
            keywords: vec!["goblin".to_string(), "thoughts".to_string()],
        };

        let mut rng = StdRng::seed_from_u64(42);
        let html = render_dynamic_page(
            &dyn_page,
            "/random-goblin-path",
            "https://goblin.geno.su",
            &mut rng,
            &images,
        );

        assert!(html.contains("Random Goblin Thoughts"));
        assert!(html.contains("/static/images/"));
    }
}
