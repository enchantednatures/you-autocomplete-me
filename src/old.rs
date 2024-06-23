use std::collections::HashMap;

use itertools::Itertools;

#[derive(Default)]
pub struct Traversal<'a> {
    matched_characters: Vec<char>,
    search: &'a [&'a char],
    search_options: SearchOptions,
    auto_completions: &'a [&'a str],
}

impl<'a> Traversal<'a> {
    fn with_search_options(mut self, search_options: SearchOptions) -> Self {
        self.search_options = search_options;
        self
    }

    fn matched_character(&mut self, c: char) {
        match self.search_options.fuzzy {
            true => {
                if let Some((idx, &next_matched_char)) =
                    self.search.iter().find_position(|&&x| x == &c)
                {}
            }
            false => {
                if let Some(&next_search_char) = self.search.first() {
                    if next_search_char == &c {
                        self.search = &self.search[1..];
                    } else {
                        return;
                    }
                }
            }
        }

        self.matched_characters.push(c);
    }
}

#[derive(Default)]
pub struct Match(Vec<u8>);

impl Match {
    fn new() -> Self {
        Self(Vec::new())
    }
}

pub struct SearchOptions {
    pub fuzzy: bool,
    pub levenshtein: u8,
    pub max_results: usize,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            fuzzy: true,
            levenshtein: 12,
            max_results: 10,
        }
    }
}

impl SearchOptions {
    fn new(fuzzy: bool, levenshtein: u8, max_results: usize) -> Self {
        Self {
            fuzzy,
            levenshtein,
            max_results,
        }
    }
}

#[derive(Debug)]
struct TrieNode {
    children: HashMap<u8, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn set_match(&mut self, word: String) {
        self.word = Some(word);
    }
}

#[derive(Debug, Default)]
pub struct Score {
    levenshtein: u8,
    fuzzy: bool,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: HashMap::with_capacity(26 * 2),
            word: None,
        }
    }
}

impl TrieNode {
    fn new(children: HashMap<u8, TrieNode>, word: Option<String>) -> Self {
        Self { children, word }
    }

    fn insert(&mut self, word: &[u8], value: &str) {
        if word.is_empty() {
            return;
        }

        let character = word[0];
        match word.len() {
            1 => self
                .children
                .entry(character)
                .or_default()
                .set_match(value.to_string()),
            _ => self
                .children
                .entry(character)
                .or_default()
                .insert(&word[1..], value),
        };
    }

    fn get_autocompletions(&self) -> impl IntoIterator<Item = &str> {
        let mut result = Vec::new();
        for node in self.children.values() {
            if let Some(word) = &node.word {
                result.push(word.as_str());
            }
            result.extend(node.get_autocompletions());
        }
        result
    }

    fn walk(&self, input: &[u8], idx: usize, m: &mut Vec<u8>) -> Vec<&str> {
        if input.is_empty() {
            return vec![];
        }

        if m.len() == input.len() && input.iter().all(|x| m.contains(x)) {
            return self.get_autocompletions().into_iter().collect();
        } else if input.len() <= idx {
            return vec![];
        }

        let mut matches = vec![];

        let character = input[idx];
        for (k, v) in self.children.iter() {
            if *k == character || k.to_ascii_lowercase() == character.to_ascii_lowercase() {
                dbg!(&k);
                dbg!(&m);
                m.push(character);
                matches.extend(v.walk(input, idx + 1, m));
            } else {
                matches.extend(v.walk(input, idx, m));
            }

            dbg!(&matches);
        }
        matches
    }
}

#[derive(Debug)]
pub struct AutocompletionEngine {
    children: HashMap<u8, TrieNode>,
    fuzzy: bool,
}
impl Default for AutocompletionEngine {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            fuzzy: true,
        }
    }
}

impl AutocompletionEngine {
    pub fn insert(&mut self, value: &str) {
        if value.is_empty() {
            return;
        }
        let word = &value.as_bytes();
        let character = word[0];
        match word.len() {
            1 => self
                .children
                .entry(character)
                .or_default()
                .set_match(value.to_string()),
            _ => self
                .children
                .entry(character)
                .or_default()
                .insert(&word[1..], value),
        };
    }
    pub fn search(&self, value: &str) -> Vec<&str> {
        if value.is_empty() {
            return vec![];
        }
        let word = value.as_bytes();
        let mut match_score = Vec::new();
        let character = word[0];

        match self.children.get(&character) {
            Some(c) => {
                match_score.push(character);
                return c
                    .children
                    .values()
                    .flat_map(|x| x.walk(word, 1, &mut match_score))
                    .collect();
            }
            None => {
                return self
                    .children
                    .values()
                    .flat_map(|x| x.walk(word, 0, &mut match_score))
                    .collect();
            }
        }
    }
}
