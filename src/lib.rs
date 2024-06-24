#![allow(dead_code)]

mod edit_distance;
mod old;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

use itertools::Itertools;

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

struct FuzzySearchOptions {
    fuzzy: bool,
    levenstein: bool,
}

struct SearchOptions {
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

struct Match {}

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
}
