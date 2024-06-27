pub trait WordBoundResolverLike {
    fn resolve(s: &str) -> Vec<String>;
    #[inline]
    fn resolve_with<T: WordBoundResolverLike>(s: &str) -> Vec<String> {
        T::resolve(s)
    }
}

pub struct WordBoundResolver;

impl WordBoundResolverLike for WordBoundResolver {
    #[inline]
    fn resolve(s: &str) -> Vec<String> {
        #[cfg(all(feature = "use_regex", not(feature = "use_fancy_regex")))]
        {
            use crate::word_bounds::impls::regex::*;
            WordBoundsResolverRegex::resolve(s)
        }
        #[cfg(all(feature = "use_fancy_regex", not(feature = "use_regex")))]
        {
            use crate::word_bounds::impls::fancy_regex::*;
            WordBoundResolverFancyRegex::resolve(s)
        }
        #[cfg(all(not(feature = "use_fancy_regex"), not(feature = "use_regex")))]
        {
            use crate::word_bounds::impls::regexless::*;
            WordBoundResolverRegexless::resolve(s)
        }
        #[cfg(all(feature = "use_fancy_regex", feature = "use_regex"))]
        compile_error!(
            "You should not have both `use_regex` and `use_fancy_regex` enabled at the \
        same time."
        )
    }
}
