#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;

#[derive(Debug)]
struct TrieNode<T> {
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

    fn full_text_search(&self, word: &[u8]) -> impl IntoIterator<Item = &T> {
        let mut result = Vec::new();
        if word.is_empty() {
            return result.into_iter();
        }
        let s = match String::from_utf8(word.to_vec()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        dbg!(&s);

        let character = word[0];

        dbg!(character as char);
        if let Some(node) = self.children.get(&character) {
            match &node.word {
                Some(value) => {
                    result.push(value);
                    result.extend(node.get_autocompletions());
                }
                None => result.extend(node.full_text_search(&word[1..])),
            }
        } else {
            for node in self.children.values() {
                result.extend(node.full_text_search(word));
            }
        }

        result.into_iter()
    }

    fn search(&self, word: &[u8]) -> impl IntoIterator<Item = &T> {
        let mut result = Vec::new();
        if word.is_empty() {
            return result;
        }

        let character = word[0];
        if let Some(node) = self.children.get(&character) {
            match word.len() {
                1 => {
                    if let Some(word) = &node.word {
                        result.push(word);
                    }
                    result.extend(node.get_autocompletions());
                }
                _ => result.extend(node.search(&word[1..])),
            }
        }
        result
    }

    fn get_autocompletions(&self) -> impl IntoIterator<Item = &T> {
        let mut result = Vec::new();
        for node in self.children.values() {
            if let Some(word) = &node.word {
                result.push(word);
            }
            result.extend(node.get_autocompletions());
        }
        result
    }
}

#[derive(Debug)]
pub struct AutocompletionEngine<T> {
    children: HashMap<u8, TrieNode<T>>,
    fuzzy: bool,
}
impl<T> Default for AutocompletionEngine<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            fuzzy: true,
        }
    }
}

impl<T> AutocompletionEngine<T>
where
    T: Clone,
{
    pub fn full_text_search(&self, word: &[u8], limit: Option<usize>) -> Vec<T> {
        if word.is_empty() {
            return Vec::new();
        }

        self.children
            .values()
            .flat_map(|node| node.full_text_search(word))
            .take(match limit {
                Some(limit) => limit,
                None => usize::MAX,
            })
            .cloned()
            .collect_vec()
    }
}

impl<T> AutocompletionEngine<T> {
    pub fn insert(&mut self, word: &[u8], value: T) {
        if word.is_empty() {
            return;
        }

        let word = word.to_ascii_lowercase();
        let character = word[0];
        self.children
            .entry(character)
            .or_default()
            .insert(&word[1..], value);

        let _ = word.iter().enumerate().map(|(idx, c)| {
            print!("{}: {}", idx, c);
        });
    }

    pub fn full_text_search_clone(&self, word: &[u8], limit: Option<usize>) -> Vec<&T> {
        if word.is_empty() {
            return Vec::new();
        }

        let word = word.to_ascii_lowercase();
        self.children
            .values()
            .flat_map(|node| node.full_text_search(word.as_slice()))
            .take(match limit {
                Some(limit) => limit,
                None => usize::MAX,
            })
            .collect_vec()
    }

    pub fn get_all(&self) -> Vec<&T> {
        let mut result = Vec::new();
        for node in self.children.values() {
            result.extend(node.get_autocompletions())
        }
        result
    }
}
impl<T> TrieNode<T>
where
    T: Eq + PartialEq + Hash,
{
    fn collect_all_descendent_words<'a>(
        &'a self,
        collection: &mut HashMap<&'a T, Score>,
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
                        word,
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
    use std::sync::Arc;

    use super::*;

    // #[test]
    // fn test_insert_and_search() {
    //     let mut trie = AutocompletionEngine::default();
    //     trie.insert("hello".as_bytes(), "hello");
    //     trie.insert("world".as_bytes(), "world");
    //     trie.insert("help".as_bytes(), "help");
    //     dbg!(trie);
    // }

    #[test]
    fn get_all_autocompletions() {
        let mut trie = AutocompletionEngine::default();
        let values = vec!["hello", "world", "help"];
        for val in &values {
            trie.insert(val.as_bytes(), val);
        }

        let result = trie.get_all();

        assert!(result.iter().all(|x| values.contains(x)))
    }

    #[test]
    fn test_full_text_search() {
        let mut trie = AutocompletionEngine::default();
        let values: Vec<Arc<str>> = vec![
            "hello".into(),
            "world".into(),
            "help".into(),
            "the".into(),
            "theodore".into(),
        ];
        for val in &values {
            trie.insert(val.as_bytes(), val);
        }

        let result = trie.full_text_search("he".as_bytes(), None);
        dbg!(&result);

        let expected: Vec<Arc<str>> = vec![
            "hello".into(),
            "help".into(),
            "the".into(),
            "theodore".into(),
        ];
        assert!(result.iter().all(|x| expected.contains(x)))
    }

    #[test]
    fn test_full_text_search_realistic() {
        let mut trie = AutocompletionEngine::default();
        let values: Vec<Arc<str>> = vec![
            "Trip To Cabo".into(),
            "San Diego".into(),
            "Slovenia 2023".into(),
            "Armenia 2024".into(),
            "Yellowstone, Wy".into(),
        ];
        for val in &values {
            trie.insert(val.as_bytes(), val.clone());
        }

        let actual = trie.full_text_search("SD".as_bytes(), None);
        dbg!(&actual);

        let expected: Vec<Arc<str>> = vec!["San Diego".into()];
        assert_eq!(expected, actual);
    }
}
