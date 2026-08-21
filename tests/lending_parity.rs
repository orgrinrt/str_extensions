//! `write_case` answers the same as the allocating conversions, or it is not the same API.
//!
//! They are two different programs reaching one answer. The allocating side segments into a
//! `Vec<String>`, drops the segments carrying no alphanumeric character, and builds a second
//! string out of what is left. The lending side writes the answer as the words arrive, which
//! means it puts each separator in before it knows whether the word after it survives, and
//! has to take both back when it does not.
//!
//! Every case below goes through both, for all six conversions.

#![cfg(feature = "no_alloc")]

use str_extensions::lending::{write_case, Case};
use str_extensions::prelude::*;

/// The allocating conversion for one case.
fn via_alloc(input: &str, case: Case) -> String {
    let out: std::borrow::Cow<'_, str> = match case {
        Case::Snake => input.to_snake_case(),
        Case::Camel => input.to_camel_case(),
        Case::Pascal => input.to_pascal_case(),
        Case::Kebab => input.to_kebab_case(),
        Case::HumanReadable => input.to_human_readable(),
        Case::Title => input.to_title_case(),
    };
    out.into_owned()
}

/// The lending conversion for one case, copied out so it can be compared.
fn via_lending(input: &str, case: Case) -> String {
    // Generous, so nothing here is testing the refusal path by accident.
    let mut out = [0u8; 512];
    write_case(input, case, &mut out)
        .unwrap_or_else(|e| {
            panic!("{input:?} at {case:?} did not fit: wanted {}, had {}", e.wanted, e.had)
        })
        .to_string()
}

const ALL_CASES: &[Case] = &[
    Case::Snake,
    Case::Camel,
    Case::Pascal,
    Case::Kebab,
    Case::HumanReadable,
    Case::Title,
];

/// Everything both sides are asked.
const INPUTS: &[&str] = &[
    // The ordinary shapes.
    "someHTTPRequest_id",
    "snake_case_name",
    "camelCaseName",
    "PascalCaseName",
    "kebab-case-name",
    "SCREAMING_SNAKE",
    "Title Case Words",
    "XMLHttpRequest",
    "IOError",
    "a",
    "aB",
    "",
    // Where the dropped-segment filter fires, which is the part the two sides reach
    // differently. A leading or trailing separator is a segment with nothing alphanumeric
    // in it, and keeping it would emit a doubled separator.
    "_PrependedUnderscore",
    "trailing_",
    "_leading",
    "__both__",
    "double__underscore",
    "a__b",
    "a_-_b",
    "...",
    "   ",
    "-",
    "_",
    "a...b",
    "...leading",
    "trailing...",
    "a b  c",
    // Punctuation that belongs to a word stays attached, so these segments are not dropped.
    "#rust",
    "c++Style",
    // Digits, which the walker treats specially.
    "digits123mixed456",
    "123",
    "a1b2c3",
    "v2Release",
    // Non-ASCII, including the sigma rule, which the lending side reproduces by hand.
    "ÄÖÜ_äöü",
    "naïveRésumé",
    "ЗдравствуйМир",
    "ΟΔΟΣ",
    "ΟΔΟΣ_ΣΤΟ",
    "ΑΣ ΣΑΣ",
    "日本語テキスト",
];

#[test]
fn both_sides_agree_on_every_case_and_every_input() {
    for &input in INPUTS {
        for &case in ALL_CASES {
            let allocating = via_alloc(input, case);
            let lending = via_lending(input, case);

            assert_eq!(
                allocating, lending,
                "{input:?} at {case:?}: allocating gave {allocating:?}, lending gave {lending:?}",
            );
        }
    }
}

#[test]
fn a_dropped_segment_takes_its_separator_with_it() {
    // The control for the test above. Without a case where a segment is dropped, the
    // parity assertion would pass over a lending side that never drops one, because every
    // other input has no segment to drop.
    //
    // These are what the doubled separator would have looked like.
    assert_eq!(via_alloc("_PrependedUnderscore", Case::Snake), "prepended_underscore");
    assert_ne!(via_alloc("_PrependedUnderscore", Case::Snake), "_prepended_underscore");

    assert_eq!(via_lending("_PrependedUnderscore", Case::Snake), "prepended_underscore");
    assert_eq!(via_lending("__both__", Case::Snake), "both");
    assert_eq!(via_lending("a__b", Case::Kebab), "a-b");
}

#[test]
fn an_input_with_no_words_at_all_writes_nothing() {
    for input in ["", "   ", "...", "_", "-"] {
        for &case in ALL_CASES {
            assert_eq!(via_lending(input, case), "", "{input:?} at {case:?}");
            assert_eq!(via_alloc(input, case), "", "{input:?} at {case:?}");
        }
    }
}

#[test]
fn a_lend_too_small_refuses_with_both_numbers() {
    let mut out = [0u8; 4];
    let refused = write_case("someHTTPRequest", Case::Snake, &mut out).unwrap_err();

    assert_eq!(refused.had, 4, "it reports what it was given");
    assert!(refused.wanted > refused.had, "and that it needed more than that");

    // And it is a lower bound rather than a lie: the whole answer is longer still, since
    // the shortfall is found partway through.
    assert!(
        refused.wanted <= "some_http_request".len(),
        "wanted is a lower bound, not an invented total",
    );
}

#[test]
fn a_lend_of_exactly_the_right_size_is_enough() {
    // The boundary the refusal above sits next to. Off by one here would make the refusal
    // fire on an answer that fits, which no positive test with a generous buffer can see.
    let expected = "some_http_request";
    let mut out = vec![0u8; expected.len()];
    assert_eq!(write_case("someHTTPRequest", Case::Snake, out.as_mut_slice()).unwrap(), expected);

    let mut one_short = vec![0u8; expected.len() - 1];
    assert!(write_case("someHTTPRequest", Case::Snake, one_short.as_mut_slice()).is_err());
}
