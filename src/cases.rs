use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use word_bounds::impls::charwalk::Charwalk;
use word_bounds::resolver::WordBoundResolver;
use word_bounds::rules::DefaultRules;

#[cfg(any(
    feature = "to_snake_case",
    feature = "to_camel_case",
    feature = "to_pascal_case",
    feature = "to_kebab_case",
    feature = "to_human_readable",
    feature = "to_title_case"
))]
/// Segments the input into words and drops the segments that carry no alphanumeric character.
///
/// Case conversion normalises an identifier, so a leading underscore in `_PrependedUnderscore` is
/// a separator rather than a word: keeping it would emit a doubled separator in the output.
/// Punctuation that belongs to a word, such as the `#` in `#rust`, stays attached and survives.
fn words_of(input: &str) -> Vec<String> {
    WordBoundResolver::<Charwalk, DefaultRules>::resolve(input)
        .into_iter()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .collect()
}

#[cfg(any(
    feature = "to_camel_case",
    feature = "to_pascal_case",
    feature = "to_title_case"
))]
/// Uppercases the first character and leaves the rest as the segmentation produced it.
fn capitalised(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(any(
    feature = "to_snake_case",
    feature = "to_kebab_case",
    feature = "to_human_readable"
))]
fn joined_with(input: &str, separator: &str) -> String {
    words_of(input).join(separator)
}

pub trait CaseConversions<'a, TOut: AsRef<str>>: AsRef<str> {
    /// `this_is_an_example`
    #[cfg(feature = "to_snake_case")]
    fn to_snake_case(&self) -> TOut;
    /// `thisIsAnExample`
    #[cfg(feature = "to_camel_case")]
    fn to_camel_case(&self) -> TOut;
    /// `ThisIsAnExample`
    #[cfg(feature = "to_pascal_case")]
    fn to_pascal_case(&self) -> TOut;
    /// `this-is-an-example`
    #[cfg(feature = "to_kebab_case")]
    fn to_kebab_case(&self) -> TOut;
    /// `this is an example`
    #[cfg(feature = "to_human_readable")]
    fn to_human_readable(&self) -> TOut;
    /// `This Is An Example`
    #[cfg(feature = "to_title_case")]
    fn to_title_case(&self) -> TOut;
}

impl<'a, TIn: AsRef<str>> CaseConversions<'a, Cow<'a, str>> for TIn {
    #[cfg(feature = "to_snake_case")]
    fn to_snake_case(&self) -> Cow<'a, str> {
        Cow::Owned(joined_with(self.as_ref(), "_"))
    }

    #[cfg(feature = "to_camel_case")]
    fn to_camel_case(&self) -> Cow<'a, str> {
        let words = words_of(self.as_ref());
        let mut out = String::new();
        for (idx, word) in words.iter().enumerate() {
            // the leading word stays lowercase, which is the only thing separating this from pascal
            if idx == 0 {
                out.push_str(word);
            } else {
                out.push_str(&capitalised(word));
            }
        }
        Cow::Owned(out)
    }

    #[cfg(feature = "to_pascal_case")]
    fn to_pascal_case(&self) -> Cow<'a, str> {
        let out: String = words_of(self.as_ref())
            .iter()
            .map(|word| capitalised(word))
            .collect();
        Cow::Owned(out)
    }

    #[cfg(feature = "to_kebab_case")]
    fn to_kebab_case(&self) -> Cow<'a, str> {
        Cow::Owned(joined_with(self.as_ref(), "-"))
    }

    #[cfg(feature = "to_human_readable")]
    fn to_human_readable(&self) -> Cow<'a, str> {
        Cow::Owned(joined_with(self.as_ref(), " "))
    }

    #[cfg(feature = "to_title_case")]
    fn to_title_case(&self) -> Cow<'a, str> {
        let out: Vec<String> = words_of(self.as_ref())
            .iter()
            .map(|word| capitalised(word))
            .collect();
        Cow::Owned(out.join(" "))
    }
}
