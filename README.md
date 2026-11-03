# `str_extensions`

> String extension methods, each behind a feature flag so you only carry what you use.

`str_extensions` adds methods to `str` and `AsRef<str>`, in groups that each sit behind a feature. The case conversions work for anything that is `AsRef<str>`, so `String`, `Cow<str>` and `&str` all have them, and they segment the input with [`word_bounds`](https://github.com/orgrinrt/word_bounds) and don't look for one separator each, which means an input in any naming convention converts to any other. Acronyms stay whole, so `JSONResponse` gives `json_response`, digits bound a word of their own, and a segment with no letter or digit is dropped since a leading underscore is a separator and not a word.

## Usage

Not published to crates.io yet, so it is a git dependency for now.

```toml
[dependencies]
str_extensions = { git = "https://github.com/orgrinrt/str_extensions", branch = "dev" }
```

```rust
use std::borrow::Cow;
use str_extensions::prelude::*;

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
```

The groups are `CaseConversions` with `to_snake_case`, `to_camel_case`, `to_pascal_case`, `to_kebab_case`, `to_title_case` and `to_human_readable`, `StringBuilding` with `append`, `prepend`, `concat` and `join`, and `TypeCoercion` with `as_cow`, `into_arc` and `to_arc`. The building methods are naive and build a new `String` per call, and `to_arc` is one allocation where `into_arc` is two, so prefer it unless the contents need to be mutable later. The coercion surface isn't settled. Names follow the rust api guidelines, `as_` is free, `to_` is expensive and `into_` varies.

```rust,ignore
use str_extensions::prelude::*;

assert_eq!("app".append(".toml"), "app.toml");
assert_eq!(".toml".prepend("app"), "app.toml");
assert_eq!("a".join(&["b", "c"], ", "), "a, b, c");
```

That block is `ignore` because a doctest compiles under whichever selection `cargo test --doc` ran with and can't say which features it needs. The building methods are checked in `tests/building.rs`, and `tests/feature_matrix.rs` runs the doc suite under every selection named here.

## Features

Segmentation is `word_bounds`, and `use_regex`, `use_fancy_regex`, `optimize_for_cpu` and `optimize_for_memory` are forwarded to it, with `Charwalk` the default implementation regardless of which are compiled in. Each flag under `full_building` selects its own method, so a build asking for `append` alone gets `append` alone.

`no_std` gives the same methods against `alloc`, and `no_alloc` implies it and adds `write_case`, which writes the converted text into storage the caller lends and allocates nothing, while keeping the allocating conversions. `no_std` can't be combined with `use_regex` or `use_fancy_regex`, because their backends need std, and the build refuses that and doesn't quietly swap in the character walker, which would give different words from the same input. `use_regex` is in the default set, so a `no_std` consumer turns the defaults off, and if the refusal shows up unasked, a sibling crate enabled a feature, and `cargo tree -e features` says which.

```bash
cargo add str_extensions --no-default-features --features no_std,full_format
```

```rust,ignore
use str_extensions::lending::{write_case, Case};

let mut out = [0u8; 64];

assert_eq!(write_case("someHTTPRequest_id", Case::Snake, &mut out).unwrap(), "some_http_request_id");
assert_eq!(write_case("someHTTPRequest_id", Case::Title, &mut out).unwrap(), "Some Http Request Id");
```

A lend that is too small refuses and says how much was wanted against how much there was. `cargo run --example case_conversions` converts one input in all six cases, and `no_allocator_at_all` is a code generator with no allocator anywhere, using `word_bounds`, this crate's sink and notko's lending contract.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/str_extensions/blob/dev/LICENSE)
