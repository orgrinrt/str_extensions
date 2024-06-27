use fancy_regex::Regex;
#[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
use once_cell::sync::Lazy;

use crate::__str_ext__instance_words_vec;
use crate::resolver::WordBoundResolverLike;

#[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"_|-|(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])
(?=[A-Z][a-z])",
    )
    .expect("Expected valid fancy_regex pattern")
});

pub struct WordBoundResolverFancyRegex;

impl WordBoundResolverLike for WordBoundResolverFancyRegex {
    fn resolve(s: &str) -> Vec<String> {
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let re = Regex::new(
            r"_|-|(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])
(?=[A-Z][a-z])",
        )
        .expect("Expected valid fancy_regex pattern");

        __str_ext__instance_words_vec!(s, words);

        // Since split function is not available in fancy_regex
        // we do it manually using find_iter

        #[cfg(all(not(feature = "optimize_for_memory"), feature = "optimize_for_cpu"))]
        let captures_iter = REGEX.find_iter(s);
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let captures_iter = re.find_iter(s);

        let mut last = 0;
        for match_ in captures_iter {
            let cap = match_.expect("Unable to find capture");
            let start = cap.start();

            if start > last {
                let part = &s[last..start];
                words.push(part.to_lowercase());
            }
            last = cap.end();
        }

        if last < s.len() {
            words.push(s[last..].to_lowercase());
        }

        words
    }
}
