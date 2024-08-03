use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::Chars;

use crate::match_profile::MatchProfile;
use crate::search::Search;

/// Represents a data type which can be searched
pub trait Searcher {
    type T<'a>;

    /// .
    fn search<'a>(&self, s: Search<'a>) -> Self::T<'a>;
}

impl Searcher for TrieNode {
    type T<'a> = Box<dyn Iterator<Item=MatchProfile<'a>>>;

    fn search<'a>(&self, _s: Search<'a>) -> Self::T<'a> {
        // _s.search
        todo!()
    }
}

/// Todo: Generic so I can bench Rc<str>
///
#[derive(Default, Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: HashSet<String>,
}

impl TrieNode {
    /// Not documenting as this will not be part of the public api
    pub fn insert(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }

        let lowercased = value.to_ascii_lowercase();

        // experiment with not doing this... its possible that the search algorithm could account for suffixes?
        for i in 0..=value.len() {
            self.m_insert(value[i..].chars(), value);
            if lowercased != value {
                self.m_insert(lowercased[i..].chars(), value)
            }
        }
    }

    fn m_insert(&mut self, mut value: Chars<'_>, word: &str) {
        match value.next() {
            Some(c) => self.children.entry(c).or_default().m_insert(value, word),
            None => {
                self.word.insert(word.into());
            }
        }
    }

    /// Not documenting due to changes to the api
    pub fn search(&self, value: &str) -> HashSet<&str> {
        match value.chars().any(|c| c.is_uppercase()) {
            false => self.search_case_insensitive(value.chars()),
            _ => todo!(),
        }
    }

    fn search_case_insensitive(&self, mut value: Chars<'_>) -> HashSet<&str> {
        match value.next() {
            Some(c) => match self.children.get(&c.to_ascii_lowercase()) {
                Some(node) => node.search_case_insensitive(value).drain().collect(),
                None => HashSet::new(),
            },
            None => HashSet::from_iter(self.collect()),
        }
    }

    fn collect(&self) -> Vec<&str> {
        self.children
            .values()
            .flat_map(|v| v.collect())
            .chain(self.word.iter().map(|x| x.as_str()))
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn get_all_autocompletions_with_mixed_cases() {
        let mut trie = TrieNode::default();
        let values = vec![
            "hello",
            "world",
            "help",
            "helium",
            "spark",
            "strange",
            "stranger",
            "World",
            "hello-world",
        ];
        for val in &values {
            trie.insert(val);
        }

        let expected = ["world", "World", "hello-world"];
        let actual = trie.search("wor");
        dbg!(&actual);
        assert_equal(expected.iter().sorted(), actual.iter().sorted())
    }

    #[test]
    fn get_all_autocompletions() {
        let mut trie = TrieNode::default();
        let values = vec![
            "hello", "world", "help", "helium", "spark", "strange", "stranger",
        ];
        for val in &values {
            trie.insert(val);
        }

        let expected = vec!["world"];
        let actual = trie.search("wor");
        dbg!(&actual);
        assert_equal(expected, actual)
    }

    // #[ignore = "WIP"]
    #[test]
    fn get_middle_completions() {
        let mut trie = TrieNode::default();
        let expected = vec![
            "This is a test!",
            "This is a test",
            "I don't think I'll pass the science test!",
            "I don't think I'll pass the science test",
            "It is important to test software",
            "testing, testing, testing",
        ];
        for val in &expected {
            trie.insert(val);
        }
        let actual = trie.search("test");
        dbg!(&actual);
        assert_equal(expected.iter().sorted(), actual.iter().sorted())
    }
}
