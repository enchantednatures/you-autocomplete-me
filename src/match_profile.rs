use crate::search::Search;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MatchProfile<'a> {
    phrase: &'a str,
    longest_match: &'a str,
    matching_characters: &'a [char],
    is_smart_case: bool,
}

impl<'a> MatchProfile<'a> {
    pub fn new(
        phrase: &'a str,
        longest_match: &'a str,
        matching_characters: &'a [char],
        is_smart_case: bool,
    ) -> Self {
        Self {
            phrase,
            longest_match,
            matching_characters,
            is_smart_case,
        }
    }
}

impl<'a> From<(&'a Search<'a>, &'a str)> for MatchProfile<'a> {
    fn from(value: (&'a Search, &'a str)) -> Self {
        todo!()
    }
    // add code here
}
