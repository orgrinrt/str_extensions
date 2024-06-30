use std::marker::PhantomData;

use crate::__str_ext__instance_words_vec;
use crate::resolver::impls::WordBoundResolverImpl;
use crate::resolver::rules::{DefaultRules, ResolverRules};
use crate::word_bounds::CompiledRules;
use crate::word_bounds::CompiledRules::NotApplicable;

pub struct Regexless<R: ResolverRules = DefaultRules> {
    _phantom_data: PhantomData<R>,
}

impl<R: ResolverRules> WordBoundResolverImpl<R> for Regexless<R> {
    fn resolver(s: &str) -> Vec<String> {
        __str_ext__instance_words_vec!(s, words);

        // let mut current_word: String = String::new();
        // let mut prev_char: char = ' ';
        //
        // for ch in s.chars() {
        //     match ch {
        //         ch if is_special_char(ch) => {
        //             // Check special_chars_are_words rule
        //             if R::special_chars_are_words() {
        //                 current_word.push(ch);
        //             } else {
        //                 if !current_word.is_empty() {
        //                     words.push(current_word.to_lowercase());
        //                     current_word.clear();
        //                 }
        //             }
        //         }
        //         // Handle alphanumeric characters
        //         ch if ch.is_alphabetic() || ch.is_numeric() => {
        //             // Check skip_prepended_underscores rule
        //             if R::skip_prepended_underscores() && prev_char == '_' && current_word.is_empty() {
        //                 continue;
        //             }
        //
        //             if prev_char == '_' || prev_char == '-' {
        //                 current_word.clear();
        //             }
        //             current_word.push(ch);
        //         }
        //         // Handle other characters
        //         _ => {
        //             if !current_word.is_empty() {
        //                 words.push(current_word.to_lowercase());
        //                 current_word.clear();
        //             }
        //         }
        //     };
        //
        //     prev_char = ch;
        // }
        //
        // if !current_word.is_empty() {
        //     words.push(current_word.to_lowercase());
        // }

        words
    }

    fn compile_rules() -> CompiledRules {
        NotApplicable
    }
}
