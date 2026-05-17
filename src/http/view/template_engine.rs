/// Single-pass HTML template replacement engine.
///
/// Scans the template string once, replacing placeholders like `{TITLE}` with values.
/// Longest-match-first so overlapping keys (e.g. `{TITLE}` vs `{TITLE2}`) never partially match.
pub fn render(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut entries: Vec<(String, &str)> = replacements
        .iter()
        .map(|(k, v)| (format!("{{{k}}}"), *v))
        .collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.0.len()));

    let mut result = String::with_capacity(template.len());
    let len = template.len();
    let bytes = template.as_bytes();
    let mut i = 0;

    while i < len {
        if bytes[i] == b'{' {
            let mut matched = false;
            for (full_key, value) in &entries {
                let key_len = full_key.len();
                if i + key_len <= len && bytes[i..i + key_len] == *full_key.as_bytes() {
                    result.push_str(value);
                    i += key_len;
                    matched = true;
                    break;
                }
            }
            if !matched {
                result.push('{');
                i += 1;
            }
        } else if bytes[i] < 0x80 {
            result.push(bytes[i] as char);
            i += 1;
        } else {
            // Multi-byte UTF-8 — decode the full char.
            let ch = template[i..].chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_replacement() {
        assert_eq!(
            render(
                "Hello {NAME}, welcome to {PLACE}!",
                &[("NAME", "Alice"), ("PLACE", "Orcland")]
            ),
            "Hello Alice, welcome to Orcland!"
        );
    }

    #[test]
    fn no_replacements() {
        assert_eq!(
            render("No placeholders here.", &[]),
            "No placeholders here."
        );
    }

    #[test]
    fn overlapping_keys_longest_wins() {
        assert_eq!(
            render(
                "{TITLE} and {TITLE2}",
                &[("TITLE", "Main"), ("TITLE2", "Alt")]
            ),
            "Main and Alt"
        );
    }

    #[test]
    fn placeholder_used_multiple_times() {
        assert_eq!(
            render(
                "{URL} says {URL} is cool",
                &[("URL", "https://goblin.geno.su")]
            ),
            "https://goblin.geno.su says https://goblin.geno.su is cool"
        );
    }

    #[test]
    fn unknown_placeholder_unchanged() {
        assert_eq!(
            render("{A} and {UNKNOWN}", &[("A", "1")]),
            "1 and {UNKNOWN}"
        );
    }

    #[test]
    fn special_chars_in_template() {
        let tpl = r##"<meta name="theme-color" content="#0f140f">"##;
        assert_eq!(render(tpl, &[]), tpl);
    }

    #[test]
    fn utf8_with_placeholder() {
        assert_eq!(
            render("<!-- Pack — {TITLE} -->", &[("TITLE", "GoblinSlop")]),
            "<!-- Pack — GoblinSlop -->"
        );
    }

    #[test]
    fn no_false_positive_on_curly_braces() {
        assert_eq!(
            render("{{DOUBLE}} and {ONE}", &[("ONE", "1")]),
            "{{DOUBLE}} and 1"
        );
    }

    #[test]
    fn partial_brace_no_match() {
        assert_eq!(
            render("{TITLE} text {OPEN", &[("TITLE", "GoblinSlop")]),
            "GoblinSlop text {OPEN"
        );
    }
}
