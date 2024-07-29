str_extensions
============
<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/str_extensions)](https://crates.io/crates/str_extensions)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/issues)
[![Current Version](https://img.shields.io/badge/version-0.0.1-red.svg)](https://github.com/orgrinrt/str_extensions)

> Useful extension methods for strings in Rust, carefully benchmarked and extensively configurable by feature flags to
> minimise its footprint.

</div>

## Usage

These extensions are implemented for all types implementing `AsRef<str>`. This covers most of the usual string types,
including `String`, `Cow<str>` and `&str` itself.

## Extensions

### Formatting, cases

❎ WIP: not yet implemented, but the underlying word bounding logic is fully implemented and passes
rudimentary tests
<details>
<summary><code>trait CaseConversions</code> (click to open details)</summary>

| Function Name       | Example               | Details                          |
|---------------------|-----------------------|----------------------------------|
| `to_snake_case`     | `this_is_an_example`  | has an uppercase variant         |
| `to_camel_case`     | `thisIsAnExample`     |                                  |
| `to_pascal_case`    | `ThisIsAnExample`     |                                  |
| `to_kebab_case`     | `this-is-an-example`  | has an uppercase variant         |
| `to_human_readable` | `This is an example.` | tries its best, work in progress |
| `to_title_case`     | `This is an Example`  | tries its best, work in progress |

</details>

### String building

⚠️ WIP: Not completely implemented; Also unstable, unoptimized, not all impls match descriptions currently
<details>
<summary><code>trait StringBuildExtensions</code> (click to open details)</summary>

| Function Name | Example                                                                           | Details                                   |
|---------------|-----------------------------------------------------------------------------------|-------------------------------------------|
| `join`        | `"foo".join("bar")` -> `"foobar"`</br> borrow -> owned                            | only naively functional, work in progress | 
| `concat`      | `"foo".concat(["bar", "bat"])` -> `"foobarbat"`</br>  borrow -> borrow            | only naively functional, work in progress | 
| `append`      | `"foo".append("bar")` -> `"foobar"`</br>                         borrow -> borrow | only naively functional, work in progress |
| `prepend`     | `"foo".prepend("bar")` -> `"barfoo"`</br>                        borrow -> borrow | only naively functional, work in progress |

</details>

### Type coercion

⚠️ WIP: not yet implemented
<details>
<summary><code>trait StringTypeExtensions</code> (click to open details)</summary>

| Function Name | Example | Details                                                                                                 |
|---------------|---------|---------------------------------------------------------------------------------------------------------|
| `as_cow`      |         | Essentially free, cost only associated with mutating the string, which turns it into `Cow::Owned` state |
| `into_arc`    |         | Allocates a `String` and wraps it into an `Arc`                                                         |

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
with `fancy_regex` crate, and a custom regexless char-walking version.

The performance of these methods is evaluated using `criterion`
benchmarking library. See [benches/bench_word_bounds.rs](benches/bench_word_bounds.rs) for the benchmarking code and
try it yourself. Here are the latest results on a macbook air m1 (which shows the relational performance, while the
exacts
will of course vary by system etc.):

| Trait                         | Execution Time       | Description                                                                                                                                                                                                                                          |
|-------------------------------|----------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `WordBoundResolverRegex`      | 119.09  µs (average) | ⚠️ **Major WIP** </br>(More) Accurate, but currently ~50x <br/><br/>slower than `no_regex`. Based on prior proof-of-concepts, we should ultimately land at around ~3x slower than the charwalk variant. Suitable for non-critical performance paths. |
| `WordBoundResolverFancyRegex` | 15.433  µs (average) | 🚧 **WIP, but almost there** </br>All-inclusive regex logic including lookahead/lookback, which should be even more accurate, but ~7x slower than `no_regex`. Use only when other variants fail.                                                     |
| `WordBoundResolverCharwalk`   | 2.4 µs (average)     | ❎ **Just needs more optimization** </br>Fastest and simplest, but could fail on certain edge cases. Officially suggested method for common cases.                                                                                                    |

The `criterion` benchmark results show that `WordBoundResolverCharwalk` is the fastest yet simplest method, taking only
about
2.4 µs on average per the benchmarking execution. The regex variants can be more accurate, and their logic is
using a tried and
tested framework, but they are significantly more expensive to run; the `WordBoundResolverRegex` that has no integrated
lookahead/lookback features, replaces this absence with a custom post-process pass, and should be about 3 times slower
than the
`WordBoundResolverCharwalk` variant (⚠️ *but is under construction and while it passes the tests, it's 50x slower at
the moment* ⚠️). The
`WordBoundResolverFancyRegex` which makes use of the regex
engine for all of
its logic (including
lookahead/lookback), is more than 7 times slower than the `WordBoundResolverCharwalk` variant, though should yield
the most accurate results.

> Note: The regex variants are somewhat optimized, and in addition the crate has two different focuses for
> optimizations with
> the feature flags
`optimize_for_cpu` and
`optimize_for_memory`. This is mostly relevant for someone doing extreme and picky optimizations on a larger project,
> otherwise one should stick to the defaults. The
> default configuration for optimizations bring the heaviest one, `fancy_regex` variant, down from around the 40 micro
> second range to its current ~15 micro second range (with the same system as for the above benchmark results).

The official suggestion is to use `WordBoundResolverCharwalk` (i.e neither `use_regex`
nor `use_fancy_regex` features are enabled),
unless you face an edge case that isn't covered yet in the manual parsing logic. After that, you should test whether
`WordBoundResolverRegex` works, and if not, try `WordBoundResolverFancyRegex`.

> Note: Ultimately the costs are not usually all that significant, since this
> shouldn't be called in any hot loops, but your mileage may vary. Any and all issues and pull requests are welcome,
> if you face an edge case that isn't covered on the `WordBoundResolverCharwalk` variant.

## Example

TODO

## The Problem

TODO

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying
me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/str_extensions/blob/master/LICENSE)

This project is licensed under the terms of the **MIT** license.
