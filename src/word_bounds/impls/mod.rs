#[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
pub(crate) mod fancy_regex;
#[cfg(any(feature = "use_regex", feature = "benchmark"))]
pub(crate) mod regex;
pub(crate) mod regexless;

#[macro_export]
macro_rules! __str_ext__instance_words_vec {
    ($s:expr, $vec:ident) => {
        #[cfg(feature = "optimize_for_cpu")]
        let mut $vec = Vec::with_capacity($s.len() / $crate::resolver::CHARS_PER_WORD_AVG);
        #[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
        let mut $vec = Vec::with_capacity($s.len() / $crate::resolver::CHARS_PER_WORD_AVG as usize);
        #[cfg(all(not(feature = "optimize_for_cpu"), not(feature = "optimize_for_memory")))]
        let mut $vec = Vec::new();
    };
}
