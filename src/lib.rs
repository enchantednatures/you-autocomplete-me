#![allow(dead_code)]
use std::ascii::AsciiExt;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub struct TrieNode<T> {
    children: HashMap<u8, TrieNode<T>>,
    word: Option<T>,
}

impl<T> TrieNode<T> {
    fn set_match(&mut self, word: T) {
        self.word = Some(word);
    }
}

#[derive(Debug, Default)]
pub struct Score {
    levenshtein: u8,
    fuzzy: bool,
}

impl<T> Default for TrieNode<T> {
    fn default() -> Self {
        Self {
            children: HashMap::with_capacity(26),
            word: None,
        }
    }
}

impl<T> TrieNode<T> {
    fn new(children: HashMap<u8, TrieNode<T>>, word: Option<T>) -> Self {
        Self { children, word }
    }

    fn insert(&mut self, word: &[u8], value: T) {
        if word.is_empty() {
            return;
        }

        let character = word[0];
        match word.len() {
            1 => self.children.entry(character).or_default().set_match(value),
            _ => self
                .children
                .entry(character)
                .or_default()
                .insert(&word[1..], value),
        };
    }
}

#[derive(Debug)]
pub struct Trie<T> {
    children: HashMap<u8, TrieNode<T>>,
    fuzzy: bool,
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            fuzzy: true,
        }
    }
}

impl<T> Trie<T> {
    pub fn insert(&mut self, word: &[u8], value: T) {
        if word.is_empty() {
            return;
        }

        let character = word[0];
        self.children
            .entry(character)
            .or_default()
            .insert(&word[1..], value);

        let _ = word.iter().enumerate().map(|(idx, c)| {
            print!("{}: {}", idx, c);
        });
    }

    pub fn search(&self, word: &str, limit: Option<usize>) -> Vec<String> {
        let mut result = Vec::new();
        if word.is_empty() {
            return result;
        }

        result
    }
}
impl TrieNode<String> {
    fn collect_all_descendent_words(
        &self,
        collection: &mut HashMap<String, Score>,
        distance: u8,
        fuzzy_used: bool,
    ) {
        for node in self.children.values() {
            if let Some(word) = &node.word {
                if let Some(previous_score) = collection.get_mut(word) {
                    if distance < previous_score.levenshtein
                        || (distance == previous_score.levenshtein
                            && previous_score.fuzzy
                            && !fuzzy_used)
                    {
                        previous_score.levenshtein = distance;
                        previous_score.fuzzy = fuzzy_used;
                    }
                } else {
                    collection.insert(
                        word.clone(),
                        Score {
                            levenshtein: distance,
                            fuzzy: fuzzy_used,
                        },
                    );
                }
            }
            node.collect_all_descendent_words(collection, distance, fuzzy_used);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::default();
        trie.insert("hello".as_bytes(), "hello");
        trie.insert("world".as_bytes(), "world");
        trie.insert("help".as_bytes(), "help");
        dbg!(trie);

        assert!(false)
    }
}
