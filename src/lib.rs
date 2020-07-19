pub fn abbreviate(word: &str, threshold_length: i32) -> String {
    if word.len() < threshold_length as usize {
        return word.to_string();
    }

    return get_first_letter(word)
        + (word.chars().count() - 2).to_string().as_str()
        + get_last_letter(word).as_str();
}

fn get_nth_letter(word: &str, index: u16) -> String {
    return word.chars().nth(index as usize).unwrap().to_string();
}

fn get_first_letter(word: &str) -> String {
    if word.chars().count() == 0 {
        return word.to_string();
    }
    return get_nth_letter(word, 0);
}

fn get_last_letter(word: &str) -> String {
    if word.chars().count() == 0 {
        return word.to_string();
    }
    return get_nth_letter(word, (word.chars().count() - 1) as u16);
}

#[cfg(test)]
mod tests {
    use crate::{abbreviate, get_first_letter, get_last_letter};

    #[test]
    fn test_get_first_letter() {
        assert_eq!(get_first_letter("hello"), "h");
        assert_eq!(get_first_letter("HELLO"), "H");
        assert_eq!(get_first_letter(""), "");
    }

    #[test]
    fn test_get_last_letter() {
        assert_eq!(get_last_letter("hello"), "o");
        assert_eq!(get_last_letter("HELLO"), "O");
        assert_eq!(get_last_letter(""), "");
    }

    #[test]
    fn test_abbreviate_should_get_initial_word() {
        let threshold = 50;
        assert_eq!(abbreviate("word", threshold), "word");
        assert_eq!(abbreviate("localization", threshold), "localization");
        assert_eq!(abbreviate("internationalization", threshold), "internationalization");
        assert_eq!(abbreviate("pneumonoultramicroscopicsilicovolcanoconiosis", threshold), "pneumonoultramicroscopicsilicovolcanoconiosis");
    }

    #[test]
    fn test_abbreviate_should_get_abbreviation() {
        let threshold = 3;
        assert_eq!(abbreviate("word", threshold), "w2d");
        assert_eq!(abbreviate("localization", threshold), "l10n");
        assert_eq!(abbreviate("internationalization", threshold), "i18n");
        assert_eq!(abbreviate("pneumonoultramicroscopicsilicovolcanoconiosis", threshold), "p43s");
    }
}
