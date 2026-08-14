#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]

#[cfg(all(feature = "optimize_for_memory", feature = "optimize_for_cpu"))]
compile_error!(
    "Features `optimize_for_memory` and `optimize_for_cpu` are mutually exclusive. \
Only select one of the two to enable at a time."
);

#[cfg(any(feature = "join", feature = "append", feature = "prepend"))]
#[doc(hidden)]
pub(crate) mod building;

#[cfg(any(
    feature = "to_snake_case",
    feature = "to_camel_case",
    feature = "to_pascal_case",
    feature = "to_kebab_case",
    feature = "to_human_readable",
    feature = "to_title_case"
))]
#[doc(hidden)]
pub(crate) mod cases;

#[cfg(any(feature = "as_cow", feature = "into_arc",))]
#[doc(hidden)]
pub(crate) mod type_coercion;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::building::*;
    #[allow(unused_imports)]
    pub use super::cases::*;
    #[allow(unused_imports)]
    pub use super::type_coercion::*;
}

/// Word segmentation, re-exported from the [`word_bounds`] crate.
///
/// The implementations used to be vendored here. They are one crate now, so a fix to the
/// segmentation rules lands in one place rather than two.
pub mod resolver {
    pub use word_bounds::resolver::WordBoundResolver;
    #[allow(unused_imports)]
    #[cfg(any(feature = "optimize_for_cpu", feature = "optimize_for_memory"))]
    pub(crate) use word_bounds::CHARS_PER_WORD_AVG;

    pub mod rules {
        pub use word_bounds::rules::*;
    }

    pub mod impls {
        pub use word_bounds::impls::charwalk::Charwalk;
        #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
        pub use word_bounds::impls::fancy_regex::FancyRegex;
        #[cfg(any(feature = "use_regex", feature = "benchmark"))]
        pub use word_bounds::impls::regex::Regex;
        pub use word_bounds::WordBoundResolverImpl;
    }
}
