#[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
use once_cell::sync::Lazy;
use regex::Regex;

use crate::__str_ext__instance_words_vec;
use crate::resolver::WordBoundResolverLike;

#[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
static REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([a-zA-Z0-9]+)").expect("Expected valid regex pattern"));

pub struct WordBoundsResolverRegex;

impl WordBoundResolverLike for WordBoundsResolverRegex {
    fn resolve(s: &str) -> Vec<String> {
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let re = Regex::new(r"([a-zA-Z0-9]+)").expect("Expected valid regex pattern");

        __str_ext__instance_words_vec!(s, words);

        #[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
        let captures_iter = REGEX.captures_iter(s);
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let captures_iter = re.captures_iter(s);

        for cap in captures_iter {
            let mut combined_word = cap[0].to_string();

            let mut chars = combined_word.chars().peekable();
            let mut split_words = Vec::new();
            let mut word = String::new();

            while let Some(c) = chars.next() {
                if c.is_uppercase() && !word.is_empty() {
                    split_words.push(word);
                    word = String::new();
                }
                word.push(c);

                // If next character is uppercase, push the current word to split_words
                if let Some(&next_char) = chars.peek() {
                    if next_char.is_uppercase() {
                        split_words.push(word.clone());
                        word.clear();
                    }
                }
            }

            split_words.push(word);
            words.extend(split_words.into_iter().map(|word| word.to_lowercase()));
        }

        words
    }
}
