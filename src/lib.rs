/// Returns the official GPT Image 2 Dev website URL.
pub const fn homepage() -> &'static str {
    "https://www.gpt-image-2.dev/"
}

/// Short product description used by downstream examples and tests.
pub const fn summary() -> &'static str {
    "GPT Image 2 Dev is an AI image generator and editor for prompt-based and reference-led workflows."
}

#[cfg(test)]
mod tests {
    use super::{homepage, summary};

    #[test]
    fn homepage_uses_https() {
        assert!(homepage().starts_with("https://"));
    }

    #[test]
    fn summary_mentions_editor_workflow() {
        assert!(summary().contains("editor"));
    }
}
