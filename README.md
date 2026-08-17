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

The case conversions are implemented for all types implementing `AsRef<str>`. This covers most of the usual string
types, including `String`, `Cow<str>` and `&str` itself. The string building and type coercion methods are implemented
for `str`.

## Extensions

### Formatting, cases

The conversions segment the input with [`word_bounds`](https://github.com/orgrinrt/word_bounds)
rather than looking for one separator each, so an input written in any of the conventions converts
to any of the others. A segment that carries no letter or digit is dropped, since a leading
underscore is a separator rather than a word.

Acronyms stay whole (`JSONResponse` gives `json_response`) and digits bound a word of their own
(`WordWithNumbers123` gives `word_with_numbers_123`).
<details>
<summary><code>trait CaseConversions</code> (click to open details)</summary>

| Function Name       | Example              | Details                          |
|---------------------|-----------------------|----------------------------------|
| `to_snake_case`     | `this_is_an_example`  |                                   |
| `to_camel_case`     | `thisIsAnExample`     |                                   |
| `to_pascal_case`    | `ThisIsAnExample`     |                                   |
| `to_kebab_case`     | `this-is-an-example`  |                                   |
| `to_human_readable` | `this is an example`  |                                   |
| `to_title_case`     | `This Is An Example`  |                                   |

</details>

### String building

WIP: not all impls match their descriptions yet.
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

WIP: the conversions are implemented, the surface is not settled.
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

Word segmentation is [`word_bounds`](https://github.com/orgrinrt/word_bounds), which this crate
depends on. It used to be vendored here as a copy of the same four files, so a fix to the
segmentation rules had to be made twice. The benchmarks for the three implementations live there
with the code they measure.

The feature flags that select an implementation are forwarded to that crate, so `use_regex` and
`use_fancy_regex` still work here and mean the same thing. `WordBoundResolver` defaults to
`Charwalk` regardless of which are compiled in, and `Charwalk` is the suggested one: it is the
fastest, and the only implementation that handles punctuation runs and variation-selector emoji.

`optimize_for_cpu` and `optimize_for_memory` are forwarded the same way. They are mutually
exclusive, and the default is `optimize_for_cpu`.

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
