//! Turning a string slice into the shapes a caller wants to hold it in.

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::sync::Arc;

pub trait TypeCoercion {
    /// This slice as a [`Cow`], borrowed.
    ///
    /// Never allocates. It is the shape a function takes when it may or may not need to
    /// modify what it was given, and this is the "did not need to" side of it.
    #[cfg(feature = "as_cow")]
    fn as_cow(&self) -> Cow<'_, str>;

    /// This slice in an [`Arc`], as `Arc<String>`.
    ///
    /// Two allocations and two hops to read a byte: the `Arc` holds a `String`, which holds
    /// the buffer. [`to_arc`](Self::to_arc) is one of each and is what most callers want.
    /// This one is for a caller that will later reach for `Arc::make_mut` and needs the
    /// `String` to grow into.
    ///
    /// The name says `into_` and takes `&self`, which is the opposite of what the
    /// convention means by it. Kept, because it is the name that exists, and the lint is
    /// silenced here rather than the method being renamed under anyone using it.
    #[cfg(feature = "into_arc")]
    #[allow(clippy::wrong_self_convention)]
    fn into_arc(&self) -> Arc<String>;

    /// This slice in an [`Arc`], as `Arc<str>`.
    ///
    /// One allocation and one hop. Prefer this to [`into_arc`](Self::into_arc) unless the
    /// contents have to be mutable later.
    ///
    /// ```
    /// use str_extensions::prelude::*;
    ///
    /// let shared = "shared".to_arc();
    /// let second = shared.clone();
    /// assert_eq!(&*second, "shared");
    /// ```
    #[cfg(feature = "to_arc")]
    fn to_arc(&self) -> Arc<str>;
}

impl TypeCoercion for str {
    #[cfg(feature = "as_cow")]
    fn as_cow(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }

    #[cfg(feature = "into_arc")]
    #[allow(clippy::wrong_self_convention)]
    fn into_arc(&self) -> Arc<String> {
        Arc::new(self.to_string())
    }

    #[cfg(feature = "to_arc")]
    fn to_arc(&self) -> Arc<str> {
        // `From<&str> for Arc<str>` copies the bytes straight into the allocation the `Arc`
        // makes. Going through a `String` would allocate once for the `String` and again to
        // move it in.
        Arc::from(self)
    }
}
