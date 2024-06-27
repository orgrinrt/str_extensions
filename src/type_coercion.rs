use std::borrow::Cow;
use std::sync::Arc;

// Type Coercion
pub(super) trait TypeCoercion {
    fn as_cow(&self) -> Cow<str>;
    fn into_arc(&self) -> Arc<String>;
}

impl TypeCoercion for str {
    fn as_cow(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }

    fn into_arc(&self) -> Arc<String> {
        Arc::new(self.to_string())
    }
}
