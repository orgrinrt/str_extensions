//! The case conversions, checked against the examples the readme states for each one.
//!
//! These segment the input with `word_bounds` rather than looking for one separator each, so an
//! input written in any of the conventions converts to any of the others.

use str_extensions::prelude::*;

/// Every convention the readme documents, spelled the same way in each of them.
const EQUIVALENT_SPELLINGS: &[&str] = &[
    "ThisIsAnExample",
    "thisIsAnExample",
    "this_is_an_example",
    "this-is-an-example",
    "this is an example",
    "This Is An Example",
];

#[test]
fn every_spelling_converts_to_snake_case() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(
            input.to_snake_case(),
            "this_is_an_example",
            "input: {input}"
        );
    }
}

#[test]
fn every_spelling_converts_to_camel_case() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(input.to_camel_case(), "thisIsAnExample", "input: {input}");
    }
}

#[test]
fn every_spelling_converts_to_pascal_case() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(input.to_pascal_case(), "ThisIsAnExample", "input: {input}");
    }
}

#[test]
fn every_spelling_converts_to_kebab_case() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(
            input.to_kebab_case(),
            "this-is-an-example",
            "input: {input}"
        );
    }
}

#[test]
fn every_spelling_converts_to_human_readable() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(
            input.to_human_readable(),
            "this is an example",
            "input: {input}"
        );
    }
}

#[test]
fn every_spelling_converts_to_title_case() {
    for input in EQUIVALENT_SPELLINGS {
        assert_eq!(
            input.to_title_case(),
            "This Is An Example",
            "input: {input}"
        );
    }
}

/// Whitespace-separated input used to pass through untouched, because the old implementation only
/// looked for a change of case.
#[test]
fn separators_other_than_case_change_are_word_bounds() {
    assert_eq!("foo bar".to_snake_case(), "foo_bar");
    assert_eq!("foo-bar".to_snake_case(), "foo_bar");
    assert_eq!("foo_bar".to_kebab_case(), "foo-bar");
}

/// An acronym is one word, not one word per capital.
#[test]
fn acronyms_stay_whole() {
    assert_eq!("JSONResponse".to_snake_case(), "json_response");
    assert_eq!("someHTML".to_snake_case(), "some_html");
    assert_eq!(
        "thisExampleHasIDELikeACRONYMS".to_snake_case(),
        "this_example_has_ide_like_acronyms"
    );
}

/// Digits bound a word of their own.
#[test]
fn numbers_are_their_own_word() {
    assert_eq!(
        "WordWithNumbers123".to_snake_case(),
        "word_with_numbers_123"
    );
    assert_eq!("Short1".to_kebab_case(), "short-1");
}

/// A separator that carries no letters or digits is dropped rather than doubled.
#[test]
fn leading_separators_do_not_double_up() {
    assert_eq!(
        "_PrependedUnderscore".to_snake_case(),
        "prepended_underscore"
    );
    assert_eq!("AppendedUnderscore_".to_snake_case(), "appended_underscore");
}

#[test]
fn empty_input_stays_empty() {
    assert_eq!("".to_snake_case(), "");
    assert_eq!("".to_pascal_case(), "");
    assert_eq!("".to_title_case(), "");
}
