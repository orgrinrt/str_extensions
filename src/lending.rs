//! Case conversion into storage the caller lends, without allocating.
//!
//! The allocating conversions segment into a `Vec<String>` and then build a second string
//! out of it, so a snake-case conversion is one allocation per word plus one for the vector
//! plus one for the answer. This one writes the answer directly as the words arrive: the
//! separators and the capitals go in at the moments the segmentation reaches them, and
//! nothing is built to be rewritten.
//!
//! That is what [`word_bounds::sink::WordSink`] is for. The walk is word_bounds', the sink
//! is here, and the output is whatever storage was handed over.

use notko::lend::{Exhausted, Lend};
use notko::outcome::Outcome;
use word_bounds::impls::charwalk::walk;
use word_bounds::rules::DefaultRules;
use word_bounds::sink::{is_cased, WordSink, FINAL_SIGMA, NON_FINAL_SIGMA};

/// Which case to write.
///
/// The same six the allocating [`CaseConversions`](crate::cases::CaseConversions) trait
/// offers, named rather than one function each, because the only thing that differs between
/// them is a separator and whether a word's first letter is capitalised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    /// `this_is_an_example`
    Snake,
    /// `thisIsAnExample`
    Camel,
    /// `ThisIsAnExample`
    Pascal,
    /// `this-is-an-example`
    Kebab,
    /// `this is an example`
    HumanReadable,
    /// `This Is An Example`
    Title,
}

impl Case {
    /// What goes between two words, if anything.
    const fn separator(self) -> Option<char> {
        match self {
            Self::Snake => Some('_'),
            Self::Kebab => Some('-'),
            Self::HumanReadable | Self::Title => Some(' '),
            Self::Camel | Self::Pascal => None,
        }
    }

    /// Whether the word at `index` starts with a capital.
    const fn capitalises(self, index: usize) -> bool {
        match self {
            Self::Pascal | Self::Title => true,
            // The leading word stays lower case, which is the only thing separating camel
            // from pascal.
            Self::Camel => index > 0,
            Self::Snake | Self::Kebab | Self::HumanReadable => false,
        }
    }
}

/// Writes `input` into `out`, converted to `case`.
///
/// Returns the converted text as a `&str` borrowed from `out`, so nothing is copied a
/// second time to read it back.
///
/// Refuses rather than truncating. An [`Exhausted`] carries how much was wanted against how
/// much was there. What it reports is a lower bound, since the shortfall is found partway
/// through: doubling from `wanted` converges, and it is the reason both numbers are there.
///
/// `?Sized`, so a bare `&mut [u8]` out of an arena is lent directly rather than by lending
/// a reference to one.
///
/// ```
/// use str_extensions::lending::{write_case, Case};
///
/// let mut out = [0u8; 64];
///
/// assert_eq!(write_case("someHTTPRequest_id", Case::Snake, &mut out).unwrap(), "some_http_request_id");
/// ```
///
/// Every case, over one input:
///
/// ```
/// use str_extensions::lending::{write_case, Case};
///
/// let mut out = [0u8; 64];
/// for (case, expected) in [
///     (Case::Snake, "some_http_request"),
///     (Case::Camel, "someHttpRequest"),
///     (Case::Pascal, "SomeHttpRequest"),
///     (Case::Kebab, "some-http-request"),
///     (Case::HumanReadable, "some http request"),
///     (Case::Title, "Some Http Request"),
/// ] {
///     assert_eq!(write_case("someHTTPRequest", case, &mut out).unwrap(), expected);
/// }
/// ```
///
/// A lend too small says so rather than handing back a short answer:
///
/// ```
/// use str_extensions::lending::{write_case, Case};
///
/// let mut out = [0u8; 4];
/// let refused = write_case("someHTTPRequest", Case::Snake, &mut out).unwrap_err();
///
/// assert_eq!(refused.had, 4);
/// assert!(refused.wanted > refused.had);
/// ```
pub fn write_case<'a, L>(input: &str, case: Case, out: &'a mut L) -> Outcome<&'a str, Exhausted>
where
    L: Lend<u8> + ?Sized,
{
    // The slice directly rather than notko's `Fill`, because a word that turns out to carry
    // no alphanumeric character is dropped, and dropping it means rewinding to where it
    // started. `Fill` is the append half of the lending protocol and says so; something
    // that rewrites what it wrote takes the slice, which is what this does.
    let slots = out.lend();
    let mut sink = CaseSink {
        slots,
        used: 0,
        case,
        words: 0,
        word_start: 0,
        word_has_alphanumeric: false,
        held: None,
        last_cased: None,
    };

    if let Err(exhausted) = walk::<DefaultRules, _>(input, &mut sink) {
        return Outcome::Err(exhausted);
    }

    let used = sink.used;
    // Every byte came from `char::encode_utf8`, written whole, so this is valid UTF-8 by
    // construction. Checked anyway, because being wrong about that would be unsound rather
    // than merely incorrect, and the check is a scan of memory written a moment ago.
    match core::str::from_utf8(&sink.slots[.. used]) {
        Ok(text) => Outcome::Ok(text),
        // Unreachable. The sink is the only writer, every write copies `encode_utf8` output
        // whole, and both rewind targets are on character boundaries by construction: a
        // word start, or a word start less one separator's own width.
        //
        // Panicking rather than returning an `Exhausted`, which an earlier version did with
        // `wanted` equal to `had`. Those two numbers exist so a caller can double from
        // `wanted` and converge, and a shortfall of zero gives it nothing to double: it
        // would have reported a refusal that says the lend was exactly the right size,
        // which is neither true nor actionable. A refusal has to mean the thing it says.
        Err(e) => unreachable!(
            "the sink wrote a partial character at byte {}: {e}",
            e.valid_up_to(),
        ),
    }
}

