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

#[cfg(not(feature = "benchmark"))]
#[doc(hidden)]
mod word_bounds;
#[cfg(feature = "benchmark")]
#[doc(hidden)]
pub mod word_bounds;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::building::*;
    #[allow(unused_imports)]
    pub use super::cases::*;
    #[allow(unused_imports)]
    pub use super::type_coercion::*;
}

pub mod resolver {
    pub use crate::word_bounds::resolver::WordBoundResolver;
    #[allow(unused_imports)]
    #[cfg(any(feature = "optimize_for_cpu", feature = "optimize_for_memory"))]
    pub(crate) use crate::word_bounds::CHARS_PER_WORD_AVG;

    pub mod rules {
        pub use crate::word_bounds::rules::*;
    }

    pub mod impls {
        pub use crate::word_bounds::impls::charwalk::Charwalk;
        #[cfg(any(feature = "use_fancy_regex", feature = "benchmark"))]
        pub use crate::word_bounds::impls::fancy_regex::FancyRegex;
        #[cfg(any(feature = "use_regex", feature = "benchmark"))]
        pub use crate::word_bounds::impls::regex::Regex;
        #[allow(unused_imports)]
        pub(crate) use crate::word_bounds::resolver::{
            contains_special_chars, is_special_char, remove_prepended_underscores,
        };
        pub use crate::word_bounds::WordBoundResolverImpl;
    }
}
