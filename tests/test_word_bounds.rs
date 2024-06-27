#[cfg(test)]
mod tests {
    #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
    use str_extensions::resolver::impls::WordBoundResolverFancyRegex;
    use str_extensions::resolver::impls::{WordBoundResolverRegexless, WordBoundsResolverRegex};
    use str_extensions::resolver::WordBoundResolverLike;

    const INPUT: &str = "This_is_SomeRandom_Text-to-split2";
    const EXPECTED: [&str; 8] = ["this", "is", "some", "random", "text", "to", "split", "2"];

    #[test]
    #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
    fn test_word_bounds_fancy_regex() {
        assert_eq!(WordBoundResolverFancyRegex::resolve(INPUT), EXPECTED);
    }

    #[test]
    fn test_word_bounds_regex() {
        assert_eq!(WordBoundsResolverRegex::resolve(INPUT), EXPECTED);
    }

    #[test]
    fn test_word_bounds_no_regex() {
        assert_eq!(WordBoundResolverRegexless::resolve(INPUT), EXPECTED);
    }
}
