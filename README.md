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

<details>
<summary><code>trait StringBuildExtensions</code> (click to open details)</summary>

| Function Name | Example                                                                           | Details                                                                                                                                                                                |
|---------------|-----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `join`        | `"foo".join("bar")` -> `"foobar"`</br> borrow -> owned                            | only naively functional, work in progress</br> has `(string)_strings`, `(string)_to_str`, `(str)_into_string` and `(str)_into_string_as_str` variants (plain join is between &strs)    | 
| `concat`      | `"foo".concat(["bar", "bat"])` -> `"foobarbat"`</br>  borrow -> borrow            | only naively functional, work in progress</br> has `(string)_strings`, `(string)_to_str`, `(str)_into_string` and `(str)_into_string_as_str` variants (plain concat is between &strs)  | 
| `append`      | `"foo".append("bar")` -> `"foobar"`</br>                         borrow -> borrow | only naively functional, work in progress</br> has `(string)_strings`, `(string)_to_str`, `(str)_into_string` and `(str)_into_string_as_str` variants (plain append is between &strs)  |
| `prepend`     | `"foo".prepend("bar")` -> `"barfoo"`</br>                        borrow -> borrow | only naively functional, work in progress</br> has `(string)_strings`, `(string)_to_str`, `(str)_into_string` and `(str)_into_string_as_str` variants (plain prepend is between &strs) |

</details>

### Type coercion

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

This repository contains code for three different methods to perform `word_bounds` operation - with standard `regex`,
with `fancy_regex`, and a no-regex version. The performance of these methods is evaluated using `criterion`
benchmarking library. See [benches/bench_word_bounds.rs](benches/bench_word_bounds.rs) for the benchmarking code and
try it yourself. Here are the latest results:

| Method                    | Execution Time      | Description                                                                                                                 |
|---------------------------|---------------------|-----------------------------------------------------------------------------------------------------------------------------|
| `word_bounds_regex`       | 16.670 µs (average) | Accurate and feature-rich, but 36x slower than `no_regex`. Suitable for non-critical performance paths.                     |
| `word_bounds_fancy_regex` | 50.482 µs (average) | All-inclusive regex logic including lookahead/lookback, but 100x slower than `no_regex`. Use only when other variants fail. |
| `word_bounds_no_regex`    | 459.17 ns (average) | Fastest and simplest, but could fail on certain edge cases. Officially suggested method for common cases.                   |

The Criterion benchmark results show that `word_bounds_no_regex` is the fastest yet simplest method, taking only about
459.17 ns on average per the benchmarking execution. The regex variants can be more accurate, and their logic is
using a tried and
tested framework, but they are significantly more expensive to run; the regular `regex` that has no integrated
lookahead/lookback features and replaces these with a post-process pass, is about 36 times slower than the
`no_regex` variant. The `fancy_regex` that makes use of the regex engine for all of its logic (including
lookahead/lookback), is more than 100 times slower than the `no_regex` variant.

The official suggestion is to use `no_regex` (i.e neither `use_regex` nor `use_fancy_regex` features are enabled),
unless you face an edge case that isn't covered yet in the manual parsing logic. After that, you should test whether
`regex` works, and if not, try `fancy_regex`.

> Note: Ultimately the costs are not usually all that significant, since this
> shouldn't be called in any hot loops, but your mileage may vary. Any and all issues and pull requests are welcome,
> if you face an edge case that isn't covered on the `no_regex` variant.

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
