#![allow(dead_code)]

mod builder;
mod edit_distance;
mod match_profile;
mod score_configuration;
mod search;
mod trie;

use std::collections::HashMap;

use itertools::Itertools;

use self::builder::YouAutoCompleteMeBuilder;
use self::match_profile::MatchProfile;
use self::score_configuration::ScoreConfiguration;
pub use self::trie::TrieNode;

type EditDistance = for<'a, 'b> fn(&'a str, &'b str) -> usize;

// trait Searcher {
//     fn search(&self, value: &Search) -> MatchProfile;
// }

/// Trait for getting scored matches
trait GetScoredMatches {
    /// Returns an iterator of matches
    fn get_scored_matches<'a, T>(&'a self, input: &str) -> T
    where
        T: Iterator<Item = &'a MatchProfile<'a>>;
}

/// Matches phrases against the input and then scores them by relevancy
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

    pub fn builder(phrase_book: &'a TrieNode) -> YouAutoCompleteMeBuilder<'a> {
        YouAutoCompleteMeBuilder::new(phrase_book)
    }
}

pub trait ScoringAlgorithm {
    fn score(&self, search: &str, matching_characters: &str) -> usize;
}

trait SortAndFilter2<T> {
    fn sort(&mut self, scorer: EditDistance) -> Vec<T>;
}

impl<'a, T: Iterator<Item = Match<'a>>> SortAndFilter2<&'a str> for T {
    fn sort(&mut self, scorer: EditDistance) -> Vec<&'a str> {
        let mut data = self.collect::<Vec<_>>();
        data.sort_by(|a, b| {
            scorer(
                &a.search,
                a.matching_characters.iter().collect::<String>().as_str(),
            )
            .cmp(&scorer(
                &b.search,
                b.matching_characters.iter().collect::<String>().as_str(),
            ))
        });
        data.into_iter().map(|v| v.result).collect()
    }
}

trait SortAndFilter<T, F> {
    fn sort_and_filter(&mut self, scorer: &F, threshold: usize) -> Vec<T>;
}

impl<'a, F, T: Iterator<Item = Match<'a>>> SortAndFilter<&'a str, F> for T
where
    F: ScoringAlgorithm,
{
    fn sort_and_filter(&mut self, scorer: &F, threshold: usize) -> Vec<&'a str> {
        let mut map = HashMap::new();
        // for item in self.iter() {
        while let Some(item) = self.next() {
            let score = item.score(scorer);
            if score >= threshold {
                map.insert(score, item.result);
            }
        }

        map.into_iter()
            .sorted_by(|(a, _), (b, _)| b.cmp(a))
            .map(|(_, v)| v)
            .collect()
    }
}

struct SlightlyMoreSophisticatedScorer;
impl ScoringAlgorithm for SlightlyMoreSophisticatedScorer {
    fn score(&self, search: &str, matching_characters: &str) -> usize {
        let mut score = 0;
        let mut search = search.chars();
        let mut matching_characters = matching_characters.chars();
        while let (Some(s), Some(m)) = (search.next(), matching_characters.next()) {
            if s == m {
                score += 1;
            }
        }
        score
    }

    // add code here
}
struct SimpleScorer;
impl ScoringAlgorithm for SimpleScorer {
    fn score(&self, search: &str, matching_characters: &str) -> usize {
        if search.is_empty() {
            return 0;
        }
        if matching_characters.is_empty() {
            return 0;
        }
        0
    }
}

pub struct Match<'a> {
    search: String,
    matching_characters: Vec<char>,
    result: &'a str,
}

impl<'a> Match<'a> {
    pub fn new(search: String, matching_characters: Vec<char>, result: &'a str) -> Self {
        Self {
            search,
            matching_characters,
            result,
        }
    }

    pub fn score<F>(&self, scorer: &F) -> usize
    where
        F: ScoringAlgorithm,
    {
        scorer.score(
            &self.search,
            self.matching_characters.iter().collect::<String>().as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use self::builder::WithScoreConfiguration;
    use self::edit_distance::levenshtein::levenshtein_distance;

    use super::*;

    #[test]
    fn test_match_scoring() {
        let result = Match::new(
            "test".to_string(),
            vec!['t', 'e', 's', 't'],
            "I don't think I'll pass the science test",
        );
        let scorer = SlightlyMoreSophisticatedScorer;

        let score = result.score(&scorer);

        assert_eq!(4, score);
        // assert!(score == 4.0);
    }

    #[ignore = "WIP"]
    #[test]
    fn sorting_scores() {
        let results = vec![
            Match::new(
                "test".to_string(),
                vec!['t', 'e', 's', 't'],
                "I don't think I'll pass the science test",
            ),
            Match::new(
                "test".to_string(),
                vec!['t', 'e', 's', 't'],
                "This is a test",
            ),
        ];

        // let mut iter = results.iter();
        let result = results.into_iter().sort(levenshtein_distance);

        assert_equal(
            vec!["This is a test", "I don't think I'll pass the science test"],
            result,
        )
    }

    #[test]
    fn builder_with_score_configuration() {
        let phrase_book: TrieNode = Default::default();

        let builder = YouAutoCompleteMe::builder(&phrase_book);
        builder.with_score_configuration(ScoreConfiguration::default());
    }
}
