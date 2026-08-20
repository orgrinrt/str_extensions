/// Putting strings together.
///
/// There was a `join` here as well, and it did what `append` does. Two names for one
/// behaviour is the smaller half of the problem: the larger half is that `join` already
/// means something else a few lines away. `[a, b].join(", ")` puts a separator *between*
/// things, and this crate's own `joined_with` uses it that way, so `"a".join("b")`
/// returning `"ab"` reads as a bug at every call site that is in fact working as designed.
///
/// It is gone. `append` says what it does.
/// The feature flags gate the methods, which they did not used to.
///
/// `append`, `prepend` and `join` were three flags that between them decided only whether
/// this module existed: turning two off and one on gave you all of the methods anyway. A
/// flag that selects nothing is a promise the manifest makes and the code does not keep,
/// and the crate's own description calls itself "extensively configurable by feature
/// flags".
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
