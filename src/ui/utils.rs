/// Sanitize text by removing emojis and other wide Unicode characters
/// that can cause terminal rendering issues.
pub fn sanitize_text(text: &str) -> String {
    text.chars()
        .filter(|c| {
            // Keep ASCII characters
            if c.is_ascii() {
                return true;
            }

            // Keep basic Latin supplement and extended Latin
            let code = *c as u32;

            // Allow common safe Unicode ranges:
            // - Basic Latin supplement (00A0-00FF) - accented chars
            // - Latin Extended-A (0100-017F)
            // - Latin Extended-B (0180-024F)
            // - General punctuation subset
            if (0x00A0..=0x024F).contains(&code) {
                return true;
            }

            // Filter out:
            // - Emojis (1F300-1F9FF and other ranges)
            // - Wide Asian characters
            // - Combining characters
            // - Other potentially problematic characters
            false
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_emojis() {
        assert_eq!(sanitize_text("Hello 🗑️ World"), "Hello  World");
        assert_eq!(sanitize_text("Test 🎉🎊 Done"), "Test  Done");
    }

    #[test]
    fn test_sanitize_keeps_ascii() {
        assert_eq!(sanitize_text("Hello World!"), "Hello World!");
        assert_eq!(sanitize_text("file.rs +10 -5"), "file.rs +10 -5");
    }

    #[test]
    fn test_sanitize_keeps_accented() {
        assert_eq!(sanitize_text("café"), "café");
        assert_eq!(sanitize_text("naïve"), "naïve");
    }
}
