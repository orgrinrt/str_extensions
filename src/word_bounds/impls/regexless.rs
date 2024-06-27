use crate::__str_ext__instance_words_vec;
use crate::resolver::WordBoundResolverLike;

pub struct WordBoundResolverRegexless;

impl WordBoundResolverLike for WordBoundResolverRegexless {
    fn resolve(s: &str) -> Vec<String> {
        __str_ext__instance_words_vec!(s, words);

        let mut current_word: String = String::new();
        let mut prev_char: char = ' ';

        for ch in s.chars() {
            match ch {
                '_' | '-' => {
                    if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                    }
                },
                ch if ch.is_alphabetic() => {
                    if prev_char.is_lowercase() && ch.is_uppercase() {
                        if !current_word.is_empty() {
                            words.push(current_word.to_lowercase());
                            current_word.clear();
                        }
                    } else if prev_char == '_' || prev_char == '-' {
                        current_word.clear();
                    }
                    current_word.push(ch);
                },
                ch if ch.is_numeric() => {
                    if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                    }
                    current_word.push(ch);
                },
                _ => {
                    if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                    }
                },
            };

            prev_char = ch;
        }

        if !current_word.is_empty() {
            words.push(current_word.to_lowercase());
        }

        words
    }
}
