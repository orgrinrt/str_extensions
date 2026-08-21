//! Renaming a batch of identifiers with no allocator anywhere in the program.
//!
//! Composes what the crate has: the segmentation from word_bounds, this crate's case
//! conversion written against word_bounds' word sink, and notko's lending contract for the
//! storage. Every buffer here is a fixed array, and nothing asks where memory came from.
//!
//! The scenario is a code generator: it is handed field names in whatever case the schema
//! used, and has to emit each in three cases, into one buffer it owns.
//!
//! ```text
//! cargo run --example no_allocator_at_all --no-default-features --features no_alloc,full_format
//! ```

use str_extensions::lending::{write_case, Case};
use str_extensions::{Exhausted, Outcome};

/// What the schema called things, in whatever case it happened to use.
const FIELDS: &[&str] = &[
    "userID",
    "created_at",
    "HTTPStatusCode",
    "is-active",
    "Last Login Time",
    "_internalFlag",
];

fn main() {
    println!("A code generator with no allocator, emitting three cases per field.\n");

    // One buffer, reused for every conversion. The borrow ends when the `&str` does, so
    // the next call gets it back.
    let mut scratch = [0u8; 64];

    println!(
        "{:<18} {:<20} {:<20} {}",
        "schema", "field", "getter", "constant"
    );
    for &field in FIELDS {
        // Each conversion borrows the buffer, is printed, and gives it back. Holding two
        // at once would need two buffers, which is the honest cost of not allocating and is
        // what the borrow checker is saying when it refuses.
        print!("{field:<18} ");
        print!("{:<20} ", one(&mut scratch, field, Case::Snake));
        print!("{:<20} ", one(&mut scratch, field, Case::Camel));
        println!("{}", shouty(&mut scratch, field));
    }

    println!("\nA buffer too small refuses, and says how much it wanted.\n");

    // Sized for the shortest answer and asked for the longest.
    let mut cramped = [0u8; 8];
    match write_case("HTTPStatusCode", Case::HumanReadable, &mut cramped) {
        Outcome::Ok(text) => println!("unexpectedly fitted: {text}"),
        Outcome::Err(Exhausted {
            wanted,
            had,
        }) => {
            println!("refused: wanted at least {wanted} bytes, had {had}");

            // Doubling from `wanted` converges, which is why both numbers are carried
            // rather than only the failure.
            let mut second_try = [0u8; 32];
            let size = second_try.len();
            assert!(size >= wanted);
            // The length is read before the call, because the call borrows the buffer for
            // as long as the answer lives and the answer is what is being printed.
            match write_case("HTTPStatusCode", Case::HumanReadable, &mut second_try) {
                Outcome::Ok(text) => println!("at {size} bytes: {text}"),
                Outcome::Err(e) => println!("still short at {size}: wanted {}", e.wanted),
            }
        },
    }
}

/// One conversion, copied into a fixed array so the caller can hold several at once.
///
/// Returns an array rather than a `&str`, because a `&str` would keep the scratch buffer
/// borrowed and the point here is to print three answers on one line.
fn one(scratch: &mut [u8; 64], input: &str, case: Case) -> Shown {
    let text = write_case(input, case, scratch.as_mut_slice())
        .expect("64 bytes is enough for a field name");
    Shown::of(text)
}

/// A snake-case conversion uppercased, which is what a generated constant name looks like.
fn shouty(scratch: &mut [u8; 64], input: &str) -> Shown {
    let text = write_case(input, Case::Snake, scratch.as_mut_slice())
        .expect("64 bytes is enough for a field name");

    // Uppercasing ASCII in place, since the answer is already in a buffer that is ours.
    let mut shown = Shown::of(text);
    shown.bytes[..shown.len].make_ascii_uppercase();
    shown
}

/// A short answer copied out of the scratch buffer, so the buffer is free again.
struct Shown {
    bytes: [u8; 64],
    len: usize,
}

impl Shown {
    fn of(text: &str) -> Self {
        let mut bytes = [0u8; 64];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Self {
            bytes,
            len: text.len(),
        }
    }
}

impl core::fmt::Display for Shown {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Written whole by `write_case` and only ASCII-uppercased since, so this is valid
        // UTF-8. Checked rather than assumed, and a `Display` impl has nowhere to report a
        // failure to anyway.
        //
        // `pad` rather than `write_str`, which ignores the width in the format string and
        // makes every `{:<20}` here do nothing.
        f.pad(core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("<invalid>"))
    }
}
