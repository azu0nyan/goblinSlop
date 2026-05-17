use crate::domain::ContentEntry;

/// Render a list of tag links.
pub fn render_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|t| format!("<a href='/tag/{}' class='tag-link'>{}</a>", t, t))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render a category as a clickable link.
pub fn render_category(category: &str) -> String {
    format!(
        r#"<a href='/category/{}' class='category-link'>{}</a>"#,
        category, category
    )
}

/// Strip leading Markdown markers and return the first ~240 chars of body text.
pub fn make_preview(markdown: &str) -> String {
    let body = markdown
        .lines()
        .find(|l| {
            let trimmed = l.trim();
            !trimmed.starts_with('#') && !trimmed.starts_with("---") && !trimmed.is_empty()
        })
        .unwrap_or("");
    let body = body
        .trim()
        .replace("**", "")
        .replace("__", "")
        .replace("~~", "")
        .replace("*", "")
        .replace("`", "");
    if body.len() > 240 {
        format!("{}…", &body[..237])
    } else {
        body
    }
}

fn date_prefix(date: &str) -> &str {
    if date.len() >= 10 { &date[..10] } else { date }
}

/// Card markup used by home, search, tag, and category grids.
/// When `with_preview` is false (search results), the preview paragraph is omitted.
pub fn render_card(entry: &ContentEntry, with_preview: bool) -> String {
    let img_file = entry.image.as_deref().unwrap_or("default.jpg");
    let date_str = date_prefix(&entry.date_added);
    let tag_links = render_tags(&entry.tags);
    let cat_link = render_category(&entry.category);
    let preview_html = if with_preview {
        format!(
            "<p class='card-preview'>{}</p>",
            make_preview(&entry.body_markdown)
        )
    } else {
        String::new()
    };

    format!(
        r#"<div class='article-card'>
            <div class='card-top'>
                <div class='card-image'>
                    <img src="/static/images/{img}" alt="{title}" class="card-img">
                </div>
                <div class='card-top-right'>
                    <a href='/{slug}' class='card-title-link'><h3 class='card-title'>{title}</h3></a>
                    <div class='card-header'>
                        <span class='card-date'>{date}</span>
                        {cat}
                    </div>
                </div>
            </div>
            {preview}
            <div class='card-footer'>
                <span class='card-tags'>{tags}</span>
            </div>
        </div>"#,
        img = img_file,
        title = entry.title,
        slug = entry.slug,
        date = date_str,
        cat = cat_link,
        preview = preview_html,
        tags = tag_links,
    )
}

/// Render a complete card grid for a list of entries.
pub fn render_card_grid(entries: &[ContentEntry], with_preview: bool) -> String {
    let mut out = String::from("<div class='article-grid'>");
    for entry in entries {
        out.push_str(&render_card(entry, with_preview));
    }
    out.push_str("</div>");
    out
}
