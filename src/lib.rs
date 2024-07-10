#![allow(dead_code)]

mod edit_distance;
mod old;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

use itertools::Itertools;

pub trait ScoringAlgorithm {
    fn score(&self, search: &str, matching_characters: &str) -> usize;
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

#[derive(Default, Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

impl Deref for TrieNode {
    type Target = HashMap<char, TrieNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for TrieNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

enum Options {
    SearchOptions(SearchOptions),
    FuzzySearchOptions(FuzzySearchOptions),
}

pub struct FuzzySearchOptions {
    fuzzy: bool,
    levenstein: bool,
}

pub struct SearchOptions {
    fuzzy: bool,
    levenstein: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            levenstein: false,
            fuzzy: true,
        }
    }
}

impl TrieNode {
    pub fn insert(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }

        self.m_insert(value.chars(), value);
    }

    fn m_insert(&mut self, mut value: Chars<'_>, word: &str) {
        match value.next() {
            Some(c) => self.entry(c).or_default().m_insert(value, word),
            None => self.word = Some(word.into()),
        }
    }

    pub fn search(&self, value: &str) -> Vec<&str> {
        self.m_search(value.chars(), SearchOptions::default())
    }

    pub fn search_with_options(&self, value: &str, options: SearchOptions) -> Vec<&str> {
        self.m_search(value.chars(), options)
    }

    fn m_search(&self, mut value: Chars<'_>, options: SearchOptions) -> Vec<&str> {
        match value.next() {
            Some(c) => match self.get(&c) {
                Some(node) => node.m_search(value, options),
                None => {
                    // if options.fuzzy
                    // {}
                    vec![]
                }
            },
            None => self.collect(),
        }
    }
    fn collect(&self) -> Vec<&str> {
        self.children
            .values()
            .flat_map(|v| {
                let words = v.collect();
                if let Some(word) = &v.word {
                    return words.into_iter().chain([word.as_str()]).collect_vec();
                }
                words
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {

    use itertools::assert_equal;

    use crate::old::Score;

    use super::*;

    #[test]
    fn get_all_autocompletions() {
        let mut trie = TrieNode::default();
        let values = vec![
            "hello", "world", "help", "helium", "spark", "strange", "stranger",
        ];
        for val in &values {
            trie.insert(val);
        }
        // dbg!(trie);
        // panic!();

        let expected = vec!["world"];
        let actual = trie.search("wor");
        dbg!(&actual);
        assert_equal(expected, actual)
    }

    #[test]
    fn get_middle_completions() {
        let mut trie = TrieNode::default();
        let values = vec![
            "This is a test",
            "I don't think I'll pass the science test",
            "It is important to test software",
            "testing, testing, testing",
        ];
        for val in &values {
            trie.insert(val);
        }
        // dbg!(trie);
        // panic!();

        let expected: Vec<&str> = Vec::new();
        let actual = trie.search("test");
        dbg!(&actual);
        assert_equal(expected, actual)
    }

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
        let result = results
            .into_iter()
            .sort_and_filter(&SlightlyMoreSophisticatedScorer, 4);

        assert_equal(
            vec!["This is a test", "I don't think I'll pass the science test"],
            result,
        )
    }
}
