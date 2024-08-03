use std::collections::HashSet;

static DELIMITERS: &str = " -/_";

/// Builder for [ScoreConfiguration]
#[derive(Debug, Default)]
pub struct ScoreConfigurationBuilder {
    word_delimiters: Option<HashSet<char>>,
    character_adjacency_bonus: Option<u8>,
    character_adjacency_multiplier: Option<u8>,
    max_character_adjacency_bonus: Option<u8>,
    word_boundary_bonus: Option<u8>,
    word_prefix_bonus: Option<u8>,
    word_suffix_bonus: Option<u8>,
    character_offset_penalty: Option<u8>,
    max_offset_penalty: Option<u8>,
}

impl ScoreConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set delimiters used to mark word boundaries in the search scoring. 
    pub fn with_word_delimiters(mut self, word_delimiters: HashSet<char>) -> Self {
        self.word_delimiters = Some(word_delimiters);
        self
    }

    /// Sets bonus to adjacent characters in match
    pub fn with_character_adjacency_bonus(mut self, character_adjacency_bonus: u8) -> Self {
        self.character_adjacency_bonus = Some(character_adjacency_bonus);
        self
    }

    /// Sets the multiplier for each additional sequential adjacent characters.
    /// todo: this would be a good candidate for a doctest
    pub fn with_character_adjacency_multiplier(
        mut self,
        character_adjacency_multiplier: u8,
    ) -> Self {
        self.character_adjacency_multiplier = Some(character_adjacency_multiplier);
        self
    }

    pub fn with_max_character_adjacency_bonus(mut self, max_character_adjacency_bonus: u8) -> Self {
        self.max_character_adjacency_bonus = Some(max_character_adjacency_bonus);
        self
    }

    pub fn with_word_boundary_bonus(mut self, word_boundary_bonus: u8) -> Self {
        self.word_boundary_bonus = Some(word_boundary_bonus);
        self
    }

    pub fn with_word_prefix_bonus(mut self, word_prefix_bonus: u8) -> Self {
        self.word_prefix_bonus = Some(word_prefix_bonus);
        self
    }

    pub fn with_word_suffix_bonus(mut self, word_suffix_bonus: u8) -> Self {
        self.word_suffix_bonus = Some(word_suffix_bonus);
        self
    }

    pub fn with_character_offset_penalty(mut self, character_offset_penalty: u8) -> Self {
        self.character_offset_penalty = Some(character_offset_penalty);
        self
    }

    pub fn with_max_offset_penalty(mut self, max_offset_penalty: u8) -> Self {
        self.max_offset_penalty = Some(max_offset_penalty);
        self
    }

    pub fn build(self) -> ScoreConfiguration {
        ScoreConfiguration {
            word_delimiters: self
                .word_delimiters
                .unwrap_or_else(|| DELIMITERS.chars().collect()),
            character_adjacency_bonus: self.character_adjacency_bonus.unwrap_or(1),
            character_adjacency_multiplier: self.character_adjacency_multiplier.unwrap_or(2),
            max_character_adjacency_bonus: self.max_character_adjacency_bonus.unwrap_or(6),
            word_boundary_bonus: self.word_boundary_bonus.unwrap_or(5),
            word_prefix_bonus: self.word_prefix_bonus.unwrap_or(3),
            word_suffix_bonus: self.word_suffix_bonus.unwrap_or(3),
            character_offset_penalty: self.character_offset_penalty.unwrap_or(1),
            max_offset_penalty: self.max_offset_penalty.unwrap_or(3),
        }
    }
}

/// Configuration for scoring the match to the input
#[derive(Debug)]
pub struct ScoreConfiguration {
    /// Characters which mark word boundaries
    word_delimiters: HashSet<char>,

    /// bonus given to adjacent characters
    character_adjacency_bonus: u8,

    /// multiplier given to subsquent adjacent matching
    character_adjacency_multiplier: u8,

    /// highest possible score given to a substring match
    max_character_adjacency_bonus: u8,

    /// bonus for the match being at the beginning of a word
    word_boundary_bonus: u8,

    /// additional bonus if the match is a prefix
    word_prefix_bonus: u8,

    /// additional bonus if the match is a word suffix
    word_suffix_bonus: u8,

    /// penalty if the match does not start at the begining
    character_offset_penalty: u8,

    /// max penalty for the input not matching the start of the phrase
    max_offset_penalty: u8,
}

impl ScoreConfiguration {
    /// create a new builder for score configuration
    pub fn builder() -> ScoreConfigurationBuilder {
        ScoreConfigurationBuilder::new()
    }
}

impl Default for ScoreConfiguration {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_with_word_delimiters_contains_only_word_delimiters() {
        const SPC: char = ' ';
        const UNDERSCORE: char = '_';
        let mut word_delimiters = HashSet::new();
        word_delimiters.insert(SPC);
        word_delimiters.insert(UNDERSCORE);
        let builder = ScoreConfiguration::builder().with_word_delimiters(word_delimiters);

        let config = builder.build();

        assert!(config.word_delimiters.contains(&SPC));
        assert!(config.word_delimiters.contains(&UNDERSCORE));
        dbg!(&config.word_delimiters);

        assert_eq!(config.word_delimiters.len(), 2);
    }

    #[test]
    fn builder_with_character_adjacency_bonus_is_correct() {
        const CHARACTER_ADJACENCY_BONUS: u8 = 7;
        let builder = ScoreConfiguration::builder();
        let config = builder
            .with_character_adjacency_bonus(CHARACTER_ADJACENCY_BONUS)
            .build();

        assert_eq!(config.character_adjacency_bonus, CHARACTER_ADJACENCY_BONUS);
    }
}
