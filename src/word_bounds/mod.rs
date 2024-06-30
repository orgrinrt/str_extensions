use crate::resolver::rules::{DefaultRules, ResolverRules};

pub(super) mod impls;
pub(super) mod resolver;
pub(super) mod rules;

#[cfg(feature = "optimize_for_cpu")]
pub(super) const CHARS_PER_WORD_AVG: usize = 3;

#[cfg(all(not(feature = "optimize_for_cpu"), feature = "optimize_for_memory"))]
pub(super) const CHARS_PER_WORD_AVG: usize = 3;

pub trait WordBoundResolverImpl<R: ResolverRules = DefaultRules> {
    fn resolver(s: &str) -> Vec<String>;
    fn compile_rules() -> CompiledRules;
}

pub enum CompiledRules {
    Regex(String),
    Str(String),
    NotApplicable,
}
