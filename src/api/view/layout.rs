use super::template_engine::render;

pub(crate) const BASE_HTML_HEAD: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="theme-color" content="#0f140f">
    <title>{TITLE}</title>
    <link rel="stylesheet" href="/static/styles.css">
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
    <meta property="og:type" content="{OG_TYPE}">
    <meta property="og:url" content="{CANONICAL}">
    <meta property="og:title" content="{OG_TITLE}">
    <meta property="og:description" content="{OG_DESC}">
    <meta property="og:image" content="{OG_IMAGE}">
    <meta property="og:site_name" content="GoblinSlop">
    <meta property="og:locale" content="en_US">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:url" content="{CANONICAL}">
    <meta name="twitter:title" content="{OG_TITLE}">
    <meta name="twitter:description" content="{OG_DESC}">
    <meta name="twitter:image" content="{OG_IMAGE}">
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

pub(crate) const BASE_HTML_FOOT: &str = r##"    </main>
    <footer class="goblin-footer">
        <p>GoblinSlop - A chaotic collection of goblin knowledge</p>
    </footer>
</body>
</html>"##;

/// Escape HTML entities for safe embedding in JSON-LD.
pub(crate) fn json_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('"', r#"\""#)
        .replace('\n', r"\n")
        .replace('\r', "")
        .replace('\t', "\\t")
}

#[allow(clippy::too_many_arguments)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_escape_handles_special_chars() {
        let escaped = json_escape(r#"He said \"hello\\nworld\""#);
        assert_eq!(escaped, r#"He said \\\"hello\\\\nworld\\\""#);
    }
}
