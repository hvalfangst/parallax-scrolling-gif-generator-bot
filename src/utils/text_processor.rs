use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ASCII_REPLACEMENTS: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
        // Smart quotes to regular quotes
        map.insert('«', "\""); map.insert('»', "\"");
        map.insert('`', "'"); map.insert('´', "'");
        // Dashes and ellipsis
        map.insert('—', "-"); map.insert('–', "-"); map.insert('−', "-");
        map.insert('‒', "-"); map.insert('―', "-"); map.insert('…', "...");
        // Accented and special letters
        map.insert('á', "a"); map.insert('à', "a"); map.insert('â', "a");
        map.insert('ä', "a"); map.insert('ã', "a"); map.insert('å', "a");
        map.insert('ā', "a"); map.insert('æ', "ae");
        map.insert('ç', "c"); map.insert('č', "c");
        map.insert('é', "e"); map.insert('è', "e"); map.insert('ê', "e");
        map.insert('ë', "e"); map.insert('ē', "e");
        map.insert('í', "i"); map.insert('ì', "i"); map.insert('î', "i");
        map.insert('ï', "i"); map.insert('ī', "i");
        map.insert('ñ', "n");
        map.insert('ó', "o"); map.insert('ò', "o"); map.insert('ô', "o");
        map.insert('ö', "o"); map.insert('õ', "o"); map.insert('ø', "o");
        map.insert('œ', "oe"); map.insert('ō', "o");
        map.insert('ú', "u"); map.insert('ù', "u"); map.insert('û', "u");
        map.insert('ü', "u"); map.insert('ū', "u");
        map.insert('ý', "y"); map.insert('ÿ', "y");
        map.insert('ß', "ss");
        // Currency and math symbols
        map.insert('€', "EUR"); map.insert('£', "GBP"); map.insert('¥', "JPY");
        map.insert('₹', "INR"); map.insert('¢', "c"); map.insert('₩', "KRW");
        map.insert('©', "(c)"); map.insert('®', "(r)"); map.insert('™', "(tm)");
        map.insert('°', " deg"); map.insert('±', "+/-"); map.insert('×', "x");
        map.insert('÷', "/");
        // Bullets and miscellaneous
        map.insert('•', "*"); map.insert('●', "*"); map.insert('‣', "*");
        map.insert('·', "*");
        // Arrows
        map.insert('→', "->"); map.insert('←', "<-"); map.insert('↑', "^");
        map.insert('↓', "v");
        map
    };
}

/// Handles text processing and ASCII normalization
pub struct TextProcessor;

impl TextProcessor {
    /// Clean text to ensure ASCII compatibility
    pub fn enforce_ascii(text: &str) -> String {
        let mut result = String::with_capacity(text.len());

        for c in text.chars() {
            if let Some(replacement) = ASCII_REPLACEMENTS.get(&c) {
                result.push_str(replacement);
            } else if c.is_ascii() {
                result.push(c);
            }
        }

        result
    }

    /// Remove the last sentence if the text does not end with allowed punctuation
    pub fn remove_incomplete_last_sentence(text: &str) -> String {
        let trimmed = text.trim_end();
        if trimmed.is_empty() {
            return String::new();
        }

        // Check if the last character is a valid sentence-ending punctuation
        if trimmed.ends_with('.') || trimmed.ends_with('!') || trimmed.ends_with('?') {
            return trimmed.to_string();
        }

        // Find the last period and return everything before it
        if let Some(last_period) = trimmed.rfind('.') {
            return trimmed[..last_period + 1].to_string();
        }

        // If no valid ending punctuation, return empty string
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_ascii() {
        let test_cases = vec![
            ("Café", "Cafe"),
            ("20°C", "20 degC"),
            ("→", "->"),
            ("±", "+/-"),
            ("÷", "/"),
            ("°", " deg"),
            ("©", "(c)"),
            ("™", "(tm)"),
            ("→", "->"),
            ("←", "<-"),
            ("↑", "^"),
            ("↓", "v"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(TextProcessor::enforce_ascii(input), expected);
        }
    }

    #[test]
    fn test_remove_incomplete_last_sentence() {
        assert_eq!(
            TextProcessor::remove_incomplete_last_sentence("First sentence. Second sentence"),
            "First sentence."
        );
        assert_eq!(
            TextProcessor::remove_incomplete_last_sentence("Complete sentence."),
            "Complete sentence."
        );
        assert_eq!(
            TextProcessor::remove_incomplete_last_sentence("Incomplete"),
            ""
        );
    }
}