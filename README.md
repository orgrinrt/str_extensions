str_extensions
============
<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/issues)
[![Current Version](https://img.shields.io/badge/version-0.0.1-red.svg)](https://github.com/orgrinrt/str_extensions)

> Useful extension methods for strings in Rust, carefully benchmarked and extensively configurable by feature flags to
> minimise its footprint.

</div>

## Usage

These extensions are implemented for all types implementing `AsRef<str>`. This covers most of the usual string types,
including `String`, `Cow<str>` and `&str` itself.

## Extensions

Note: on the current `main`, the extension traits below (`CaseConversions`, `StringBuilding`,
`TypeCoercion`) are declared `pub(super)` in their source modules, so none of their methods are
reachable from outside the crate yet, regardless of the WIP status noted per trait.

### Formatting, cases

❎ WIP: not exposed as public API yet (the trait is `pub(super)`). The current implementation in
`src/cases.rs` is a standalone char/whitespace based splitter; it does not use the
`resolver`/`word_bounds` machinery described under Performance below.
<details>
<summary><code>trait CaseConversions</code> (click to open details)</summary>

| Function Name       | Example              | Details                          |
|---------------------|-----------------------|----------------------------------|
| `to_snake_case`     | `this_is_an_example`  |                                   |
| `to_camel_case`     | `thisIsAnExample`     |                                   |
| `to_pascal_case`    | `ThisIsAnExample`     |                                   |
| `to_kebab_case`     | `this-is-an-example`  |                                   |
| `to_human_readable` | `this is an example`  | tries its best, work in progress |
| `to_title_case`     | `This Is An Example`  | tries its best, work in progress |

</details>

### String building

⚠️ WIP: not exposed as public API yet (the trait is `pub(super)`), and not all impls match
descriptions currently
<details>
<summary><code>trait StringBuilding</code> (click to open details)</summary>

| Function Name | Example                                                                  | Details                                   |
|---------------|---------------------------------------------------------------------------|-------------------------------------------|
| `join`        | `"foo".join("bar")` -> `"foobar"`</br> borrow -> owned                    | only naively functional, work in progress |
| `concat`      | `"foo".concat(&["bar", "bat"])` -> `"foobarbat"`</br> borrow -> owned     | only naively functional, work in progress |
| `append`      | `"foo".append("bar")` -> `"foobar"`</br> borrow -> owned                  | only naively functional, work in progress |
| `prepend`     | `"foo".prepend("bar")` -> `"barfoo"`</br> borrow -> owned                 | only naively functional, work in progress |

</details>

### Type coercion

⚠️ WIP: not exposed as public API yet (the trait is `pub(super)`)
<details>
<summary><code>trait TypeCoercion</code> (click to open details)</summary>

| Function Name | Example | Details                                                                                                |
|---------------|---------|----------------------------------------------------------------------------------------------------------|
| `as_cow`      |         | Free; cost only applies when mutating the string, which turns it into `Cow::Owned` state              |
| `into_arc`    |         | Allocates a `String` and wraps it into an `Arc`                                                        |

</details>

### Naming conventions

<details>
<summary>Naming conventions (click to open details)</summary>

We try to follow
the [official rust naming guidelines](https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv),
i.e:

| Prefix | Cost      | Ownership                                                                                     |
|--------|-----------|-----------------------------------------------------------------------------------------------|
| as_    | Free      | borrowed -> borrowed                                                                          |
| to_    | Expensive | borrowed -> borrowed <br/>borrowed -> owned (non-Copy types) <br/>owned -> owned (Copy types) |
| into_  | Variable  | owned -> owned (non-Copy types)                                                               |

This means that you can expect the extension methods to follow the official semantics and behave similarly, especially
regarding the cost.
</details>

## Performance

This repository contains three different methods to perform word bounds resolution - with standard `regex` crate,
with `fancy_regex` crate, and a custom char-walking version (`Charwalk`).

The performance of these methods is evaluated using `criterion`
benchmarking library. See [benches/bench_word_bounds.rs](benches/bench_word_bounds.rs) for the benchmarking code and
try it yourself (the bench target needs `regex` and `fancy_regex` compiled in, so run it with
`cargo bench --features benchmark`; the default feature set alone does not export `FancyRegex` and the bench fails to
compile without it). Here are the latest results on a macbook air m1 (which shows the relational performance, while
the exacts
will of course vary by system etc.):

| Type                | Execution Time       | Description                                                                                                                                                                                                                                          |
|---------------------|----------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Regex`      | 119.09  µs (average) | ⚠️ **Major WIP** </br>(More) Accurate, but currently ~50x <br/><br/>slower than `Charwalk`. Based on prior proof-of-concepts, we should ultimately land at around ~3x slower than the charwalk variant. Suitable for non-critical performance paths. |
| `FancyRegex` | 15.433  µs (average) | 🚧 **WIP, but almost there** </br>All-inclusive regex logic including lookahead/lookback, which should be even more accurate, but ~7x slower than `Charwalk`. Use only when other variants fail.                                                     |
| `Charwalk`   | 2.4 µs (average)     | ❎ **Just needs more optimization** </br>Fastest and simplest, but could fail on certain edge cases. Officially suggested method for common cases.                                                                                                    |

The `criterion` benchmark results show that `Charwalk` is the fastest yet simplest method, taking only
about
2.4 µs on average per the benchmarking execution. The regex variants can be more accurate, and their logic is
using a tried and
tested framework, but they are significantly more expensive to run; the `Regex` variant that has no integrated
lookahead/lookback features, replaces this absence with a custom post-process pass, and should be about 3 times slower
than the
`Charwalk` variant (⚠️ *but is under construction and while it passes the tests, it's 50x slower at
the moment* ⚠️). The
`FancyRegex` variant, which makes use of the regex
engine for all of
its logic (including
lookahead/lookback), is more than 7 times slower than the `Charwalk` variant, though should yield
the most accurate results.

> Note: The regex variants are somewhat optimized, and in addition the crate has two different focuses for
> optimizations with
> the feature flags
`optimize_for_cpu` and
`optimize_for_memory`. This is mostly relevant for someone doing extreme and picky optimizations on a larger project,
> otherwise one should stick to the defaults. The
> default configuration for optimizations bring the heaviest one, `fancy_regex` variant, down from around the 40 micro
> second range to its current ~15 micro second range (with the same system as for the above benchmark results).

The official suggestion is to use `Charwalk` (the resolver `WordBoundResolver` defaults to, regardless of which of
`use_regex`/`use_fancy_regex` are compiled in), unless you face an edge case that isn't covered yet in the manual
parsing logic. After that, you should test whether `Regex` works, and if not, try `FancyRegex`. Note that `use_regex`
is currently in the `default` feature set in `Cargo.toml`, which only controls whether `Regex` is compiled in and
available to select explicitly; it does not change which resolver `WordBoundResolver` uses by default.

> Note: Ultimately the costs are not usually all that significant, since this
> shouldn't be called in any hot loops, but your mileage may vary. Any and all issues and pull requests are welcome,
> if you face an edge case that isn't covered on the `Charwalk` variant.

## Example

TODO

## The Problem

TODO

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying
me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/str_extensions/blob/main/LICENSE)

This project is licensed under the terms of the **MIT** license.
