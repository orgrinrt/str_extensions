str_extensions
============
<div style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/str_extensions)](https://crates.io/crates/str_extensions)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/str_extensions.svg)](https://github.com/orgrinrt/str_extensions/issues)
[![Current Version](https://img.shields.io/badge/version-0.0.1-red.svg)](https://github.com/orgrinrt/str_extensions)

> Useful extension methods for strings in Rust, configurable by feature flags to minimise its footprint

</div>

## Usage

The methods take in a `&str` parameter, which means you can simply chain a call after either a variable
of the type or a literal.

### Extensions

#### Formatting/cases

| Function Name       | Example               | Details                                      |
|---------------------|-----------------------|----------------------------------------------|
| `to_snake_case`     | `this_is_an_example`  |                                              |
| `to_camel_case`     | `ThisIsAnExample`     | (or is it the other way around with pascal?) |
| `to_pascal_case`    | `thisIsAnExample`     | (or is it the other way around with camel?)  |
| `to_human_readable` | `This is an example.` |                                              |

#### Building

| Function Name | Example | Details                             |
|---------------|---------|-------------------------------------|
| `join`        |         | joins another string to this string |
| `append`      |         |                                     |
| `prepend`     |         |                                     |

#### Type coercion

| Function Name | Example | Details                                                         |
|---------------|---------|-----------------------------------------------------------------|
| `as_cow`      |         | if this is extremely cheap, keep as_, otherwise change to into_ |
| `as_arc`      |         | if this is extremely cheap, keep as_, otherwise change to into_ |

### Naming conventions

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
