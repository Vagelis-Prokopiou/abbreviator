pub fn abbreviate(word: &str) -> String {
    let word_length = word.chars().count();
    if word_length < 3 {
        return word.to_string();
    }
    /*
        At this point of the code the following unwraps should always  
        succeed because they can only return None if the word is empty.

        If they fail they return the default value for char, which is '\x00':
        https://doc.rust-lang.org/std/primitive.char.html#impl-Default-for-char
    */
    format!(
        "{}{}{}",
        get_first_letter(word).unwrap_or_default(),
        word_length - 2,
        get_last_letter(word).unwrap_or_default()
    )
}

fn get_nth_letter(word: &str, index: usize) -> Option<char> {
    word.chars().nth(index)
}

fn get_first_letter(word: &str) -> Option<char> {
    get_nth_letter(word, 0)
}

fn get_last_letter(word: &str) -> Option<char> {
    let index = word.chars().count().saturating_sub(1);
    get_nth_letter(word, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_letter() {
        assert_eq!(get_first_letter("hello"), Some('h'));
        assert_eq!(get_first_letter("HELLO"), Some('H'));
        assert_eq!(get_first_letter("a"), Some('a'));
        assert_eq!(get_first_letter(""), None);
    }

    #[test]
    fn test_get_last_letter() {
        assert_eq!(get_last_letter("hello"), Some('o'));
        assert_eq!(get_last_letter("HELLO"), Some('O'));
        assert_eq!(get_last_letter("a"), Some('a'));
        assert_eq!(get_last_letter(""), None);
    }

    #[test]
    fn test_abbreviate() {
        // English
        assert_eq!(abbreviate(""), "");
        assert_eq!(abbreviate("a"), "a");
        assert_eq!(abbreviate("ab"), "ab");
        assert_eq!(abbreviate("abc"), "a1c");
        assert_eq!(abbreviate("word"), "w2d");
        assert_eq!(abbreviate("localization"), "l10n");
        assert_eq!(abbreviate("internationalization"), "i18n");
        assert_eq!(abbreviate("pneumonoultramicroscopicsilicovolcanoconiosis"), "p43s");

        // Greek
        assert_eq!(abbreviate("τι κάνεις"), "τ7ς");
        assert_eq!(abbreviate("Καλημέρα αρχηγέ"), "Κ13έ");
        assert_eq!(abbreviate("Άντε και στα δικά σου!"), "Ά20!");
        assert_eq!(abbreviate("σκουλικομερμυγκότρυπα"), "σ19α");
        assert_eq!(abbreviate("Άσπρη πέτρα ξέξασπρη κι απ' τον ήλιο ξεξασπρώτερη"), "Ά47η");
    }
}
