use crate::score_configuration::ScoreConfiguration;
use crate::{TrieNode, YouAutoCompleteMe};

/// Builder for [YouAutoCompleteMe]
pub struct YouAutoCompleteMeBuilder<'a> {
    // match_configuration: Option<MatchConfiguration>,
    score_configuration: Option<ScoreConfiguration>,
    phrase_book: &'a TrieNode,
}

impl<'a> YouAutoCompleteMeBuilder<'a> {
    /// Create a new [YouAutoCompleteMeBuilder] with a Phrasebook
    ///warning: the TrieNode as a public interface is going away
    pub fn new(phrase_book: &'a TrieNode) -> Self {
        Self {
            // match_configuration: None,
            score_configuration: None,
            phrase_book,
        }
    }

    /// the Created [YouAutoCompleteMe] with the given [ScoreConfiguration]
    pub fn with_score_configuration(mut self, score_config: ScoreConfiguration) -> Self {
        self.score_configuration = Some(score_config);
        self
    }

    /// the Created [YouAutoCompleteMe] with an optional [ScoreConfiguration]
    pub fn with_optional_score_configuration(
        mut self,
        score_config: Option<ScoreConfiguration>,
    ) -> Self {
        self.score_configuration = score_config;
        self
    }

    /// Build the [YouAutoCompleteMe]
    pub fn build(self) -> YouAutoCompleteMe<'a> {
        YouAutoCompleteMe {
            // match_configuration: self.match_configuration.unwrap_or_default(),
            score_configuration: self.score_configuration.unwrap_or_default(),
            phrase_book: self.phrase_book,
        }
    }
}
