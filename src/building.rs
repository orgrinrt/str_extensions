/// Putting strings together.
///
/// `join` used to be here doing what `append` does: `append` was written as
/// `self.join(other)`, so the two were one function under two names. The name is back, on
/// the behaviour it means everywhere else. `[a, b].join(", ")` puts a separator *between*
/// things, and this crate's own case converters join words that way, so `"a".join("b")`
/// returning `"ab"` read as a bug at every call site that was working as designed.
///
/// `append` is the concatenation. `join` is the separator form, and it is the one thing
/// this module could do internally and could not be asked for from outside.
/// The feature flags gate the methods, which they did not used to.
///
/// `append`, `prepend` and `join` were three flags that between them decided only whether
/// this module existed: turning two off and one on gave you all of the methods anyway. A
/// flag that selects nothing is a promise the manifest makes and the code does not keep,
/// and the crate's own description calls itself "extensively configurable by feature
/// flags".
use alloc::string::String;
pub trait StringBuilding {
    /// This string followed by `other`.
    #[cfg(feature = "append")]
    fn append(&self, other: &str) -> String;
    /// `other` followed by this string.
    #[cfg(feature = "prepend")]
    fn prepend(&self, other: &str) -> String;
    /// This string followed by each of `others`, in order.
    #[cfg(feature = "concat")]
    fn concat(&self, others: &[&str]) -> String;
    /// This string and each of `others`, with `separator` between each pair.
    ///
    /// The separator goes *between* elements and not after the last, which is what the
    /// name means on a slice and what it means here.
    ///
    /// ```
    /// use str_extensions::prelude::*;
    ///
    /// assert_eq!("a".join(&["b", "c"], ", "), "a, b, c");
    /// assert_eq!("alone".join(&[], ", "), "alone", "nothing to separate from");
    /// assert_eq!("a".join(&["b"], ""), "ab", "an empty separator is a concatenation");
    /// ```
    #[cfg(feature = "join")]
    fn join(&self, others: &[&str], separator: &str) -> String;
}

impl StringBuilding for str {
    #[cfg(feature = "append")]
    fn append(&self, other: &str) -> String {
        let mut s = String::with_capacity(self.len() + other.len());
        s.push_str(self);
        s.push_str(other);
        s
    }

    #[cfg(feature = "prepend")]
    fn prepend(&self, other: &str) -> String {
        // Written out rather than calling `append`, which may not be compiled in.
        let mut s = String::with_capacity(self.len() + other.len());
        s.push_str(other);
        s.push_str(self);
        s
    }

    #[cfg(feature = "join")]
    fn join(&self, others: &[&str], separator: &str) -> String {
        // One allocation, sized before anything is written: every piece, plus one
        // separator for each gap, and a gap exists between pairs rather than after each
        // element.
        let separators = others.len() * separator.len();
        let total = self.len() + others.iter().map(|o| o.len()).sum::<usize>() + separators;
        let mut s = String::with_capacity(total);
        s.push_str(self);
        for other in others {
            s.push_str(separator);
            s.push_str(other);
        }
        s
    }

    #[cfg(feature = "concat")]
    fn concat(&self, others: &[&str]) -> String {
        // One allocation rather than one per element. `concat` used to call `append` in a
        // loop, and `append` returns a fresh `String`, so joining n pieces allocated n
        // times and copied the accumulated prefix each time.
        let total = self.len() + others.iter().map(|o| o.len()).sum::<usize>();
        let mut s = String::with_capacity(total);
        s.push_str(self);
        for other in others {
            s.push_str(other);
        }
        s
    }
}
