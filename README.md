str_extensions
============
<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/issues)
[![Current Version](https://img.shields.io/badge/version-0.0.2-red.svg)](https://github.com/orgrinrt/str_extensions)

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

Each of the four produces the result its row states. What is provisional is the surface, not the
behaviour: these are naive implementations that build a new `String` on every call.
<details>
<summary><code>trait StringBuilding</code> (click to open details)</summary>

| Function Name | Example                                                                  | Details                                   |
|---------------|---------------------------------------------------------------------------|-------------------------------------------|
| `append`      | `"foo".append("bar")` -> `"foobar"`<br/> borrow -> owned                  | one allocation                            |
| `prepend`     | `"foo".prepend("bar")` -> `"barfoo"`<br/> borrow -> owned                 | one allocation                            |
| `concat`      | `"foo".concat(&["bar", "bat"])` -> `"foobarbat"`<br/> borrow -> owned     | one allocation for the whole result       |

</details>

### Type coercion

WIP: the conversions are implemented, the surface is not settled.
<details>
<summary><code>trait TypeCoercion</code> (click to open details)</summary>

| Function Name | Example | Details                                                                                                |
|---------------|---------|----------------------------------------------------------------------------------------------------------|
| `as_cow`      |         | Free; cost only applies when mutating the string, which turns it into `Cow::Owned` state              |
| `into_arc`    |         | Allocates a `String` and wraps it into an `Arc`. Two allocations and two hops to read a byte           |
| `to_arc`      |         | One allocation and one hop, as `Arc<str>`. Prefer it unless the contents have to be mutable later      |

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
depends on. The benchmarks for its three implementations live there, with the code they measure.

The feature flags that select an implementation are forwarded to that crate, so `use_regex` and
`use_fancy_regex` mean the same thing here as they do there. `WordBoundResolver` defaults to
`Charwalk` regardless of which are compiled in, and `Charwalk` is the suggested one: it is the
fastest, and the only implementation that handles punctuation runs and variation-selector emoji.

`optimize_for_cpu` and `optimize_for_memory` are forwarded the same way. They are mutually
exclusive, and the default is `optimize_for_cpu`.

## Example

One name, carried through every spelling a codebase asks for. The conversions resolve word bounds
rather than splitting on a delimiter, which is why `UserProfileSettings` comes apart correctly
without being told where its words are.

```rust
use std::borrow::Cow;
use str_extensions::prelude::*;

// one concept, six spellings
let ty: Cow<str> = "user profile settings".to_pascal_case();
assert_eq!(ty, "UserProfileSettings");

let field: Cow<str> = "UserProfileSettings".to_snake_case();
assert_eq!(field, "user_profile_settings");

let flag: Cow<str> = "UserProfileSettings".to_kebab_case();
assert_eq!(flag, "user-profile-settings");

let js: Cow<str> = "user_profile_settings".to_camel_case();
assert_eq!(js, "userProfileSettings");

let label: Cow<str> = "user_profile_settings".to_title_case();
assert_eq!(label, "User Profile Settings");

let prose: Cow<str> = "UserProfileSettings".to_human_readable();
assert_eq!(prose, "user profile settings");

// building, implemented on `str`
assert_eq!("app".append(".toml"), "app.toml");
assert_eq!(".toml".prepend("app"), "app.toml");
assert_eq!("app".concat(&[".", "toml"]), "app.toml");
assert_eq!("a".join(&["b", "c"], ", "), "a, b, c");
```

Every assertion above was run against the crate rather than written from the method names.

`concat` takes several pieces in one allocation. `join` puts a separator between each pair
and never after the last, so `"alone".join(&[], ", ")` is `"alone"`.

Each flag under `full_building` selects its own method, so a build asking for `append` alone
gets `append` alone. Both directions are checked: `tests/feature_matrix.rs` builds a
throwaway consumer against each selection and asserts that a method whose flag is off is
genuinely absent, with the positive case beside it as the control.

## Allocation

Three positions, each a feature, and each of them built by `tests/feature_matrix.rs`.

| Feature | What is available |
|---|---|
| default | Everything, against `std`. |
| `no_std` | The same methods, against `alloc`. Every one of them returns something owned, so `alloc` is what they need. |
| `no_alloc` | Adds `write_case`, which writes the converted text into storage the caller lends and allocates nothing. |

`no_alloc` implies `no_std` and does not take the allocating conversions away.

The allocating conversions segment into a `Vec<String>` and build a second string out of it,
so a snake-case conversion is one allocation per word plus one for the vector plus one for
the answer. `write_case` writes the answer as the words arrive: it implements
`word_bounds`' word sink, so the separators and the capitals go in at the moments the
segmentation reaches them.

The block below is `ignore`, because a doctest compiles under whatever selection
`cargo test --doc` ran with and this one needs `no_alloc`. The same two assertions are
checked in `src/lending.rs`, where the doctest runs in the selection that has it.

```rust,ignore
use str_extensions::lending::{write_case, Case};

let mut out = [0u8; 64];

assert_eq!(write_case("someHTTPRequest_id", Case::Snake, &mut out).unwrap(), "some_http_request_id");
assert_eq!(write_case("someHTTPRequest_id", Case::Title, &mut out).unwrap(), "Some Http Request Id");
```

A lend too small refuses rather than handing back a short answer, and says how much was
wanted against how much was there, so doubling from `wanted` converges.

Both sides drop a segment carrying no alphanumeric character, which is why
`_PrependedUnderscore` becomes `prepended_underscore` rather than `_prepended_underscore`.
The lending side writes each separator before it knows whether the word after it survives,
and takes both back when it does not. `tests/lending_parity.rs` runs 39 inputs through both
sides in all six cases and compares.

## Examples

```text
cargo run --example case_conversions
cargo run --example no_allocator_at_all --no-default-features --features no_alloc,full_format
```

The first is the six conversions over one input, then the same answer reached from five
different spellings of it. The second is a code generator with no allocator anywhere:
`word_bounds` for the segmentation, this crate's sink for the case, notko's lending
contract for the storage, and every buffer a fixed array. Both are run by `cargo test`, in
`tests/examples_run.rs`, which checks what they print rather than only that they built.

## The Problem

Rust gives you `to_uppercase` and `to_lowercase` and stops. Everything between a display label and an
identifier is left to the caller, and the caller reaches for `split('_')`, which is wrong the moment
the input is already camel case.

Splitting on a delimiter cannot round-trip. `to_snake_case` on `UserProfileSettings` has no delimiter
to split on, and `to_pascal_case` on `user profile settings` has a different one. The conversions here
resolve word bounds first, through [`word_bounds`](https://github.com/orgrinrt/word_bounds), so any
spelling converts to any other from any starting point.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying
me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> You can check out the full license [here](https://github.com/orgrinrt/str_extensions/blob/main/LICENSE)

This project is licensed under the terms of the **MPL-2.0** license.
