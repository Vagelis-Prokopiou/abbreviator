use unicode_segmentation::{Graphemes, UnicodeSegmentation};

/// Abbreviates a word by keeping the first and last letters
/// and replacing the middle with the count of removed letters.
/// Words shorter than 3 letters are returned unchanged.
///
/// # Examples
/// ```
/// assert_eq!(abbreviate("word"), "w2d");
/// assert_eq!(abbreviate("internationalization"), "i18n");
/// ```
pub fn abbreviate(word: &str) -> String {
    let word_length = graphemes_count(word);
    if word_length < 3 {
        return word.to_string();
    }

    format!(
        "{}{}{}",
        get_first_letter(word).unwrap_or_default(),
        word_length - 2,
        get_last_letter(word).unwrap_or_default()
    )
}

fn get_nth_letter(word: &str, index: usize) -> Option<&str> {
    word_graphemes(word).nth(index)
}

fn get_first_letter(word: &str) -> Option<&str> {
    get_nth_letter(word, 0)
}

fn get_last_letter(word: &str) -> Option<&str> {
    let index = graphemes_count(word).saturating_sub(1);
    get_nth_letter(word, index)
}

fn word_graphemes(word: &str) -> Graphemes {
    word.graphemes(true)
}

fn graphemes_count(word: &str) -> usize {
    word_graphemes(word).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_letter() {
        assert_eq!(get_first_letter("hello"), Some("h"));
        assert_eq!(get_first_letter("HELLO"), Some("H"));
        assert_eq!(get_first_letter("a"), Some("a"));
        assert_eq!(get_first_letter(""), None);
    }

    #[test]
    fn test_get_last_letter() {
        assert_eq!(get_last_letter("hello"), Some("o"));
        assert_eq!(get_last_letter("HELLO"), Some("O"));
        assert_eq!(get_last_letter("a"), Some("a"));
        assert_eq!(get_last_letter(""), None);
    }

    #[test]
    fn test_abbreviate_english() {
        assert_eq!(abbreviate(""), "");
        assert_eq!(abbreviate("a"), "a");
        assert_eq!(abbreviate("ab"), "ab");
        assert_eq!(abbreviate("abc"), "a1c");
        assert_eq!(abbreviate("word"), "w2d");
        assert_eq!(abbreviate("localization"), "l10n");
        assert_eq!(abbreviate("internationalization"), "i18n");
        assert_eq!(abbreviate("pneumonoultramicroscopicsilicovolcanoconiosis"), "p43s");
    }

    #[test]
    fn test_abbreviate_greek() {
        assert_eq!(abbreviate("τι κάνεις"), "τ7ς");
        assert_eq!(abbreviate("Καλημέρα αρχηγέ"), "Κ13έ");
        assert_eq!(abbreviate("Άντε και στα δικά σου!"), "Ά20!");
        assert_eq!(abbreviate("σκουλικομερμυγκότρυπα"), "σ19α");
        assert_eq!(abbreviate("Άσπρη πέτρα ξέξασπρη κι απ' τον ήλιο ξεξασπρώτερη"), "Ά47η");
    }

    #[test]
    fn test_abbreviate_grapheme_clusters() {
        // https://doc.rust-lang.org/book/ch08-02-strings.html#creating-a-new-string
        // https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.grapheme_indices
        assert_eq!(abbreviate("عليكم"), "ع3م");
        assert_eq!(abbreviate("السلام"), "ا4م");
        assert_eq!(abbreviate("Dobrý"), "D3ý");
        assert_eq!(abbreviate("שָׁלוֹם"), "שָׁ2ם");
        assert_eq!(abbreviate("नमस्ते"), "न2ते");
        assert_eq!(abbreviate("こんにちは"), "こ3は");
        assert_eq!(abbreviate("안녕하세요"), "안3요");
        assert_eq!(abbreviate("你好"), "你好");
        assert_eq!(abbreviate("Olá"), "O1á");
        assert_eq!(abbreviate("Здравствуйте"), "З10е");
        assert_eq!(abbreviate("Hola"), "H2a");
        assert_eq!(abbreviate("a̐éö̲"), "a̐1ö̲");
    }

    #[test]
    fn test_abbreviate_with_spaces() {
        assert_eq!(abbreviate("hello world"), "h9d");
        assert_eq!(abbreviate(" space "), " 5 ");
        assert_eq!(abbreviate("   "), " 1 ");
    }

    #[test]
    fn test_abbreviate_with_special_characters() {
        assert_eq!(abbreviate("hello!"), "h4!");
        assert_eq!(abbreviate("@#$"), "@1$");
        assert_eq!(abbreviate("!@#$%"), "!3%");
    }

    #[test]
    fn test_abbreviate_with_numbers() {
        assert_eq!(abbreviate("h3llo"), "h3o");
        assert_eq!(abbreviate("1234567"), "157");
        assert_eq!(abbreviate("y2k"), "y1k");
    }

    #[test]
    fn test_abbreviate_mixed_scripts() {
        assert_eq!(abbreviate("Hello世界"), "H4界");
        assert_eq!(abbreviate("Café☕"), "C2☕");
        assert_eq!(abbreviate("🌟star🌟"), "🌟3🌟");
    }
}
