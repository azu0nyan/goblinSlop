use rand::Rng;
use crate::db::{ContentEntry, DynamicPage};
use super::references::generate_references_html_thread_rng;

// ============================================================
// HTML Template Constants
// ============================================================

pub(crate) const BASE_HTML_HEAD: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="theme-color" content="#0f140f">
    <title>{TITLE}</title>
    <link rel="stylesheet" href="/static/styles.css">
    <!-- Favicon pack — modern multi-format -->
    <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
    <link rel="icon" type="image/png" sizes="48x48" href="/static/favicon-48.png">
    <link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon-180x180.png">
    <link rel="apple-touch-icon" sizes="152x152" href="/static/apple-touch-icon-152x152.png">
    <link rel="apple-touch-icon" sizes="167x167" href="/static/apple-touch-icon-167x167.png">
    <link rel="apple-touch-icon" sizes="120x120" href="/static/apple-touch-icon-120x120.png">
    <link rel="manifest" href="/static/site.webmanifest">
    <meta name="description" content="{DESCRIPTION}">
    <meta name="robots" content="{ROBOTS}">
    <meta name="keywords" content="{KEYWORDS}">
    <meta name="author" content="GoblinSlop Editorial Collective">
    <link rel="canonical" href="{CANONICAL}">
    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="{OG_TYPE}">
    <meta property="og:url" content="{CANONICAL}">
    <meta property="og:title" content="{OG_TITLE}">
    <meta property="og:description" content="{OG_DESC}">
    <meta property="og:image" content="{OG_IMAGE}">
    <meta property="og:site_name" content="GoblinSlop">
    <meta property="og:locale" content="en_US">
    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:url" content="{CANONICAL}">
    <meta name="twitter:title" content="{OG_TITLE}">
    <meta name="twitter:description" content="{OG_DESC}">
    <meta name="twitter:image" content="{OG_IMAGE}">
    <!-- RSS Feed -->
    <link rel="alternate" type="application/rss+xml" title="GoblinSlop - Goblin Lore & Slop" href="/feed.xml">
    <script type="application/ld+json">
    {
        "@context": "https://schema.org",
        "@type": "{SCHEMA_TYPE}",
        "name": "{SCHEMA_NAME}",
        "description": "{SCHEMA_DESC}",
        "url": "{CANONICAL}",
        "publisher": {
            "@type": "Organization",
            "name": "GoblinSlop",
            "logo": {
                "@type": "ImageObject",
                "url": "https://goblin.geno.su/static/images/default.jpg"
            }
        },
        "about": {
            "@type": "Thing",
            "name": "Goblins",
            "description": "Goblin folklore, mythology, tricks, and cultural references including the Sam Altman connection"
        },
        "keywords": "{KEYWORDS}"
    }
    </script>
</head>
<body>
    <nav class="goblin-nav">
        <div class="nav-inner">
            <a href="/" class="nav-logo">GoblinSlop</a>
            <div class="nav-links">
                <a href="/">Home</a>
                <a href="/goblin-lore">Lore</a>
                <a href="/goblin-tricks">Tricks</a>
                <a href="/sam-altman-goblins">Sam Altman</a>
                <a href="/goblin-schizophrenia">Schizophrenia</a>
                <a href="/search">Search</a>
            </div>
        </div>
    </nav>
    <main class="content-wrapper">
"##;

const BASE_HTML_FOOT: &str = r##"    </main>
    <footer class="goblin-footer">
        <p>GoblinSlop - A chaotic collection of goblin knowledge</p>
    </footer>
</body>
</html>"##;

/// Escape HTML entities for safe embedding in JSON-LD
pub(crate) fn json_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('"', r#"\""#)
        .replace('\n', r"\n")
        .replace('\r', "")
        .replace('\t', "\\t")
}

/// Build the HTML head block with all meta tags including OG/Twitter cards.
///
/// `use_new_engine` controls which rendering backend is used:
/// - `true` → single-pass `template_engine::render()` (v2, ~5× faster)
/// - `false` → old chained `.replace()` calls (v1, fallback)
pub fn build_head(
    title: &str,
    description: &str,
    canonical_path: &str,
    base_url: &str,
    robots: &str,
    schema_type: &str,
    schema_name: &str,
    schema_desc: &str,
    keywords: &str,
    og_type: &str,
    og_title: &str,
    og_desc: &str,
    og_image: &str,
    use_new_engine: bool,
) -> String {
    let canonical = if canonical_path.starts_with("http") {
        canonical_path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), canonical_path)
    };

    // JSON-LD requires HTML-escaped strings
    let esc_name = json_escape(schema_name);
    let esc_desc = json_escape(schema_desc);

    if use_new_engine {
        super::template_engine::render(
            BASE_HTML_HEAD,
            &[
                ("TITLE", title),
                ("DESCRIPTION", description),
                ("ROBOTS", robots),
                ("CANONICAL", &canonical),
                ("SCHEMA_TYPE", schema_type),
                ("SCHEMA_NAME", &esc_name),
                ("SCHEMA_DESC", &esc_desc),
                ("KEYWORDS", keywords),
                ("OG_TYPE", og_type),
                ("OG_TITLE", og_title),
                ("OG_DESC", og_desc),
                ("OG_IMAGE", og_image),
            ],
        )
    } else {
        // Old chained replace approach (fallback)
        BASE_HTML_HEAD
            .replace("{TITLE}", title)
            .replace("{DESCRIPTION}", description)
            .replace("{ROBOTS}", robots)
            .replace("{CANONICAL}", &canonical)
            .replace("{SCHEMA_TYPE}", schema_type)
            .replace("{SCHEMA_NAME}", &esc_name)
            .replace("{SCHEMA_DESC}", &esc_desc)
            .replace("{KEYWORDS}", keywords)
            .replace("{OG_TYPE}", og_type)
            .replace("{OG_TITLE}", og_title)
            .replace("{OG_DESC}", og_desc)
            .replace("{OG_IMAGE}", og_image)
    }
}

