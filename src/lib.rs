#![allow(dead_code)]

mod edit_distance;
mod old;

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::Chars;

use itertools::Itertools;

#[derive(Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: HashMap::with_capacity(26),
        }
    }
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

impl TrieNode {
    pub fn insert(&mut self, value: &str) {
        if (value.is_empty()) {
            return;
        }

        self.m_insert(value.chars());
    }

    fn m_insert(&mut self, mut value: Chars<'_>) {
        match value.next() {
            Some(c) => self.entry(c).or_default().m_insert(value),
            None => return,
        }
    }

    pub fn search(&self, value: &str) -> Vec<&str> {
        todo!()
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
        dbg!(trie);
        panic!();

        // let expected = vec!["world"];
        // let actual = trie.search("wrd");
        // dbg!(&actual);
        // assert_equal(expected, actual)
    }
}
