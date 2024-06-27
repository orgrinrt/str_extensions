use crate::__str_ext__instance_words_vec;
use crate::resolver::WordBoundResolverLike;

pub struct WordBoundResolverRegexless;

impl WordBoundResolverLike for WordBoundResolverRegexless {
    fn resolve(s: &str) -> Vec<String> {
        __str_ext__instance_words_vec!(s, words);

        let mut buffer: String = String::new();
        let mut current_word: String = String::new();
        let mut prev_is_uppercase = false;

        for ch in s.chars() {
            match ch {
                '_' | '-' => {
                    if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                    }
                    prev_is_uppercase = false;
                },
                ch if ch.is_uppercase() => {
                    if prev_is_uppercase {
                        buffer.push(ch);
                    } else if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                    }
                    prev_is_uppercase = true;
                },
                ch if ch.is_lowercase() => {
                    if prev_is_uppercase {
                        words.push(current_word.to_lowercase());
                        current_word.clear();
                        if !buffer.is_empty() {
                            current_word.push_str(&buffer);
                            buffer.clear();
                        }
                    }
                    prev_is_uppercase = false;
                },
                _ => {},
            }

            current_word.push(ch);
        }

        if !current_word.is_empty() {
            words.push(current_word.to_lowercase());
        }

        words
    }
}
