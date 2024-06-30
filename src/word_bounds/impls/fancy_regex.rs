use std::cmp::max;
use std::marker::PhantomData;

use fancy_regex::Regex as RE;

use crate::__str_ext__instance_words_vec;
use crate::resolver::impls::WordBoundResolverImpl;
use crate::resolver::rules::{DefaultRules, ResolverProcessingRule, ResolverRules, RuleTarget};
use crate::word_bounds::CompiledRules;

#[cfg(not(feature = "optimize_for_memory"))]
static mut REGEX: Option<RE> = None;

#[cfg(not(feature = "optimize_for_memory"))]
unsafe fn set_regex<R>()
where
    R: ResolverRules + 'static,
{
    let new_re = match FancyRegex::<R>::compile_rules() {
        CompiledRules::Regex(r) => RE::new(r.as_str()).expect("Expected valid fancy_regex pattern"),
        _ => panic!("Compiled rules were not a Regex"),
    };

    REGEX = Some(new_re);
}

pub struct FancyRegex<R: ResolverRules = DefaultRules> {
    _phantom_data: PhantomData<R>,
}

impl<R: ResolverRules> WordBoundResolverImpl<R> for FancyRegex<R>
where
    R: 'static,
{
    fn resolver(s: &str) -> Vec<String> {
        #[cfg(feature = "optimize_for_memory")]
        let re = RE::new(match FancyRegex::<R>::compile_rules() {
            CompiledRules::Regex(r) => &r,
            _ => panic!("Compiled rules were not a Regex"),
        })
        .expect("Expected valid fancy_regex pattern");

        __str_ext__instance_words_vec!(s, words);

        // Since split function is not available in fancy_regex
        // we do it manually using find_iter
        #[cfg(not(feature = "optimize_for_memory"))]
        let captures_iter = unsafe {
            if REGEX.is_none() {
                set_regex::<R>()
            }
            REGEX.as_ref().unwrap().find_iter(s)
        };
        #[cfg(feature = "optimize_for_memory")]
        let captures_iter = re.find_iter(s);

        let mut last = 0;
        for match_ in captures_iter {
            let cap = match_.expect("Unable to find capture");
            let start = cap.start();

            if start > last {
                let part = &s[last..max(0, start)];
                words.push(part.to_lowercase());
            }
            last = cap.end();
        }

        if last < s.len() {
            words.push(s[last..].to_lowercase());
        }

        words
    }

    fn compile_rules() -> CompiledRules {
        let mut pattern: Vec<&str> = vec![];
        let mut flag_case_change = false;
        let mut flag_punct = false;

        // Get punctuation characters from rules
        let punct_chars = R::punct_chars();

        // // Ensure punctuations are properly escaped for regex
        // let punct_chars = regex::escape(&punct_chars);

        // Create Regex string for punct_char boundaries
        let punct_char_pattern = format!(r"[{}]", punct_chars);

        for rule in R::resolution_pass_rules() {
            match rule {
                // Handling BoundStart and BoundEnd rules
                ResolverProcessingRule::BoundStart(target)
                | ResolverProcessingRule::BoundEnd(target) => match target {
                    RuleTarget::CaseChange(_) => {
                        if !flag_case_change {
                            pattern.push(r"(?<=\p{Ll})(?=\p{Lu})"); // lowercase to uppercase
                            pattern.push(r"(?<=\p{Lu})(?=\p{Lu}\p{Ll})"); // UPPERCASE to camelCase
                            flag_case_change = true;
                        }
                    },
                    RuleTarget::PunctSpecialChar => {
                        if !flag_punct {
                            pattern.push(&*punct_char_pattern); // treat punctuation defined in rules as boundaries
                            flag_punct = true;
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }

        // Join all the regex strings with |
        let compiled_pattern = pattern.join("|");

        // Return compiled_pattern as Regex with CompiledRules::Regex enum variant
        CompiledRules::Regex(compiled_pattern)
    }
}

#[cfg(test)]
mod tests {
    use crate::resolver::rules::Direction::{Auto, Next, Previous};
    use crate::resolver::rules::IncludeMode::{Attach, Dedicated};
    use crate::resolver::rules::RemoveMode::All;
    use crate::resolver::rules::ResolverProcessingRule::{BoundEnd, BoundStart, Include, Remove};
    use crate::resolver::rules::RuleTarget::{
        Acronym, CaseChange, DetectionArtifacts, NonPunctSpecialChar, Numerics, PunctSpecialChar,
        Word,
    };

    use super::*;

    pub struct _DefaultRules;

    impl ResolverRules for _DefaultRules {
        fn punct_chars() -> String {
            String::from("-_.,:;?!")
        }

        fn pre_pass_rules() -> Vec<ResolverProcessingRule> {
            vec![]
        }

        fn resolution_pass_rules() -> Vec<ResolverProcessingRule> {
            vec![
                Remove(DetectionArtifacts, All),
                Include(Word, Dedicated),
                Include(Acronym, Dedicated),
                Include(NonPunctSpecialChar, Attach(Auto)),
                Include(Numerics, Attach(Auto)),
                BoundStart(CaseChange(Next)),
                BoundEnd(CaseChange(Previous)),
                BoundStart(PunctSpecialChar),
                BoundEnd(PunctSpecialChar),
            ]
        }

        fn post_pass_rules() -> Vec<ResolverProcessingRule> {
            vec![]
        }
    }

    // Testing compile_rules function
    #[test]
    fn test_compile_rules() {
        // Result from compile_rules
        let result = match FancyRegex::<_DefaultRules>::compile_rules() {
            CompiledRules::Regex(r) => r,
            _ => panic!("Compiled rules were not a Regex"),
        };

        // Established pattern as per default rules, adjust as required
        let expected = r"(?<=\p{Ll})(?=\p{Lu})|(?<=\p{Lu})(?=\p{Lu}\p{Ll})|[-_.,:;?!]";

        // Test that the result from compile_rules matches the established pattern
        assert_eq!(
            result, expected,
            "Compiled pattern does not match expectations"
        );
    }
}
