use crate::score_configuration::ScoreConfiguration;
use crate::{TrieNode, YouAutoCompleteMe};

pub trait WithOptionalScoreConfiguration<'a> {
    fn with_optional_score_configuration(self, match_config: Option<ScoreConfiguration>) -> Self;
}

// pub trait WithOptionalMatchConfiguration {
//     fn with_optional_match_configuration(self, match_config: Option<MatchConfiguration>) -> Self;
// }

pub trait WithScoreConfiguration<'a> {
    fn with_score_configuration(self, score_configuration: ScoreConfiguration) -> Self;
}

// pub trait WithMatchConfiguration {
//     fn with_match_configuration(self, match_config: MatchConfiguration) -> Self;
// }

pub struct YouAutoCompleteMeBuilder<'a> {
    // match_configuration: Option<MatchConfiguration>,
    score_configuration: Option<ScoreConfiguration>,
    phrase_book: &'a TrieNode,
}

impl<'a> YouAutoCompleteMeBuilder<'a> {
    pub fn new(phrase_book: &'a TrieNode) -> Self {
        Self {
            // match_configuration: None,
            score_configuration: None,
            phrase_book,
        }
    }
    pub fn build(self) -> YouAutoCompleteMe<'a> {
        YouAutoCompleteMe {
            // match_configuration: self.match_configuration.unwrap_or_default(),
            score_configuration: self.score_configuration.unwrap_or_default(),
            phrase_book: self.phrase_book,
        }
    }
}

// impl<'a> WithMatchConfiguration for YouAutoCompleteMeBuilder<'a> {
//     fn with_match_configuration(mut self, match_config: MatchConfiguration) -> Self {
//         self.match_configuration = Some(match_config);
//         self
//     }
// }

// impl<'a> WithOptionalMatchConfiguration for YouAutoCompleteMeBuilder<'a> {
//     fn with_optional_match_configuration(
//         mut self,
//         match_config: Option<MatchConfiguration>,
//     ) -> Self {
//         self.match_configuration = match_config;
//         self
//     }
// }

impl<'a> WithScoreConfiguration<'a> for YouAutoCompleteMeBuilder<'a> {
    fn with_score_configuration(mut self, score_config: ScoreConfiguration) -> Self {
        self.score_configuration = Some(score_config);
        self
    }
}

impl<'a> WithOptionalScoreConfiguration<'a> for YouAutoCompleteMeBuilder<'a> {
    fn with_optional_score_configuration(
        mut self,
        score_config: Option<ScoreConfiguration>,
    ) -> Self {
        self.score_configuration = score_config;
        self
    }
}
