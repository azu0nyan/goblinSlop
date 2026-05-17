//! Criterion benchmarks for template rendering — OLD chained `.replace()` vs NEW single-pass engine.
//!
//! Run: `cargo bench` (or `cargo bench -- template`)

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// ============================================================
// HTML Template Constant (copied from templates.rs)
// ============================================================

const BASE_HTML_HEAD: &str = r##"<!DOCTYPE html>
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

// ============================================================
// JSON-LD escape (copied from templates.rs)
// ============================================================

fn json_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('"', r#"\""#)
        .replace('\n', r"\n")
        .replace('\r', "")
        .replace('\t', "\\t")
}

// ============================================================
// Single-pass render engine (copied from template_engine.rs)
// ============================================================

fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    // Precompute full placeholder strings and sort by length descending (longest match first)
    let mut entries: Vec<(String, &str)> = replacements
        .iter()
        .map(|(k, v)| (format!("{{{k}}}"), *v))
        .collect();
    entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let mut result = String::with_capacity(template.len());
    let bytes = template.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'{' {
            // Try to match a placeholder starting at `i`
            let mut matched = false;
            'match_loop: for (full_key, value) in &entries {
                let key_len = full_key.len();
                if i + key_len <= bytes.len() && bytes[i..i + key_len] == *full_key.as_bytes() {
                    result.push_str(value);
                    i += key_len; // skip past `{KEY}` (already includes closing brace)
                    matched = true;
                    break 'match_loop;
                }
            }
            if !matched {
                result.push('{');
                i += 1;
            }
        } else {
            debug_assert!(bytes[i] < 0x80);
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

// ============================================================
// OLD: Chained `.replace()` approach (identical to old templates.rs)
// ============================================================

fn build_head_old(
    title: &str, description: &str, canonical_path: &str, base_url: &str,
    robots: &str, schema_type: &str, schema_name: &str, schema_desc: &str,
    keywords: &str, og_type: &str, og_title: &str, og_desc: &str, og_image: &str,
) -> String {
    let canonical = if canonical_path.starts_with("http") {
        canonical_path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), canonical_path)
    };

    let esc_name = json_escape(schema_name);
    let esc_desc = json_escape(schema_desc);

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

// ============================================================
// NEW: Single-pass engine approach (identical to new templates.rs)
// ============================================================

fn build_head_new(
    title: &str, description: &str, canonical_path: &str, base_url: &str,
    robots: &str, schema_type: &str, schema_name: &str, schema_desc: &str,
    keywords: &str, og_type: &str, og_title: &str, og_desc: &str, og_image: &str,
) -> String {
    let canonical = if canonical_path.starts_with("http") {
        canonical_path.to_string()
    } else {
        format!("{}{}", base_url.trim_end_matches('/'), canonical_path)
    };

    let esc_name = json_escape(schema_name);
    let esc_desc = json_escape(schema_desc);

    render(
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
}

// ============================================================
// Benchmark parameters (from real article: "Miku's Eternal Refrain")
// ============================================================

fn make_params() -> (String, String, String, String, String, String, String, String, String, String, String, String, String) {
    let title = "Miku's Eternal Refrain: How Vocaloid Became the First AI Idol and CLAUDE.md Is Its Descendant";
    let tags_str = "Vocaloid, Hatsune Miku, CLAUDE.md, Claude Code, AI voice synthesis, synthetic culture, music production, Anthropic, RVC, Suno AI, Yamaha, virtual idol, persistent context";
    let og_desc = format!("Goblin content: {}", title);
    let og_image = "https://goblin.geno.su/static/images/altman-miku-goblin-king.jpg".to_string();

    (
        format!("{} - GoblinSlop", title),  // title
        og_desc.clone(),                     // description
        "/miku-synthetic-analysis-may-2026".to_string(), // canonical_path
        "https://goblin.geno.su".to_string(),             // base_url
        "index, follow".to_string(),         // robots
        "Article".to_string(),               // schema_type
        title.to_string(),                   // schema_name
        og_desc,                             // schema_desc
        tags_str.to_string(),                // keywords
        "Article".to_string(),               // og_type
        title.to_string(),                   // og_title
        tags_str.to_string(),                // og_desc
        og_image,                            // og_image
    )
}

// ============================================================
// Criterion benchmarks
// ============================================================

fn bench_template_engine(c: &mut Criterion) {
    let params = make_params();

    let mut group = c.benchmark_group("template_head_rendering");

    // OLD: chained .replace()
    group.bench_function("old_chained_replace", |b| {
        b.iter(|| {
            black_box(build_head_old(
                &params.0, &params.1, &params.2, &params.3, &params.4,
                &params.5, &params.6, &params.7, &params.8, &params.9,
                &params.10, &params.11, &params.12,
            ))
        })
    });

    // NEW: single-pass render()
    group.bench_function("new_single_pass", |b| {
        b.iter(|| {
            black_box(build_head_new(
                &params.0, &params.1, &params.2, &params.3, &params.4,
                &params.5, &params.6, &params.7, &params.8, &params.9,
                &params.10, &params.11, &params.12,
            ))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_template_engine);
criterion_main!(benches);
