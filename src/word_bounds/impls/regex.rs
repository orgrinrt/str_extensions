use std::marker::PhantomData;

#[cfg(not(feature = "optimize_for_memory"))]
use once_cell::sync::Lazy;
use regex::Regex as RE;

use crate::__str_ext__instance_words_vec;
use crate::resolver::impls::WordBoundResolverImpl;
use crate::resolver::rules::{DefaultRules, ResolverRules};
use crate::word_bounds::CompiledRules;

const REGEX_CRATE_PATTERN: &str = r"([a-zA-Z][a-z]*|[A-Z]+[a-z]*|[0-9]+)";

#[cfg(not(feature = "optimize_for_memory"))]
static REGEX: Lazy<RE> =
    Lazy::new(|| RE::new(REGEX_CRATE_PATTERN).expect("Expected valid regex pattern"));

pub struct Regex<R: ResolverRules = DefaultRules> {
    _phantom_data: PhantomData<R>,
}

impl<R: ResolverRules> WordBoundResolverImpl<R> for Regex<R> {
    fn resolver(s: &str) -> Vec<String> {
        #[cfg(feature = "optimize_for_memory")]
        let re = RE::new(REGEX_CRATE_PATTERN).expect("Expected valid regex pattern");

        __str_ext__instance_words_vec!(s, words);

        #[cfg(not(feature = "optimize_for_memory"))]
        let captures_iter = REGEX.captures_iter(s);
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let captures_iter = re.captures_iter(s);

        for cap in captures_iter {
            words.push(cap[0].to_lowercase());
        }

        words
    }

    fn compile_rules() -> CompiledRules {
        CompiledRules::Regex(String::from("TODO"))
    }
}
