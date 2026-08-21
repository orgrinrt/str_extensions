//! Putting strings together, and turning them into other shapes.
//!
//! Neither module had a test. The case conversions have eleven, so the gap was not that
//! nobody wrote tests here, it was that these two looked too simple to need any.
//!
//! One of them was not. `append` and `join` did the same thing under two names, and `join`
//! is a name that already means something else: `[a, b].join(", ")` puts a separator
//! between things, and this crate's own `joined_with` uses it that way. A test naming both
//! would have made the duplication visible on the day it was written.

use std::borrow::Cow;

use str_extensions::prelude::*;

#[test]
fn append_puts_the_other_string_after_this_one() {
    assert_eq!("config".append(".toml"), "config.toml");
}

#[test]
fn prepend_puts_it_before() {
    assert_eq!("toml".prepend("config."), "config.toml");
    // The two are each other's mirror, which is the only thing that makes `prepend` worth
    // having rather than swapping the arguments at the call site.
    assert_eq!("b".prepend("a"), "a".append("b"));
}

#[test]
fn concat_appends_each_in_order() {
    assert_eq!("a".concat(&["b", "c", "d"]), "abcd");
}

#[test]
fn concat_agrees_with_repeated_appends() {
    // `concat` builds into one allocation rather than calling `append` in a loop, so this
    // is what says the faster form still produces the same string.
    let pieces = ["one", "two", "three", "four"];
    let by_concat = "start".concat(&pieces);
    let by_append = pieces
        .iter()
        .fold(String::from("start"), |acc, p| acc.append(p));
    assert_eq!(by_concat, by_append);
}

#[test]
fn the_empty_cases_are_the_identity() {
    assert_eq!("a".append(""), "a");
    assert_eq!("".append("a"), "a");
    assert_eq!("a".prepend(""), "a");
    assert_eq!("a".concat(&[]), "a");
    assert_eq!("".concat(&["a"]), "a");
    assert_eq!("".concat(&[]), "");
}

#[test]
fn multibyte_characters_survive() {
    // Every one of these counts bytes for its capacity and copies by `push_str`, so a
    // character that is more than one byte is worth naming.
    assert_eq!("mä".append("ki"), "mäki");
    assert_eq!("日".concat(&["本", "語"]), "日本語");
    assert_eq!("→".prepend("←"), "←→");
}

#[test]
fn as_cow_borrows_rather_than_copying() {
    let owned = String::from("borrowed, not copied");
    let cow = owned.as_str().as_cow();
    assert!(
        matches!(cow, Cow::Borrowed(_)),
        "as_cow borrows; got {cow:?}"
    );
    assert_eq!(cow, "borrowed, not copied");
}

#[test]
fn into_arc_gives_a_shareable_copy() {
    let arc = "shared".into_arc();
    assert_eq!(*arc, "shared");

    let second = arc.clone();
    assert_eq!(
        std::sync::Arc::strong_count(&arc),
        2,
        "cloning shares rather than copies"
    );
    assert_eq!(*second, "shared");
}

#[test]
fn an_arc_of_a_string_crosses_a_thread() {
    // Which is the reason to want one.
    let arc = "carried across".into_arc();
    let moved = arc.clone();
    let seen = std::thread::spawn(move || moved.to_string())
        .join()
        .expect("no panic");
    assert_eq!(seen, "carried across");
}

// --- join, which puts the separator between and not after --------------------------------

#[test]
#[cfg(feature = "join")]
fn join_puts_the_separator_between_each_pair() {
    assert_eq!("a".join(&["b", "c"], ", "), "a, b, c");
}

#[test]
#[cfg(feature = "join")]
fn join_with_nothing_to_join_to_is_the_string_itself() {
    // No separator, because a separator goes between two things and there is one thing.
    assert_eq!("alone".join(&[], ", "), "alone");
}

#[test]
#[cfg(feature = "join")]
fn join_does_not_put_a_separator_after_the_last() {
    // The property the name carries and the old `join` did not have. That one was `append`
    // under another name, so `"a".join("b")` gave `"ab"`.
    let joined = "one".join(&["two", "three"], "-");
    assert_eq!(joined, "one-two-three");
    assert!(!joined.ends_with('-'), "nothing trails the last element");
}

#[test]
#[cfg(all(feature = "join", feature = "concat"))]
fn an_empty_separator_makes_join_agree_with_concat_exactly() {
    assert_eq!("a".join(&["b", "c"], ""), "abc");
    assert_eq!("a".join(&["b", "c"], ""), "a".concat(&["b", "c"]));
}

#[test]
#[cfg(feature = "join")]
fn join_carries_multibyte_separators_and_elements() {
    assert_eq!("日".join(&["本", "語"], "・"), "日・本・語");
    assert_eq!("a".join(&["é"], "…"), "a…é");
}

#[test]
#[cfg(feature = "join")]
fn join_sizes_its_allocation_for_the_exact_length() {
    // The size is computed before anything is written: every piece, plus one separator per
    // gap. Wrong arithmetic still produces the right string, so the length is what says the
    // count of gaps was right.
    let joined = "aa".join(&["bbb", "c"], "--");
    assert_eq!(joined, "aa--bbb--c");
    assert_eq!(joined.len(), 2 + 2 + 3 + 2 + 1);
}
