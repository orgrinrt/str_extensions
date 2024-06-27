pub(super) mod impls;
pub(super) mod resolver;

#[cfg(feature = "optimize_for_cpu")]
pub(super) const CHARS_PER_WORD_AVG: usize = 3;

#[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
pub(super) const CHARS_PER_WORD_AVG: usize = 3;
