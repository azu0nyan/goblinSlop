/// Single-pass HTML template replacement engine.
///
/// Scans the template string exactly once, replacing placeholders like `{TITLE}` with values.
/// Uses longest-match-first strategy so overlapping keys (e.g. `{TITLE}` vs `{TITLE2}`) never
/// partially match — the longer key always wins.

/// Render a template by replacing all `{KEY}` placeholders in a single pass.
///
/// `replacements` is an ordered list of `(key, value)` pairs. Keys are expected to be bare
/// identifiers (e.g. `"TITLE"`), not including braces. The engine auto-builds full placeholder
/// tokens like `"{TITLE}"`. When multiple keys could match at the same position, the longest key
/// wins — callers should provide replacements sorted by key length descending for correct behavior.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    // Precompute full placeholder strings and sort by length descending (longest match first)
    let mut entries: Vec<(String, &str)> = replacements
        .iter()
        .map(|(k, v)| (format!("{{{k}}}"), *v))
        .collect();
    entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let mut result = String::with_capacity(template.len());
    let len = template.len();
    let bytes = template.as_bytes();
    let mut i = 0;

    while i < len {
        if bytes[i] == b'{' {
            // Try to match a placeholder starting at byte position `i`.
            // Keys are always ASCII identifiers, so byte-level comparison is safe.
            let mut matched = false;
            'match_loop: for (full_key, value) in &entries {
                let key_len = full_key.len();
                if i + key_len <= len && bytes[i..i + key_len] == *full_key.as_bytes() {
                    result.push_str(value);
                    i += key_len; // skip past `{KEY}`
                    matched = true;
                    break 'match_loop;
                }
            }
            if !matched {
                // Not a placeholder — output `{` and advance by 1 byte to
                // re-enter the loop for the next character naturally.
                result.push('{');
                i += 1;
            }
        } else {
            // ASCII fast path: single byte < 0x80 is always a valid ASCII char
            if bytes[i] < 0x80 {
                result.push(bytes[i] as char);
                i += 1;
            } else {
                // Multi-byte UTF-8 character — decode it properly.
                let ch = template[i..].chars().next().unwrap();
                result.push(ch);
                i += ch.len_utf8();
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        let tpl = "Hello {NAME}, welcome to {PLACE}!";
        let result = render(tpl, &[("NAME", "Alice"), ("PLACE", "Orcland")]);
        assert_eq!(result, "Hello Alice, welcome to Orcland!");
    }

    #[test]
    fn test_no_replacements() {
        let tpl = "No placeholders here.";
        let result = render(tpl, &[]);
        assert_eq!(result, "No placeholders here.");
    }

    #[test]
    fn test_overlapping_keys_longest_wins() {
        // `{TITLE2}` should not be partially matched by `{TITLE}`
        let tpl = "{TITLE} and {TITLE2}";
        let result = render(tpl, &[("TITLE", "Main"), ("TITLE2", "Alt")]);
        assert_eq!(result, "Main and Alt");
    }

    #[test]
    fn test_placeholder_used_multiple_times() {
        // `{CANONICAL}` appears 4 times in BASE_HTML_HEAD
        let tpl = "{URL} says {URL} is cool";
        let result = render(tpl, &[("URL", "https://goblin.geno.su")]);
        assert_eq!(result, "https://goblin.geno.su says https://goblin.geno.su is cool");
    }

    #[test]
    fn test_unknown_placeholder_unchanged() {
        // Only specified placeholders get replaced; other braces pass through
        let tpl = "{A} and {UNKNOWN}";
        let result = render(tpl, &[("A", "1")]);
        assert_eq!(result, "1 and {UNKNOWN}");
    }

    #[test]
    fn test_empty_replacement_value() {
        let tpl = "Hello {NAME}!";
        let result = render(tpl, &[("NAME", "")]);
        assert_eq!(result, "Hello !");
    }

    #[test]
    fn test_special_chars_in_template() {
        // Template with `content="#0f140f"` — uses r## delimiters as project convention
        let tpl = r##"<meta name="theme-color" content="#0f140f">"##;
        let result = render(tpl, &[]);
        assert_eq!(result, r##"<meta name="theme-color" content="#0f140f">"##);
    }

    #[test]
    fn test_many_placeholders() {
        // Simulate BASE_HTML_HEAD style: same key appears multiple times
        let tpl = "<title>{T}</title>\n<link rel=\"canonical\" href=\"{U}\">\n<meta property=\"og:url\" content=\"{U}\">";
        let result = render(
            tpl,
            &[("T", "GoblinSlop"), ("U", "https://goblin.geno.su/test")],
        );
        assert!(result.contains("<title>GoblinSlop</title>"));
        assert!(result.contains(r#"<link rel="canonical" href="https://goblin.geno.su/test">"#));
        assert!(result.contains(r#"<meta property="og:url" content="https://goblin.geno.su/test">"#));
    }

    #[test]
    fn test_no_false_positive_on_curly_braces() {
        // `{{` should not match anything — just pass through literally
        let tpl = "{{DOUBLE}} and {ONE}";
        let result = render(tpl, &[("ONE", "1")]);
        assert_eq!(result, "{{DOUBLE}} and 1");
    }

    #[test]
    fn test_partial_brace_no_match() {
        // `{OPEN` without closing brace passes through unchanged
        let tpl = "{TITLE} text {OPEN";
        let result = render(tpl, &[("TITLE", "GoblinSlop")]);
        assert_eq!(result, "GoblinSlop text {OPEN");
    }

    #[test]
    fn test_utf8_emdash_in_comment() {
        // Em-dash (—) is a 3-byte UTF-8 sequence in HTML comments
        let tpl = "<!-- Favicon pack — modern multi-format -->";
        let result = render(tpl, &[]);
        assert_eq!(result, "<!-- Favicon pack — modern multi-format -->");
    }

    #[test]
    fn test_utf8_with_placeholder() {
        // Mixed: UTF-8 non-ASCII + placeholder replacement
        let tpl = "<!-- Favicon pack — {TITLE} -->";
        let result = render(tpl, &[("TITLE", "GoblinSlop")]);
        assert_eq!(result, "<!-- Favicon pack — GoblinSlop -->");
    }

    #[test]
    fn test_mixed_utf8_and_replacements() {
        // Multiple UTF-8 chars + placeholders in realistic HTML context
        let tpl = "Föo—Bär <title>{T}</title> café";
        let result = render(tpl, &[("T", "test")]);
        assert_eq!(result, "Föo—Bär <title>test</title> café");
    }
}
