//! # YouAutoCompleteMe
//! 
//! A Fuzzy Finder library in Rust

#![allow(dead_code)]
#![warn(missing_docs)]

mod builder;
mod edit_distance;
mod r#match;
mod match_profile;
mod score_configuration;
pub mod search;
mod trie;

pub use self::builder::YouAutoCompleteMeBuilder;
pub use self::score_configuration::ScoreConfiguration;
pub use self::trie::TrieNode;

/// Matches phrases against the input and then scores them by relevancy
/// ```
/// use you_autocomplete_me::YouAutoCompleteMe;
/// use you_autocomplete_me::TrieNode;
/// let mut phrasebook = TrieNode::default();
/// phrasebook.insert("test");
/// phrasebook.insert("temporary");
/// let completer = YouAutoCompleteMe::builder(&phrasebook).build();
/// ```
#[derive(Debug)]
pub struct YouAutoCompleteMe<'a> {
    /// The configuration for the scoring algorithm
    score_configuration: ScoreConfiguration,
    /// Phrasebook
    phrase_book: &'a TrieNode,
}

impl<'a> YouAutoCompleteMe<'a> {
    /// create a new instance of the auto completer
    pub fn new(phrase_book: &'a TrieNode) -> Self {
        Self {
            phrase_book,
            score_configuration: Default::default(),
        }
    }

    /// Creates a [`YouAutoCompleteMeBuilder`] for configuring the auto completer
    /// warning: the phrasebook will go away in future builds
    pub fn builder(phrase_book: &'a TrieNode) -> YouAutoCompleteMeBuilder<'a> {
        YouAutoCompleteMeBuilder::new(phrase_book)
    }
}

/// A trait use to complete from an input
trait Completer {
    /// Completes a phrase
    fn complete(&self, input: &str) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_with_score_configuration() {
        let phrase_book: TrieNode = Default::default();

        let builder = YouAutoCompleteMe::builder(&phrase_book);
        builder.with_score_configuration(ScoreConfiguration::default());
    }
}
