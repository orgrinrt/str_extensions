#[cfg(test)]
mod tests {
    #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
    use str_extensions::resolver::impls::FancyRegex;
    #[cfg(any(feature = "use_regex", feature = "benchmark"))]
    use str_extensions::resolver::impls::Regex;
    use str_extensions::resolver::impls::{Regexless, WordBoundResolverImpl};
    use str_extensions::resolver::rules::DefaultRules;
    use str_extensions::resolver::WordBoundResolver;

    // FOR DEFAULT RULES
    const TESTS: &[(&str, &[&str])] = &[
        (
            "This_is_SomeRandom_Text-to-split2",
            &["this", "is", "some", "random", "text", "to", "split", "2"],
        ),
        ("_PrependedUnderscore", &["_prepended", "underscore"]),
        ("AppendedUnderscore_", &["appended", "underscore_"]),
        ("UPPERCASELETTERS", &["uppercaseletters"]),
        ("lowercaseletters", &["lowercaseletters"]),
        ("CamelCase", &["camel", "case"]),
        ("snake_case", &["snake", "case"]),
        ("kebab-case", &["kebab", "case"]),
        ("WordWithNumbers123", &["word", "with", "numbers", "123"]),
        ("Short1", &["short", "1"]),
        ("number123456", &["number", "123456"]),
        ("someHTML", &["some", "html"]),
        ("JSONResponse", &["json", "response"]),
        (
            "WithSpecial-_*Characters",
            &["with", "special", "-", "_", "characters"],
        ),
        ("hashtag#rust", &["hashtag", "#rust"]),
    ];

    #[test]
    #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
    fn test_word_bounds_fancy_regex() {
        for (input, expected) in TESTS {
            assert_eq!(
                WordBoundResolver::<FancyRegex, DefaultRules>::resolve(input),
                *expected
            );
        }
    }

    #[test]
    #[cfg(any(feature = "use_regex", feature = "benchmark"))]
    fn test_word_bounds_regex() {
        for (input, expected) in TESTS {
            assert_eq!(
                WordBoundResolver::<Regex, DefaultRules>::resolve(input),
                *expected
            );
        }
    }

    #[test]
    fn test_word_bounds_no_regex() {
        for (input, expected) in TESTS {
            assert_eq!(
                WordBoundResolver::<Regexless, DefaultRules>::resolve(input),
                *expected
            );
        }
    }
}