/// A sink that writes the converted text as the walk produces words.
struct CaseSink<'a> {
    slots:                 &'a mut [u8],
    used:                  usize,
    case:                  Case,
    /// How many words have been committed, which decides the separator and the capital.
    words:                 usize,
    /// Where in `slots` the word being built starts, so it can be dropped whole.
    word_start:            usize,
    /// Whether anything in the word being built is alphanumeric.
    ///
    /// The allocating conversions drop a segment carrying no alphanumeric character,
    /// because a leading underscore in `_Prefixed` is a separator rather than a word and
    /// keeping it would emit a doubled separator. This is the streaming form of that
    /// filter.
    word_has_alphanumeric: bool,
    /// The character pushed most recently, not yet written.
    ///
    /// Held for the same reason word_bounds holds one: a capital sigma's lower case form
    /// depends on whether it ends the word, and that is not known when it arrives.
    held:                  Option<char>,
    /// The most recent cased character in the word being built, held or written.
    ///
    /// The rule asks for a cased character before the sigma, and `str::to_lowercase` walks
    /// backwards past `Case_Ignorable` characters looking for one. Tracking the last cased
    /// character rather than the immediately preceding one performs that skip exactly, and
    /// costs nothing: an ignorable character simply never updates this.
    last_cased:            Option<char>,
}

impl CaseSink<'_> {
    /// Writes one character's UTF-8, refusing if it does not fit.
    fn write(&mut self, c: char) -> Result<(), Exhausted> {
        let mut buf = [0u8; 4];
        let encoded = c.encode_utf8(&mut buf);
        let bytes = encoded.as_bytes();

        if self.used + bytes.len() > self.slots.len() {
            return Err(Exhausted {
                // A lower bound: what this write needed is known, and what the rest of the
                // input still needs is not.
                wanted: self.used + bytes.len(),
                had:    self.slots.len(),
            });
        }

        self.slots[self.used .. self.used + bytes.len()].copy_from_slice(bytes);
        self.used += bytes.len();
        Ok(())
    }

    /// Writes the held character, lowercased or capitalised as the position calls for.
    fn flush_held(&mut self, is_final: bool) -> Result<(), Exhausted> {
        let Some(c) = self.held.take() else {
            return Ok(());
        };

        // The first character of a word is capitalised in the cases that ask for it. It is
        // the first when nothing has been written into this word yet.
        //
        // Lowercased first, then the leading character of that uppercased. The allocating
        // path segments into lowercased words and capitalises the result, so uppercasing
        // the raw character is a different operation wherever a character's
        // lowercase-then-uppercase is not its direct uppercase. `İ` uppercases to itself
        // and lowercases to `i` plus a combining dot, whose uppercase is `I` plus that dot;
        // `ẞ` uppercases to itself and goes through `ß` to `SS`. Turkish and German, and
        // both came out wrong.
        if self.used == self.word_start && self.case.capitalises(self.words) {
            let mut lowered = c.to_lowercase();
            if let Some(first) = lowered.next() {
                for upper in first.to_uppercase() {
                    self.write(upper)?;
                }
            }
            for rest in lowered {
                self.write(rest)?;
            }
            return Ok(());
        }

        // Otherwise lowercased, with the one rule `char::to_lowercase` does not carry.
        if c == 'Σ' {
            let final_position = is_final && self.last_cased.is_some();
            return self.write(if final_position { FINAL_SIGMA } else { NON_FINAL_SIGMA });
        }

        for lowered in c.to_lowercase() {
            self.write(lowered)?;
        }
        Ok(())
    }
}

impl WordSink for CaseSink<'_> {
    type Err = Exhausted;

    fn push_char(&mut self, c: char) -> Result<(), Self::Err> {
        // Read before the flush, which takes it.
        let was_held = self.held;

        // The separator goes in before the first character of a word that is not the
        // first, which is where it belongs and is why it cannot be appended after each.
        if self.held.is_none() && self.used == self.word_start && self.words > 0 {
            if let Some(separator) = self.case.separator() {
                self.write(separator)?;
                // The separator is not part of the word, so the word starts after it and a
                // dropped word does not take it with it.
                self.word_start = self.used;
            }
        }

        self.flush_held(false)?;
        // Only a cased character updates this, which is what performs the skip past
        // `Case_Ignorable` characters that `str::to_lowercase` does.
        if was_held.is_some_and(is_cased) {
            self.last_cased = was_held;
        }
        self.held = Some(c);

        if c.is_alphanumeric() {
            self.word_has_alphanumeric = true;
        }
        Ok(())
    }

    fn pending_is_empty(&self) -> bool {
        self.used == self.word_start && self.held.is_none()
    }

    fn pending_ends_with(&self, c: char) -> bool {
        self.held == Some(c)
    }

    fn commit(&mut self) -> Result<(), Self::Err> {
        self.flush_held(true)?;

        if self.word_has_alphanumeric {
            self.words += 1;
            self.word_start = self.used;
        } else {
            // Dropped: rewind over it, and over the separator that was written for it,
            // which is why `word_start` moved past the separator when it went in.
            self.used = self.word_start;
            if self.words > 0 {
                if let Some(separator) = self.case.separator() {
                    self.used -= separator.len_utf8();
                    self.word_start = self.used;
                }
            }
        }

        self.word_has_alphanumeric = false;
        self.last_cased = None;
        Ok(())
    }
}
