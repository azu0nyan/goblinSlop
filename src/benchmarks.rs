//! Benchmarks comparing OLD chained `.replace()` vs NEW single-pass template rendering
//! for GoblinSlop's `build_head()` HTML head generator.
//!
//! Run: `cargo test --release benchmarks`

use std::hint::black_box;
use std::time::{Duration, Instant};

use crate::routes::template_engine::render;
use crate::routes::templates::{json_escape, BASE_HTML_HEAD};

// ============================================================
// Constants
// ============================================================

const ITERATIONS: u64 = 10_000;
const BASE_URL: &str = "https://goblin.geno.su";

/// Simulates the OLD chained `.replace()` approach that was used before
/// the single-pass template engine. Chains ~15 `.replace()` calls on
/// `BASE_HTML_HEAD` sequentially, building intermediate String allocations.
fn build_head_old(
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

    // JSON-LD requires HTML-escaped strings (must match json_escape logic)
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

/// Uses the NEW single-pass template engine (identical to `build_head` in templates.rs).
/// Scans the template exactly once, replacing all `{KEY}` placeholders in a single pass.
fn build_head_new(
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

// ============================================================
// Real article data (loaded from JSON at compile-time via build script or runtime)
// ============================================================

/// Returns realistic test parameters from the article with the most tags.
fn get_real_article_params() -> (&'static str, &'static str) {
    (
        // Title: longest title in content library (13 tags — rich keywords list)
        "Miku's Eternal Refrain: How Vocaloid Became the First AI Idol and CLAUDE.md Is Its Descendant",
        // Tags joined with ", " → 13 keywords for realistic KEYWORDS/OG_DESC size
        "Vocaloid, Hatsune Miku, CLAUDE.md, Claude Code, AI voice synthesis, synthetic culture, music production, Anthropic, RVC, Suno AI, Yamaha, virtual idol, persistent context",
    )
}

fn make_head_params(
    title: &str,
    tags_str: &str,
    _category: &str,
    base_url: &str,
) -> (String, String, String, String, String, String, String, String, String, String, String, String, String) {
    let og_desc = format!("Goblin content: {}", title);
    let og_image = format!(
        "{}{}{}",
        base_url.trim_end_matches('/'),
        "/static/images/",
        "altman-miku-goblin-king.jpg"
    );

    (
        format!("{} - GoblinSlop", title), // title
        og_desc.clone(),                   // description
        "/miku-synthetic-analysis-may-2026".to_string(), // canonical_path
        base_url.to_string(),              // base_url
        "index, follow".to_string(),       // robots
        "Article".to_string(),             // schema_type
        title.to_string(),                 // schema_name
        og_desc,                           // schema_desc
        tags_str.to_string(),              // keywords
        "Article".to_string(),             // og_type
        title.to_string(),                 // og_title
        tags_str.to_string(),              // og_desc (using tags for realistic length)
        og_image,                          // og_image
    )
}

// ============================================================
// Benchmark tests
// ============================================================

#[cfg(test)]
mod benchmarks {
    use super::*;

    /// Benchmarks the OLD chained `.replace()` approach.
    /// Loops ITERATIONS times (10,000), measures total time, prints results.
    #[test]
    fn bench_old_chained_replaces() {
        let (title, tags_str) = get_real_article_params();
        let params = make_head_params(title, tags_str, "schizophrenia", BASE_URL);

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let result = black_box(build_head_old(
                &params.0, // title
                &params.1, // description
                &params.2, // canonical_path
                &params.3, // base_url
                &params.4, // robots
                &params.5, // schema_type
                &params.6, // schema_name
                &params.7, // schema_desc
                &params.8, // keywords
                &params.9, // og_type
                &params.10,// og_title
                &params.11,// og_desc
                &params.12,// og_image
            ));
            assert!(!result.is_empty());
        }
        let elapsed = start.elapsed();

        print_results("OLD (chained .replace())", elapsed);
    }

    /// Benchmarks the NEW single-pass template engine approach.
    /// Loops ITERATIONS times (10,000), measures total time, prints results.
    #[test]
    fn bench_new_single_pass() {
        let (title, tags_str) = get_real_article_params();
        let params = make_head_params(title, tags_str, "schizophrenia", BASE_URL);

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let result = black_box(build_head_new(
                &params.0, // title
                &params.1, // description
                &params.2, // canonical_path
                &params.3, // base_url
                &params.4, // robots
                &params.5, // schema_type
                &params.6, // schema_name
                &params.7, // schema_desc
                &params.8, // keywords
                &params.9, // og_type
                &params.10,// og_title
                &params.11,// og_desc
                &params.12,// og_image
            ));
            assert!(!result.is_empty());
        }
        let elapsed = start.elapsed();

        print_results("NEW (single-pass render())", elapsed);
    }

    /// Verifies that both approaches produce identical output.
    #[test]
    fn verify_output_equality() {
        let (title, tags_str) = get_real_article_params();
        let params = make_head_params(title, tags_str, "schizophrenia", BASE_URL);

        let old_result = build_head_old(
            &params.0, &params.1, &params.2, &params.3, &params.4,
            &params.5, &params.6, &params.7, &params.8, &params.9,
            &params.10, &params.11, &params.12,
        );
        let new_result = build_head_new(
            &params.0, &params.1, &params.2, &params.3, &params.4,
            &params.5, &params.6, &params.7, &params.8, &params.9,
            &params.10, &params.11, &params.12,
        );

        assert_eq!(
            old_result, new_result,
            "OLD and NEW produce different output!\n\
             OLD length: {}\n\
             NEW length: {}",
            old_result.len(),
            new_result.len()
        );

        println!(
            "✅ Output equality verified (both {} bytes)",
            old_result.len()
        );
    }
}

// ============================================================
// Helper: print benchmark results
// ============================================================

fn print_results(label: &str, elapsed: Duration) {
    let total_ns = elapsed.as_secs_f64() * 1_000_000_000.0;
    let ns_per_op = total_ns / ITERATIONS as f64;
    let ms_total = elapsed.as_secs_f64() * 1_000.0;

    println!(
        "\n─────────────────────────────────────────────\n\
         Benchmarks: {}\n\
         Iterations: {} ({:.1} ms total)\n\
         Time per iteration: {:.2} ns/op\n\
         ╰───────────────────────────────────────────",
        label, ITERATIONS, ms_total, ns_per_op
    );
}
