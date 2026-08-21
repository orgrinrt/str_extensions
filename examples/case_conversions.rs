//! The six case conversions, over one input each.
//!
//! The smallest thing this crate does. Every conversion segments the input into words first,
//! so what goes in can be any case, or several mixed together, and the answer is the same.
//!
//! ```text
//! cargo run --example case_conversions
//! ```

use str_extensions::prelude::*;

fn main() {
    println!("One input, six conversions.\n");

    let input = "someHTTPRequest_id";
    println!("{:<20} {input}", "input");
    println!("{:<20} {}", "to_snake_case", input.to_snake_case());
    println!("{:<20} {}", "to_camel_case", input.to_camel_case());
    println!("{:<20} {}", "to_pascal_case", input.to_pascal_case());
    println!("{:<20} {}", "to_kebab_case", input.to_kebab_case());
    println!("{:<20} {}", "to_human_readable", input.to_human_readable());
    println!("{:<20} {}", "to_title_case", input.to_title_case());

    println!("\nThe input's own case does not matter, because it is segmented first.\n");

    // Five spellings of one identifier, all reaching one answer.
    for spelling in [
        "someHTTPRequest",
        "some_http_request",
        "SOME_HTTP_REQUEST",
        "some-http-request",
        "Some Http Request",
    ] {
        println!("{spelling:<22} -> {}", spelling.to_snake_case());
    }

    println!("\nA separator with nothing between it and the next is not a word.\n");

    // A leading underscore is a separator rather than a word, so it is dropped: keeping it
    // would put a doubled separator in the answer.
    for input in ["_PrependedUnderscore", "trailing_", "__both__", "a__b"] {
        println!("{input:<22} -> {}", input.to_snake_case());
    }
}
