use std::borrow::Cow;

pub(super) trait CaseConversions<'a, TOut: AsRef<str>>: AsRef<str> {
    fn to_snake_case(&self) -> TOut;
    fn to_camel_case(&self) -> TOut;
    fn to_pascal_case(&self) -> TOut;
    fn to_kebab_case(&self) -> TOut;
    fn to_human_readable(&self) -> TOut;
    fn to_title_case(&self) -> TOut;
}

impl<'a, TIn: AsRef<str>> CaseConversions<'a, Cow<'a, str>> for TIn {
    fn to_snake_case(&self) -> Cow<'a, str> {
        let text = self.as_ref();

        let mut buffer = String::with_capacity(text.len() + text.len() / 2);

        let mut text = text.chars();

        if let Some(first) = text.next() {
            let mut n2: Option<(bool, char)> = None;
            let mut n1: (bool, char) = (first.is_lowercase(), first);

            for c in text {
                let prev_n1 = n1.clone();

                let n3 = n2;
                n2 = Some(n1);
                n1 = (c.is_lowercase(), c);

                // insert underscore if an acronym is at the beginning, ABc -> a_bc
                if let Some((n3_is_uppercase, c3)) = n3 {
                    if let Some((n2_is_uppercase, c2)) = n2 {
                        if n1.0 && c3.is_uppercase() && c2.is_uppercase() {
                            buffer.push('_');
                        }
                    }
                }

                buffer.push_str(&prev_n1.1.to_lowercase().to_string());

                // insert underscore before the next word, abC -> ab_c
                if let Some((n2_is_uppercase, _)) = n2 {
                    if n1.1.is_uppercase() {
                        buffer.push('_');
                    }
                }
            }

            buffer.push_str(&n1.1.to_lowercase().to_string());
        }
        Cow::Owned(buffer)
    }

    fn to_camel_case(&self) -> Cow<'a, str> {
        let mut pascal_case = self.to_pascal_case();
        let mut camel_case = pascal_case.to_mut();
        camel_case.make_ascii_lowercase();

        Cow::Owned(camel_case.to_string())
    }

    fn to_pascal_case(&self) -> Cow<'a, str> {
        let input: &str = self.as_ref();
        let words = input.split_whitespace();

        let pascal_case: String = words
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first_char) => {
                        first_char.to_uppercase().collect::<String>() + chars.as_str()
                    },
                }
            })
            .collect();

        Cow::Owned(pascal_case)
    }

    fn to_kebab_case(&self) -> Cow<'a, str> {
        let input: &str = self.as_ref();
        Cow::Owned(input.replace(" ", "-").to_lowercase())
    }

    fn to_human_readable(&self) -> Cow<'a, str> {
        let input: &str = self.as_ref();
        Cow::Owned(input.replace("_", " ").replace("-", " ").to_lowercase())
    }

    fn to_title_case(&self) -> Cow<'a, str> {
        let input = self.to_human_readable().into_owned();
        let title_cases: Vec<String> = input
            .split_whitespace()
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                }
            })
            .collect();

        Cow::Owned(title_cases.join(" "))
    }
}
