use crate::resolver::rules::Direction::{Auto, Next, Previous};
use crate::resolver::rules::IncludeMode::{Attach, Dedicated};
use crate::resolver::rules::RemoveMode::All;
use crate::resolver::rules::ResolverProcessingRule::{BoundEnd, BoundStart, Include, Remove};
use crate::resolver::rules::RuleTarget::{
    Acronym, CaseChange, DetectionArtifacts, NonPunctSpecialChar, Numerics, PunctSpecialChar, Word,
};

pub trait ResolverRules {
    /// The rules consider the chars in this string punctuation characters, delimiting words.
    ///
    /// This also means inversely, that non-punct chars are everything but these.
    fn punct_chars() -> String {
        String::from("-_.,:;?!")
    }
    /// These are operations and rules that are run on the entire input before we pass it on to the
    /// resolution step
    ///
    /// When pre_pass_rules is empty, no pre-process pass will be run.
    fn pre_pass_rules() -> Vec<ResolverProcessingRule> {
        Vec::new()
    }
    /// This is the ruleset passed on to the word bound resolving implementations
    ///
    /// When resolution_pass_rules is empty, the defaults will be used.
    fn resolution_pass_rules() -> Vec<ResolverProcessingRule>;
    /// These are operations and rules that are run on the resolved output of the resolution pass,
    /// which should be a fairly finished vector of properly bounded words.
    ///
    /// When post_pass_rules is empty, no post-process pass will be run.
    fn post_pass_rules() -> Vec<ResolverProcessingRule> {
        Vec::new()
    }
}

pub struct DefaultRules;

impl ResolverRules for DefaultRules {
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

pub enum Scope {
    SingleWord,
    FullInput, // i.e prepends only considered for first word, and appends for the last
}

pub enum RemoveMode {
    None,
    Prepended(Scope),
    Appended(Scope),
    PrependedOrAppended(Scope),
    All,
}

pub enum IncludeMode {
    Dedicated, // as its own word, separately
    Attach(Direction),
}

pub enum RuleTarget {
    Word,
    String(String),
    Char(char),
    Numerics,
    SpecialChars,
    DetectionArtifacts,
    Acronym,
    PunctSpecialChar,
    NonPunctSpecialChar,
    CaseChange(Direction),
}

pub enum ResolverProcessingRule {
    Remove(RuleTarget, RemoveMode),
    Include(RuleTarget, IncludeMode),
    BoundStart(RuleTarget),
    BoundEnd(RuleTarget),
}

pub enum Direction {
    Previous,
    Next,
    /// e.g hashtags go to next usually (i.e prefix), but
    /// percent symbol would go to previous (i.e postfix)
    ///
    /// For numerics, we'd consider if the next word starts with a case change,
    /// and if it does, it's probably meant to go to next, but if there's
    /// a case change prior, or no case, probably goes to previous
    Auto,
}