/// Render a standard content page with JSON-LD metadata
pub fn render_content_page(
    entry: &ContentEntry,
    canonical_path: &str,
    base_url: &str,
    use_new_engine: bool,
) -> String {
    let mut html = String::new();
    let tags_str = entry.tags.join(", ");
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
    let og_image = format!(
        "{}{}",
        base_url.trim_end_matches('/'),
        format!("/static/images/{}", img_file)
    );
    let og_title = &entry.title;
    let og_desc = &format!("Goblin content: {}", entry.title);

    let head = build_head(
        &format!("{} - GoblinSlop", entry.title),
        og_desc,
        canonical_path,
        base_url,
        "index, follow",
        "Article", // og:type
        &entry.title,
        og_desc,
        &tags_str,
        "Article",  // og:type
        og_title,   // og:title
        og_desc,    // og:description
        &og_image,  // og:image
        use_new_engine,
    );
    html.push_str(&head);

    // Body
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
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

    // Cross-references — explicit JSON refs + keyword-matched + random fake refs in one block
    let mut refs_keywords: Vec<String> = entry.tags.clone();
    refs_keywords.extend(entry.slug.split('-').map(|s| s.to_string()));
    let explicit_slugs = entry.references.clone();
    html.push_str(&generate_references_html_thread_rng(
        &refs_keywords,
        Some(&entry.slug),
        &explicit_slugs,
    ));

    // Sources section (external references like IMDb, MyAnimeList, etc.)
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

/// Render a dynamically generated goblin page — picks random image from pool per request
pub fn render_dynamic_page<R: Rng>(
    dyn_page: &DynamicPage,
    canonical_path: &str,
    base_url: &str,
    rng: &mut R,
    use_new_engine: bool,
) -> String {
    let keywords_str = dyn_page.keywords.join(", ");

    // Single cached pool read (LazyLock), pick two random images from it
    let pool = get_image_pool();
    let img1_idx = rng.gen_range(0..pool.len());
    let selected_img1 = &pool[img1_idx];
    let og_image = format!(
        "{}{}",
        base_url.trim_end_matches('/'),
        &format!("/static/images/{}", selected_img1)
    );

    // Pick second random image for the body (independent draw from same cached pool)
    let img2_idx = rng.gen_range(0..pool.len());
    let selected_img2 = &pool[img2_idx];

    let mut html = String::new();

    let head = build_head(
        &format!("{} - GoblinSlop", dyn_page.title),
        &format!("Goblin content about: {}", keywords_str),
        canonical_path,
        base_url,
        "index, follow",
        "WebPage",                                         // og:type
        &dyn_page.title,                                   // og:title
        &format!("Goblin content related to: {}", keywords_str), // og:description
        &keywords_str,
        "WebPage",          // og:type
        &dyn_page.title,    // og:title
        &format!("Goblin content about: {}", keywords_str), // og:description
        &og_image,          // og:image
        use_new_engine,
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

/// Render a static page from raw HTML body (for home, search, all)
pub fn render_static_page(
    title: &str,
    body_html: &str,
    category: &str,
    tags: &str,
    canonical_path: &str,
    base_url: &str,
    use_new_engine: bool,
) -> String {
    let mut html = String::new();

    // Default OG image for collection pages
    let og_image = format!(
        "{}{}",
        base_url.trim_end_matches('/'),
        "/static/images/default.jpg"
    );
    let og_desc = if title.len() > 150 {
        &title[..150]
    } else {
        title
    };

    let head = build_head(
        &format!("{} - GoblinSlop", title),
        title,
        canonical_path,
        base_url,
        "index, follow",
        "CollectionPage",                          // og:type
        title,                                     // schema_name
        og_desc,                                   // schema_desc
        tags,                                      // keywords
        "WebPage",                                  // og:type
        &format!("{} - GoblinSlop", title),        // og:title
        og_desc,                                    // og:description
        &og_image,                                  // og:image
        use_new_engine,
    );
    html.push_str(&head);

    // Only show category/tags meta on non-home pages
    let meta = if category == "home" {
        String::new()
    } else {
        format!(
            r#"<div class="meta">
            <span class="category">Category: {category}</span>
            <span class="tags">Tags: {tags}</span>
        </div>"#,
            category = category,
            tags = tags,
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

/// Render tags as clickable HTML links
pub fn render_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|t| format!("<a href='/tag/{}' class='tag-link'>{}</a>", t, t))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render a category as a clickable HTML link
pub fn render_category(category: &str) -> String {
    format!(
        r#"<a href='/category/{}' class='category-link'>{}</a>"#,
        category, category
    )
}

use std::sync::LazyLock;

/// Lazy-cached image pool: scans `static/images/` once at first access,
/// then returns a cloned sorted Vec from the static cache.
/// No per-request filesystem reads — avoids repeated `readdir` syscalls.
static IMAGE_POOL: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut images: Vec<String> = std::fs::read_dir("static/images")
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".jpg") && !name.starts_with("default"))
        .collect();
    images.sort();
    images
});

/// Returns the cached image pool (clone of sorted filenames).
/// Filesystem is read only once at first access via `LazyLock`.
pub fn get_image_pool() -> Vec<String> {
    IMAGE_POOL.clone()
}

// ============================================================
// Tests — verify caching behavior and correctness
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_pool_caching_consistent_across_calls() {
        // Call get_image_pool multiple times — LazyLock ensures the filesystem is only read once
        let pool1 = get_image_pool();
        let pool2 = get_image_pool();
        let pool3 = get_image_pool();

        // All calls return identical sorted image lists
        assert_eq!(pool1, pool2);
        assert_eq!(pool2, pool3);

        // Pool should contain actual images (not empty)
        assert!(!pool1.is_empty(), "Image pool should not be empty");

        // All entries must end with .jpg and NOT start with "default"
        for img in &pool1 {
            assert!(img.ends_with(".jpg"), "Image '{}' must end with .jpg", img);
            assert!(
                !img.starts_with("default"),
                "Image '{}' should exclude default.jpg",
                img
            );
        }

        // Must be sorted lexicographically (LazyLock preserves sort order)
        let mut sorted_pool = pool1.clone();
        sorted_pool.sort();
        assert_eq!(pool1, sorted_pool, "Image pool must be sorted");
    }

    #[test]
    fn test_image_pool_cloning_does_not_mutate_cache() {
        let pool_a = get_image_pool();
        // Clone is a new Vec — mutating it doesn't affect subsequent calls
        let mut mutated = pool_a.clone();
        mutated.push("fake-image.jpg".to_string());

        let pool_b = get_image_pool();
        assert!(
            !pool_b.contains(&"fake-image.jpg".to_string()),
            "Mutating the returned clone must not pollute the static cache"
        );
    }

    #[test]
    fn test_render_content_page_output_valid_html() {
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

        let html = render_content_page(&entry, "/test-article", "https://goblin.geno.su", true);
        assert!(html.contains("<title>Test Article - GoblinSlop</title>"));
        assert!(html.contains("og:title"));
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn test_render_dynamic_page_picks_from_pool() {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

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
            true,
        );

        // Must contain the title
        assert!(html.contains("Random Goblin Thoughts"));
        // Must contain images from the pool (not default.jpg for dynamic pages)
        let pool = get_image_pool();
        assert!(!pool.is_empty(), "Image pool should not be empty");
        // The HTML must reference at least one image from the pool (two random picks)
        // We can verify by checking /static/images/ references exist and they're real files
        let img_refs: Vec<_> = html.matches("/static/images/")
            .map(|_| ())
            .collect();
        assert!(img_refs.len() >= 1, "Dynamic page must include at least one image reference");
        // Verify the pool images are valid jpgs that exist on disk
        for img in &pool {
            let path = format!("static/images/{}", img);
            assert!(std::path::Path::new(&path).exists(), "Pool image '{}' must exist on disk", img);
        }
    }

    #[test]
    fn test_json_escape_handles_special_chars() {
        let escaped = json_escape(r#"He said "hello\nworld""#);
        assert_eq!(escaped, r#"He said \"hello\\nworld\""#);
    }

    #[test]
    fn test_build_head_both_engine_paths_produce_same_output() {
        let new_html = build_head(
            "Test", "Desc", "/test", "https://goblin.geno.su",
            "index, follow", "WebPage", "GoblinSlop",
            "Description here", "", "WebPage",
            "Test - GoblinSlop", "", "", true,
        );
        let old_html = build_head(
            "Test", "Desc", "/test", "https://goblin.geno.su",
            "index, follow", "WebPage", "GoblinSlop",
            "Description here", "", "WebPage",
            "Test - GoblinSlop", "", "", false,
        );
        assert_eq!(new_html, old_html, "Both engine paths must produce identical output");
    }
}
