#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]
#![cfg_attr(feature = "no_std", no_std)]

// A refusal for `no_std` with a regex backend lived here and could never fire: cargo builds
// dependencies first, so word_bounds refuses while this crate has not been compiled at all.
// Its message is what a consumer sees, and it names `cargo tree -e features`, which is how
// they find out the backend came from this crate's defaults rather than from them.
//
// What is worth saying here is in the README, under the feature table: `--features no_std`
// leaves `use_regex` on, because it is a default, so a `no_std` consumer wants
// `--no-default-features --features no_std,<the rest>`.

// `alloc` rather than `std`, in both configurations. It is a sysroot crate, so naming it
// here costs a std build nothing and is what lets one set of paths serve both. Every
// method this crate has returns something owned, so `alloc` is what it needs and `std` is
// what it happened to be written against.
extern crate alloc;

#[cfg(all(feature = "optimize_for_memory", feature = "optimize_for_cpu"))]
compile_error!(
    "Features `optimize_for_memory` and `optimize_for_cpu` are mutually exclusive. \
Only select one of the two to enable at a time."
);

#[cfg(any(feature = "append", feature = "prepend", feature = "concat", feature = "join"))]
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

#[cfg(any(feature = "as_cow", feature = "into_arc", feature = "to_arc"))]
#[doc(hidden)]
pub(crate) mod type_coercion;

/// Case conversion into storage the caller lends.
///
/// Behind `no_alloc`, which is what supplies the lending contract it is written against.
#[cfg(feature = "no_alloc")]
pub mod lending;

// The lending API answers in notko's types, so they are re-exported here. Without this a
// consumer has to take a direct dependency on notko in order to name what this crate
// handed it, which is a dependency it did not choose and would have to keep in step.
#[cfg(feature = "no_alloc")]
pub use notko::lend::{Exhausted, Lend};
#[cfg(feature = "no_alloc")]
pub use notko::outcome::Outcome;

/// Everything the enabled features provide.
///
/// Each re-export carries the same condition as the module it names. Without that the
/// prelude referred to modules that the feature flags had compiled out, so **any**
/// selection short of all three failed with `could not find 'cases' in the crate root`,
/// and the per-feature configurability the crate advertises could not be used at all.
pub mod prelude {
    #[cfg(feature = "no_alloc")]
    #[allow(unused_imports)]
    pub use super::lending::*;

    #[cfg(any(feature = "append", feature = "prepend", feature = "concat", feature = "join"))]
    #[allow(unused_imports)]
    pub use super::building::*;

    #[cfg(any(
        feature = "to_snake_case",
        feature = "to_camel_case",
        feature = "to_pascal_case",
        feature = "to_kebab_case",
        feature = "to_human_readable",
        feature = "to_title_case"
    ))]
    #[allow(unused_imports)]
    pub use super::cases::*;

    #[cfg(any(feature = "as_cow", feature = "into_arc", feature = "to_arc"))]
    #[allow(unused_imports)]
    pub use super::type_coercion::*;
}

/// Word segmentation, re-exported from the [`word_bounds`] crate.
///
/// The implementations used to be vendored here. They are one crate now, so a fix to the
/// segmentation rules lands in one place rather than two.
pub mod resolver {
    pub use word_bounds::resolver::WordBoundResolver;

    pub mod rules {
        pub use word_bounds::rules::*;
    }

    pub mod impls {
        pub use word_bounds::impls::charwalk::Charwalk;
        // `not(no_std)` as well as the backend flag, because word_bounds compiles both
        // regex backends out under `no_std`: the regex crates need std. Without this arm
        // the default selection plus `no_std`, which is exactly what Cargo's feature
        // unification produces when anything in the graph asks for it, fails to resolve
        // a module that is not there.
        #[cfg(all(
            not(feature = "no_std"),
            any(feature = "use_fancy_regex", feature = "benchmark")
        ))]
        pub use word_bounds::impls::fancy_regex::FancyRegex;
        #[cfg(all(
            not(feature = "no_std"),
            any(feature = "use_regex", feature = "benchmark")
        ))]
        pub use word_bounds::impls::regex::Regex;
        pub use word_bounds::WordBoundResolverImpl;
    }
}
